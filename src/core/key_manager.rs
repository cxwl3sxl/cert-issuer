use std::sync::Arc;

use crate::utils::error::Error;
use crate::utils::crypto::CryptoService;

pub struct KeyManager;

impl KeyManager {
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }
    
    pub fn generate_key(&self, algorithm: &crate::models::certificate::KeyAlgorithm) 
        -> Result<Vec<u8>, Error> {
        let key_pair = CryptoService::generate_key_pair(algorithm)?;
        Ok(key_pair.serialize_der())
    }
}