use std::io::BufReader;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::{fs::File, path::Path};

use log::{error, info};
use rdata::db::Database;
use serde::Deserialize;

use crate::error::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_database_path")]
    pub database_path: String,
    #[serde(default = "default_result_limit")]
    pub result_limit: usize,
    pub localization_override: Option<String>,
}

fn default_database_path() -> String {
    String::from("database/")
}

const fn default_result_limit() -> usize {
    40000
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

        match serde_yaml::from_reader(BufReader::new(file)) {
            Ok(config) => config,
            Err(e) => {
                error!("failed to parse config file: {}", e);
                Self::default()
            }
        }
    }

    pub fn load_override_localization_json(&self) -> String {
        let path = match self.localization_override.as_ref() {
            Some(path) => path,
            None => return String::new(),
        };
        let path = Path::new(path);
        if !path.exists() {
            error!(
                "localization override file not found at {}. Ignoring",
                path.display()
            );
            return String::new();
        }
        info!("loading localization override from {}", path.display());
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                error!("failed to open localization override file: {}", e);
                return String::new();
            }
        };
        let value: serde_json::Value = match serde_yaml::from_reader(BufReader::new(file)) {
            Ok(value) => value,
            Err(e) => {
                error!("failed to parse localization override file: {}", e);
                return String::new();
            }
        };
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(e) => {
                error!("failed to convert localization override to JSON: {}", e);
                String::new()
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_path: default_database_path(),
            result_limit: default_result_limit(),
            localization_override: None,
        }
    }
}

/// Create a lazy-loaded database handle
pub fn create_database() -> LazyLock<Result<Database, Error>> {
    LazyLock::new(|| {
        // LazyLock requires 'static
        let config = Config::load();
        let path = config.database_path;
        info!("opening database from {}", path);
        #[cfg(not(feature = "devtools"))]
        let result = Database::open(&path);
        #[cfg(feature = "devtools")]
        let result = {
            use std::fs::File;
            use std::path::Path;
            info!("in devtools mode, bypassing lock check");
            let lock_path = Path::new(&path).join(".lock");
            let lock_file = if lock_path.exists() {
                File::open(&lock_path).map_err(|_| Error::Generic("locked".to_string()))?
            } else {
                File::create(&lock_path).map_err(|_| Error::Generic("locked".to_string()))?
            };
            Database::open_locked(&path, lock_file)
        };
        result.map_err(|e| {
            info!("failed to open database: {}", e);
            e.into()
        })
    })
}
