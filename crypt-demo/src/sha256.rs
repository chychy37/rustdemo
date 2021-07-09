use sha2::digest::Output;
use sha2::{Digest, Sha256};

fn sha256<T: AsRef<[u8]>>(input: T) -> Output<Sha256> {
    let mut sha256 = sha2::Sha256::new();
    sha256.update(input);
    sha256.finalize()
}

pub fn sha256_bytes<T: AsRef<[u8]>>(input: T) -> Vec<u8> {
    let finalize = sha256(input);
    finalize.to_vec()
}

pub fn sha256_lower_hex<T: AsRef<[u8]>>(input: T) -> String {
    let finalize = sha256(input);
    hex::encode(finalize)
}

pub fn sha256_upper_hex<T: AsRef<[u8]>>(input: T) -> String {
    let finalize = sha256(input);
    hex::encode_upper(finalize)
}

#[cfg(test)]
mod tests {
    use crate::sha256::{sha256_bytes, sha256_lower_hex, sha256_upper_hex};

    #[test]
    fn test_sha256() {
        let sha256_bytes = sha256_bytes("a");
        let sha256_lower_hex = sha256_lower_hex("a");
        let sha256_upper_hex = sha256_upper_hex("a");
        // [202, 151, 129, 18, 202, 27, 189, 202, 250, 194, 49, 179, 154, 35, 220, 77, 167, 134, 239, 248, 20, 124, 78, 114, 185, 128, 119, 133, 175, 238, 72, 187] | 32
        // ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb | 64
        // CA978112CA1BBDCAFAC231B39A23DC4DA786EFF8147C4E72B9807785AFEE48BB | 64
        println!(
            "{:?}, {}, {}",
            sha256_bytes, sha256_lower_hex, sha256_upper_hex
        );
    }
}
