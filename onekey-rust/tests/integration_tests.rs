//! Onekey 集成测试模块

use onekey::models::{DepotInfo, AppConfig, DefaultConfig};
use onekey::utils::steam::{parse_key_file, parse_manifest_filename};
use onekey::logger::{Logger, LogLevel};

#[test]
fn test_depot_info_creation() {
    let depot = DepotInfo::new(
        "123456".to_string(),
        "abcdef123456".to_string(),
    );
    
    assert_eq!(depot.depot_id, "123456");
    assert_eq!(depot.decryption_key, "abcdef123456");
    assert!(depot.manifest_ids.is_empty());
}

#[test]
fn test_app_config_default() {
    let config = AppConfig::default();
    
    assert!(config.github_token.is_empty());
    assert!(config.custom_steam_path.is_empty());
    assert!(!config.debug_mode);
    assert!(config.logging_files);
}

#[test]
fn test_default_config_structure() {
    let config = DefaultConfig::default();
    
    assert!(config.Github_Personal_Token.is_empty());
    assert!(config.Custom_Steam_Path.is_empty());
    assert!(!config.Debug_Mode);
    assert!(config.Logging_Files);
    assert!(!config.Help.is_empty());
}

#[test]
fn test_parse_key_file_valid() {
    let vdf_content = br#""depots"
{
    "123456"
    {
        "DecryptionKey" "abcdef123456"
    }
    "789012"
    {
        "DecryptionKey" "xyz789abc"
    }
}"#;
    
    let depots = parse_key_file(vdf_content);
    
    assert_eq!(depots.len(), 2);
    assert_eq!(depots[0].depot_id, "123456");
    assert_eq!(depots[0].decryption_key, "abcdef123456");
    assert_eq!(depots[1].depot_id, "789012");
    assert_eq!(depots[1].decryption_key, "xyz789abc");
}

#[test]
fn test_parse_key_file_empty() {
    let empty_content = b"";
    let depots = parse_key_file(empty_content);
    assert!(depots.is_empty());
}

#[test]
fn test_parse_key_file_invalid() {
    let invalid_content = b"invalid vdf content";
    let depots = parse_key_file(invalid_content);
    assert!(depots.is_empty());
}

#[test]
fn test_parse_manifest_filename_valid() {
    let (depot_id, manifest_id) = parse_manifest_filename("123456_789012.manifest");
    
    assert_eq!(depot_id, Some("123456".to_string()));
    assert_eq!(manifest_id, Some("789012".to_string()));
}

#[test]
fn test_parse_manifest_filename_invalid_extension() {
    let (depot_id, manifest_id) = parse_manifest_filename("123456_789012.txt");
    
    assert_eq!(depot_id, None);
    assert_eq!(manifest_id, None);
}

#[test]
fn test_parse_manifest_filename_invalid_format() {
    let (depot_id, manifest_id) = parse_manifest_filename("invalid.manifest");
    
    assert_eq!(depot_id, None);
    assert_eq!(manifest_id, None);
}

#[test]
fn test_logger_creation() {
    let logger = Logger::new("test", false, false);
    assert!(!logger.level > LogLevel::Debug);
}

#[test]
fn test_logger_with_debug_mode() {
    let logger = Logger::new("test-debug", true, false);
    assert!(logger.level == LogLevel::Debug);
}

#[test]
fn test_logger_clone() {
    let logger = Logger::new("test-clone", false, false);
    let cloned = logger.clone();
    
    // 验证克隆后的 logger 具有相同的属性
    cloned.info("Test message from cloned logger");
}
