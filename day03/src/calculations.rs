pub fn calculate_char_score(char: char) -> u32 {
    match char.is_ascii_lowercase() {
        true => char as u32 - 'a' as u32 + 1,
        false => char as u32 - 'A' as u32 + 27,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowercase_start() {
        let result = calculate_char_score('a');
        assert_eq!(result, 1);
    }

    #[test]
    fn test_lowercase_end() {
        let result = calculate_char_score('z');
        assert_eq!(result, 26);
    }

    #[test]
    fn test_uppercase_start() {
        let result = calculate_char_score('A');
        assert_eq!(result, 27);
    }

    #[test]
    fn test_uppercase_end() {
        let result = calculate_char_score('Z');
        assert_eq!(result, 52);
    }
}
