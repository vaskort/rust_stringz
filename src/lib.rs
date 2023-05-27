pub fn to_lowercase(s: &str) -> String {
    s.to_lowercase()
}

pub fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod to_lowercase {
        use super::*;

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

    mod to_uppercase {
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
}
