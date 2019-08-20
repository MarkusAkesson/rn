use crate::error::RnError;
use serde::{Deserialize, Serialize};
use std::fs;

pub static FILE_PATH: &str = ".rn";

pub type Result<T, E = RnError> = std::result::Result<T, E>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    default_dir: String,
    default_bin: String,
    default_args: String,
}

impl Config {
    pub fn new(default_dir: &str, default_bin: &str, default_args: &str) -> Self {
        Config {
            default_dir: String::from(default_dir),
            default_bin: String::from(default_bin),
            default_args: String::from(default_args),
        }
    }

    pub fn from_file() -> Result<Config> {
        let f = match std::fs::File::open(FILE_PATH) {
            Ok(f) => f,
            Err(_) => return Err(RnError::OpeningConfigFile),
        };
        let cfg = match serde_yaml::from_reader(&f) {
            Ok(cfg) => cfg,
            Err(_) => return Err(RnError::ReadYaml),
        };
        Ok(cfg)
    }

    pub fn save(&self) -> Result<()> {
        let mut yaml_string = serde_yaml::to_string(&self).unwrap();
        yaml_string += "\n";
        match fs::write(FILE_PATH, yaml_string) {
            Ok(_) => Ok(()),
            Err(_) => Err(RnError::FailedToSaveFile),
        }
    }

    pub fn print(&self) {
        let yaml_string = serde_yaml::to_string(&self).unwrap();
        println!("{}", yaml_string);
    }

    pub fn get_directory(&self) -> &str {
        &self.default_dir
    }

    pub fn get_binary(&self) -> &str {
        &self.default_bin
    }

    pub fn get_args(&self) -> &str {
        &self.default_args
    }

    pub fn update_directory(&mut self, dir: &str) {
        self.default_dir = String::from(dir);
    }

    pub fn update_binary(&mut self, bin: &str) {
        self.default_bin = String::from(bin);
    }

    pub fn update_args(&mut self, args: &str) {
        self.default_args = String::from(args);
    }
}
