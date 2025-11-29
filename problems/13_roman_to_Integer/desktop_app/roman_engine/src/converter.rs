//! Roman numeral conversion algorithms
//! 
//! Core logic from the Titan-Machine Citadel

use std::collections::HashMap;

/// Convert Roman numeral to integer
pub fn roman_to_integer(s: &str) -> Result<i32, &'static str> {
    if s.is_empty() {
        return Err("Empty string");
    }

    let roman_map: HashMap<char, i32> = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ].iter().cloned().collect();

    let chars: Vec<char> = s.chars().collect();
    let mut result = 0;

    for i in 0..chars.len() {
        let current = *roman_map.get(&chars[i]).ok_or("Invalid character")?;
        
        // Look ahead to next character
        let next = if i + 1 < chars.len() {
            *roman_map.get(&chars[i + 1]).ok_or("Invalid character")?
        } else {
            0
        };

        // Subtractive notation: if current < next, subtract current
        if current < next {
            result -= current;
        } else {
            result += current;
        }
    }

    Ok(result)
}

/// Convert integer to Roman numeral
pub fn integer_to_roman(mut num: i32) -> Result<String, &'static str> {
    if num < 1 || num > 3999 {
        return Err("Number out of range (1-3999)");
    }

    // Value-symbol pairs in descending order, including subtractive cases
    let values = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut result = String::new();

    for (value, symbol) in values.iter() {
        while num >= *value {
            result.push_str(symbol);
            num -= value;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_integer() {
        assert_eq!(roman_to_integer("III").unwrap(), 3);
        assert_eq!(roman_to_integer("IV").unwrap(), 4);
        assert_eq!(roman_to_integer("IX").unwrap(), 9);
        assert_eq!(roman_to_integer("LVIII").unwrap(), 58);
        assert_eq!(roman_to_integer("MCMXCIV").unwrap(), 1994);
    }

    #[test]
    fn test_integer_to_roman() {
        assert_eq!(integer_to_roman(3).unwrap(), "III");
        assert_eq!(integer_to_roman(4).unwrap(), "IV");
        assert_eq!(integer_to_roman(9).unwrap(), "IX");
        assert_eq!(integer_to_roman(58).unwrap(), "LVIII");
        assert_eq!(integer_to_roman(1994).unwrap(), "MCMXCIV");
    }
}
