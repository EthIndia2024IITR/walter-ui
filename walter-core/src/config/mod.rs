use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    pub isEncrypted: bool,
    pub blobs: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    files: HashMap<String, FileInfo>,
}

impl Config {
    pub fn get_files(&self) -> &HashMap<String, FileInfo> {
        return &self.files;
    }

    pub fn load_config_file(filename: &str) -> Config {
        let config_file = fs::read_to_string(filename).expect("Unable to read config file");
        let config: Config =
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
        let config = Config::load_config_file("tests/test_config.json");
        assert_eq!(config.files.len(), 2);
        assert_eq!(
            config.files.get("test_sharder.txt").unwrap().isEncrypted,
            false
        );
        assert_eq!(
            config.files.get("test_sharder2.txt").unwrap().isEncrypted,
            false
        );
    }

    #[test]
    fn test_save_config_file() {
        let mut config = Config {
            files: HashMap::new(),
        };
        let file_info = FileInfo {
            isEncrypted: false,
            blobs: vec!["blob1".to_string(), "blob2".to_string()],
        };
        config.files.insert("test.txt".to_string(), file_info);
        config.save_config_file("tests/test_save_config.json");
        let new_config = Config::load_config_file("tests/test_save_config.json");
        assert_eq!(new_config.files.len(), 1);
        assert_eq!(
            new_config.files.get("test.txt").unwrap().blobs,
            vec!["blob1".to_string(), "blob2".to_string()]
        );
    }
}
