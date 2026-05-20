use packet_filter::sni::extract_sni;

#[test]
fn extract_sni_handles_short_packets() {
    // Very short input should gracefully return None
    let data: [u8; 3] = [0, 1, 2];
    let result = extract_sni(&data).expect("should not return an error");
    assert!(result.is_none());
}
