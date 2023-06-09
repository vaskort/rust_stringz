pub fn to_lowercase(s: &str) -> String {
    s.to_lowercase()
}

mod tests {
    use super::to_lowercase;

    #[test]
    fn test_all_uppercase() {
        let result = to_lowercase("TEST");
        assert_eq!(result, "test");
    }

    #[test]
    fn test_mixed_case() {
        let result: String = to_lowercase("TeSt_TeSt");
        assert_eq!(result, "test_test");
    }
}
