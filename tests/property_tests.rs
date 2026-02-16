use proptest::prelude::*;
use base64::{Engine as _, engine::general_purpose};

proptest! {
    #[test]
    fn test_base64_roundtrip(s in "\\PC*") {
        let encoded = general_purpose::STANDARD.encode(s.as_bytes());
        let decoded = general_purpose::STANDARD.decode(encoded).unwrap();
        let decoded_str = String::from_utf8(decoded).unwrap();
        prop_assert_eq!(s, decoded_str);
    }

    #[test]
    fn test_html_escape_roundtrip(s in "\\PC*") {
        let escaped = html_escape::encode_safe(&s).to_string();
        let unescaped = html_escape::decode_html_entities(&escaped).to_string();
        prop_assert_eq!(s, unescaped);
    }

    #[test]
    fn test_url_escape_roundtrip(s in "\\PC*") {
        let encoded = urlencoding::encode(&s).to_string();
        let decoded = urlencoding::decode(&encoded).unwrap();
        prop_assert_eq!(s, decoded.into_owned());
    }
}
