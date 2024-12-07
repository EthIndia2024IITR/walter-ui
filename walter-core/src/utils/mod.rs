use std::io::Write;

use super::types::*;
use crate::config::WalterConfig;
use crate::encryptor::{decrypt_file, encrypt_file};
use crate::sharder::Sharder;

pub struct WalrusClient {
    config: WalterConfig,
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

        let shards = Sharder::new(file_path, self.config.get_default_shard_size());
        let mut blobs: Vec<String> = Vec::new();

        for shard in shards {
            let temp_file_path = std::env::temp_dir().join(format!(
                "shard_{}.tmp",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_nanos()
            ));

            std::fs::write(&temp_file_path, shard.get_shard())?;

            let blob_id = upload_blob(
                temp_file_path.to_str().unwrap(),
                self.config.get_default_epochs(),
            )
            .await?;

            blobs.push(blob_id);
            std::fs::remove_file(&temp_file_path)?;
        }

        self.config.add_file(file_path, to_encrypt, blobs);

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
            let temp_file_path = std::env::temp_dir().join(format!(
                "shard_{}.tmp",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_nanos()
            ));

            let success = download_blob(blob, temp_file_path.to_str().unwrap()).await?;

            if !success {
                return Err("Failed to download blob".into());
            }

            let shard = std::fs::read(temp_file_path)?;
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

/// Fetches blob entries belonging to a wallet from Walruscan API
///
/// # Arguments
///
/// * `search_str` - The search string (typically an object ID)
/// * `page` - The page number to fetch (default is 0)
/// * `size` - Number of entries per page (default is 20)
///
/// # Returns
///
/// A Result containing the parsed BlobResponse or a reqwest::Error
pub async fn fetch_walruscan_blobs(
    search_str: &str,
    page: Option<u32>,
    size: Option<u32>,
) -> Result<BlobResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let page = page.unwrap_or(0);
    let size = size.unwrap_or(20);

    let url = format!(
        "https://t.walruscan.com/api/walscan-backend/testnet/api/blobs?page={}&sortBy=TIMESTAMP&searchStr={}&size={}",
        page, search_str, size
    );

    let response = client
        .get(&url)
        .send()
        .await?
        .json::<BlobResponse>()
        .await?;

    Ok(response)
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
    let output = String::from_utf8_lossy(&output.stdout);
    let output: serde_json::Value = match serde_json::from_str(&output) {
        Ok(val) => val,
        Err(_) => {
            return Err("Failed to parse output JSON".into());
        }
    };

    let blob_id = get_blob_id(output);
    if blob_id.is_none() {
        return Err("Failed to get blob ID".into());
    }

    Ok(blob_id.unwrap())
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

    if output.status.success() {
        println!("Successfully downloaded file from walrus");
    } else {
        return Err("Failed to download file from walrus".into());
    }

    let output = String::from_utf8_lossy(&output.stdout);

    let output: serde_json::Value = match serde_json::from_str(&output) {
        Ok(val) => val,
        Err(_) => {
            return Err("Failed to parse output JSON".into());
        }
    };

    Ok(output["success"].as_bool().unwrap_or(false))
}

fn get_blob_id_from_response(response: NewlyCreated) -> Option<String> {
    Some(response.blobObject.blobId)
}

fn get_blob_id_from_already_certified(response: serde_json::Value) -> Option<String> {
    response
        .get("blobId")
        .and_then(|id| id.as_str().map(|s| s.to_string()))
}

// decide if response is from BlobResponse or AlreadyCertified
pub fn get_blob_id(response: serde_json::Value) -> Option<String> {
    if response.get("alreadyCertified").is_some() {
        get_blob_id_from_already_certified(response.get("alreadyCertified").unwrap().clone())
    } else {
        match serde_json::from_value(response.get("newlyCreated").unwrap().clone()) {
            Ok(newly_created) => get_blob_id_from_response(newly_created),
            Err(e) => {
                println!("Failed to deserialize newlyCreated: {}", e.to_string());
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_blob_id() {
        let response = serde_json::json!({
            "blobId": "DVZWz_QCEb2D_UPQzswv-DUqg-etmV6rEPzoERY4Tgg",
            "endEpoch": 0,
            "eventOrObject": "Object"
        });

        let output = get_blob_id(response);
        assert!(output.is_some());
        assert_eq!(
            output.unwrap(),
            "DVZWz_QCEb2D_UPQzswv-DUqg-etmV6rEPzoERY4Tgg"
        );
    }

    #[test]
    fn test_get_blob_id_from_already_certified() {
        let response = AlreadyCertified {
            blobId: "DVZWz_QCEb2D_UPQzswv-DUqg-etmV6rEPzoERY4Tgg".to_string(),
            endEpoch: 0,
            eventOrObject: EventOrObject::Object,
        };

        let output = get_blob_id_from_already_certified(response);
        assert!(output.is_some());
        assert_eq!(
            output.unwrap(),
            "DVZWz_QCEb2D_UPQzswv-DUqg-etmV6rEPzoERY4Tgg"
        );
    }

    #[test]
    fn test_get_blob_id_from_response() {
        let response = BlobResponse {
            content: vec![BlobEntry {
                blobId: "DVZWz_QCEb2D_UPQzswv-DUqg-etmV6rEPzoERY4Tgg".to_string(),
                blobIdBase64: "DVZWz_QCEb2D_UPQzswv-DUqg-etmV6rEPzoERY4Tgg".to_string(),
                objectId: "0xaa5cd02a25fc90c6dc419a52d02a59d6c484a27f88aeb698cd3570212cae9ba0"
                    .to_string(),
                startEpoch: 0,
                endEpoch: 0,
                size: 0,
                timestamp: 0,
            }],
            pageable: Pageable {
                pageNumber: 0,
                pageSize: 0,
                sort: Sort {
                    sorted: false,
                    empty: false,
                    unsorted: false,
                },
                offset: 0,
                paged: false,
                unpaged: false,
            },
            totalPages: 0,
            totalElements: 0,
            last: false,
            size: 0,
            number: 0,
            sort: Sort {
                sorted: false,
                empty: false,
                unsorted: false,
            },
            numberOfElements: 0,
            first: false,
            empty: false,
        };
        let response_already_certified = AlreadyCertified {
            blobId: "DVZWz_QCEb2D_UPQzswv-DUqg-etmV6rEPzoERY4Tgg".to_string(),
            endEpoch: 0,
            eventOrObject: EventOrObject::Object,
        };

        let output = get_blob_id_from_response(response);
        assert!(output.is_some());
        assert_eq!(
            output.unwrap(),
            "DVZWz_QCEb2D_UPQzswv-DUqg-etmV6rEPzoERY4Tgg"
        );
    }

    #[tokio::test]
    async fn test_download_from_walrus() {
        let output = download_blob(
            "DVZWz_QCEb2D_UPQzswv-DUqg-etmV6rEPzoERY4Tgg",
            "./test_files/lmao",
        )
        .await;

        assert!(output.is_ok());
    }

    #[tokio::test]
    async fn test_upload_to_walrus() {
        let output = upload_blob("./test_files/uploadcopy.test", 10).await;
        assert!(output.is_ok());
    }

    #[tokio::test]
    async fn test_fetch_walruscan_blobs() {
        let search_str = "0xaa5cd02a25fc90c6dc419a52d02a59d6c484a27f88aeb698cd3570212cae9ba0";
        let result = fetch_walruscan_blobs(search_str, None, None).await;

        assert!(result.is_ok());
        let blobs = result.unwrap();
        println!("{:?}", blobs);
        assert!(!blobs.content.is_empty());
    }
}
