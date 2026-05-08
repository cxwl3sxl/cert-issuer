use std::sync::Arc;

use crate::models::certificate::{CertificateRequest, CertificateStatus};
use crate::models::certificate::Certificate as CertModel;
use crate::utils::crypto::CryptoService;
use crate::utils::error::Error;

pub struct Ca {
    key_pair: rcgen::KeyPair,
    ca_cert: rcgen::Certificate,
}

impl Ca {
    pub fn new(key_pair: rcgen::KeyPair, ca_cert: rcgen::Certificate) -> Arc<Self> {
        Arc::new(Self { key_pair, ca_cert })
    }
    
    pub fn get_ca_cert(&self) -> &rcgen::Certificate {
        &self.ca_cert
    }
    
    pub fn get_key_pair(&self) -> &rcgen::KeyPair {
        &self.key_pair
    }
}

pub struct Issuer {
    ca: Arc<Ca>,
}

impl Issuer {
    pub fn new(ca: Arc<Ca>) -> Arc<Self> {
        Arc::new(Self { ca })
    }
    
    pub fn issue(&self, request: &CertificateRequest) -> Result<CertModel, Error> {
        // Issue the certificate using CA
        let cert = CryptoService::issue_cert(
            self.ca.get_key_pair(),
            request,
        )?;
        
        // Get certificate DER bytes
        let cert_der = cert.der();
        
        // Parse and extract info  
        let parsed_cert = CryptoService::parse_cert_der(cert_der)?;
        let fingerprint = CryptoService::get_fingerprint(cert_der);
        
        let not_before = parsed_cert.validity().not_before.timestamp();
        let not_after = parsed_cert.validity().not_after.timestamp();
        
        let cert_model = CertModel {
            id: uuid::Uuid::new_v4().to_string(),
            subject: request.common_name.clone(),
            issuer: "Certificate Issuer CA".to_string(),
            not_before,
            not_after,
            serial: "serial_placeholder".to_string(),
            fingerprint,
            status: CertificateStatus::Valid,
        };
        
        Ok(cert_model)
    }
}