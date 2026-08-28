//! SteamTools 解锁工具实现

use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};

use crate::models::DepotInfo;
use crate::tools::base::UnlockTool;

/// SteamTools 解锁工具
pub struct SteamTools {
    steam_path: PathBuf,
}

impl SteamTools {
    pub fn new(steam_path: PathBuf) -> Self {
        Self { steam_path }
    }

    /// 设置 SteamTools 解锁
    pub async fn setup_with_options(
        &self,
        depot_data: &[DepotInfo],
        app_id: &str,
        depot_map: Option<&std::collections::HashMap<String, Vec<String>>>,
        version_lock: bool,
    ) -> Result<bool> {
        let st_path = self.steam_path.join("config").join("stplug-in");
        fs::create_dir_all(&st_path).context("创建 SteamTools 目录失败")?;

        let mut lua_content = format!("addappid({}, 1, \"None\")\n", app_id);

        for depot in depot_data {
            if version_lock && depot_map.is_some() {
                let map = depot_map.unwrap();
                if let Some(manifest_ids) = map.get(&depot.depot_id) {
                    for manifest_id in manifest_ids {
                        lua_content.push_str(&format!(
                            "addappid({}, 1, \"{}\")\n",
                            depot.depot_id, depot.decryption_key
                        ));
                        lua_content.push_str(&format!("setManifestid({},\"{}\")\n", depot.depot_id, manifest_id));
                    }
                    continue;
                }
            }
            
            lua_content.push_str(&format!(
                "addappid({}, 1, \"{}\")\n",
                depot.depot_id, depot.decryption_key
            ));
        }

        let lua_file = st_path.join(format!("{}.lua", app_id));
        fs::write(&lua_file, lua_content).context("写入 Lua 文件失败")?;

        Ok(true)
    }
}

#[async_trait::async_trait]
impl UnlockTool for SteamTools {
    fn new(steam_path: PathBuf) -> Self {
        Self { steam_path }
    }

    async fn setup(&self, depot_data: &[DepotInfo], app_id: &str) -> Result<bool> {
        self.setup_with_options(depot_data, app_id, None, false).await
    }
}
