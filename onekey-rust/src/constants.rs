//! 常量定义

use std::path::PathBuf;

pub const APP_NAME: &str = "Onekey";
pub const VERSION: &str = "2.0.0";
pub const AUTHOR: &str = "ikun0014";
pub const WEBSITE: &str = "ikunshare.top";

pub const BANNER: &str = r#"
    _____   __   _   _____   _   _    _____  __    __
   /  _  \ |  \ | | | ____| | | / /  | ____| \ \  / /
   | | | | |   \| | | |__   | |/ /   | |__    \ \/ /
   | | | | | |\   | |  __|  | |\ \   |  __|    \  /
   | |_| | | | \  | | |___  | | \ \  | |___    / /
   \_____/ |_|  \_| |_____| |_|  \_\ |_____|  /_/
"#;

/// 清单仓库列表
pub const REPO_LIST: &[&str] = &[
    "SteamAutoCracks/ManifestHub",
    "ikun0014/ManifestHub",
    "Auiowu/ManifestAutoUpdate",
    "tymolu233/ManifestAutoUpdate-fix",
];

/// 中国大陆 CDN 列表
pub const CN_CDN_LIST: &[&str] = &[
    "https://cdn.jsdmirror.com/gh/{repo}@{sha}/{path}",
    "https://raw.gitmirror.com/{repo}/{sha}/{path}",
    "https://raw.dgithub.xyz/{repo}/{sha}/{path}",
    "https://gh.akass.cn/{repo}/{sha}/{path}",
];

/// 全球 CDN 列表
pub const GLOBAL_CDN_LIST: &[&str] = &["https://raw.githubusercontent.com/{repo}/{sha}/{path}"];

pub const GITHUB_API_BASE: &str = "https://api.github.com";
pub const REGION_CHECK_URL: &str = "https://mips.kugou.com/check/iscn?&format=json";

/// 获取日志目录
pub fn get_log_dir() -> PathBuf {
    let mut path = std::env::current_dir().unwrap_or_default();
    path.push("logs");
    path
}

/// 获取配置文件路径
pub fn get_config_file() -> PathBuf {
    std::env::current_dir().unwrap_or_default().join("config.json")
}
