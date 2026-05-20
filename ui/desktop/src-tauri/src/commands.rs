use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum VoidBlockError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsSnapshot {
    pub blocked: u64,
    pub latency_ms: u64,
    pub memory_mb: u64,
}

#[tauri::command]
pub fn toggle_blocking(enabled: bool) -> Result<(), VoidBlockError> {
    if enabled {
        Ok(())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub fn get_stats() -> Result<StatsSnapshot, VoidBlockError> {
    Ok(StatsSnapshot {
        blocked: 42,
        latency_ms: 1,
        memory_mb: 12,
    })
}

#[tauri::command]
pub fn update_blocklist() -> Result<(), VoidBlockError> {
    Ok(())
}

#[tauri::command]
pub fn add_to_allowlist(domain: String) -> Result<(), VoidBlockError> {
    let trimmed = domain.trim();
    if trimmed.is_empty() {
        return Err(VoidBlockError::InvalidInput("domain must not be empty".to_string()));
    }
    Ok(())
}
