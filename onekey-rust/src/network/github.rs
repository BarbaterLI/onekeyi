//! GitHub API 封装

use std::collections::HashMap;
use anyhow::{Context, Result};
use chrono::DateTime;
use serde_json::Value;

use crate::constants::{GITHUB_API_BASE, CN_CDN_LIST, GLOBAL_CDN_LIST};
use crate::models::RepoInfo;
use crate::network::client::HttpClient;
use crate::logger::Logger;

/// GitHub API 封装
pub struct GitHubAPI {
    client: HttpClient,
    headers: Option<Vec<(String, String)>>,
    logger: Logger,
    pub is_cn: bool,
}

impl Clone for GitHubAPI {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            headers: self.headers.clone(),
            logger: self.logger.clone(),
            is_cn: self.is_cn,
        }
    }
}

impl GitHubAPI {
    pub fn new(client: HttpClient, headers: Option<Vec<(String, String)>>, logger: Logger) -> Self {
        Self {
            client,
            headers,
            logger,
            is_cn: true,
        }
    }

    /// 检查 API 请求限制
    pub async fn check_rate_limit(&self) -> Result<()> {
        let url = format!("{}/rate_limit", GITHUB_API_BASE);
        
        match self.client.get(&url, self.headers.as_ref()).await {
            Ok(response) => {
                if response.status().is_success() {
                    let json: Value = response.json().await.context("解析响应失败")?;
                    
                    if let Some(rate) = json.get("rate").and_then(|v| v.as_object()) {
                        let remaining = rate.get("remaining")
                            .and_then(|v| v.as_i64())
                            .unwrap_or(0);
                        let reset_time = rate.get("reset")
                            .and_then(|v| v.as_i64())
                            .unwrap_or(0);

                        self.logger.info(&format!("剩余 Github API 请求次数：{}", remaining));
                        
                        if remaining == 0 {
                            let reset_formatted = DateTime::from_timestamp(reset_time, 0)
                                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                                .unwrap_or_else(|| "未知".to_string());
                            self.logger.warning(&format!(
                                "GitHub API 请求数已用尽，将在 {} 重置",
                                reset_formatted
                            ));
                        }
                    }
                } else {
                    self.logger.error("Github 请求数检查失败，网络错误");
                }
            }
            Err(e) => {
                self.logger.error(&format!("检查 Github API 请求数失败：{}", e));
            }
        }
        
        Ok(())
    }

    /// 获取最新的仓库信息
    pub async fn get_latest_repo_info(
        &self,
        repos: &[&str],
        app_id: &str,
    ) -> Result<Option<RepoInfo>> {
        let mut latest_date: Option<DateTime<chrono::Utc>> = None;
        let mut selected_repo: Option<&str> = None;
        let mut selected_sha: Option<String> = None;

        for repo in repos {
            let url = format!("{}/repos/{}/branches/{}", GITHUB_API_BASE, repo, app_id);
            
            match self.client.get(&url, self.headers.as_ref()).await {
                Ok(response) => {
                    if response.status().is_success() {
                        let json: Value = response.json().await.context("解析响应失败")?;
                        
                        if let Some(commit) = json.get("commit").and_then(|v| v.as_object()) {
                            if let Some(commit_obj) = commit.get("commit").and_then(|v| v.as_object()) {
                                if let Some(author) = commit_obj.get("author").and_then(|v| v.as_object()) {
                                    if let Some(date_str) = author.get("date").and_then(|v| v.as_str()) {
                                        if let Ok(date) = DateTime::parse_from_rfc3339(date_str) {
                                            if latest_date.is_none() || date > latest_date.unwrap() {
                                                latest_date = Some(date.with_timezone(&chrono::Utc));
                                                selected_repo = Some(repo);
                                                selected_sha = commit.get("sha")
                                                    .and_then(|v| v.as_str())
                                                    .map(String::from);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    self.logger.warning(&format!("检查仓库 {} 失败：{}", repo, e));
                }
            }
        }

        match (selected_repo, latest_date, selected_sha) {
            (Some(name), Some(last_update), Some(sha)) => {
                Ok(Some(RepoInfo {
                    name: name.to_string(),
                    last_update,
                    sha,
                }))
            }
            _ => Ok(None),
        }
    }

    /// 获取文件内容
    pub async fn fetch_file(&self, repo: &str, sha: &str, path: &str) -> Result<Vec<u8>> {
        let cdn_list = if self.is_cn { CN_CDN_LIST } else { GLOBAL_CDN_LIST };

        // 尝试每个 CDN 最多 3 次
        for _attempt in 0..3 {
            for cdn_template in cdn_list {
                let url = cdn_template
                    .replace("{repo}", repo)
                    .replace("{sha}", sha)
                    .replace("{path}", path);

                match self.client.get_bytes(&url, self.headers.as_ref()).await {
                    Ok(content) => return Ok(content),
                    Err(e) => {
                        self.logger.debug(&format!("从 {} 下载失败：{}", url, e));
                    }
                }
            }
        }

        anyhow::bail!("无法下载文件：{}", path)
    }
}
