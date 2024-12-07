use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AlreadyCertified {
    pub blobId: String,
    pub endEpoch: u32,
    pub eventOrObject: EventOrObject,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventOrObject {
    Event,
    Object,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct NewlyCreated {
    pub blobObject: BlobObject,
    pub cost: u64,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct BlobObject {
    pub blobId: String,
    pub certifiedEpoch: u32,
    pub deletable: bool,
    pub encodingType: String,
    pub id: String,
    pub registeredEpoch: u32,
    pub size: u64,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct BlobResponse {
    pub content: Vec<BlobEntry>,
    pub pageable: Pageable,
    pub totalPages: u32,
    pub totalElements: u32,
    pub last: bool,
    pub size: u32,
    pub number: u32,
    pub sort: Sort,
    pub numberOfElements: u32,
    pub first: bool,
    pub empty: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct BlobEntry {
    pub blobId: String,
    pub blobIdBase64: String,
    pub objectId: String,
    pub startEpoch: u32,
    pub endEpoch: u32,
    pub size: u64,
    pub timestamp: u64,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Pageable {
    pub pageNumber: u32,
    pub pageSize: u32,
    pub sort: Sort,
    pub offset: u32,
    pub paged: bool,
    pub unpaged: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sort {
    pub sorted: bool,
    pub empty: bool,
    pub unsorted: bool,
}
