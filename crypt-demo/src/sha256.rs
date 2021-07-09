use sha2::Digest;

// len = 64
pub fn sha256_hex<T: AsRef<[u8]>>(input: T) -> String {
    let mut sha256 = sha2::Sha256::new();
    sha256.update(input);
    let finalize = sha256.finalize();
    hex::encode(finalize)
}

#[cfg(test)]
mod tests {
    use crate::sha256::sha256_hex;

    #[test]
    fn test_sha256_hex() {
        let sha256_hex = sha256_hex("a");
        // ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb
        println!("{}", sha256_hex);
    }
}
