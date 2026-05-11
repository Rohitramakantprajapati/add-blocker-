use std::collections::HashSet;
use std::path::Path;

use libbpf_rs::{Map, Object, ObjectBuilder};

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
    obj: Option<Object>,
    map: Option<Map>,
}

impl Controller {
    pub fn new() -> Self {
        Self { blocked: HashSet::new(), obj: None, map: None }
    }

    pub fn sync_blocklist(&mut self, domains: &[String]) {
        for domain in domains {
            self.blocked.insert(hash_domain(domain));
        }
    }

    /// Load the compiled BPF object and cache the map handle for updates.
    pub fn load_object(&mut self, path: &Path) -> Result<()> {
        let obj = ObjectBuilder::default()
            .open_file(path)
            .map_err(|e| ControllerError::Libbpf(e.to_string()))?;

        let mut obj = obj
            .load()
            .map_err(|e| ControllerError::Libbpf(e.to_string()))?;

        let map = obj
            .map("blocked_domains")
            .ok_or_else(|| ControllerError::Libbpf("missing blocked_domains map".to_string()))?;

        self.map = Some(map);
        self.obj = Some(obj);
        Ok(())
    }

    /// Insert a 64-bit domain hash into the blocked map.
    pub fn insert_hash(&mut self, hash: u64) -> Result<()> {
        let map = match &self.map {
            Some(m) => m,
            None => return Err(ControllerError::Libbpf("map not loaded".to_string())),
        };

        let key = hash.to_ne_bytes();
        let val = [1u8];

        map.update(&key, &val, 0)
            .map_err(|e| ControllerError::Libbpf(e.to_string()))?;

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
