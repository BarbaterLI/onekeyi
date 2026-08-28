//! 地区检测器

use anyhow::Result;
use serde_json::Value;

use crate::constants::REGION_CHECK_URL;
use crate::network::client::HttpClient;
use crate::logger::Logger;

/// 地区检测器
pub struct RegionDetector {
    client: HttpClient,
    logger: Logger,
}

impl RegionDetector {
    pub fn new(client: HttpClient, logger: Logger) -> Self {
        Self { client, logger }
    }

    /// 检查是否在中国大陆
    pub async fn check_cn(&self) -> Result<(bool, String)> {
        match self.client.get(REGION_CHECK_URL, None).await {
            Ok(response) => {
                match response.json::<Value>().await {
                    Ok(body) => {
                        let is_cn = body.get("flag")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(true);
                        let country = body.get("country")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Unknown")
                            .to_string();

                        if !is_cn {
                            self.logger.info(&format!(
                                "您在非中国大陆地区 ({}) 上使用了项目，已自动切换回 Github 官方下载 CDN",
                                country
                            ));
                        }

                        Ok((is_cn, country))
                    }
                    Err(e) => {
                        self.logger.warning("检查服务器位置失败，自动认为你在中国大陆");
                        Ok((true, "CN".to_string()))
                    }
                }
            }
            Err(_) => {
                self.logger.warning("检查服务器位置失败，自动认为你在中国大陆");
                Ok((true, "CN".to_string()))
            }
        }
    }
}
