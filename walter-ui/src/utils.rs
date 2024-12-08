use std::{
    error::Error,
    process::{Command, Stdio},
};
use walter_core::types::BlobInfo;

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
