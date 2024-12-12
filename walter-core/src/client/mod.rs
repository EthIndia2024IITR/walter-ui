use std::io::Write;

use crate::config::WalterConfig;
use crate::encryptor::{decrypt_file, encrypt_file};
use crate::sharder::Sharder;
use crate::types::*;

pub struct WalrusClient {
    pub config: WalterConfig,
}

impl WalrusClient {
    pub fn new(config: WalterConfig) -> Self {
        WalrusClient { config }
    }

    pub async fn upload_file(
        &mut self,
        file_path: &str,
        password: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let to_encrypt: bool = password.is_some();
        if to_encrypt {
            encrypt_file(file_path, file_path, &password.unwrap())?;
        }

        let shards = Sharder::new(file_path, self.config.get_default_shard_size())?;
        let mut blobs: Vec<String> = Vec::new();

        for shard in shards {
            let temp_file_path = std::env::temp_dir().join(format!(
                "shard_{}.tmp",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_nanos()
            ));

            std::fs::write(&temp_file_path, shard)?;

            let blob_id = upload_blob(
                temp_file_path.to_str().unwrap(),
                self.config.get_default_epochs(),
            )
            .await?;

            blobs.push(blob_id);
            std::fs::remove_file(&temp_file_path)?;
        }

        self.config.add_file(file_path, to_encrypt, blobs);
        self.config.save_config_file();
        Ok(true)
    }

    pub async fn download_file(
        &self,
        file_path: &str,
        password: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let to_decrypt: bool = password.is_some();
        let blobs = self.config.get_file_blobs(file_path).unwrap();
        let mut file_data = Vec::new();

        for blob in blobs {
            let temp_file_path = std::env::current_dir()?.join(format!(
                "{}.tmp",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_nanos()
            ));

            let success = download_blob(blob, &temp_file_path.to_str().unwrap()).await;
            let success = success.unwrap();

            if !success {
                return Err("Failed to download blob".into());
            }

            let shard = std::fs::read(&temp_file_path)?;
            std::fs::remove_file(&temp_file_path)?;
            file_data.extend(shard);
        }

        let mut file = std::fs::File::create(file_path)?;
        file.write_all(&file_data.as_slice())?;

        if to_decrypt {
            decrypt_file(file_path, file_path, &password.unwrap())?;
        }

        Ok(true)
    }
}

pub async fn upload_blob(
    file_path: &str,
    epochs: u16,
) -> Result<String, Box<dyn std::error::Error>> {
    let command_json = serde_json::json!({
        "command": {
            "store": {
                "file": file_path,
                "epochs": epochs,
            },
        }
    });

    let output = std::process::Command::new("walrus")
        .arg("json")
        .arg(command_json.to_string())
        .output()
        .expect("failed to execute process");

    // che ck if output execution is successful gracefully
    if !output.status.success() {
        return Err("Failed to upload file to walrus".into());
    }

    // serialize stdout in output to JSON

    let output_json = String::from_utf8_lossy(&output.stdout);

    // Try deserializing to WalrusNewlyCreated
    let status_new: Result<WalrusNewlyCreated, serde_json::Error> =
        serde_json::from_str(&output_json);

    if let Ok(new_status) = status_new {
        let blob_id = new_status.newlyCreated.blobObject.blobId;
        return Ok(blob_id.to_string());
    } else {
        // Try deserializing to WalrusAlreadyCertified
        let status_certified: Result<WalrusAlreadyCertified, serde_json::Error> =
            serde_json::from_str(&output_json);

        if let Ok(certified_status) = status_certified {
            let blob_id = certified_status.alreadyCertified.blobId;
            return Ok(blob_id.to_string());
        } else {
            return Err("Failed to parse output JSON to WalrusResponse".into());
        }
    }
}

