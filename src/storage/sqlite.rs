use std::sync::Arc;

use rusqlite::Connection;

use crate::utils::error::Error;

pub struct SqliteStorage {
    conn: Connection,
}

impl SqliteStorage {
    pub fn new(path: &str) -> Result<Arc<Self>, Error> {
        let conn = Connection::open(path).map_err(|e| Error::Storage(e.to_string()))?;
        
        // Initialize tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS certificates (
                id TEXT PRIMARY KEY,
                subject TEXT NOT NULL,
                issuer TEXT NOT NULL,
                not_before INTEGER NOT NULL,
                not_after INTEGER NOT NULL,
                serial TEXT NOT NULL,
                fingerprint TEXT NOT NULL,
                status TEXT NOT NULL,
                cert_der BLOB,
                private_key_der BLOB,
                created_at INTEGER NOT NULL,
                revoked_at INTEGER
            )",
            [],
        ).map_err(|e| Error::Storage(e.to_string()))?;
        
        Ok(Arc::new(Self { conn }))
    }
    
    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }
}