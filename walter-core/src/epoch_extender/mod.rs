use ::serde_json::Value;
use reqwest::{Error, Result};
use serde::{Deserialize, Serialize};
mod types;
use types::*;

/// Fetches blob entries from Walruscan API
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
) -> Result<BlobResponse> {
    let client = reqwest::Client::new();
    let page = page.unwrap_or(0);
    let size = size.unwrap_or(20);

    let url = format!(
        "https://t.walruscan.com/api/walscan-backend/testnet//api/blobs?page={}&sortBy=TIMESTAMP&searchStr={}&size={}",
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

//Input: fileName: String
//Process: Reads from the filesystem and uploads using "walrus store <fileNamewithpath>" installed on OS
//Output: None
pub fn upload_to_walrus(file_name: String, epochs: Option<u16>) -> Option<serde_json::Value> {
    // epochs if not specified, then 1
    let epochs = epochs.unwrap_or(1);
    let command_json = serde_json::json!({
        "command": {
            "store": {
                "file": file_name,
                "epochs": epochs
            },
        }
    });

    let output = std::process::Command::new("walrus")
        .arg("json")
        .arg(command_json.to_string())
        .output()
        .expect("failed to execute process");

    // check if output execution is successful gracefully
    if output.status.success() {
        println!("Successfully uploaded file to walrus");
    } else {
        println!("Failed to upload file to walrus");
        return None;
    }

    // serialize stdout in output to JSON
    let output = String::from_utf8_lossy(&output.stdout);
    let output: serde_json::Value = match serde_json::from_str(&output) {
        Ok(val) => val,
        Err(_) => {
            println!("Failed to parse output JSON");
            return None;
        }
    };

    Some(output)
}

//Input: blobId: String
//Process: Reads from Walrus and outputs into a file at given path
//Output: success or failure bool
pub fn download_from_walrus(blob_id: String, file_location: String) -> Option<bool> {
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
        println!("Failed to download file from walrus");
        return None;
    }

    let output = String::from_utf8_lossy(&output.stdout);
    let output: serde_json::Value = match serde_json::from_str(&output) {
        Ok(val) => val,
        Err(_) => {
            println!("Failed to parse output JSON");
            return None;
        }
    };

    Some(output["success"].as_bool().unwrap_or(false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_from_walrus() {
        let output = download_from_walrus(
            "DVZWz_QCEb2D_UPQzswv-DUqg-etmV6rEPzoERY4Tgg".to_string(),
            "./test_files/lmao".to_string(),
        );
        assert!(output.is_some());
        println!("{:?}", output);
    }

    #[test]
    fn test_upload_to_walrus() {
        let output = upload_to_walrus("./test_files/uploadcopy.test".to_string(), None);
        assert!(output.is_some());
        println!("{:?}", output);
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
