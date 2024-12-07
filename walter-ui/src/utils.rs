use serde::{Deserialize, Serialize};

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
