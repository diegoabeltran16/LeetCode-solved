//! Right-to-left Roman numeral conversion algorithm
//! 
//! Educational variant showing the backward iteration approach

use std::collections::HashMap;

pub fn convert(s: &str) -> Result<i32, &'static str> {
    let roman_map: HashMap<char, i32> = [
        ('I', 1), ('V', 5), ('X', 10), ('L', 50),
        ('C', 100), ('D', 500), ('M', 1000),
    ].iter().cloned().collect();

    let chars: Vec<char> = s.chars().collect();
    let mut result = 0;
    let mut prev_value = 0;

    for i in (0..chars.len()).rev() {
        let current = *roman_map.get(&chars[i]).ok_or("Invalid character")?;
        
        if current < prev_value {
            result -= current;
        } else {
            result += current;
        }
        
        prev_value = current;
    }

    Ok(result)
}
