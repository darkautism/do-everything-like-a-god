#[cfg(test)]
mod integration_tests {
    use base64::{engine::general_purpose, Engine as _};
    use md5::Md5;
    use sha2::Digest;

    #[test]
    fn test_base64_empty_input() {
        let encoded = general_purpose::STANDARD.encode(b"");
        assert_eq!(encoded, "");
        let decoded = general_purpose::STANDARD.decode("").unwrap();
        assert!(decoded.is_empty());
    }

    #[test]
    fn test_base64_unicode() {
        let inputs = vec!["你好世界", "🎉🚀🔥", "日本語テスト", "العربية"];
        for input in inputs {
            let encoded = general_purpose::STANDARD.encode(input.as_bytes());
            let decoded = general_purpose::STANDARD.decode(&encoded).unwrap();
            assert_eq!(String::from_utf8(decoded).unwrap(), input);
        }
    }

    #[test]
    fn test_hash_consistency() {
        let input = "test input for hash consistency check";

        let mut md5 = Md5::new();
        md5.update(input);
        let md5_result = hex::encode(md5.finalize());

        let mut md5_2 = Md5::new();
        md5_2.update(input);
        let md5_result_2 = hex::encode(md5_2.finalize());

        assert_eq!(md5_result, md5_result_2);
        assert_eq!(md5_result.len(), 32);
    }

    #[test]
    fn test_json_edge_cases() {
        let cases = vec![
            ("null", "null"),
            ("true", "true"),
            ("false", "false"),
            ("123", "123"),
            ("\"string\"", "\"string\""),
            ("[]", "[]"),
            ("{}", "{}"),
        ];

        for (input, expected) in cases {
            let v: serde_json::Value = serde_json::from_str(input).unwrap();
            let output = serde_json::to_string(&v).unwrap();
            assert_eq!(output, expected);
        }
    }

    #[test]
    fn test_url_special_chars() {
        let special = "!@#$%^&*()+=[]{}|;':\",./<>?";
        let encoded = urlencoding::encode(special);
        let decoded = urlencoding::decode(&encoded).unwrap();
        assert_eq!(decoded, special);
    }

    #[test]
    fn test_jwt_valid_decode() {
        let header = r#"{"alg":"HS256","typ":"JWT"}"#;
        let payload = r#"{"sub":"test"}"#;
        let h_b64 = general_purpose::URL_SAFE_NO_PAD.encode(header);
        let p_b64 = general_purpose::URL_SAFE_NO_PAD.encode(payload);
        let token = format!("{}.{}.signature", h_b64, p_b64);

        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3);

        let h_dec =
            String::from_utf8(general_purpose::URL_SAFE_NO_PAD.decode(parts[0]).unwrap()).unwrap();
        assert!(h_dec.contains("HS256"));

        let p_dec =
            String::from_utf8(general_purpose::URL_SAFE_NO_PAD.decode(parts[1]).unwrap()).unwrap();
        assert!(p_dec.contains("test"));
    }

    #[test]
    fn test_regex_special_patterns() {
        use regex::Regex;

        let test_cases = vec![
            (r"^\d+$", "12345", true),
            (r"^\d+$", "12a45", false),
            (
                r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$",
                "test@example.com",
                true,
            ),
            (
                r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$",
                "invalid-email",
                false,
            ),
            (r"^https?://.+", "https://example.com", true),
            (r"^https?://.+", "ftp://example.com", false),
        ];

        for (pattern, text, should_match) in test_cases {
            let re = Regex::new(pattern).unwrap();
            assert_eq!(
                re.is_match(text),
                should_match,
                "Pattern: {}, Text: {}",
                pattern,
                text
            );
        }
    }

    #[test]
    fn test_diff_empty_inputs() {
        use similar::TextDiff;

        let diff1 = TextDiff::from_lines("", "new content");
        let changes: Vec<_> = diff1.iter_all_changes().collect();
        assert!(!changes.is_empty());

        let diff2 = TextDiff::from_lines("old content", "");
        let changes: Vec<_> = diff2.iter_all_changes().collect();
        assert!(!changes.is_empty());

        let diff3 = TextDiff::from_lines("", "");
        let changes: Vec<_> = diff3.iter_all_changes().collect();
        assert!(changes.is_empty());
    }

    #[test]
    fn test_base_converter_boundaries() {
        let test_cases = vec![
            ("0", 10u32, "0"),
            ("1", 10u32, "1"),
            ("ff", 16u32, "255"),
            ("FF", 16u32, "255"),
            ("11111111", 2u32, "255"),
            ("377", 8u32, "255"),
        ];

        for (input, from_base, expected_dec) in test_cases {
            let result = u128::from_str_radix(input, from_base).unwrap();
            assert_eq!(format!("{}", result), expected_dec);
        }
    }

    #[test]
    fn test_timestamp_edge_cases() {
        use chrono::{TimeZone, Utc};

        let epoch = Utc.timestamp_opt(0, 0).single().unwrap();
        assert_eq!(epoch.to_rfc3339(), "1970-01-01T00:00:00+00:00");

        let negative = Utc.timestamp_opt(-1, 0).single().unwrap();
        assert_eq!(negative.to_rfc3339(), "1969-12-31T23:59:59+00:00");
    }

    #[test]
    fn test_html_escape_xss_patterns() {
        let xss_vectors = vec![
            "<script>alert('xss')</script>",
            "<img src=x onerror=alert('xss')>",
            "javascript:alert('xss')",
            "<svg onload=alert('xss')>",
        ];

        for vector in xss_vectors {
            let escaped = html_escape::encode_safe(vector).to_string();
            assert!(!escaped.contains('<'), "Failed to escape < in: {}", vector);
            assert!(!escaped.contains('>'), "Failed to escape > in: {}", vector);
        }
    }
}
