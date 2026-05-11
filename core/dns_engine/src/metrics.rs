use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Default)]
pub struct Metrics {
    blocked: AtomicU64,
    allowed: AtomicU64,
}

impl Metrics {
    pub fn record_block(&self) {
        self.blocked.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_allow(&self) {
        self.allowed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            blocked: self.blocked.load(Ordering::Relaxed),
            allowed: self.allowed.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetricsSnapshot {
    pub blocked: u64,
    pub allowed: u64,
}
