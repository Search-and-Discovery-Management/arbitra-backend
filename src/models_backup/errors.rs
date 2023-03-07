use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorTypes {
    #[error("Index [{0}] not found")]
    IndexNotFound(String),
    #[error("Document ID [{0}] not found")]
    DocumentNotFound(String),
    #[error("Bad data request")]
    BadDataRequest,
    #[error("Failed to create new index, index [{0}] already exists")]
    IndexExists(String),
    #[error("Server currently unavailable")]
    ServerDown,
    #[error("Unknown error occured")]
    Unknown
}