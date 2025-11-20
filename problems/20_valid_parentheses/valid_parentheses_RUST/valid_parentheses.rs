//! problems/20_valid_parentheses/valid_parentheses.rs
//!
//! Provides `is_valid` to check if a string of brackets is valid using a stack and lookup table.
//!
//! **Story (Castle Gatekeeper)**
//! You are the castle gatekeeper with a magical ledger. When a traveler opens a gate (an opening bracket), you record it on a scroll (the stack). For each closing gate, you consult your ledger (the HashMap) to see which opening gate matches. If it matches the last opened gate on your scroll, you close it; otherwise, you raise the alarm. At the end, if your scroll is empty, the castle is secure.

use std::collections::HashMap;

/// Checks whether the input string `s` containing only '()[]{}' is valid.
///
/// # Arguments
///
/// * `s` - A `String` of brackets to validate
///
/// # Returns
///
/// * `bool` - `true` if valid, `false` otherwise
pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let pairs: HashMap<char, char> =
        [(')', '('), (']', '['), ('}', '{')].iter().cloned().collect();

    for ch in s.chars() {
        if let Some(&opening) = pairs.get(&ch) {
            if stack.pop() != Some(opening) {
                return false;
            }
        } else {
            stack.push(ch);
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::is_valid;

    #[test]
    fn test_valid_parentheses() {
        let cases = vec![
            ("()", true),
            ("()[]{}", true),
            ("(]", false),
            ("([])", true),
            ("([)]", false),
            ("{[]}", true),
        ];
        for (s, expected) in cases {
            assert_eq!(is_valid(s.to_string()), expected, "failed on {}", s);
        }
    }
}
