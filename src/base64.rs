use std::io::{self, Read, Write};

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode<R: Read, W: Write>(mut input: R, mut output: W) -> io::Result<()> {
    let mut buf = [0u8; 3 * 1024]; // 3KB chunks
    let mut out_buf = Vec::with_capacity(4096); // Allocate once
    
    loop {
        let mut bytes_read = 0;
        while bytes_read < buf.len() {
            match input.read(&mut buf[bytes_read..]) {
                Ok(0) => break,
                Ok(n) => bytes_read += n,
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            }
        }
        
        if bytes_read == 0 {
            break;
        }
        
        out_buf.clear(); // Reuse buffer
        
        for chunk in buf[..bytes_read].chunks(3) {
            match chunk.len() {
                3 => {
                    let n = (chunk[0] as u32) << 16 | (chunk[1] as u32) << 8 | (chunk[2] as u32);
                    out_buf.push(ALPHABET[(n >> 18) as usize]);
                    out_buf.push(ALPHABET[((n >> 12) & 63) as usize]);
                    out_buf.push(ALPHABET[((n >> 6) & 63) as usize]);
                    out_buf.push(ALPHABET[(n & 63) as usize]);
                }
                2 => {
                    let n = (chunk[0] as u32) << 16 | (chunk[1] as u32) << 8;
                    out_buf.push(ALPHABET[(n >> 18) as usize]);
                    out_buf.push(ALPHABET[((n >> 12) & 63) as usize]);
                    out_buf.push(ALPHABET[((n >> 6) & 63) as usize]);
                    out_buf.push(b'=');
                }
                1 => {
                    let n = (chunk[0] as u32) << 16;
                    out_buf.push(ALPHABET[(n >> 18) as usize]);
                    out_buf.push(ALPHABET[((n >> 12) & 63) as usize]);
                    out_buf.push(b'=');
                    out_buf.push(b'=');
                }
                _ => unreachable!(),
            }
        }
        
        output.write_all(&out_buf)?;
    }
    
    Ok(())
}

fn decode_char(c: u8) -> Result<u32, &'static str> {
    match c {
        b'A'..=b'Z' => Ok((c - b'A') as u32),
        b'a'..=b'z' => Ok((c - b'a' + 26) as u32),
        b'0'..=b'9' => Ok((c - b'0' + 52) as u32),
        b'+' => Ok(62),
        b'/' => Ok(63),
        _ => Err("Invalid base64 character"),
    }
}

