use std::path::Path;
use std::sync::Arc;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::{BlockDecision, Result};

pub struct Blocklist {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl Blocklist {
    pub fn open(db_path: &Path) -> Result<Self> {
        let manager = SqliteConnectionManager::file(db_path).with_flags(
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
        );
        let pool = Pool::builder().max_size(16).build(manager)?;
        {
            let conn = pool.get()?;
            conn.execute_batch("PRAGMA query_only = ON; PRAGMA temp_store = MEMORY;")?;
        }
        Ok(Self { pool: Arc::new(pool) })
    }

    pub fn check(&self, domain: &str) -> Result<BlockDecision> {
        let normalized = domain.trim().trim_end_matches('.').to_ascii_lowercase();
        if normalized.is_empty() {
            return Ok(BlockDecision::Allow);
        }

        let conn = self.pool.get()?;
        let parts: Vec<&str> = normalized.split('.').collect();
        for index in 0..parts.len() {
            let candidate = parts[index..].join(".");
            let present = conn
                .query_row(
                    "SELECT 1 FROM blocked WHERE domain = ?1 LIMIT 1",
                    rusqlite::params![candidate],
                    |row| row.get::<_, i64>(0),
                )
                .is_ok();
            if present {
                return Ok(BlockDecision::Block);
            }
        }

        Ok(BlockDecision::Allow)
    }
}
