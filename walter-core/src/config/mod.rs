use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub is_encrypted: bool,
    pub blobs: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WalterConfig {
    default_file_download_dir: String,
    default_epochs: u16,
    default_shard_size: usize,
    files: HashMap<String, FileInfo>,
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

    pub fn load_config_file(filename: &str) -> WalterConfig {
        let config_file = fs::read_to_string(filename).expect("Unable to read config file");
        let config: WalterConfig =
            serde_json::from_str(&config_file).expect("Unable to parse config file");
        return config;
    }

    pub fn save_config_file(&self, filename: &str) {
        let config_json = serde_json::to_string(&self).expect("Unable to serialize config");
        fs::write(filename, config_json).expect("Unable to write config file");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_file() {
        let config = WalterConfig::load_config_file("tests/test_config.json");
        assert_eq!(config.files.len(), 2);
        assert_eq!(
            config.files.get("test_sharder.txt").unwrap().is_encrypted,
            false
        );
        assert_eq!(
            config.files.get("test_sharder2.txt").unwrap().is_encrypted,
            false
        );
    }

    #[test]
    fn test_save_config_file() {
        let mut config = WalterConfig {
            default_file_download_dir: "~/.walter".to_string(),
            default_epochs: 10,
            default_shard_size: 100,
            files: HashMap::new(),
        };

        let file_info = FileInfo {
            is_encrypted: false,
            blobs: vec!["blob1".to_string(), "blob2".to_string()],
        };

        config.files.insert("test.txt".to_string(), file_info);
        config.save_config_file("tests/test_save_config.json");
        let new_config = WalterConfig::load_config_file("tests/test_save_config.json");
        assert_eq!(new_config.files.len(), 1);
        assert_eq!(
            new_config.files.get("test.txt").unwrap().blobs,
            vec!["blob1".to_string(), "blob2".to_string()]
        );
    }
}
