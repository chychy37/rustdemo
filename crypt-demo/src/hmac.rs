use hmac::crypto_mac::Output;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

fn hmac_sha256<T: AsRef<[u8]>>(secret: T, input: T) -> Output<Hmac<Sha256>> {
    let mut mac = HmacSha256::new_from_slice(secret.as_ref()).expect("hmac new from slice error");
    mac.update(input.as_ref());
    mac.finalize()
}

pub fn hmac_sha256_bytes<T: AsRef<[u8]>>(secret: T, input: T) -> Vec<u8> {
    let finalize = hmac_sha256(secret, input);
    let bytes = finalize.into_bytes();
    bytes.to_vec()
}

pub fn hmac_sha256_lower_hex<T: AsRef<[u8]>>(secret: T, input: T) -> String {
    let finalize = hmac_sha256(secret, input);
    let bytes = finalize.into_bytes();
    hex::encode(bytes)
}

pub fn hmac_sha256_upper_hex<T: AsRef<[u8]>>(secret: T, input: T) -> String {
    let finalize = hmac_sha256(secret, input);
    let bytes = finalize.into_bytes();
    hex::encode_upper(bytes)
}

#[cfg(test)]
mod tests {
    use crate::hmac::{hmac_sha256_bytes, hmac_sha256_lower_hex, hmac_sha256_upper_hex};

    #[test]
    fn test_hmac_sha256() {
        let hmac_sha256_bytes = hmac_sha256_bytes("secret", "input");
        let hmac_sha256_lower_hex = hmac_sha256_lower_hex("secret", "input");
        let hmac_sha256_upper_hex = hmac_sha256_upper_hex("secret", "input");
        // [141, 137, 133, 208, 75, 122, 189, 50, 203, 170, 55, 121, 163, 218, 160, 25, 224, 210, 105, 162, 42, 236, 21, 175, 142, 114, 150, 247, 2, 204, 104, 198] | 32
        // 8d8985d04b7abd32cbaa3779a3daa019e0d269a22aec15af8e7296f702cc68c6 | 64
        // 8D8985D04B7ABD32CBAA3779A3DAA019E0D269A22AEC15AF8E7296F702CC68C6 | 64
        println!(
            "{:?}, {}, {}",
            hmac_sha256_bytes, hmac_sha256_lower_hex, hmac_sha256_upper_hex
        );
    }
}
