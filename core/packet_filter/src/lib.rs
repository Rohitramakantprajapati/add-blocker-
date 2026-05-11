pub mod cosmetic;
pub mod rules;
pub mod sni;
pub mod url_matcher;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, VoidBlockError>;

#[derive(Debug, Error)]
pub enum VoidBlockError {
    #[error("pattern error: {0}")]
    Pattern(String),
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("invalid TLS client hello: {0}")]
    InvalidHello(String),
}
