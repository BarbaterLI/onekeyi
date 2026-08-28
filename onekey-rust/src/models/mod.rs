//! Onekey - Steam Depot Manifest Downloader
//! 
//! 本项目 fork 自 https://github.com/ikunshare/Onekey/
//! 为免费的旧版 onekey 进行现代化 Rust 改造

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Depot 仓库信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepotInfo {
    pub depot_id: String,
    pub decryption_key: String,
    pub manifest_ids: Vec<String>,
}

impl DepotInfo {
    pub fn new(depot_id: String, decryption_key: String) -> Self {
        Self {
            depot_id,
            decryption_key,
            manifest_ids: Vec::new(),
        }
    }
}

/// GitHub 仓库信息
#[derive(Debug, Clone)]
pub struct RepoInfo {
    pub name: String,
    pub last_update: DateTime<Utc>,
    pub sha: String,
}

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub github_token: String,
    pub custom_steam_path: String,
    pub debug_mode: bool,
    pub logging_files: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            github_token: String::new(),
            custom_steam_path: String::new(),
            debug_mode: false,
            logging_files: true,
        }
    }
}

/// 默认配置结构
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultConfig {
    pub Github_Personal_Token: String,
    pub Custom_Steam_Path: String,
    pub Debug_Mode: bool,
    pub Logging_Files: bool,
    pub Help: String,
}

impl Default for DefaultConfig {
    fn default() -> Self {
        Self {
            Github_Personal_Token: String::new(),
            Custom_Steam_Path: String::new(),
            Debug_Mode: false,
            Logging_Files: true,
            Help: "Github Personal Token 可在 GitHub 设置的 Developer settings 中生成".to_string(),
        }
    }
}
