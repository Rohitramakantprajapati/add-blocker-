pub mod blocklist;
pub mod cache;
pub mod doh;
pub mod metrics;
pub mod resolver;

use std::path::Path;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, VoidBlockError>;

#[derive(Debug, Error)]
pub enum VoidBlockError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("connection pool error: {0}")]
    Pool(#[from] r2d2::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("invalid DNS query: {0}")]
    InvalidQuery(String),
    #[error("resolver error: {0}")]
    Resolver(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockDecision {
    Block,
    Allow,
}

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub bind_addr: std::net::SocketAddr,
    pub blocklist_db: std::path::PathBuf,
    pub doh_endpoints: Vec<reqwest::Url>,
    pub cache_capacity: usize,
}

impl Default for EngineConfig {
    fn default() -> Self {
        let mut doh_endpoints = Vec::new();
        if let Ok(url) = reqwest::Url::parse("https://cloudflare-dns.com/dns-query") {
            doh_endpoints.push(url);
        }
        if let Ok(url) = reqwest::Url::parse("https://dns.quad9.net/dns-query") {
            doh_endpoints.push(url);
        }

        Self {
            bind_addr: std::net::SocketAddr::from(([127, 0, 0, 1], 53)),
            blocklist_db: Path::new("blocklists/voidblock.db").to_path_buf(),
            doh_endpoints,
            cache_capacity: 4096,
        }
    }
}

pub struct VoidBlockEngine {
    pub blocklist: blocklist::Blocklist,
    pub cache: cache::DnsCache,
    pub metrics: metrics::Metrics,
    pub doh: doh::DohClient,
}

impl VoidBlockEngine {
    pub fn open(config: &EngineConfig) -> Result<Self> {
        let blocklist = blocklist::Blocklist::open(&config.blocklist_db)?;
        let cache = cache::DnsCache::new(config.cache_capacity)?;
        let metrics = metrics::Metrics::default();
        let doh = doh::DohClient::new(config.doh_endpoints.clone())?;
        Ok(Self { blocklist, cache, metrics, doh })
    }

    pub async fn handle_query(&self, packet: &[u8]) -> Result<Vec<u8>> {
        let question = resolver::extract_question(packet)?;
        match self.blocklist.check(&question.domain)? {
            BlockDecision::Block => {
                self.metrics.record_block();
                Ok(resolver::build_nxdomain_response(packet)?)
            }
            BlockDecision::Allow => {
                if let Some(cached) = self.cache.get(packet) {
                    self.metrics.record_allow();
                    return Ok(cached);
                }

                let response = self.doh.forward(packet).await?;
                self.cache.insert(packet.to_vec(), response.clone(), question.ttl_seconds);
                self.metrics.record_allow();
                Ok(response)
            }
        }
    }
}
