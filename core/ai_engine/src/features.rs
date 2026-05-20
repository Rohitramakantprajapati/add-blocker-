use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FeatureVector {
    pub url_entropy: f32,
    pub domain_age_proxy: f32,
    pub cname_depth: f32,
    pub response_size: f32,
    pub timing_ms: f32,
}

impl FeatureVector {
    pub fn from_observations(url: &str, cname_depth: u32, response_size: usize, timing_ms: f32) -> Self {
        let url_entropy = shannon_entropy(url);
        let domain_age_proxy = domain_age_proxy(url);
        Self {
            url_entropy: (url_entropy / 8.0).clamp(0.0, 1.0),
            domain_age_proxy,
            cname_depth: (cname_depth as f32 / 6.0).clamp(0.0, 1.0),
            response_size: ((response_size as f32) / 10_000.0).clamp(0.0, 1.0),
            timing_ms: (timing_ms / 30.0).clamp(0.0, 1.0),
        }
    }

    pub fn as_slice(&self) -> [f32; 5] {
        [self.url_entropy, self.domain_age_proxy, self.cname_depth, self.response_size, self.timing_ms]
    }
}

fn shannon_entropy(value: &str) -> f32 {
    let mut counts = [0u32; 256];
    let bytes = value.as_bytes();
    if bytes.is_empty() {
        return 0.0;
    }

    for byte in bytes {
        counts[*byte as usize] += 1;
    }

    let len = bytes.len() as f32;
    counts
        .iter()
        .filter(|count| **count > 0)
        .map(|count| {
            let probability = *count as f32 / len;
            -probability * probability.log2()
        })
        .sum()
}

fn domain_age_proxy(url: &str) -> f32 {
    let host = url
        .split_once("//")
        .map(|(_, rest)| rest)
        .unwrap_or(url)
        .split(&['/', '?', '#'][..])
        .next()
        .unwrap_or(url);
    let segments = host.split('.').filter(|segment| !segment.is_empty()).count() as f32;
    1.0 / segments.max(1.0)
}
