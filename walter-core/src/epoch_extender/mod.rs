mod types;
use reqwest::*;
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

fn get_blob_id_from_response(response: BlobResponse) -> Option<String> {
    if !response.content.is_empty() {
        Some(response.content[0].blobId.clone())
    } else {
        None
    }
}

fn get_blob_id_from_already_certified(response: AlreadyCertified) -> Option<String> {
    Some(response.blobId.clone())
}

// decide if response is from BlobResponse or AlreadyCertified
fn get_blob_id(response: serde_json::Value) -> Option<String> {
    if response["blobId"].is_string() {
        get_blob_id_from_already_certified(serde_json::from_value(response).unwrap())
    } else {
        get_blob_id_from_response(serde_json::from_value(response).unwrap())
    }
}

pub fn epoch_extender(blob_id: String, epochs: Option<u16>) -> Option<bool> {
    let output = download_from_walrus(blob_id, "/tmp/epoch_extender".to_string());
    println!("{:?}", output);
    if output.is_none() {
        return None;
    }
    
    let output = upload_to_walrus("/tmp/epoch_extender".to_string(), epochs);
    if output.is_none() {
        return None;
    }
    println!("{:?}", output);
    Some(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epoch_extender() {
        let output = epoch_extender(
            "DVZWz_QCEb2D_UPQzswv-DUqg-etmV6rEPzoERY4Tgg".to_string(),
            Some(1),
        );
        assert!(output.is_some());
        assert_eq!(output.unwrap(), true);
    }

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
        let epochs = Some(1);
        let output = upload_to_walrus("./test_files/uploadcopy.test".to_string(), epochs);
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
