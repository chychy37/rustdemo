use hex;

pub fn encode_to_lower_string<T: AsRef<[u8]>>(input: T) -> String {
    hex::encode(input)
}

pub fn encode_to_upper_string<T: AsRef<[u8]>>(input: T) -> String {
    hex::encode_upper(input)
}

pub fn decode_to_vec<T: AsRef<[u8]>>(input: T) -> Vec<u8> {
    hex::decode(input).expect("hex decode error")
}

#[cfg(test)]
mod tests {
    use crate::hex::{decode_to_vec, encode_to_lower_string, encode_to_upper_string};

    #[test]
    fn test_hex_encode() {
        let encode_1 = encode_to_lower_string("z");
        let encode_2 = encode_to_upper_string("z");
        // 7a, 7A
        println!("{}, {}", encode_1, encode_2);
    }

    #[test]
    fn test_hex_decode() {
        let decode_1 = decode_to_vec("FF");
        let decode_2 = decode_to_vec("7a");
        // [255], [122]
        println!("{:?}, {:?}", decode_1, decode_2);
    }
}
