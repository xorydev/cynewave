use serde::Deserialize;
use toml;
use std::{fs, io, path::Path};
mod logger;
use logger::{log, LogLevel};


#[derive(Deserialize, Debug, PartialEq)]
pub struct StageConfig {
    pub src_files: Vec<String>,
    pub target_dir: String,
    pub recursive: bool
}


#[derive(Deserialize, Debug, PartialEq)]
pub struct Config {
    pub stage_1: StageConfig
}

impl Config {
    pub fn new(config_toml: String) -> Config {
        toml::from_str(&config_toml).expect("Invalid Config")
    }
}

pub fn stage1(config: &Config) -> io::Result<()> {
    for file in &config.stage_1.src_files {
        let filename = match file.split('/').last() {
            Some(filename) => filename,
            None => { eprintln!("E: Failed to derive filename from {file}"); break }
        };
        
        let target_path = format!("{}/{filename}", &config.stage_1.target_dir);

        if Path::new(file).is_file() {
            if let Err(err) = fs::copy(file, target_path) {
                log("./log.txt".to_string(), err.to_string(), LogLevel::Error)?;
            };
        } else if Path::new(file).is_dir() && config.stage_1.recursive {
            if let Err(err) = fs::create_dir(target_path) {
                log("./log.txt".to_string(), err.to_string(), LogLevel::Error)?;
            }
            for entry in fs::read_dir(file)? {
                let target_dir = format!("{}/{}", &config.stage_1.target_dir, filename);
                let file = entry?;
                if let Err(err) = fs::copy(Path::new(&file.path()), format!("{}/{}", target_dir, file.file_name().to_str().unwrap())) {
                    log("./log.txt".to_string(), err.to_string(), LogLevel::Error)?;
                };   
            }
        } else {
            log("./log.txt".to_string(), "Copy conditions not matched for file {file}, skipping.".to_string(), LogLevel::Warning)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_parser_test() {
        let toml_config = "[stage_1]
        src_files = [ './testfiles/src/file1.txt', './testfiles/src/file2.txt', './testfiles/src/a' ]
        target_dir = './testfiles/dir'
        recursive = true";
        
        let parsed_config = Config::new(toml_config.to_string());
        assert_eq!(
            Config {
                stage_1: StageConfig { 
                    src_files: vec![ "./testfiles/src/file1.txt".to_string(), "./testfiles/src/file2.txt".to_string(), "./testfiles/src/a".to_string() ],
                    target_dir: "./testfiles/dir".to_string(),
                    recursive: true
                } 
            },
            parsed_config
        );
    }
}
