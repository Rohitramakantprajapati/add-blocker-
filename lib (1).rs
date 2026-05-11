//! VoidBlock Packet Filter
//!
//! SNI-based HTTPS blocking without MITM decryption.
//! When a blocked SNI is detected, the TCP connection is dropped (RST).
//! No certificate is presented. No traffic is decrypted.

use anyhow::Result;

/// Result of SNI inspection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SniDecision {
    /// Allow the connection to proceed.
    Allow,
    /// Drop the connection — RST the TCP stream.
    Drop { sni: String },
}

/// Extract the SNI hostname from a TLS ClientHello packet.
///
/// Returns `None` if the packet is not a TLS ClientHello or contains no SNI extension.
/// This function does not decrypt or modify the packet — it reads only the plaintext header.
pub fn extract_sni(payload: &[u8]) -> Option<String> {
    // TLS record header: ContentType(1) + Version(2) + Length(2) = 5 bytes
    if payload.len() < 5 {
        return None;
    }
    // ContentType 0x16 = Handshake
    if payload[0] != 0x16 {
        return None;
    }
    // Handshake type 0x01 = ClientHello
    if payload.len() < 6 || payload[5] != 0x01 {
        return None;
    }

    // Walk the extensions to find SNI (type 0x0000)
    parse_sni_from_client_hello(&payload[5..])
}

fn parse_sni_from_client_hello(handshake: &[u8]) -> Option<String> {
    // Handshake: type(1) + length(3) + client_hello_body
    if handshake.len() < 4 {
        return None;
    }
    let body = &handshake[4..];

    // client_hello: version(2) + random(32) + session_id_len(1) + ...
    if body.len() < 35 {
        return None;
    }
    let session_id_len = body[34] as usize;
    let after_session = 35 + session_id_len;

    if body.len() < after_session + 2 {
        return None;
    }
    let cipher_suites_len =
        u16::from_be_bytes([body[after_session], body[after_session + 1]]) as usize;
    let after_ciphers = after_session + 2 + cipher_suites_len;

    if body.len() < after_ciphers + 1 {
        return None;
    }
    let compression_len = body[after_ciphers] as usize;
    let after_compression = after_ciphers + 1 + compression_len;

    if body.len() < after_compression + 2 {
        return None;
    }
    let extensions_len =
        u16::from_be_bytes([body[after_compression], body[after_compression + 1]]) as usize;
    let extensions = &body[after_compression + 2..];

    if extensions.len() < extensions_len {
        return None;
    }

    let mut pos = 0;
    while pos + 4 <= extensions_len {
        let ext_type = u16::from_be_bytes([extensions[pos], extensions[pos + 1]]);
        let ext_len = u16::from_be_bytes([extensions[pos + 2], extensions[pos + 3]]) as usize;
        pos += 4;

        if ext_type == 0x0000 {
            // SNI extension
            // server_name_list_len(2) + server_name_type(1) + server_name_len(2) + server_name
            if ext_len < 5 {
                return None;
            }
            let name_len =
                u16::from_be_bytes([extensions[pos + 3], extensions[pos + 4]]) as usize;
            if extensions.len() < pos + 5 + name_len {
                return None;
            }
            let name = &extensions[pos + 5..pos + 5 + name_len];
            return String::from_utf8(name.to_vec()).ok();
        }

        pos += ext_len;
    }

    None
}

#[cfg(test)]
mod tests {
    // SNI parsing tests will use captured TLS ClientHello fixtures.
    // Fixtures are binary files in tests/fixtures/ — not generated here.
}
