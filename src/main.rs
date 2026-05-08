pub mod api;
pub mod core;
pub mod models;
pub mod storage;
pub mod utils;

use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);
    
    let app = Router::new()
        .merge(api::health::router())
        .merge(api::certificate::router())
        .layer(cors);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Certificate Issuer Service started on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}