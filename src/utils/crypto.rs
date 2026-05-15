use rcgen::{
    BasicConstraints, Certificate, CertificateParams, DistinguishedName, DnType,
    ExtendedKeyUsagePurpose, Ia5String, IsCa, KeyPair,
};

use crate::models::certificate::{CertificateRequest, KeyAlgorithm};
use crate::utils::error::Error;

pub struct CryptoService;

impl CryptoService {
    pub fn generate_key_pair(algorithm: &KeyAlgorithm) -> Result<KeyPair, Error> {
        match algorithm {
            KeyAlgorithm::Rsa2048 | KeyAlgorithm::Rsa4096 => {
                KeyPair::generate_for(&rcgen::PKCS_RSA_SHA256).map_err(|e| Error::KeyOperation(e.to_string()))
            }
            KeyAlgorithm::EcdsaP256 => {
                KeyPair::generate_for(&rcgen::PKCS_ECDSA_P256_SHA256).map_err(|e| Error::KeyOperation(e.to_string()))
            }
            KeyAlgorithm::EcdsaP384 => {
                KeyPair::generate_for(&rcgen::PKCS_ECDSA_P384_SHA384).map_err(|e| Error::KeyOperation(e.to_string()))
            }
        }
    }

    pub fn generate_ca_cert(key_pair: &KeyPair, subject: &str) -> Result<Certificate, Error> {
        let mut cert_params = CertificateParams::default();
        cert_params.distinguished_name = DistinguishedName::new();
        cert_params.distinguished_name.push(DnType::CommonName, subject);
        
        cert_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        
        cert_params.self_signed(key_pair).map_err(|e| Error::CertOperation(e.to_string()))
    }

    pub fn issue_cert(
        ca_key_pair: &KeyPair,
        request: &CertificateRequest,
    ) -> Result<Certificate, Error> {
        let mut cert_params = CertificateParams::default();
        cert_params.distinguished_name = DistinguishedName::new();
        cert_params.distinguished_name.push(DnType::CommonName, request.common_name.clone());
        
        if let Some(ref org) = request.organization {
            cert_params.distinguished_name.push(DnType::OrganizationName, org.clone());
        }
        if let Some(ref ou) = request.organizational_unit {
            cert_params.distinguished_name.push(DnType::OrganizationalUnitName, ou.clone());
        }
        if let Some(ref locality) = request.locality {
            cert_params.distinguished_name.push(DnType::LocalityName, locality.clone());
        }
        if let Some(ref state) = request.state {
            cert_params.distinguished_name.push(DnType::StateOrProvinceName, state.clone());
        }
        if let Some(ref country) = request.country {
            cert_params.distinguished_name.push(DnType::CountryName, country.clone());
        }

        // Set Subject Alternative Names
        if !request.san.is_empty() {
            use rcgen::SanType;
            cert_params.subject_alt_names = request.san
                .iter()
                .filter_map(|s| {
                    if s.parse::<std::net::IpAddr>().is_ok() {
                        Some(SanType::IpAddress(s.parse().unwrap()))
                    } else {
                        Ia5String::try_from(s.as_str()).ok().map(SanType::DnsName)
                    }
                })
                .collect();
        }

        // Extended key usage
        cert_params.extended_key_usages = vec![
            ExtendedKeyUsagePurpose::ServerAuth,
            ExtendedKeyUsagePurpose::ClientAuth,
        ];
        
        // Self-signed cert (simple for testing)
        cert_params.self_signed(ca_key_pair).map_err(|e| Error::CertOperation(e.to_string()))
    }

    pub fn parse_cert_der(_cert_der: &[u8]) -> Result<x509_parser::certificate::X509Certificate<'_>, Error> {
        // TODO: Fix x509_parser FromDer API
        Err(Error::CertOperation("Not implemented".to_string()))
    }

    pub fn get_fingerprint(cert_der: &[u8]) -> String {
        format!("{:x}", md5::compute(cert_der))
    }
}