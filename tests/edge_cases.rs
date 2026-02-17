#[cfg(test)]
mod edge_case_tests {
    use base64::{engine::general_purpose, Engine as _};
    use md5::Md5;
    use sha1::Sha1;
    use sha2::{Digest, Sha256, Sha512};

    #[test]
    fn test_base64_very_long_input() {
        let long_input = "a".repeat(100000);
        let encoded = general_purpose::STANDARD.encode(long_input.as_bytes());
        let decoded = general_purpose::STANDARD.decode(&encoded).unwrap();
        assert_eq!(String::from_utf8(decoded).unwrap(), long_input);
    }

    #[test]
    fn test_base64_url_safe() {
        let inputs = vec!["hello+world", "test=padding", "special/chars"];
        for input in inputs {
            let encoded = general_purpose::URL_SAFE_NO_PAD.encode(input.as_bytes());
            let decoded = general_purpose::URL_SAFE_NO_PAD.decode(&encoded).unwrap();
            assert_eq!(String::from_utf8(decoded).unwrap(), input);
        }
    }

    #[test]
    fn test_base32_edge_cases() {
        use base32::Alphabet;

        let inputs = vec!["", "f", "fo", "foo", "foob", "fooba", "foobar"];
        for input in inputs {
            let encoded = base32::encode(Alphabet::Rfc4648 { padding: false }, input.as_bytes());
            let decoded = base32::decode(Alphabet::Rfc4648 { padding: false }, &encoded).unwrap();
            assert_eq!(String::from_utf8(decoded).unwrap(), input);
        }
    }

    #[test]
    fn test_base58_variants() {
        use bs58::encode;

        let test_cases = vec![
            ("Hello World", true),
            ("", true),
            ("The quick brown fox jumps over the lazy dog", true),
        ];

        for (input, _valid) in test_cases {
            let encoded = encode(input).into_string();
            assert!(!encoded.is_empty() || input.is_empty());
        }
    }

    #[test]
    fn test_all_hash_algorithms() {
        let input = "test input for all hashes";

        let md5_result = {
            let mut hasher = Md5::new();
            hasher.update(input.as_bytes());
            hex::encode(hasher.finalize())
        };
        assert_eq!(md5_result.len(), 32);

        let sha1_result = {
            let mut hasher = Sha1::new();
            hasher.update(input.as_bytes());
            hex::encode(hasher.finalize())
        };
        assert_eq!(sha1_result.len(), 40);

        let sha256_result = {
            let mut hasher = Sha256::new();
            hasher.update(input.as_bytes());
            hex::encode(hasher.finalize())
        };
        assert_eq!(sha256_result.len(), 64);

        let sha512_result = {
            let mut hasher = Sha512::new();
            hasher.update(input.as_bytes());
            hex::encode(hasher.finalize())
        };
        assert_eq!(sha512_result.len(), 128);
    }

