//! Left-to-right Roman numeral conversion algorithm
//! 
//! Educational variant showing the forward iteration approach

use std::collections::HashMap;

pub fn convert(s: &str) -> Result<i32, &'static str> {
    let roman_map: HashMap<char, i32> = [
        ('I', 1), ('V', 5), ('X', 10), ('L', 50),
        ('C', 100), ('D', 500), ('M', 1000),
    ].iter().cloned().collect();

    let chars: Vec<char> = s.chars().collect();
    let mut result = 0;

    for i in 0..chars.len() {
        let current = *roman_map.get(&chars[i]).ok_or("Invalid character")?;
        let next = if i + 1 < chars.len() {
            *roman_map.get(&chars[i + 1]).ok_or("Invalid character")?
        } else {
            0
        };

        if current < next {
            result -= current;
        } else {
            result += current;
        }
    }

    Ok(result)
}
