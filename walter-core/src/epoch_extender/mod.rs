use super::client::*;

pub async fn epoch_extender(blob_id: &str, epochs: u16) -> Option<bool> {
    let output = download_blob(blob_id, "/tmp/epoch_extender").await;

    if output.is_err() {
        return None;
    }

    let output = upload_blob("/tmp/epoch_extender", epochs).await;

    if output.is_err() {
        return None;
    }

    Some(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_epoch_extender() {
        let output = epoch_extender("DVZWz_QCEb2D_UPQzswv-DUqg-etmV6rEPzoERY4Tgg", 1).await;
        assert!(output.is_some());
        assert_eq!(output.unwrap(), true);
    }
}
