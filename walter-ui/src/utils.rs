use std::{
    error::Error,
    io::Write,
    process::{Command, Stdio},
};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct WalrusCommand {
    // pub config: String,
    pub command: StoreCommand,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreCommand {
    pub store: StoreDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreDetails {
    pub file: String,
    pub epochs: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalrusResponse {
    #[serde(rename = "newlyCreated", skip_serializing_if = "Option::is_none")]
    pub newly_created: Option<NewlyCreatedResponse>,

    #[serde(rename = "alreadyCertified", skip_serializing_if = "Option::is_none")]
    pub already_certified: Option<AlreadyCertifiedResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewlyCreatedResponse {
    #[serde(rename = "blobObject")]
    pub blob_object: NewlyCreatedBlobObject,

    #[serde(rename = "resourceOperation")]
    pub resource_operation: ResourceOperation,

    pub cost: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewlyCreatedBlobObject {
    pub id: String,

    #[serde(rename = "registeredEpoch")]
    pub registered_epoch: u64,

    #[serde(rename = "blobId")]
    pub blob_id: String,

    pub size: u64,

    #[serde(rename = "encodingType")]
    pub encoding_type: String,

    #[serde(rename = "certifiedEpoch")]
    pub certified_epoch: u64,

    pub storage: StorageInfo,

    pub deletable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageInfo {
    pub id: String,

    #[serde(rename = "startEpoch")]
    pub start_epoch: u64,

    #[serde(rename = "endEpoch")]
    pub end_epoch: u64,

    #[serde(rename = "storageSize")]
    pub storage_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceOperation {
    #[serde(rename = "RegisterFromScratch")]
    pub register_from_scratch: RegisterFromScratchDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterFromScratchDetails {
    #[serde(rename = "encoded_length")]
    pub encoded_length: u64,

    #[serde(rename = "epochs_ahead")]
    pub epochs_ahead: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlreadyCertifiedResponse {
    #[serde(rename = "blobId")]
    pub blob_id: String,

    #[serde(rename = "eventOrObject")]
    pub event_or_object: EventOrObject,

    #[serde(rename = "endEpoch")]
    pub end_epoch: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventOrObject {
    #[serde(rename = "Event")]
    pub event: Option<EventDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventDetails {
    #[serde(rename = "txDigest")]
    pub tx_digest: String,

    #[serde(rename = "eventSeq")]
    pub event_seq: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlobInfo {
    pub blob_id: String,
    pub unencoded_size: String, // Keep as string to preserve original format
    pub is_certified: bool,
    pub is_deletable: bool,
    pub expiration_epoch: u64,
    pub object_id: String,
}

pub async fn sui_active_address() -> Result<String, Box<dyn Error>> {
    let child = Command::new("sui")
        .arg("client")
        .arg("active-address")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to start process: {}", e))?;

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to wait on process: {}", e))?;

    // Check for errors
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("Process failed: {}", stderr).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    Ok(stdout)
}


pub async fn sui_active_env() -> Result<String, Box<dyn Error>> {
    let child = Command::new("sui")
        .arg("client")
        .arg("active-env")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to start process: {}", e))?;

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to wait on process: {}", e))?;

    // Check for errors
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("Process failed: {}", stderr).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    Ok(stdout)
}


pub async fn run_walrus(json_data: String) -> Result<String, Box<dyn Error>> {
    let mut child = Command::new("walrus")
        .arg("json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to start process: {}", e))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(json_data.as_bytes())
            .map_err(|e| format!("Failed to write to stdin: {}", e))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to wait on process: {}", e))?;

    // Check for errors
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("Process failed: {}", stderr).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let response: WalrusResponse = serde_json::from_str(&stdout)?;

    let output_json = if let Some(newly_created) = response.newly_created {
        json!({
            "blobId": newly_created.blob_object.blob_id,
            "size": newly_created.blob_object.size,
            "objectId": newly_created.blob_object.id,
            "registeredEpoch": newly_created.blob_object.registered_epoch
        })
    } else if let Some(already_certified) = response.already_certified {
        json!({
            "blobId": already_certified.blob_id,
            "endEpoch": already_certified.end_epoch
        })
    } else {
        return Err("Unexpected response from Walrus".into());
    };

    Ok(output_json.to_string())
}

pub fn parse_blob_list(stdout: &str) -> Result<Vec<BlobInfo>, Box<dyn Error>> {
    let lines: Vec<&str> = stdout
        .lines()
        .map(str::trim)
        .skip_while(|line| !line.contains("Blob ID")) // Skip until the "Blob ID" header
        .skip(2)
        .take_while(|line| !line.is_empty() && !line.contains("-----")) // Stop at the next empty or separator line
        .collect();

    let mut blobs = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        if parts.len() < 6 {
            return Err(format!("Invalid line format: {}", line).into());
        }

        let blob_info = BlobInfo {
            blob_id: parts[0].to_string(),
            unencoded_size: parts[1..=2].join(" "),
            is_certified: parts[3] == "true",
            is_deletable: parts[4] == "true",
            expiration_epoch: parts[5]
                .parse()
                .map_err(|_| format!("Failed to parse epoch: {}", parts[5]))?,
            object_id: parts[6].to_string(),
        };

        blobs.push(blob_info);
    }

    Ok(blobs)
}

pub async fn walrus_list_blobs() -> Result<String, Box<dyn Error>> {
    let output = Command::new("walrus")
        .arg("list-blobs")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .map_err(|e| format!("Failed to start process: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Process failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let blobs =
        parse_blob_list(&stdout).map_err(|e| format!("Failed to parse blob list: {}", e))?;

    serde_json::to_string_pretty(&blobs)
        .map_err(|e| format!("Failed to serialize JSON: {}", e).into())
}

pub async fn walrus_info_system() -> Result<String, Box<dyn Error>> {
    let output = Command::new("walrus")
        .arg("info")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .map_err(|e| format!("Failed to start process: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Process failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    Ok(stdout.to_string())
}