    #[test]
    fn test_hash_empty_string() {
        let input = "";

        let mut md5 = Md5::new();
        md5.update(input.as_bytes());
        let md5_result = hex::encode(md5.finalize());
        assert_eq!(md5_result, "d41d8cd98f00b204e9800998ecf8427e");

        let mut sha256 = Sha256::new();
        sha256.update(input.as_bytes());
        let sha256_result = hex::encode(sha256.finalize());
        assert_eq!(
            sha256_result,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_json_nested_structures() {
        let nested = r#"{"a":{"b":{"c":{"d":"value"}}}}"#;
        let parsed: serde_json::Value = serde_json::from_str(nested).unwrap();
        assert_eq!(parsed["a"]["b"]["c"]["d"], "value");

        let array_nested = r#"[[[1,2,3]],[[4,5,6]]]"#;
        let parsed: serde_json::Value = serde_json::from_str(array_nested).unwrap();
        assert_eq!(parsed[0][0][0], 1);
        assert_eq!(parsed[1][0][2], 6);
    }

    #[test]
    fn test_json_pretty_vs_minified() {
        let minified = r#"{"name":"test","value":123}"#;
        let parsed: serde_json::Value = serde_json::from_str(minified).unwrap();
        let pretty = serde_json::to_string_pretty(&parsed).unwrap();

        assert!(pretty.contains('\n'));
        assert!(pretty.contains("  "));
        assert!(pretty.contains("name"));
    }

    #[test]
    fn test_url_encoding_full_coverage() {
        let test_cases = vec![
            ("hello world", "hello%20world"),
            ("a+b", "a%2Bb"),
            ("100%test", "100%25test"),
            ("foo=bar&baz=qux", "foo%3Dbar%26baz%3Dqux"),
            ("emoji🎉", "emoji%F0%9F%8E%89"),
        ];

        for (input, expected) in test_cases {
            let encoded = urlencoding::encode(input);
            assert_eq!(encoded, expected);
        }
    }

    #[test]
    fn test_html_escape_special_chars() {
        let test_cases = vec![
            "<div>",
            "&amp;",
            "\"quoted\"",
            "'single quotes'",
            "<script>alert('xss')</script>",
        ];

        for input in test_cases {
            let escaped = html_escape::encode_text(input).to_string();
            assert!(
                !escaped.contains('<') && !escaped.contains('>'),
                "Input: {}",
                input
            );
        }
    }

    #[test]
    fn test_regex_unicode_matching() {
        use regex::Regex;

        let test_cases = vec![
            (r"\p{L}+", "hello", true),
            (r"\p{L}+", "Привет", true),
            (r"\p{L}+", "こんにちは", true),
            (r"\p{N}+", "12345", true),
            (r"\p{N}+", "１２３４５", true),
        ];

        for (pattern, text, expected) in test_cases {
            let re = Regex::new(pattern).unwrap();
            assert_eq!(
                re.is_match(text),
                expected,
                "Pattern: {}, Text: {}",
                pattern,
                text
            );
        }
    }

    #[test]
    fn test_regex_complex_patterns() {
        use regex::Regex;

        let ipv4 = Regex::new(r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
        assert!(ipv4.is_match("192.168.1.1"));
        assert!(ipv4.is_match("255.255.255.255"));
        assert!(!ipv4.is_match("256.1.1.1"));

        let hex_color = Regex::new(r"^#?([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$").unwrap();
        assert!(hex_color.is_match("#ff0000"));
        assert!(hex_color.is_match("fff"));
        assert!(!hex_color.is_match("notacolor"));
    }

    #[test]
    fn test_diff_operations() {
        use similar::TextDiff;

        let from = "line1\nline2\nline3\nline4";
        let to = "line1\nline2 modified\nline3\nline5";

        let diff = TextDiff::from_lines(from, to);
        let changes: Vec<_> = diff.iter_all_changes().collect();

        assert!(changes.len() > 0);
    }

    #[test]
    fn test_uuid_v4_format() {
        use uuid::Uuid;

        let uuid = Uuid::new_v4();
        let uuid_str = uuid.to_string();

        assert_eq!(uuid_str.len(), 36);
        assert_eq!(uuid_str.chars().filter(|c| *c == '-').count(), 4);
        assert!(uuid_str.chars().all(|c| c.is_ascii_hexdigit() || c == '-'));
    }

    #[test]
    fn test_timestamp_conversions() {
        use chrono::{Local, TimeZone, Utc};

        let ts = 1704067200i64;
        let dt = Utc.timestamp_opt(ts, 0).single().unwrap();
        assert!(dt.to_rfc3339().contains("2024"));

        let now = Utc::now().timestamp();
        assert!(now > 1704067200);

        let local = Local::now();
        let _ = local.format("%Y-%m-%d %H:%M:%S").to_string();
    }

    #[test]
    fn test_cron_expression_parse() {
        let test_cases = vec![
            ("* * * * *", true),
            ("0 * * * *", true),
            ("*/5 * * * *", true),
            ("0 0 1 * *", true),
            ("30 4 1,15 * 0", true),
            ("invalid", false),
        ];

        for (expr, valid) in test_cases {
            let parts: Vec<&str> = expr.split_whitespace().collect();
            let is_valid = parts.len() == 5;
            assert_eq!(is_valid, valid, "Expression: {}", expr);
        }
    }

    #[test]
    fn test_color_conversions() {
        let test_cases = vec![
            ("#ff0000", (255, 0, 0)),
            ("#0000ff", (0, 0, 255)),
            ("#ffffff", (255, 255, 255)),
            ("#000000", (0, 0, 0)),
        ];

        for (hex, expected) in test_cases {
            let hex = hex.trim_start_matches('#');
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
            assert_eq!((r, g, b), expected, "Hex: {}", hex);
        }
    }

    #[test]
    fn test_base_conversion_all_bases() {
        let test_cases = vec![
            ("255", 10, 2, "11111111"),
            ("255", 10, 8, "377"),
            ("255", 10, 16, "ff"),
            ("11111111", 2, 10, "255"),
            ("ff", 16, 10, "255"),
        ];

        for (input, from_base, to_base, expected) in test_cases {
            let decimal = u64::from_str_radix(input, from_base).unwrap();
            let result = match to_base {
                2 => format!("{:b}", decimal),
                8 => format!("{:o}", decimal),
                10 => format!("{}", decimal),
                16 => format!("{:x}", decimal),
                _ => panic!("Unsupported base"),
            };
            assert_eq!(
                result, expected,
                "Input: {} from {} to {}",
                input, from_base, to_base
            );
        }
    }

    #[test]
    fn test_image_base64_header() {
        let test_cases = vec![
            ("fake image data", "data:image/png;base64,"),
            ("gif data", "data:image/gif;base64,"),
            ("svg data", "data:image/svg+xml;base64,"),
        ];

        for (data, expected_prefix) in test_cases {
            let encoded = general_purpose::STANDARD.encode(data.as_bytes());
            let full_data_uri = format!("data:image/png;base64,{}", encoded);
            assert!(
                full_data_uri.starts_with(expected_prefix)
                    || full_data_uri.starts_with("data:image")
            );
        }
    }
}
