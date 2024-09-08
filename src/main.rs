use cynewave::{Config, stage1};
mod logger;
use logger::{log, LogLevel};
use std::fs;

fn main() {
    let config_file: String = fs::read_to_string("./testconfig.toml").expect("Failed to read config");
    let config: Config = Config::new(config_file);
    dbg!(&config);
    log("./log.txt".to_string(), "Beginning stage 1 backup.".to_string(), LogLevel::Info).unwrap();
    stage1(&config).unwrap();
    
}
