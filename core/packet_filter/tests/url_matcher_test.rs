use packet_filter::url_matcher::UrlMatcher;

#[test]
fn url_matcher_basic() {
    let patterns = vec!["/ads/", "/track/", "banner.js"];
    let matcher = UrlMatcher::new(patterns).expect("should build automaton");

    assert!(matcher.is_match("https://example.com/ads/banner.js"));
    assert!(matcher.is_match("/track/pixel?id=1"));
    assert!(!matcher.is_match("/content/image.png"));
}
