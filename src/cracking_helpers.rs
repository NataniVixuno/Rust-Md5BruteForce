pub fn is_valid_md5_hash(hash: &str) -> bool {
    // MD5 hashes are exactly 32 hexadecimal characters
    if hash.len() != 32 {
        return false;
    }
    
    // Check if all characters are valid hexadecimal (0-9, a-f, A-F)
    hash.chars().all(|c| c.is_ascii_hexdigit())
}

