use std::num::NonZeroUsize;
use std::time::{Duration, Instant};
use std::sync::Mutex;

use lru::LruCache;

use crate::Result;

#[derive(Clone)]
struct CachedResponse {
    response: Vec<u8>,
    expires_at: Instant,
}

pub struct DnsCache {
    inner: Mutex<LruCache<Vec<u8>, CachedResponse>>,
}

impl DnsCache {
    pub fn new(capacity: usize) -> Result<Self> {
        let size = NonZeroUsize::new(capacity.max(1)).ok_or_else(|| {
            crate::VoidBlockError::Resolver("cache capacity must be non-zero".to_string())
        })?;
        Ok(Self { inner: Mutex::new(LruCache::new(size)) })
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let mut guard = self.inner.lock().ok()?;
        guard.get(key).and_then(|entry| {
            if Instant::now() <= entry.expires_at {
                Some(entry.response.clone())
            } else {
                None
            }
        })
    }

    pub fn insert(&self, key: Vec<u8>, response: Vec<u8>, ttl_seconds: u32) -> Result<()> {
        let expires_at = Instant::now() + Duration::from_secs(u64::from(ttl_seconds.max(1)));
        let mut guard = self.inner.lock()
            .map_err(|e| crate::VoidBlockError::Resolver(format!("cache lock poisoned: {}", e)))?;
        guard.put(key, CachedResponse { response, expires_at });
        Ok(())
    }
}
