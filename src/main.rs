pub mod api;
pub mod core;
pub mod models;
pub mod storage;
pub mod utils;

use clap::Parser;
use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[derive(Parser)]
#[command(name = "cert-issuer", about = "Certificate Authority Service")]
struct Args {
    /// Server address to bind to (default: 0.0.0.0:8080)
    #[arg(short, long, default_value = "0.0.0.0:8080")]
    addr: String,

    /// Root CA certificate path (optional - generates new if not provided)
    #[arg(short, long)]
    ca_cert: Option<String>,

    /// Root CA private key path (required if --ca-cert is specified)
    #[arg(long)]
    ca_key: Option<String>,

    /// Data storage directory (default: ./data)
    #[arg(short, long, default_value = "./data")]
    data_dir: String,

    /// Verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let args = Args::parse();
    let data_dir = &args.data_dir;

    // Create data directories
    std::fs::create_dir_all(data_dir)?;
    std::fs::create_dir_all(format!("{}/certificates", data_dir))?;

    // Handle CA configuration
    if let (Some(cert_path), Some(key_path)) = (&args.ca_cert, &args.ca_key) {
        if std::path::Path::new(cert_path).exists() && std::path::Path::new(key_path).exists() {
            println!("✓ Using existing CA files:");
            println!("  CA Key: {}", key_path);
            println!("  CA Cert: {}", cert_path);
        } else {
            return Err("CA files must exist when specified".into());
        }
    } else if args.ca_cert.is_some() || args.ca_key.is_some() {
        return Err("Both CA key and certificate must be specified".into());
    } else {
        println!("ℹ No existing CA found, will generate new CA");
        println!("  CA will be stored at:");
        println!("  Key: {}/ca.key", data_dir);
        println!("  Cert: {}/ca.crt", data_dir);
    }

    println!("Certificate Issuer Service Configuration:");
    println!("========================================");
    println!("Server Address: {}", args.addr);
    println!("Data Directory: {}", data_dir);
    println!("========================================");

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    // Create application state (with optional external CA)
    let state = api::certificate::create_state(args.ca_cert, args.ca_key);

    let app = Router::new()
        .merge(api::health::router())
        .merge(api::certificate::router(state))
        .layer(cors);

    let addr: SocketAddr = args.addr.parse()?;
    println!("Certificate Issuer Service started on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}