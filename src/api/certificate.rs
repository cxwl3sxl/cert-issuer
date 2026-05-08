use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Response,
    routing::{get, post},
    Json, Router,
};
use rcgen::{Certificate, CertificateParams, DistinguishedName, DnType, KeyPair, PKCS_RSA_SHA256};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct DownloadQuery {
    #[serde(default = "default_format")]
    pub format: String,
    #[serde(default)]
    pub password: Option<String>,
}

fn default_format() -> String {
    "pem".to_string()
}

use crate::models::certificate::{CertificateStatus, KeyAlgorithm};

#[derive(Debug, serde::Serialize, Deserialize)]
pub struct IssueCertRequest {
    pub cn: String,
    #[serde(default)]
    pub o: Option<String>,
    #[serde(default)]
    pub ou: Option<String>,
    #[serde(default)]
    pub l: Option<String>,
    #[serde(default)]
    pub st: Option<String>,
    #[serde(default)]
    pub c: Option<String>,
    #[serde(default = "default_validity_days")]
    pub validity_days: u32,
}

fn default_validity_days() -> u32 {
    365
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CertificateSubject {
    pub cn: String,
    pub o: Option<String>,
    pub ou: Option<String>,
    pub l: Option<String>,
    pub st: Option<String>,
    pub c: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ApiCertificate {
    pub id: String,
    pub subject: CertificateSubject,
    pub issuer: String,
    pub serial_number: String,
    pub not_before: String,
    pub not_after: String,
    pub fingerprint: Option<String>,
    pub status: String,
}

#[derive(Clone)]
pub struct StoredCertificate {
    pub id: String,
    pub subject: String,
    pub issuer: String,
    pub not_before: i64,
    pub not_after: i64,
    pub serial: String,
    pub fingerprint: String,
    pub status: CertificateStatus,
    pub cert_pem: String,
    pub private_key_pem: String,
}

impl From<StoredCertificate> for ApiCertificate {
    fn from(cert: StoredCertificate) -> Self {
        ApiCertificate {
            id: cert.id,
            subject: CertificateSubject {
                cn: cert.subject.clone(),
                o: None,
                ou: None,
                l: None,
                st: None,
                c: None,
            },
            issuer: cert.issuer,
            serial_number: cert.serial,
            not_before: format_timestamp(cert.not_before),
            not_after: format_timestamp(cert.not_after),
            fingerprint: Some(cert.fingerprint),
            status: match cert.status {
                CertificateStatus::Valid => "valid".to_string(),
                CertificateStatus::Revoked => "revoked".to_string(),
                CertificateStatus::Expired => "expired".to_string(),
            },
        }
    }
}

fn format_timestamp(ts: i64) -> String {
    let datetime = chrono::DateTime::from_timestamp(ts, 0)
        .unwrap_or_else(|| chrono::Utc::now());
    datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

// Application state - includes CA key pair
pub struct AppStateInner {
    pub certificates: HashMap<String, StoredCertificate>,
    pub ca_key_pair: KeyPair,
    pub ca_cert: Certificate,
}

pub type AppState = Arc<RwLock<AppStateInner>>;

pub fn create_state() -> AppState {
    // Generate CA key pair - try RSA first, fallback to ECDSA
    let ca_key_pair = match KeyPair::generate_for(&PKCS_RSA_SHA256) {
        Ok(kp) => kp,
        Err(_) => {
            // Fallback: generate a simple key for testing
            // In production, use proper key generation with proper RNG seeding
            tracing::warn!("RSA key generation failed, using ECDSA");
            KeyPair::generate_for(&rcgen::PKCS_ECDSA_P256_SHA256).unwrap_or_else(|_| {
                // Last resort - create a dummy key (this will fail gracefully later)
                panic!("Failed to generate any key pair")
            })
        }
    };
    
    let mut ca_params = CertificateParams::default();
    ca_params.distinguished_name = DistinguishedName::new();
    ca_params.distinguished_name.push(DnType::CommonName, "Certificate Issuer CA");
    ca_params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
    
    let ca_cert = ca_params.self_signed(&ca_key_pair)
        .expect("Failed to generate CA certificate");
    
    Arc::new(RwLock::new(AppStateInner {
        certificates: HashMap::new(),
        ca_key_pair,
        ca_cert,
    }))
}

pub async fn list_certificates(
    State(state): State<AppState>,
) -> Result<Json<Vec<ApiCertificate>>, StatusCode> {
    let state_read = state.read().await;
    let result: Vec<ApiCertificate> = state_read.certificates.values()
        .map(|c| ApiCertificate::from(c.clone()))
        .collect();
    Ok(Json(result))
}

pub async fn get_certificate(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiCertificate>, StatusCode> {
    let state_read = state.read().await;
    match state_read.certificates.get(&id) {
        Some(cert) => Ok(Json(ApiCertificate::from(cert.clone()))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn issue_certificate(
    State(state): State<AppState>,
    Json(req): Json<IssueCertRequest>,
) -> Result<Json<ApiCertificate>, StatusCode> {
    if req.cn.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Generate user key pair - try RSA first, fallback to ECDSA
    let user_key_pair = match KeyPair::generate_for(&PKCS_RSA_SHA256) {
        Ok(kp) => kp,
        Err(_) => {
            match KeyPair::generate_for(&rcgen::PKCS_ECDSA_P256_SHA256) {
                Ok(kp) => kp,
                Err(e) => {
                    tracing::error!("Failed to generate key pair: {}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
        }
    };

    // Build certificate parameters
    let mut cert_params = CertificateParams::default();
    cert_params.distinguished_name = DistinguishedName::new();
    cert_params.distinguished_name.push(DnType::CommonName, req.cn.clone());
    
    if let Some(ref o) = req.o {
        cert_params.distinguished_name.push(DnType::OrganizationName, o.clone());
    }
    if let Some(ref ou) = req.ou {
        cert_params.distinguished_name.push(DnType::OrganizationalUnitName, ou.clone());
    }
    if let Some(ref l) = req.l {
        cert_params.distinguished_name.push(DnType::LocalityName, l.clone());
    }
    if let Some(ref st) = req.st {
        cert_params.distinguished_name.push(DnType::StateOrProvinceName, st.clone());
    }
    if let Some(ref c) = req.c {
        cert_params.distinguished_name.push(DnType::CountryName, c.clone());
    }
    
    // Set validity
    let now = chrono::Utc::now();
    let not_before = now.timestamp();
    let not_after = (now + chrono::Duration::days(req.validity_days as i64)).timestamp();
    
    // Convert to time::OffsetDateTime
    use time::OffsetDateTime;
    use std::time::UNIX_EPOCH;
    
    let not_before_dt = UNIX_EPOCH + std::time::Duration::from_secs(not_before as u64);
    let not_after_dt = UNIX_EPOCH + std::time::Duration::from_secs(not_after as u64);
    
    cert_params.not_before = OffsetDateTime::from(not_before_dt);
    cert_params.not_after = OffsetDateTime::from(not_after_dt);

    // Sign with CA
    let state_read = state.read().await;
    let user_cert = match cert_params.signed_by(&user_key_pair, &state_read.ca_cert, &state_read.ca_key_pair) {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to sign certificate: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    drop(state_read);

    // Get PEM encoded certificate
    let cert_pem = user_cert.pem();
    let private_key_pem = user_key_pair.serialize_pem();
    
    // Calculate fingerprint from DER
    let cert_der = user_cert.der();
    let fingerprint = format!("{:x}", md5::compute(cert_der));
    
    let serial = format!("{:032x}", u128::from_le_bytes(cert_der[..16].try_into().unwrap_or([0; 16])));

    let stored_cert = StoredCertificate {
        id: uuid::Uuid::new_v4().to_string(),
        subject: req.cn.clone(),
        issuer: "Certificate Issuer CA".to_string(),
        not_before,
        not_after,
        serial,
        fingerprint,
        status: CertificateStatus::Valid,
        cert_pem,
        private_key_pem,
    };

    let api_cert = ApiCertificate::from(stored_cert.clone());
    
    let mut state_write = state.write().await;
    state_write.certificates.insert(stored_cert.id.clone(), stored_cert);
    
    Ok(Json(api_cert))
}

pub async fn download_certificate(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<DownloadQuery>,
) -> Result<Response, StatusCode> {
    let state_read = state.read().await;
    match state_read.certificates.get(&id) {
        Some(cert) => {
            let (body, content_type, filename) = match query.format.as_str() {
                "pem" => {
                    // Return the actual certificate PEM as bytes
                    (cert.cert_pem.as_bytes().to_vec(), "application/x-pem-file".to_string(), format!("{}.pem", cert.subject))
                }
                "der" => {
                    // Convert PEM to DER
                    let der = pem_to_der(&cert.cert_pem);
                    (der.as_bytes().to_vec(), "application/x-x509-ca-cert".to_string(), format!("{}.der", cert.subject))
                }
                "nginx" => {
                    let body = format!(
                        "# Nginx SSL configuration for {}\n# \n# 1. Save your certificate as /etc/nginx/ssl/{}.pem\n# 2. Save your private key as /etc/nginx/ssl/{}.key\n# 3. Use the configuration below:\n\nserver {{\n    listen 443 ssl;\n    server_name {};\n\n    ssl_certificate /etc/nginx/ssl/{}.pem;\n    ssl_certificate_key /etc/nginx/ssl/{}.key;\n\n    ssl_protocols TLSv1.2 TLSv1.3;\n    ssl_ciphers HIGH:!aNULL:!MD5;\n}}",
                        cert.subject, cert.subject, cert.subject, cert.subject, cert.subject, cert.subject
                    );
                    (body.into_bytes(), "text/plain".to_string(), format!("{}-nginx.conf", cert.subject))
                }
                "iis" => {
                    let body = format!(
                        "# IIS Certificate Import Instructions for {}\n#\n# Step 1: Convert to PFX format (requires OpenSSL)\n#   openssl pkcs12 -export -out {}.pfx -inkey private_key.pem -in certificate.pem\n#\n# Step 2: Import to Windows Server/IIS\n#   - Open IIS Manager\n#   - Select server node\n#   - Open \"Server Certificates\"\n#   - Click \"Import\"\n#   - Select the .pfx file and enter password\n#\n# Certificate Details:\n#   Subject: {}\n#   Serial: {}\n#   Valid From: {}\n#   Valid Until: {}",
                        cert.subject, cert.subject, cert.subject, cert.serial, 
                        format_timestamp(cert.not_before), format_timestamp(cert.not_after)
                    );
                    (body.into_bytes(), "text/plain".to_string(), format!("{}-iis.txt", cert.subject))
                }
                "pfx" | "iis" => {
                    let password = query.password.as_deref().unwrap_or("changeit");
                    
                    // Generate real binary PFX using p12 crate
                    let pfx_result = create_pfx(&cert.cert_pem, &cert.private_key_pem, password, &cert.subject);
                    
                    match pfx_result {
                        Ok(pfx_der) => {
                            // Return binary PFX file - convert to base64 for HTTP transfer
                            let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &pfx_der);
                            (encoded.into_bytes(), "application/x-pkcs12".to_string(), format!("{}.pfx", cert.subject))
                        }
                        Err(e) => {
                            tracing::error!("Failed to generate PFX: {}", e);
                            // Fallback: combined PEM with instructions
                            let combined = format!(
                                "# =============================================\n# IIS/Windows Certificate Bundle (Fallback)\n# Subject: {}\n# Password: {}\n# =============================================\n#\n# The PFX generation failed. Please use this PEM file instead.\n#\n# How to use (PowerShell):\n#   $pem = Get-Content '{}' -Raw\n#   $cert = New-Object System.Security.Cryptography.X509Certificates.X509Certificate2\n#   $cert.Import([Text.Encoding]::UTF8.GetBytes($pem), '{}', 'Exportable')\n#   [System.IO.File]::WriteAllBytes('{}.pfx', $cert.Export('Pfx', '{}'))\n#\n# =============================================\n# CERTIFICATE\n{}\n#\n# PRIVATE KEY\n{}\n",
                                cert.subject, password,
                                cert.subject, password,
                                cert.subject, password,
                                cert.cert_pem,
                                cert.private_key_pem
                            );
                            (combined.into_bytes(), "application/x-pem-file".to_string(), format!("{}-iis.pem", cert.subject))
                        }
                    }
                }
                _ => {
                    (cert.cert_pem.as_bytes().to_vec(), "application/x-pem-file".to_string(), format!("{}.pem", cert.subject))
                }
            };

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", content_type)
                .header("Content-Disposition", format!("attachment; filename=\"{}\"", filename))
                .body(axum::body::Body::from(body))
                .unwrap())
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

// Helper function to convert PEM to DER
fn pem_to_der(pem: &str) -> String {
    // Simple base64 encode of the PEM content (placeholder for real DER)
    // In production, would properly parse PEM and extract DER
    let content = pem.lines()
        .filter(|l| !l.starts_with("-----"))
        .collect::<Vec<_>>()
        .join("");
    format!("[DER bytes - {} bytes]", content.len())
}

// Create real PKCS#12 (PFX) format
fn create_pfx(cert_pem: &str, key_pem: &str, password: &str, _name: &str) -> Result<Vec<u8>, String> {
    // Parse certificate from PEM
    let cert_pem_obj = pem::parse(cert_pem.as_bytes())
        .map_err(|e| format!("Failed to parse certificate: {}", e))?;
    let cert_der = cert_pem_obj.contents().to_vec();
    
    // Parse private key from PEM
    let key_pem_obj = pem::parse(key_pem.as_bytes())
        .map_err(|e| format!("Failed to parse private key: {}", e))?;
    
    // Check key format and convert if needed
    let key_der = match key_pem_obj.tag() {
        "RSA PRIVATE KEY" => {
            // PKCS#1 format - need to convert to PKCS#8
            convert_pkcs1_to_pkcs8(key_pem_obj.contents())?
        }
        "PRIVATE KEY" => {
            // Already PKCS#8 format
            key_pem_obj.contents().to_vec()
        }
        _ => {
            return Err(format!("Unknown key format: {}", key_pem_obj.tag()));
        }
    };
    
    // Create PFX using the p12 crate
    let pfx = p12::PFX::new(&cert_der, &key_der, None, password, "certificate")
        .ok_or_else(|| "Failed to create PFX structure".to_string())?;
    
    // Use to_der() method to get DER bytes
    let der_bytes = pfx.to_der();
    
    Ok(der_bytes)
}

// Convert PKCS#1 (RSA PRIVATE KEY) to PKCS#8 (PRIVATE KEY) format
fn convert_pkcs1_to_pkcs8(pkcs1_key: &[u8]) -> Result<Vec<u8>, String> {
    use der::Encode;
    
    // RSA algorithm OID: 1.2.840.113549.1.1.1
    // Parameters: NULL
    let rsa_oid = [0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01];
    let null_oid = [0x05, 0x00];
    
    // Build AlgorithmIdentifier SEQUENCE
    let algorithm_der = [
        0x30, 0x0D,  // SEQUENCE length 13
        0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01,  // OID
        0x05, 0x00,  // NULL
    ].to_vec();
    
    // Build PrivateKeyInfo
    // Version = 0 (INTEGER)
    // Algorithm = AlgorithmIdentifier
    // PrivateKey = OCTET STRING containing PKCS#1 key
    
    let mut pkcs8 = Vec::new();
    
    // SEQUENCE
    pkcs8.push(0x30);
    
    // Calculate total length
    // version (3) + algorithm (15) + privateKey (2 + key len)
    let total_len = 3 + algorithm_der.len() + 2 + pkcs1_key.len();
    let tag = if total_len > 127 { 0x82 } else { 0x04 };
    if total_len > 127 {
        pkcs8.push((total_len >> 8) as u8);
    }
    pkcs8.push(total_len as u8);
    
    // Version INTEGER
    pkcs8.extend_from_slice(&[0x02, 0x01, 0x00]); // version = 0
    
    // AlgorithmIdentifier
    pkcs8.extend_from_slice(&algorithm_der);
    
    // PrivateKey OCTET STRING
    pkcs8.push(0x04); // OCTET STRING
    if pkcs1_key.len() > 127 {
        pkcs8.push(0x82);
        pkcs8.push((pkcs1_key.len() >> 8) as u8);
        pkcs8.push(pkcs1_key.len() as u8);
    } else {
        pkcs8.push(pkcs1_key.len() as u8);
    }
    pkcs8.extend_from_slice(pkcs1_key);
    
    Ok(pkcs8)
}

fn generate_fingerprint() -> String {
    let bytes: [u8; 32] = rand::random();
    bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(":")
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/certificates", get(list_certificates))
        .route("/api/certificates/issue", post(issue_certificate))
        .route("/api/certificates/:id", get(get_certificate))
        .route("/api/certificates/:id/cert", get(download_certificate))
        .with_state(state)
}