//! 工具模块

pub mod region;
pub mod steam;

pub use region::RegionDetector;
pub use steam::{parse_key_file, parse_manifest_filename};
