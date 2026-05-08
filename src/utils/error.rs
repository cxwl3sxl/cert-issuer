use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Certificate not found: {0}")]
    NotFound(String),
    
    #[error("Invalid certificate request: {0}")]
    InvalidRequest(String),
    
    #[error("Certificate operation failed: {0}")]
    CertOperation(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Key operation failed: {0}")]
    KeyOperation(String),
    
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}