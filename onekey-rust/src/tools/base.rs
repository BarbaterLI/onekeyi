//! 解锁工具基类

use std::path::PathBuf;
use anyhow::Result;

use crate::models::DepotInfo;

/// 解锁工具特征
#[async_trait::async_trait]
pub trait UnlockTool {
    fn new(steam_path: PathBuf) -> Self where Self: Sized;
    
    /// 设置解锁
    async fn setup(&self, depot_data: &[DepotInfo], app_id: &str) -> Result<bool>;
}
