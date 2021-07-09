use base64;

pub fn encode_to_string<T: AsRef<[u8]>>(input: T) -> String {
    base64::encode(input)
}

pub fn decode_to_string<T: AsRef<[u8]>>(input: T) -> String {
    let decode_bytes = base64::decode(input).expect("base64 decode error");
    String::from_utf8(decode_bytes).expect("decode_bytes parse error")
}

#[cfg(test)]
mod tests {
    use crate::base64::{decode_to_string, encode_to_string};

    #[test]
    fn test_base64_encode() {
        let encode_1 = encode_to_string("qwe");
        let encode_2 = encode_to_string([113, 119, 101]);
        let encode_3 = encode_to_string(vec![113, 119, 101]);
        // cXdl, cXdl, cXdl
        println!("{}, {}, {}", encode_1, encode_2, encode_3);
    }

    #[test]
    fn test_base_64_decode() {
        let decode_1 = decode_to_string("cXdl");
        let decode_2 = decode_to_string([99, 88, 100, 108]);
        let decode_3 = decode_to_string(vec![99, 88, 100, 108]);
        // qwe, qwe, qwe
        println!("{}, {}, {}", decode_1, decode_2, decode_3);
    }
}
