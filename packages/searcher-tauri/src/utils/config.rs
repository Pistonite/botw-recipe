use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use log::{error, info};
use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Config {
    /// Path to the database directory
    #[serde(default = "default_database_path")]
    pub database_path: String,

    /// Limit the number of results returned by filtering
    #[serde(default = "default_result_limit")]
    pub result_limit: usize,

    /// Bypass locking the database when opening
    #[serde(default)]
    pub bypass_lock: bool,
}

fn default_database_path() -> String {
    String::from("database/")
}

const fn default_result_limit() -> usize {
    5000
}

impl Config {
    pub fn load() -> Self {
        let path = match std::env::var("BOTWRDB_CONFIG_PATH") {
            Ok(path) => {
                info!("using override config path from env: {}", path);
                let path = PathBuf::from(path);
                if !path.exists() {
                    error!(
                        "config file not found at {}. Falling back to default",
                        path.display()
                    );
                    return Self::default();
                }
                path
            }
            Err(_) => {
                let path = PathBuf::from("config.yaml");
                if !path.exists() {
                    error!("config file not found. Creating default config");
                    if let Err(e) = std::fs::write(&path, include_str!("./config.yaml")) {
                        error!("failed to write default config: {}", e);
                    }
                    return Self::default();
                }
                path
            }
        };

        info!("loading config from {}", path.display());
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => {
                error!("failed to open config file: {}", e);
                return Self::default();
            }
        };

        match serde_yaml_ng::from_reader(BufReader::new(file)) {
            Ok(config) => config,
            Err(e) => {
                error!("failed to parse config file: {}", e);
                Self::default()
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_path: default_database_path(),
            result_limit: default_result_limit(),
            bypass_lock: false,
        }
    }
}