pub fn decode<R: Read, W: Write>(mut input: R, mut output: W) -> io::Result<()> {
    let mut buf = [0u8; 4096];
    let mut chunk = [0u8; 4];
    let mut chunk_len = 0;
    let mut out_buf = Vec::with_capacity(4096);
    let mut end_of_stream = false; // Set true once padding is seen; any non-ws after is an error

    loop {
        // Match encode: fill the whole buffer before processing
        let mut bytes_read = 0;
        while bytes_read < buf.len() {
            match input.read(&mut buf[bytes_read..]) {
                Ok(0) => break,
                Ok(n) => bytes_read += n,
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            }
        }

        if bytes_read == 0 {
            break;
        }

        out_buf.clear();

        for &b in &buf[..bytes_read] {
            // Only allow whitespace (newlines, spaces) between whole 4-char blocks,
            // not mid-block. Track via chunk_len == 0.
            if b.is_ascii_whitespace() {
                if chunk_len == 0 {
                    continue; // whitespace between blocks: ok
                } else {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Whitespace inside base64 block"));
                }
            }

            if end_of_stream {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Trailing data after padding"));
            }

            chunk[chunk_len] = b;
            chunk_len += 1;

            if chunk_len == 4 {
                let c0 = decode_char(chunk[0]).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                let c1 = decode_char(chunk[1]).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

                if chunk[2] == b'=' {
                    if chunk[3] != b'=' {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid padding"));
                    }
                    if (c1 & 0xF) != 0 {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, "Non-zero padding bits"));
                    }
                    let n = (c0 << 18) | (c1 << 12);
                    out_buf.push((n >> 16) as u8);
                    end_of_stream = true;
                } else if chunk[3] == b'=' {
                    let c2 = decode_char(chunk[2]).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                    if (c2 & 0x3) != 0 {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, "Non-zero padding bits"));
                    }
                    let n = (c0 << 18) | (c1 << 12) | (c2 << 6);
                    out_buf.push((n >> 16) as u8);
                    out_buf.push(((n >> 8) & 0xFF) as u8);
                    end_of_stream = true;
                } else {
                    let c2 = decode_char(chunk[2]).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                    let c3 = decode_char(chunk[3]).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                    let n = (c0 << 18) | (c1 << 12) | (c2 << 6) | c3;
                    out_buf.push((n >> 16) as u8);
                    out_buf.push(((n >> 8) & 0xFF) as u8);
                    out_buf.push((n & 0xFF) as u8);
                }
                chunk_len = 0;
            }
        }

        if !out_buf.is_empty() {
            output.write_all(&out_buf)?;
        }
    }

    if chunk_len != 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Truncated base64 input"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn encode_str(input: &[u8]) -> String {
        let mut output = Vec::new();
        encode(Cursor::new(input), &mut output).unwrap();
        String::from_utf8(output).unwrap()
    }

    fn decode_str(input: &str) -> Result<Vec<u8>, io::Error> {
        let mut output = Vec::new();
        decode(Cursor::new(input.as_bytes()), &mut output)?;
        Ok(output)
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode_str(b""), "");
        assert_eq!(encode_str(b"f"), "Zg==");
        assert_eq!(encode_str(b"fo"), "Zm8=");
        assert_eq!(encode_str(b"foo"), "Zm9v");
        assert_eq!(encode_str(b"foob"), "Zm9vYg==");
        assert_eq!(encode_str(b"fooba"), "Zm9vYmE=");
        assert_eq!(encode_str(b"foobar"), "Zm9vYmFy");
    }

    #[test]
    fn test_decode_rfc_vectors() {
        assert_eq!(decode_str("").unwrap(), b"");
        assert_eq!(decode_str("Zg==").unwrap(), b"f");
        assert_eq!(decode_str("Zm8=").unwrap(), b"fo");
        assert_eq!(decode_str("Zm9v").unwrap(), b"foo");
        assert_eq!(decode_str("Zm9vYg==").unwrap(), b"foob");
        assert_eq!(decode_str("Zm9vYmE=").unwrap(), b"fooba");
        assert_eq!(decode_str("Zm9vYmFy").unwrap(), b"foobar");
    }

    #[test]
    fn test_decode_whitespace_between_blocks() {
        // Whitespace between whole 4-char blocks is fine
        assert_eq!(decode_str("Zm9v\nYmFy").unwrap(), b"foobar");
        assert_eq!(decode_str("Zm9v YmFy").unwrap(), b"foobar");
    }

    #[test]
    fn test_decode_whitespace_mid_block_rejected() {
        // Whitespace mid-block is an error
        assert!(decode_str("Zg\n==").is_err());
        assert!(decode_str("Z g==").is_err());
    }

    #[test]
    fn test_decode_padding_strictness() {
        // Data after padding must be rejected
        assert!(decode_str("Zg==Z").is_err());           // non-ws after ==
        assert!(decode_str("Zg==Zg==").is_err());        // concatenated blocks rejected
        assert!(decode_str("Zm8=X").is_err());           // non-ws after =
        assert!(decode_str("Zm8=a===").is_err());        // junk after =
    }

    #[test]
    fn test_decode_malleability() {
        // 'Zg==' -> 'f'. 'g' = 100000; 'h' = 100001 (non-zero trailing bits)
        assert!(decode_str("Zh==").is_err());
        // 'Zm8=' -> 'fo'. '8' = 111100; '9' = 111101 (non-zero trailing bits)
        assert!(decode_str("Zm9=").is_err());
    }

    #[test]
    fn test_decode_invalid_padding_position() {
        // '=' in position 0 or 1 must fail (decode_char returns Err)
        assert!(decode_str("====").is_err());
        assert!(decode_str("=AAA").is_err());
        assert!(decode_str("A=AA").is_err());
    }

    #[test]
    fn test_decode_truncated_input() {
        // Fewer than 4 chars without padding is truncated
        assert!(decode_str("Zg").is_err());
        assert!(decode_str("Zm8").is_err());
    }
}

