//! Onekey - Steam Depot Manifest Downloader
//! 
//! 本项目 fork 自 https://github.com/ikunshare/Onekey/
//! 为免费的旧版 onekey 进行现代化 Rust 改造
//! 
//! CLI 版本入口

use anyhow::Result;
use colored::Colorize;

mod config;
mod constants;
mod logger;
mod models;
mod network;
mod tools;
mod utils;

use config::ConfigManager;
use constants::{APP_NAME, VERSION, BANNER};
use logger::Logger;
use network::client::HttpClient;
use network::github::GitHubAPI;
use utils::region::RegionDetector;
use utils::steam::{parse_key_file, parse_manifest_filename};
use tools::{SteamTools, GreenLuma, UnlockTool};
use models::DepotInfo;

#[tokio::main]
async fn main() -> Result<()> {
    // 显示横幅
    println!("{}", BANNER.cyan());
    println!("{} v{} - CLI Version\n", APP_NAME, VERSION);
    
    // 初始化配置
    let config = ConfigManager::new()?;
    let logger = Logger::new(
        "onekey",
        config.app_config.debug_mode,
        config.app_config.logging_files,
    );
    
    logger.info("Onekey 启动中...");
    
    // 检测地区
    let http_client = HttpClient::new()?;
    let region_detector = RegionDetector::new(http_client.clone(), logger.clone());
    let (is_cn, country) = region_detector.check_cn().await?;
    logger.info(&format!("当前地区：{}", country));
    
    // 初始化 GitHub API
    let headers = config.get_github_headers();
    let mut github_api = GitHubAPI::new(http_client.clone(), headers.clone(), logger.clone());
    github_api.is_cn = is_cn;
    
    // 检查 API 限制
    github_api.check_rate_limit().await?;
    
    // 获取 App ID
    println!("\n{}", "请输入要解锁的游戏 App ID:".green());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let app_id = input.trim();
    
    if app_id.is_empty() {
        logger.error("App ID 不能为空");
        return Ok(());
    }
    
    logger.info(&format!("正在获取 App ID {} 的清单数据...", app_id));
    
    // 获取仓库信息
    let repo_info = github_api.get_latest_repo_info(&constants::REPO_LIST, app_id).await?;
    
    match repo_info {
        Some(info) => {
            logger.info(&format!("找到最新仓库：{} (更新时间：{})", 
                info.name, info.last_update.format("%Y-%m-%d %H:%M:%S")));
            
            // 下载密钥文件
            let key_path = format!("keys/{}.txt", app_id);
            let key_content = github_api.fetch_file(&info.name, &info.sha, &key_path).await?;
            let depot_data = parse_key_file(&key_content);
            
            if depot_data.is_empty() {
                logger.warning("未找到任何 Depot 信息");
                return Ok(());
            }
            
            logger.info(&format!("找到 {} 个 Depot", depot_data.len()));
            
            // 下载清单文件
            let mut manifest_map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
            for depot in &depot_data {
                let manifest_path = format!("manifests/{}", app_id);
                match github_api.fetch_file(&info.name, &info.sha, &manifest_path).await {
                    Ok(content) => {
                        if let Ok(text) = std::str::from_utf8(&content) {
                            for line in text.lines() {
                                let filename = line.trim();
                                if filename.ends_with(".manifest") {
                                    let (depot_id, manifest_id) = parse_manifest_filename(filename);
                                    if let Some(did) = depot_id {
                                        if did == depot.depot_id {
                                            if let Some(mid) = manifest_id {
                                                manifest_map.entry(did).or_insert_with(Vec::new).push(mid);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        logger.debug(&format!("下载清单文件失败：{}", e));
                    }
                }
            }
            
            // 选择解锁工具
            println!("\n{}", "请选择解锁工具:".green());
            println!("1. SteamTools");
            println!("2. GreenLuma");
            
            let mut tool_choice = String::new();
            std::io::stdin().read_line(&mut tool_choice)?;
            
            let steam_path = config.steam_path.clone().expect("未找到 Steam 路径");
            let success = match tool_choice.trim() {
                "2" => {
                    let greenluma = GreenLuma::new(steam_path);
                    greenluma.setup(&depot_data, app_id).await?
                },
                _ => {
                    let steamtools = SteamTools::new(steam_path);
                    if !manifest_map.is_empty() {
                        // 使用版本锁定
                        steamtools.setup_with_options(&depot_data, app_id, Some(&manifest_map), true).await?
                    } else {
                        steamtools.setup(&depot_data, app_id).await?
                    }
                }
            };
            
            if success {
                logger.info("解锁配置成功！请重启 Steam 以应用更改。");
            } else {
                logger.error("解锁配置失败");
            }
        }
        None => {
            logger.error("未找到该 App ID 的清单数据");
        }
    }
    
    println!("\n按任意键退出...");
    let _ = console::Term::stdout().read_char();
    
    Ok(())
}
