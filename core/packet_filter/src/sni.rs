use crate::{Result, VoidBlockError};

pub fn extract_sni(client_hello: &[u8]) -> Result<Option<String>> {
    if client_hello.len() < 5 {
        return Ok(None);
    }

    let record_type = client_hello[0];
    if record_type != 22 {
        return Ok(None);
    }

    let mut index = 5usize;
    if client_hello.get(index).copied() != Some(1) {
        return Ok(None);
    }

    if client_hello.len() < index + 4 {
        return Err(VoidBlockError::InvalidHello("client hello too short".to_string()));
    }

    let handshake_length = ((client_hello[index + 1] as usize) << 16)
        | ((client_hello[index + 2] as usize) << 8)
        | client_hello[index + 3] as usize;
    index += 4;
    if client_hello.len() < index + handshake_length {
        return Err(VoidBlockError::InvalidHello("handshake length exceeds packet size".to_string()));
    }

    if client_hello.len() < index + 34 {
        return Err(VoidBlockError::InvalidHello("missing TLS version and random".to_string()));
    }
    index += 34;

    let session_id_len = *client_hello.get(index).ok_or_else(|| VoidBlockError::InvalidHello("missing session id length".to_string()))? as usize;
    index += 1 + session_id_len;
    if client_hello.len() < index + 2 {
        return Err(VoidBlockError::InvalidHello("missing cipher suite length".to_string()));
    }
    let cipher_len = u16::from_be_bytes([client_hello[index], client_hello[index + 1]]) as usize;
    index += 2 + cipher_len;
    if client_hello.len() <= index {
        return Err(VoidBlockError::InvalidHello("missing compression methods".to_string()));
    }
    let compression_len = *client_hello.get(index).ok_or_else(|| VoidBlockError::InvalidHello("missing compression length".to_string()))? as usize;
    index += 1 + compression_len;
    if client_hello.len() < index + 2 {
        return Ok(None);
    }

    let extension_total = u16::from_be_bytes([client_hello[index], client_hello[index + 1]]) as usize;
    index += 2;
    let end = index + extension_total;
    while index + 4 <= end && end <= client_hello.len() {
        let extension_type = u16::from_be_bytes([client_hello[index], client_hello[index + 1]]);
        let extension_len = u16::from_be_bytes([client_hello[index + 2], client_hello[index + 3]]) as usize;
        index += 4;
        let extension_end = index + extension_len;
        if extension_end > client_hello.len() {
            return Err(VoidBlockError::InvalidHello("extension exceeds packet size".to_string()));
        }
        if extension_type == 0 && extension_len >= 5 {
            let list_len = u16::from_be_bytes([client_hello[index], client_hello[index + 1]]) as usize;
            let mut name_index = index + 2;
            let limit = name_index + list_len;
            while name_index + 3 <= limit && limit <= extension_end {
                let name_type = client_hello[name_index];
                let name_len = u16::from_be_bytes([client_hello[name_index + 1], client_hello[name_index + 2]]) as usize;
                name_index += 3;
                let name_end = name_index + name_len;
                if name_type == 0 && name_end <= extension_end {
                    let host = std::str::from_utf8(&client_hello[name_index..name_end])
                        .map_err(|_| VoidBlockError::InvalidHello("SNI is not UTF-8".to_string()))?;
                    return Ok(Some(host.to_string()));
                }
                name_index = name_end;
            }
        }
        index = extension_end;
    }

    Ok(None)
}
