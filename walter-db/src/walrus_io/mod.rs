use std::fs::File;
use std::io::{Read, Write};

mod types;
use types::*;

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
                "epochs": epochs,
                "deletable": true,
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

    Some(output["success"].as_bool().unwrap_or(true))
}

fn get_blob_id_from_response(response: NewlyCreated) -> Option<String> {
    Some(response.blobObject.blobId)
}

fn get_blob_id_from_already_certified(response: serde_json::Value) -> Option<String> {
    response.get("blobId").and_then(|blob_id| blob_id.as_str().map(|s| s.to_string()))
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

pub fn append_id_and_upload(file_location: String, blob_id: String, epochs: Option<u16>) -> Option<serde_json::Value> {
    // Read the file content
    let mut file = File::open(&file_location).expect("Unable to open file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).expect("Unable to read file");

    // Append the blob_id to the file content
    contents.extend_from_slice(blob_id.as_bytes());

    // Write the modified content back to the file
    let mut file = File::create(&file_location).expect("Unable to create file");
    file.write_all(&contents).expect("Unable to write to file");

    // Upload the modified file to walrus
    upload_to_walrus(file_location, epochs)
}

pub fn download_and_extract_id(blob_id: String, file_location: String) -> Option<String> {
    // Download the file from walrus
    let success = download_from_walrus(blob_id.clone(), file_location.clone());

    // Read the file content
    let mut file = File::open(&file_location).expect("Unable to open file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).expect("Unable to read file");

    // Extract the blob_id from the file content
    if contents.len() >= blob_id.len() {
        let extracted_blob_id = String::from_utf8_lossy(&contents[contents.len() - blob_id.len()..]).to_string();
        println!("Extracted blob_id: {}", extracted_blob_id);
        Some(extracted_blob_id)
    } else {
        println!("Failed to extract blob_id: content length is less than blob_id length");
        None
    }
}


// WRITE TESTS FOR THE LAST TWO FUNCTIONS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_and_extract_id() {
        let blob_id = "NKzOvrC2ksXDwOqTdk5NdqZ5aglAW5_dCS4GtbEChZ0".to_string();
        let file_location = "/tmp/test_file".to_string();
        let content = "test_content".to_string();
        let mut file = File::create(&file_location).expect("Unable to create file");
        file.write_all(content.as_bytes()).expect("Unable to write to file");

        let extracted_blob_id = download_and_extract_id(blob_id.clone(), file_location.clone());
        assert_eq!(extracted_blob_id, Some(blob_id));
    }

    #[test]
    fn test_append_id_and_upload() {
        let file_location = "/tmp/test_file".to_string();
        let blob_id = "Saih8gqlyGPC4LZhP5Co3KmJsJ1DWWTyyd-pdY9jYx0".to_string();
        let content = "test_content".to_string();
        let mut file = File::create(&file_location).expect("Unable to create file");
        file.write_all(content.as_bytes()).expect("Unable to write to file");

        let uploaded = append_id_and_upload(file_location.clone(), blob_id.clone(), None);
        println!("{:?}", uploaded);
        assert!(uploaded.is_some());
    }
}