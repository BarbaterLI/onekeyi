//! 日志模块

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use chrono::Local;
use colored::Colorize;

use crate::constants::get_log_dir;

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

/// 日志器
#[derive(Clone)]
pub struct Logger {
    name: String,
    level: LogLevel,
    log_file: bool,
    log_dir: PathBuf,
}

impl Logger {
    pub fn new(name: &str, debug_mode: bool, log_file: bool) -> Self {
        let level = if debug_mode { LogLevel::Debug } else { LogLevel::Info };
        let log_dir = get_log_dir();
        
        // 创建日志目录
        if log_file && !log_dir.exists() {
            let _ = fs::create_dir_all(&log_dir);
        }

        Self {
            name: name.to_string(),
            level,
            log_file,
            log_dir,
        }
    }

    fn format_message(&self, level: LogLevel, msg: &str) -> String {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        format!("[{}] [{}:{}] - {}", timestamp, self.name, format!("{:?}", level), msg)
    }

    fn write_to_file(&self, msg: &str) {
        if !self.log_file {
            return;
        }

        let logfile = self.log_dir.join(format!("{}.log", self.name));
        if let Ok(mut file) = File::options().create(true).append(true).open(&logfile) {
            let _ = writeln!(file, "{}", msg);
        }
    }

    pub fn debug(&self, msg: &str) {
        if self.level <= LogLevel::Debug {
            let formatted = self.format_message(LogLevel::Debug, msg);
            println!("{}", formatted.cyan());
            self.write_to_file(&formatted);
        }
    }

    pub fn info(&self, msg: &str) {
        if self.level <= LogLevel::Info {
            let formatted = self.format_message(LogLevel::Info, msg);
            println!("{}", formatted.green());
            self.write_to_file(&formatted);
        }
    }

    pub fn warning(&self, msg: &str) {
        if self.level <= LogLevel::Warning {
            let formatted = self.format_message(LogLevel::Warning, msg);
            println!("{}", formatted.yellow());
            self.write_to_file(&formatted);
        }
    }

    pub fn error(&self, msg: &str) {
        let formatted = self.format_message(LogLevel::Error, msg);
        eprintln!("{}", formatted.red());
        self.write_to_file(&formatted);
    }
}
