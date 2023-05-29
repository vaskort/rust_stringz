pub fn count_occurrences(s: &str, substring: &str) -> usize {
    let mut occurence = 0;
    for word in s.split_whitespace() {
        if word == substring {
            occurence += 1;
        }
    }
    occurence
}

mod tests {
    use super::count_occurrences;

    #[test]
    fn test_two_occurences() {
        let result: usize = count_occurrences("test test", "test");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_zero_occurences() {
        let result: usize = count_occurrences("test test", "testz");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_single_occurence() {
        let result: usize = count_occurrences("A random sentence", "random");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_zero_occurences2() {
        let result: usize = count_occurrences("Hello", "o");
        assert_eq!(result, 0);
    }
}
