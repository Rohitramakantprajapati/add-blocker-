//! VoidBlock AI Engine
//!
//! On-device ONNX gradient-boosted classifier for ad pattern detection.
//! Ships as optional "Enhanced Blocking" toggle — off by default.
//!
//! Precision requirement: >92% on held-out test set before any model ships.
//! False positive rate: <0.1% on Tranco top 1000.
//!
//! Phase 4 implementation. This module is a placeholder until Phase 4.
//! The feature flag `ai-engine` gates all runtime code.

use anyhow::Result;

/// Features extracted from a URL for the classifier.
#[derive(Debug, Clone)]
pub struct UrlFeatures {
    /// Shannon entropy of the URL path
    pub url_entropy: f32,
    /// Domain age in days (from WHOIS — requires async lookup, cached)
    pub domain_age_days: Option<u32>,
    /// Number of CNAME hops (from DNS resolution)
    pub cname_depth: u8,
    /// HTTP response content-length if known
    pub response_size_bytes: Option<u32>,
    /// Request timing anomaly score (0.0–1.0)
    pub timing_anomaly: f32,
    /// TLD category: 0=com/net/org, 1=country, 2=new-tld, 3=known-ad-tld
    pub tld_category: u8,
}

/// Decision from the AI classifier.
#[derive(Debug, Clone, PartialEq)]
pub enum AiDecision {
    /// Likely an ad — block with `confidence` score.
    Block { confidence: f32 },
    /// Likely legitimate — allow.
    Allow { confidence: f32 },
}

/// Extract URL features for classification.
pub fn extract_features(url: &str) -> UrlFeatures {
    UrlFeatures {
        url_entropy: shannon_entropy(url),
        domain_age_days: None, // Populated asynchronously — see platform layer
        cname_depth: 0,        // Populated from DNS resolution result
        response_size_bytes: None,
        timing_anomaly: 0.0,
        tld_category: classify_tld(url),
    }
}

fn shannon_entropy(s: &str) -> f32 {
    if s.is_empty() {
        return 0.0;
    }
    let mut freq = [0u32; 256];
    for b in s.bytes() {
        freq[b as usize] += 1;
    }
    let len = s.len() as f32;
    freq.iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f32 / len;
            -p * p.log2()
        })
        .sum()
}

fn classify_tld(url: &str) -> u8 {
    // Known ad-network TLDs get category 3
    const AD_TLDS: &[&str] = &[".doubleclick.", ".googlesyndication.", ".2mdn."];
    for ad_tld in AD_TLDS {
        if url.contains(ad_tld) {
            return 3;
        }
    }
    // Country-code TLDs
    // Simplified — full implementation uses a proper TLD list
    if url.len() > 3 {
        let tld = &url[url.rfind('.').unwrap_or(0)..];
        if tld.len() == 3 {
            return 1; // 2-letter ccTLD
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entropy_empty_string() {
        assert_eq!(shannon_entropy(""), 0.0);
    }

    #[test]
    fn entropy_uniform_string() {
        // "aaaa" has entropy 0
        assert_eq!(shannon_entropy("aaaa"), 0.0);
    }

    #[test]
    fn entropy_random_looking_url_higher_than_clean() {
        let clean = shannon_entropy("example.com/page");
        let random = shannon_entropy("a8f3k2.xyz/q?id=xK92mP3&t=1a2b3c");
        assert!(random > clean, "random URL should have higher entropy");
    }

    #[test]
    fn extract_features_does_not_panic_on_empty() {
        let _ = extract_features("");
    }
}
