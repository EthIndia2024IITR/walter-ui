use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Event {
    pub txDigest: String,
    pub eventSeq: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct EventOrObject {
    pub Event: Event,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct AlreadyCertified {
    pub blobId: String,
    pub eventOrObject: EventOrObject,
    pub endEpoch: u32,
}
// Define a struct matching the JSON structure
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Storage {
    pub id: String,
    pub startEpoch: u32,
    pub endEpoch: u32,
    pub storageSize: u64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct BlobObject {
    pub id: String,
    pub registeredEpoch: u32,
    pub blobId: String,
    pub size: u32,
    pub encodingType: String,
    pub certifiedEpoch: u32,
    pub storage: Storage,
    pub deletable: bool,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ResourceOperation {
    RegisterFromScratch: RegisterFromScratch,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct RegisterFromScratch {
    encoded_length: u64,
    epochs_ahead: u32,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct NewlyCreated {
    pub blobObject: BlobObject,
    pub resourceOperation: ResourceOperation,
    pub cost: u32,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct WalrusNewlyCreated {
    pub newlyCreated: NewlyCreated,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct WalrusAlreadyCertified {
    pub alreadyCertified: AlreadyCertified,
}

#[derive(Deserialize, Debug)]
pub enum WalrusResponse {
    NewlyCreated(WalrusNewlyCreated),
    AlreadyCertified(WalrusAlreadyCertified),
}
