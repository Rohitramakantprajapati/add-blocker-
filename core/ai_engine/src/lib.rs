pub mod features;
pub mod model;
pub mod runner;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, VoidBlockError>;

#[derive(Debug, Error)]
pub enum VoidBlockError {
    #[error("model error: {0}")]
    Model(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[cfg(feature = "onnx")]
    #[error("ONNX runtime error: {0}")]
    Onnx(#[from] ort::Error),
}
