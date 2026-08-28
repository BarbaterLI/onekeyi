//! 配置管理模块

use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use serde_json;

#[cfg(windows)]
use winreg::enums::*;
#[cfg(windows)]
use winreg::RegKey;

use crate::constants::get_config_file;
use crate::models::{AppConfig, DefaultConfig};

/// 配置管理器
pub struct ConfigManager {
    config_path: PathBuf,
    pub app_config: AppConfig,
    pub steam_path: Option<PathBuf>,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let config_path = get_config_file();
        let mut manager = Self {
            config_path,
            app_config: AppConfig::default(),
            steam_path: None,
        };
        manager.load_config()?;
        Ok(manager)
    }

    /// 生成默认配置文件
    fn generate_config(&self) -> Result<()> {
        let default_config = DefaultConfig::default();
        let content = serde_json::to_string_pretty(&default_config)
            .context("序列化配置失败")?;
        
        fs::write(&self.config_path, content)
            .context("写入配置文件失败")?;
        
        println!("配置文件已生成：{}", self.config_path.display());
        Ok(())
    }

    /// 加载配置文件
    fn load_config(&mut self) -> Result<()> {
        if !self.config_path.exists() {
            self.generate_config()?;
            println!("\n请填写配置文件后重新运行程序");
            println!("按任意键退出...");
            let _ = console::Term::stdout().read_char();
            std::process::exit(1);
        }

        let content = fs::read_to_string(&self.config_path)
            .context("读取配置文件失败")?;
        
        let config_data: DefaultConfig = serde_json::from_str(&content)
            .context("解析配置文件失败")?;

        self.app_config = AppConfig {
            github_token: config_data.Github_Personal_Token,
            custom_steam_path: config_data.Custom_Steam_Path,
            debug_mode: config_data.Debug_Mode,
            logging_files: config_data.Logging_Files,
        };

        self.steam_path = self.get_steam_path();
        Ok(())
    }

    /// 获取 Steam 安装路径
    #[cfg(windows)]
    fn get_steam_path(&self) -> Option<PathBuf> {
        // 优先使用自定义路径
        if !self.app_config.custom_steam_path.is_empty() {
            let path = PathBuf::from(&self.app_config.custom_steam_path);
            if path.exists() {
                return Some(path);
            }
        }

        // 从注册表获取 Steam 路径
        match RegKey::predef(HKEY_CURRENT_USER)
            .open_subkey("Software\\Valve\\Steam")
        {
            Ok(key) => {
                match key.get_value::<String, _>("SteamPath") {
                    Ok(path_str) => {
                        let path = PathBuf::from(path_str);
                        if path.exists() {
                            return Some(path);
                        }
                    }
                    Err(e) => eprintln!("读取注册表 SteamPath 失败：{}", e),
                }
            }
            Err(e) => eprintln!("打开注册表键失败：{}", e),
        }

        None
    }

    /// 获取 Steam 安装路径 (非 Windows 平台返回 None)
    #[cfg(not(windows))]
    fn get_steam_path(&self) -> Option<PathBuf> {
        // 仅支持自定义路径
        if !self.app_config.custom_steam_path.is_empty() {
            let path = PathBuf::from(&self.app_config.custom_steam_path);
            if path.exists() {
                return Some(path);
            }
        }
        eprintln!("警告：非 Windows 平台，请手动配置 Custom_Steam_Path");
        None
    }

    /// 获取 GitHub 请求头
    pub fn get_github_headers(&self) -> Option<Vec<(String, String)>> {
        if self.app_config.github_token.is_empty() {
            return None;
        }
        Some(vec![
            ("Authorization".to_string(), format!("Bearer {}", self.app_config.github_token)),
        ])
    }
}
