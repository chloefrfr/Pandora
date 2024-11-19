use chrono::Local;
use env_logger::{Builder, Target};
use log::{info, LevelFilter};
use std::env;
use std::io::Write;

pub fn init_logger() {
    let log_level = env::args()
        .find(|arg| arg.starts_with("--log-level="))
        .and_then(|arg| arg.split('=').nth(1).map(|s| s.to_string()))
        .unwrap_or_else(|| "Info".to_string());

    let log_level = match log_level.as_str() {
        "Debug" => LevelFilter::Debug,
        "Warn" => LevelFilter::Warn,
        "Error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    Builder::new()
        .parse_filters(&log_level.to_string())
        .target(Target::Stdout)
        .format(|buf, record| {
            let now = Local::now();
            let timestamp = now.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            let level = record.level();
            let message = record.args();
            let module = record.module_path().unwrap_or_default();

            let color = match level {
                log::Level::Error => "\x1b[31m", // Red for errors
                log::Level::Warn => "\x1b[33m",  // Yellow for warnings
                log::Level::Info => "\x1b[32m",  // Green for info
                log::Level::Debug => "\x1b[36m", // Cyan for debug
                _ => "\x1b[37m",                 // White for other levels
            };

            let module_color = "\x1b[90m"; // Gray color for all module paths

            let message_color = "\x1b[37m";

            write!(
                buf,
                "{} {}{} {}{}{} {}\n",
                timestamp,     // Timestamp (no color)
                color,         // Log level color
                level,         // Log level
                module_color,  // Module path color (gray)
                module,        // Module path
                message_color, // Message color (white)
                message        // Actual log message
            )?;
            Ok(())
        })
        .init();

    info!("Logger initialized with level: {}", log_level);
}
