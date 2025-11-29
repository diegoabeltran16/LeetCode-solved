//! Roman numeral validation
//! 
//! Ensures input strings are valid Roman numerals

use std::collections::HashSet;

/// Check if a string is a valid Roman numeral
pub fn is_valid_roman(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    // Valid Roman numeral characters
    let valid_chars: HashSet<char> = ['I', 'V', 'X', 'L', 'C', 'D', 'M'].iter().cloned().collect();

    // Check all characters are valid
    if !s.chars().all(|c| valid_chars.contains(&c)) {
        return false;
    }

    // Basic validation rules
    // More sophisticated validation could be added here
    // (e.g., no more than 3 consecutive same symbols, proper subtractive notation)

    true
}

/// Check if an integer is in valid Roman numeral range
pub fn is_valid_range(value: i32) -> bool {
    value >= 1 && value <= 3999
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_roman() {
        assert!(is_valid_roman("III"));
        assert!(is_valid_roman("MCMXCIV"));
        assert!(is_valid_roman("LVIII"));
    }

    #[test]
    fn test_invalid_roman() {
        assert!(!is_valid_roman(""));
        assert!(!is_valid_roman("ABC"));
        assert!(!is_valid_roman("123"));
    }

    #[test]
    fn test_valid_range() {
        assert!(is_valid_range(1));
        assert!(is_valid_range(1994));
        assert!(is_valid_range(3999));
        assert!(!is_valid_range(0));
        assert!(!is_valid_range(4000));
    }
}
