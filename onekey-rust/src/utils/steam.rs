//! Steam 工具模块

use std::collections::HashMap;
use anyhow::{Context, Result};
use vdf;

use crate::models::DepotInfo;

/// 解析密钥文件 (VDF 格式)
pub fn parse_key_file(content: &[u8]) -> Vec<DepotInfo> {
    match std::str::from_utf8(content) {
        Ok(text) => {
            match vdf::from_str::<vdf::Value>(text) {
                Ok(vdf::Value::Object(obj)) => {
                    if let Some(vdf::Value::Object(depots)) = obj.get("depots") {
                        return depots
                            .iter()
                            .filter_map(|(depot_id, depot_info)| {
                                if let vdf::Value::Object(info_obj) = depot_info {
                                    if let Some(vdf::Value::String(key)) = info_obj.get("DecryptionKey") {
                                        return Some(DepotInfo::new(
                                            depot_id.clone(),
                                            key.clone(),
                                        ));
                                    }
                                }
                                None
                            })
                            .collect();
                    }
                }
                _ => {}
            }
        }
        Err(_) => {}
    }
    
    Vec::new()
}

/// 解析清单文件名
pub fn parse_manifest_filename(filename: &str) -> (Option<String>, Option<String>) {
    if !filename.ends_with(".manifest") {
        return (None, None);
    }

    let name = filename.trim_end_matches(".manifest");
    
    if let Some(pos) = name.find('_') {
        let depot_id = &name[..pos];
        let manifest_id = &name[pos + 1..];
        
        if depot_id.chars().all(|c| c.is_ascii_digit()) 
            && manifest_id.chars().all(|c| c.is_ascii_digit()) {
            return (Some(depot_id.to_string()), Some(manifest_id.to_string()));
        }
    }
    
    (None, None)
}
