pub use crate::to_lowercase::to_lowercase;

pub fn is_palindrome(s: &str) -> bool {
    let mut characters: Vec<char> = to_lowercase(s).chars().collect();

    characters.retain(|c| c.is_alphanumeric());

    let length = characters.len();

    for i in 0..length / 2 {
        let left_character = characters[i];
        let right_character: char = characters[length - i - 1];

        if left_character != right_character {
            return false;
        }
    }

    true
}

mod tests {
    use super::*;

    #[test]
    fn test_small_string() {
        let result = is_palindrome("radar");
        assert_eq!(result, true);
    }

    #[test]
    fn test_another_small_string() {
        let result = is_palindrome("mom");
        assert_eq!(result, true)
    }

    #[test]
    fn test_small_string_that_is_not() {
        let result = is_palindrome("test");
        assert_eq!(result, false);
    }

    #[test]
    fn long_sentence() {
        let result = is_palindrome("A man, a plan, a canal, Panama");
        assert_eq!(result, true);
    }

    #[test]
    fn test_empty_string() {
        let result = is_palindrome("");
        assert_eq!(result, true);
    }

    #[test]
    fn test_string_with_spaces_only() {
        let result = is_palindrome("     ");
        assert_eq!(result, true);
    }

    #[test]
    fn test_string_with_punctuation_only() {
        let result = is_palindrome(".,;:");
        assert_eq!(result, true);
    }

    #[test]
    fn test_string_with_spaces_and_punctuation() {
        let result = is_palindrome(" , ; :  ");
        assert_eq!(result, true);
    }
}
