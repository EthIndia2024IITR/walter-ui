use serde::{Deserialize, Serialize};
use shellexpand;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const CONFIG_FILE_PATH: &str = "~/.walter/config.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub is_encrypted: bool,
    pub blobs: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WalterConfig {
    pub default_file_download_dir: String,
    pub default_epochs: u16,
    pub default_shard_size: usize,
    pub renew_epoch_threshold: u16,
    pub default_renewal_value: u16,
    pub files: HashMap<String, FileInfo>,
}

impl WalterConfig {
    pub fn get_default_file_download_dir(&self) -> &str {
        return &self.default_file_download_dir;
    }

    pub fn get_default_epochs(&self) -> u16 {
        return self.default_epochs;
    }

    pub fn get_default_shard_size(&self) -> usize {
        return self.default_shard_size;
    }

    pub fn get_files(&self) -> &HashMap<String, FileInfo> {
        return &self.files;
    }

    pub fn get_file_blobs(&self, file_path: &str) -> Option<&Vec<String>> {
        return self.files.get(file_path).map(|file_info| &file_info.blobs);
    }

    pub fn add_file(&mut self, file_path: &str, is_encrypted: bool, blobs: Vec<String>) {
        let file_info = FileInfo {
            is_encrypted,
            blobs,
        };

        self.files.insert(file_path.to_string(), file_info);
    }

    pub fn load_config_file() -> WalterConfig {
        let path = shellexpand::tilde(CONFIG_FILE_PATH).to_string();
        let path = Path::new(&path);

        if !path.exists() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("Unable to create config directory");
            }

            let default_config = WalterConfig {
                default_file_download_dir: "~/.walter/downloads".to_string(),
                default_epochs: 10,
                default_shard_size: 1024, // 1MB
                renew_epoch_threshold: 2,
                default_renewal_value: 10,
                files: HashMap::new(),
            };

            let config_json = serde_json::to_string(&default_config)
                .expect("Unable to serialize default config!");
            fs::write(path, config_json).expect("Unable to write default config file!");

            return default_config;
        }

        let config_json = fs::read_to_string(path).expect("Unable to read config file!");
        serde_json::from_str(&config_json).expect("Unable to deserialize config file!")
    }

    pub fn save_config_file(&self) {
        let path = shellexpand::tilde(CONFIG_FILE_PATH).to_string();
        let path = Path::new(&path);

        if !path.exists() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("Unable to create config directory");
            }
        }

        let config_json = serde_json::to_string(self).expect("Unable to serialize config!");
        fs::write(path, config_json).expect("Unable to write config file!");
    }
}