pub async fn download_blob(
    blob_id: &str,
    file_location: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let command_json = serde_json::json!({
        "command": {
            "read": {
                "blobId": blob_id,
                "out": file_location,
            },
        }
    });

    let output = std::process::Command::new("walrus")
        .arg("json")
        .arg(command_json.to_string())
        .output()
        .expect("failed to execute process");

    if !output.status.success() {
        return Err("Failed to download file from walrus".into());
    }

    let output = String::from_utf8_lossy(&output.stdout);

    let output: serde_json::Value = match serde_json::from_str(&output) {
        Ok(val) => val,
        Err(_) => {
            return Err("Failed to parse output JSON".into());
        }
    };

    Ok(true)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_download_from_walrus() {
        let output = download_blob(
            "DVZWz_QCEb2D_UPQzswv-DUqg-etmV6rEPzoERY4Tgg",
            "./test_files/download_test.txt",
        )
        .await;

        assert!(output.is_ok());
    }

    #[test]
    fn test_already_certified_deserialization() {
        let json = r#"{
                            "alreadyCertified": {
                                "blobId": "WNj9kV-79ScIKYpGmXsBBT0PjjyCeTkZYvUNtwUEr-A",
                                "eventOrObject": {
                                "Event": {
                                    "txDigest": "DXkGxNHqK8dZXsi6E1krAXs3gKq8ULZsQjiryBfgjkq4",
                                    "eventSeq": "0"
                                }
                                },
                                "endEpoch": 61
                            }
                            }"#;

        let status: WalrusAlreadyCertified = serde_json::from_str(json).unwrap();
        assert_eq!(
            status.alreadyCertified.blobId,
            "WNj9kV-79ScIKYpGmXsBBT0PjjyCeTkZYvUNtwUEr-A"
        );
    }

    #[test]
    fn test_newly_created_deserialization() {
        let json = r#"{
                                "newlyCreated": {
                                    "blobObject": {
                                        "id": "0x6ddf05fbd44f522a49d1eef75dab70769b986857c192f108bd52ffd1bdb732d4",
                                        "registeredEpoch": 51,
                                        "blobId": "DVZWz_QCEb2D_UPQzswv-DUqg-etmV6rEPzoERY4Tgg",
                                        "size": 46,
                                        "encodingType": "RedStuff",
                                        "certifiedEpoch": 51,
                                        "storage": {
                                            "id": "0xe9be566bec206862e3807225e1a190700fcfd144250d412f51d2776571050e13",
                                            "startEpoch": 51,
                                            "endEpoch": 52,
                                            "storageSize": 65023000
                                        },
                                        "deletable": false
                                    },
                                    "resourceOperation": {
                                        "RegisterFromScratch": {
                                            "encoded_length": 65023000,
                                            "epochs_ahead": 1
                                        }
                                    },
                                    "cost": 132300
                                }
                            }"#;

        let status: WalrusNewlyCreated = serde_json::from_str(json).unwrap();
        assert_eq!(
            status.newlyCreated.blobObject.blobId,
            "DVZWz_QCEb2D_UPQzswv-DUqg-etmV6rEPzoERY4Tgg"
        );
    }

    #[tokio::test]
    async fn test_upload_to_walrus() {
        let output = upload_blob("test_files/uploadcopy.test", 10).await;
        assert!(output.is_ok());
    }

    #[tokio::test]
    async fn test_file_upload() {
        let config = WalterConfig::load_config_file();
        let mut client = WalrusClient::new(config);
        let output = client.upload_file("test_files/test_upload.txt", None).await;
        client.config.save_config_file();
        assert!(output.is_ok());
    }

    #[tokio::test]
    async fn test_file_download() {
        let config = WalterConfig::load_config_file();
        let client = WalrusClient::new(config);
        let output = client
            .download_file("test_files/test_upload.txt", None)
            .await;
        assert!(output.is_ok());
    }

    #[tokio::test]
    async fn final_test() {
        let config = WalterConfig::load_config_file();
        let mut client = WalrusClient::new(config);
        let output = client
            .upload_file(
                "test_files/test_upload.txt",
                Some("Password@123".to_string()),
            )
            .await;

        assert!(output.is_ok());

        let output = client
            .download_file(
                "test_files/test_upload.txt",
                Some("Password@123".to_string()),
            )
            .await;
        assert!(output.is_ok());
    }
}
