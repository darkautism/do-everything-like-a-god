#[cfg(test)]
mod advanced_tests {
    use base64::{engine::general_purpose, Engine as _};

    #[test]
    fn test_large_base64() {
        let large_input = "a".repeat(1024 * 1024); // 1MB
        let encoded = general_purpose::STANDARD.encode(large_input.as_bytes());
        assert!(encoded.len() > large_input.len());
        let decoded = general_purpose::STANDARD.decode(encoded).unwrap();
        assert_eq!(decoded.len(), 1024 * 1024);
    }

    #[test]
    fn test_complex_json_nesting() {
        let mut json_str = "{\"inner\":".to_string();
        let depth = 50;
        for _ in 0..depth {
            json_str.push_str("{\"a\":");
        }
        json_str.push_str("1");
        for _ in 0..depth {
            json_str.push_str("}");
        }
        json_str.push_str("}");

        let v: serde_json::Value =
            serde_json::from_str(&json_str).expect("Failed to parse deep JSON");
        let prettified = serde_json::to_string_pretty(&v).unwrap();
        assert!(prettified.lines().count() > depth);
    }

    #[test]
    fn test_uuid_v4_randomness() {
        use uuid::Uuid;
        let mut uuids = std::collections::HashSet::new();
        for _ in 0..1000 {
            let u = Uuid::new_v4();
            assert!(uuids.insert(u), "Duplicate UUID generated!");
        }
    }

    #[test]
    fn test_regex_backtracking_limit() {
        use regex::Regex;
        // This is a classic "evil regex" pattern, but Rust's regex engine is linear time
        // and doesn't suffer from exponential backtracking.
        let re = Regex::new(r"(a|a)*b").unwrap();
        let input = "a".repeat(100);
        assert!(!re.is_match(&input));
    }
}
