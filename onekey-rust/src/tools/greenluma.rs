//! GreenLuma 解锁工具实现

use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};
use vdf;

use crate::models::DepotInfo;
use crate::tools::base::UnlockTool;

/// GreenLuma 解锁工具
pub struct GreenLuma {
    steam_path: PathBuf,
}

impl GreenLuma {
    pub fn new(steam_path: PathBuf) -> Self {
        Self { steam_path }
    }

    /// 设置 GreenLuma 解锁
    pub async fn setup(&self, depot_data: &[DepotInfo], _app_id: &str) -> Result<bool> {
        let applist_dir = self.steam_path.join("AppList");
        fs::create_dir_all(&applist_dir).context("创建 AppList 目录失败")?;

        // 清理旧的 txt 文件
        if let Ok(entries) = fs::read_dir(&applist_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "txt") {
                    let _ = fs::remove_file(&path);
                }
            }
        }

        // 写入新的 depot ID 文件
        for (idx, depot) in depot_data.iter().enumerate() {
            let file_path = applist_dir.join(format!("{}.txt", idx + 1));
            fs::write(&file_path, &depot.depot_id)
                .context(format!("写入 depot 文件失败：{}", file_path.display()))?;
        }

        // 更新 config.vdf
        let config_path = self.steam_path.join("config").join("config.vdf");
        
        if !config_path.exists() {
            // 如果 config.vdf 不存在，创建一个新的
            let mut root = vdf::Map::new();
            let mut depots = vdf::Map::new();
            
            for depot in depot_data {
                let mut depot_info = vdf::Map::new();
                depot_info.insert("DecryptionKey".to_string(), vdf::Value::String(depot.decryption_key.clone()));
                depots.insert(depot.depot_id.clone(), vdf::Value::Object(depot_info));
            }
            
            root.insert("depots".to_string(), vdf::Value::Object(depots));
            
            let content = vdf::to_string(&vdf::Value::Object(root))
                .context("序列化 VDF 失败")?;
            fs::write(&config_path, content).context("写入 config.vdf 失败")?;
        } else {
            // 读取并更新现有的 config.vdf
            let content = fs::read_to_string(&config_path)
                .context("读取 config.vdf 失败")?;
            
            match vdf::from_str::<vdf::Value>(&content) {
                Ok(mut vdf::Value::Object(root)) => {
                    let depots_entry = root.entry("depots".to_string())
                        .or_insert_with(|| vdf::Value::Object(vdf::Map::new()));
                    
                    if let vdf::Value::Object(ref mut depots) = depots_entry {
                        for depot in depot_data {
                            let mut depot_info = vdf::Map::new();
                            depot_info.insert("DecryptionKey".to_string(), vdf::Value::String(depot.decryption_key.clone()));
                            depots.insert(depot.depot_id.clone(), vdf::Value::Object(depot_info));
                        }
                    }
                    
                    let new_content = vdf::to_string(&vdf::Value::Object(root))
                        .context("序列化 VDF 失败")?;
                    fs::write(&config_path, new_content).context("写入 config.vdf 失败")?;
                }
                _ => return Err(anyhow::anyhow!("解析 config.vdf 失败")),
            }
        }

        Ok(true)
    }
}

#[async_trait::async_trait]
impl UnlockTool for GreenLuma {
    fn new(steam_path: PathBuf) -> Self {
        Self { steam_path }
    }

    async fn setup(&self, depot_data: &[DepotInfo], app_id: &str) -> Result<bool> {
        self.setup(depot_data, app_id).await
    }
}
