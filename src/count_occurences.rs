pub fn count_occurences(s: &str, substring: &str) -> usize {
    let mut occurence = 0;
    for word in s.split_whitespace() {
        if word == substring {
            occurence += 1;
        }
    }
    occurence
}

mod tests {
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
