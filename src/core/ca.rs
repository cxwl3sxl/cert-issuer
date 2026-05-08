use std::sync::Arc;

use rcgen::KeyPair;

use crate::core::key_manager::KeyManager;
use crate::core::issuer::Ca;
use crate::utils::crypto::CryptoService;
use crate::utils::error::Error;

pub fn load_or_create_ca(
    ca_key_path: &str,
    ca_cert_path: &str,
) -> Result<Arc<Ca>, Error> {
    let key_manager = KeyManager::new();
    let ca_key_der = key_manager.generate_key(&crate::models::certificate::KeyAlgorithm::EcdsaP256)?;
    let ca_key_pem = String::from_utf8(ca_key_der).map_err(|e| Error::KeyOperation(e.to_string()))?;
    let ca_key_pair = KeyPair::from_pem(&ca_key_pem).map_err(|e| Error::KeyOperation(e.to_string()))?;
    
    let ca_cert = CryptoService::generate_ca_cert(&ca_key_pair, "Certificate Issuer CA")?;
    
    let _ = std::fs::write(ca_key_path, ca_key_pair.serialize_pem());
    let _ = std::fs::write(ca_cert_path, ca_cert.pem());
    
    Ok(Ca::new(ca_key_pair, ca_cert))
}