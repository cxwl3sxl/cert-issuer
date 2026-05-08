use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Response,
    routing::get,
    Json, Router,
};
use serde::Deserialize;

use crate::models::certificate::{Certificate, CertificateRequest};

#[derive(Debug, serde::Serialize, Deserialize)]
pub struct IssueCertRequest {
    #[serde(flatten)]
    pub cert_request: CertificateRequest,
}

#[derive(Debug, serde::Serialize, Deserialize)]
pub struct CertResponse {
    pub certificate: Certificate,
}

#[derive(Debug, serde::Serialize, Deserialize)]
pub struct CertListResponse {
    pub certificates: Vec<Certificate>,
    pub total: usize,
}

#[derive(Debug, Deserialize)]
pub struct DownloadCertQuery {
    pub format: String,
    #[serde(default)]
    pub password: Option<String>,
}

// Placeholder handlers
pub async fn issue_certificate(
    Json(_req): Json<IssueCertRequest>,
) -> Result<Json<CertResponse>, StatusCode> {
    // TODO: 实现证书颁发
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn get_certificate(
    Path(_id): Path<String>,
) -> Result<Json<CertResponse>, StatusCode> {
    // TODO: 实现获取证书
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn download_certificate(
    Path(_id): Path<String>,
    Query(_query): Query<DownloadCertQuery>,
) -> Result<Response, StatusCode> {
    // TODO: 实现证书下载(支持多格式: nginx, iis)
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn revoke_certificate(
    Path(_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // TODO: 实现证书吊销
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn list_certificates() -> Result<Json<CertListResponse>, StatusCode> {
    // TODO: 实现证书列表
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub fn router() -> Router {
    Router::new()
        .route("/api/certificates/issue", get(issue_certificate))
        .route("/api/certificates", get(list_certificates))
        .route("/api/certificates/:id", get(get_certificate))
        .route("/api/certificates/:id/cert", get(download_certificate))
        .route("/api/certificates/:id/revoke", get(revoke_certificate))
}