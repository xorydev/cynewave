use std::io::{Result, Write};
use std::fs::OpenOptions;


pub enum LogLevel {
    Error,
    Warning,
    Info
}

pub fn log(logfile: String, text: String, level: LogLevel) -> Result<()> {
    let prefix = match level {
        LogLevel::Error => "E".to_string(),
        LogLevel::Warning => "W".to_string(),
        LogLevel::Info => "I".to_string()
    };
    let msg = format!("{prefix}: {text}");
    println!("{msg}");
    let mut logfile = OpenOptions::new()
        .read(false)
        .write(false)
        .append(true)
        .open(logfile)?;
    logfile.write(msg.as_bytes())?;
    logfile.write(b"
")?;
    Ok(())
}

