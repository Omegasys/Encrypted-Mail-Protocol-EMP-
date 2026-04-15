use std::time::{SystemTime, UNIX_EPOCH};

pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

pub struct Logger;

impl Logger {
    pub fn log(level: LogLevel, message: &str) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let level_str = match level {
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Debug => "DEBUG",
        };

        println!("[{}][{}] {}", timestamp, level_str, message);
    }
}
