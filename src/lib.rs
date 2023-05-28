pub fn to_lowercase(s: &str) -> String {
    s.to_lowercase()
}

pub fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

pub fn count_occurences(s: &str, substring: &str) -> usize {
    let mut occurence = 0;
    for word in s.split_whitespace() {
        if word == substring {
            occurence += 1;
        }
    }
    occurence
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

    mod count_occurences {
        use super::*;

        #[test]
        fn test_two_occurences() {
            let result: usize = count_occurences("test test", "test");
            assert_eq!(result, 2);
        }

        #[test]
        fn test_zero_occurences() {
            let result: usize = count_occurences("test test", "testz");
            assert_eq!(result, 0);
        }

        #[test]
        fn test_single_occurence() {
            let result: usize = count_occurences("A random sentence", "random");
            assert_eq!(result, 1);
        }

        #[test]
        fn test_zero_occurences2() {
            let result: usize = count_occurences("Hello", "o");
            assert_eq!(result, 0);
        }
    }
}
