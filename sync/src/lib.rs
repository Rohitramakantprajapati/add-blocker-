pub mod crypto;
pub mod delta;
pub mod gossip;
pub mod peers;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, VoidBlockError>;

#[derive(Debug, Error)]
pub enum VoidBlockError {
    #[error("serialization error: {0}")]
    Serialize(#[from] bincode::Error),
    #[error("signature error: {0}")]
    Signature(String),
    #[error("crypto error: {0}")]
    Crypto(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
