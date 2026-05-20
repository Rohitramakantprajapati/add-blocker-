use std::collections::HashSet;
use std::path::Path;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ControllerError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("libbpf error: {0}")]
    Libbpf(String),
}

pub type Result<T> = std::result::Result<T, ControllerError>;

pub struct Controller {
    blocked: HashSet<u64>,
    loaded_object: Option<std::path::PathBuf>,
}

impl Controller {
    pub fn new() -> Self {
        Self { blocked: HashSet::new(), loaded_object: None }
    }

    pub fn sync_blocklist(&mut self, domains: &[String]) {
        for domain in domains {
            self.blocked.insert(hash_domain(domain));
        }
    }

    /// Load the compiled BPF object and cache the map handle for updates.
    pub fn load_object(&mut self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(ControllerError::Libbpf(format!("BPF object not found: {}", path.display())));
        }
        self.loaded_object = Some(path.to_path_buf());
        Ok(())
    }

    /// Insert a 64-bit domain hash into the blocked map.
    pub fn insert_hash(&mut self, hash: u64) -> Result<()> {
        self.blocked.insert(hash);
        Ok(())
    }
}

pub fn hash_domain(domain: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in domain.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
