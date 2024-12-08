use crate::config::WalterConfig;
use crate::client::WalrusClient;
use failure;
use reqwest::Client;
use std::error::Error;
use std::fs::write as write_file;

const JWT : &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySW5mb3JtYXRpb24iOnsiaWQiOiI4YzAxZGVjYy1iZmFiLTQ4Y2UtOTQyMy05NjJkMWNkYjlhODYiLCJlbWFpbCI6InByYW5lZXRoc2Fyb2RlQGdtYWlsLmNvbSIsImVtYWlsX3ZlcmlmaWVkIjp0cnVlLCJwaW5fcG9saWN5Ijp7InJlZ2lvbnMiOlt7ImRlc2lyZWRSZXBsaWNhdGlvbkNvdW50IjoxLCJpZCI6IkZSQTEifSx7ImRlc2lyZWRSZXBsaWNhdGlvbkNvdW50IjoxLCJpZCI6Ik5ZQzEifV0sInZlcnNpb24iOjF9LCJtZmFfZW5hYmxlZCI6ZmFsc2UsInN0YXR1cyI6IkFDVElWRSJ9LCJhdXRoZW50aWNhdGlvblR5cGUiOiJzY29wZWRLZXkiLCJzY29wZWRLZXlLZXkiOiJmOTg4MzJhZDZkZmI0Mzk0NWM3MyIsInNjb3BlZEtleVNlY3JldCI6IjhlMTE3NTFlMjE2ZTczYWI4MWIxYWQ5NDkwYjliYWYyN2RiNDVhNjU3NzQzNzVhZTNjMzI2N2U4NDMzODBhNDUiLCJleHAiOjE3NjUxMTQ2OTF9.Gl5_t61lvIF4jds9ZNnXiEZdE_O4E9_imFeuYPiJqEE";
const PINATA_URL: &str = "https://api.pinata.cloud/v3/";

pub async fn get_file_list(jwt: &str) -> Result<serde_json::Value, failure::Error> {
    let client = Client::new();

    let response = client
        .get(format!("{}files", PINATA_URL))
        .header("Authorization", format!("Bearer {}", jwt))
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let response = response.json::<serde_json::Value>().await?;
    Ok(response)
}

pub async fn download_ipfs_file(file_path: &str, cid: &str) -> Result<(), failure::Error> {
    let url = format!("https://ipfs.io/ipfs/{}", cid);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    let body = response.bytes().await?;
    write_file(file_path, body)?;
    Ok(())
}

pub async fn migrate_files(config: &WalterConfig) -> Result<(), Box<dyn Error>> {
    let files = get_file_list(JWT).await?;

    let default_file_download_dir = config.get_default_file_download_dir();
    let download_dir = std::path::Path::new(&default_file_download_dir);
    std::fs::create_dir_all(download_dir)?;
    let mut walrus_client = WalrusClient::new(config.clone());

    if let Some(file_list) = files["data"]["files"].as_array() {
        for file in file_list {
            if let (Some(name), Some(cid)) = (file["name"].as_str(), file["cid"].as_str()) {
                let file_path = download_dir.join(name);

                let file_path = file_path
                    .to_str()
                    .ok_or_else(|| failure::err_msg("Invalid file path"))?;

                download_ipfs_file(file_path, cid).await?;
                walrus_client.upload_file(file_path, None).await?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_migration() {
        let config: WalterConfig = WalterConfig::load_config_file("tests/test_config.json");
        let result = migrate_files(&config).await;
        println!("{:?}", result);
    }
}
