use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorTypes {
    #[error("Application ID [{0}] not found")]
    ApplicationNotFound(String),
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

#[derive(Error, Debug)]
pub enum FileErrorTypes {
    #[error("Found inconsistent data at line [{0}]")]
    InvalidLine(usize),
    #[error("File Too Large, found [{0}] of maximum [{0}]")]
    FileTooLarge(usize, usize),
    #[error("Invalid file extension, valid file extensions: [{0}]")]
    InvalidFileExtension(String),
    #[error("Invalid [{0}] File")]
    InvalidFile(String)
}