# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Certificate Issuer Service - A Rust-based certificate authority web service that issues and manages X.509 certificates with a Vue.js frontend.

**Core Technologies:**
- Backend: Rust with Axum web framework
- Frontend: Vue 3 with TypeScript
- Certificate Generation: rcgen library
- Database: SQLite (rusqlite)
- Certificate Formats: PEM, DER, PFX/PKCS#12

## Development Commands

### Rust Backend
```bash
# Build and run
cargo build --release
cargo run --release

# Development mode
cargo run

# Run tests
cargo test

# Check code quality
cargo check
cargo clippy

# Format code
cargo fmt
```

### Frontend (Vue.js)
```bash
cd cert-issuer-web

# Install dependencies
npm install

# Development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

## API Endpoints

| Method | Endpoint | Description |
|--------|---------|------------|
| GET | `/api/health` | Health check |
| GET | `/api/certificates` | List all certificates |
| POST | `/api/certificates/issue` | Issue new certificate |
| GET | `/api/certificates/:id` | Get certificate details |
| GET | `/api/certificates/:id/cert` | Download certificate |

**Certificate Download Formats:**
- `?format=pem` - PEM format (default)
- `?format=der` - DER binary format
- `?format=pfx` - PKCS#12/PFX format (requires `?password=xxx`)
- `?format=nginx` - Nginx configuration template
- `?format=iis` - IIS import instructions

## Architecture

### Backend Structure
```
src/
├── main.rs              # Application entry point (port 8080)
├── api/                 # HTTP handlers
│   ├── mod.rs          # API module exports
│   ├── health.rs       # Health check endpoint
│   └── certificate.rs  # Certificate management endpoints
├── core/               # Business logic
│   ├── mod.rs          # Core module exports
│   ├── ca.rs          # Certificate Authority operations
│   ├── issuer.rs      # Certificate issuance logic
│   └── key_manager.rs # Key pair management
├── models/            # Data structures
│   ├── mod.rs         # Model exports
│   └── certificate.rs # Certificate data types
├── storage/           # Data persistence
│   ├── mod.rs         # Storage exports
│   └── sqlite.rs      # SQLite implementation
└── utils/             # Utilities
    ├── mod.rs         # Utility exports
    ├── config.rs      # Configuration management
    ├── crypto.rs      # Cryptographic utilities
    └── error.rs       # Error handling
```

### Frontend Structure
```
cert-issuer-web/
├── src/
│   ├── assets/        # Static assets
│   ├── components/    # Vue components
│   ├── router/        # Vue Router configuration
│   ├── stores/        # State management
│   ├── views/         # Page components
│   ├── App.vue        # Root component
│   └── main.ts        # Application entry point
├── index.html         # HTML template
└── package.json       # Dependencies and scripts
```

## Key Dependencies

### Backend
- `axum` 0.7 - HTTP web framework
- `tokio` 1 - Async runtime
- `rcgen` 0.13 - Certificate generation
- `rusqlite` 0.31 - SQLite database
- `serde` 1 - Serialization/deserialization
- `tower-http` 0.5 - HTTP middleware (CORS, tracing)
- `p12` 0.5 - PKCS#12/PFX generation

### Frontend
- `vue` 3.4 - Frontend framework
- `vue-router` 4.3 - Routing
- `axios` 1.6 - HTTP client
- `typescript` 5.4 - Type safety

## Configuration

The service runs on port 8080 by default and provides:
- CORS enabled for all origins
- In-memory certificate storage (no persistent database)
- Automatic CA certificate generation on startup
- Support for RSA and ECDSA key algorithms

## Known Issues and Limitations

1. **Certificate Storage**: Currently uses in-memory HashMap instead of persistent SQLite storage
2. **CA Certificate**: Generates new CA on each startup instead of loading from persistent storage
3. **Key Generation**: Falls back from RSA to ECDSA if RSA generation fails
4. **PEM to DER Conversion**: Simplified implementation in `pem_to_der()` function
5. **Fingerprint Generation**: Uses MD5 instead of more secure SHA-256

## Certificate Request Format

```json
{
  "cn": "example.com",
  "o": "Organization Name",
  "ou": "Organizational Unit",
  "l": "City",
  "st": "State",
  "c": "US",
  "validity_days": 365
}
```

## Running the Complete Application

1. **Start Backend:**
   ```bash
   cargo run --release
   # Service starts on http://0.0.0.0:8080
   ```

2. **Start Frontend:**
   ```bash
   cd cert-issuer-web
   npm run dev
   # Frontend typically runs on http://localhost:5173
   ```

3. **Test Health Endpoint:**
   ```bash
   curl http://localhost:8080/api/health
   ```

4. **Issue Certificate:**
   ```bash
   curl -X POST http://localhost:8080/api/certificates/issue \
     -H "Content-Type: application/json" \
     -d '{"cn": "example.com", "validity_days": 365}'
   ```