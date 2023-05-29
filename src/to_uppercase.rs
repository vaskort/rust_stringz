pub fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

mod tests {
    use super::*;

    #[test]
    fn test_all_lowercase() {
        let result = to_uppercase("test");
        assert_eq!(result, "TEST");
    }

    #[test]
    fn test_mixed_case() {
        let result: String = to_uppercase("TeSt_TeSt");
        assert_eq!(result, "TEST_TEST");
    }
}
