use super::client::*;
use std::error::Error;

pub async fn extend_epoch(blob_id: &str, epochs: u16) -> Result<(), Box<dyn Error>> {
    let output = download_blob(blob_id, "/tmp/epoch_extender").await;

    if output.is_err() {
        return Err(output.err().unwrap());
    }

    let output = upload_blob("/tmp/epoch_extender", epochs).await;

    if output.is_err() {
        return Err(output.err().unwrap());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_epoch_extender() {
        let output = extend_epoch("DVZWz_QCEb2D_UPQzswv-DUqg-etmV6rEPzoERY4Tgg", 1).await;
        assert!(output.is_ok());
    }
}
