# Certificate Issuer Service - AGENTS.md

## Project Overview

- **Name**: cert-issuer
- **Type**: Rust web service (certificate authority)
- **Edition**: 2021
- **Port**: 8080

## Developer Commands

```bash
# Build and run
cargo build --release
cargo run --release

# Run tests
cargo test

# Check code
cargo check
cargo clippy
```

## API Endpoints

| Method | Endpoint | Description |
|--------|---------|------------|
| GET | `/api/health` | Health check |
| GET | `/api/certificates` | List certificates |
| POST | `/api/certificates/issue` | Issue certificate |
| GET | `/api/certificates/:id` | Get certificate |
| GET | `/api/certificates/:id/cert` | Download certificate |

## Architecture

```
src/
├── main.rs         # Entry: axum server on :8080
├── api/          # HTTP handlers
│   ├── health.rs
│   └── certificate.rs
├── core/          # Business logic
│   ├── issuer.rs
│   ├── key_manager.rs
│   └── ca.rs
├── models/         # Data types
├── storage/       # SQLite persistence
└── utils/         # crypto, config, error
```

## Key Dependencies

- `axum` 0.7 - HTTP framework
- `rcgen` 0.13 - Certificate generation
- `tokio` 1 - Async runtime
- `rusqlite` 0.31 - SQLite

## Known Issues

- `parse_cert_der` in `utils/crypto.rs` is stubbed (x509-parser API mismatch in rcgen 0.13)
- CA certificate loading uses simplified approach (always creates new CA)
- Some unused imports exist (rcgen::KeyPair in key_manager.rs)

## Running the Service

```bash
cargo run --release
# Output: Certificate Issuer Service started on http://0.0.0.0:8080
```

Then access:
- http://localhost:8080/api/health