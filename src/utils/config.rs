use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing config: {0}")]
    Missing(String),
    
    #[error("Invalid config: {0}")]
    Invalid(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct Config {
    pub server_addr: String,
    pub database_path: String,
    pub cert_storage_path: String,
    pub ca_key_path: String,
    pub ca_cert_path: String,
}

impl Config {
    pub fn default() -> Self {
        Self {
            server_addr: "0.0.0.0:8080".to_string(),
            database_path: "./data/certs.db".to_string(),
            cert_storage_path: "./data/certs".to_string(),
            ca_key_path: "./data/ca.key".to_string(),
            ca_cert_path: "./data/ca.crt".to_string(),
        }
    }
}