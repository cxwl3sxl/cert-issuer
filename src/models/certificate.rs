use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    pub id: String,
    pub subject: String,
    pub issuer: String,
    pub not_before: i64,
    pub not_after: i64,
    pub serial: String,
    pub fingerprint: String,
    pub status: CertificateStatus,
    // PEM编码的证书，用于下载
    pub cert_pem: String,
    // 私钥PEM（用于PFX生成）
    pub private_key_pem: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CertificateStatus {
    Valid,
    Revoked,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateRequest {
    pub common_name: String,
    pub organization: Option<String>,
    pub organizational_unit: Option<String>,
    pub locality: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub email: Option<String>,
    pub san: Vec<String>,
    pub key_algorithm: KeyAlgorithm,
    pub validity_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KeyAlgorithm {
    Rsa2048,
    Rsa4096,
    EcdsaP256,
    EcdsaP384,
}

impl Default for KeyAlgorithm {
    fn default() -> Self {
        KeyAlgorithm::Rsa2048
    }
}