/// # LeetCode Problem 13: Roman to Integer :)
///
/// This module provides a solution for converting a Roman numeral string
/// into its corresponding integer value.
///
/// The implemented function handles standard Roman numeral notation,
/// including subtractive combinations (e.g., IV for 4, IX for 9, etc.).

/// Defines an empty `Solution` struct to match LeetCode's expected structure.
///
/// In LeetCode, solutions are often implemented as methods of a `Solution` struct.
/// This placeholder struct enables the `impl Solution` block to compile
/// without affecting runtime or functionality.
struct Solution;

impl Solution {
    /// Converts a Roman numeral string to its integer representation.
    ///
    /// # Arguments
    ///
    /// * `s` - A `String` containing the Roman numeral (e.g., "MCMXCIV")
    ///
    /// # Returns
    ///
    /// * `i32` - The integer value corresponding to the Roman numeral.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = Solution::roman_to_int("MCMXCIV".to_string());
    /// assert_eq!(result, 1994);
    /// ```
    pub fn roman_to_int(s: String) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let mut res = 0;

        /// Returns the integer value for a single Roman numeral character.
        ///
        /// # Arguments
        ///
        /// * `c` - A Roman numeral character (I, V, X, L, C, D, M)
        ///
        /// # Returns
        ///
        /// * `i32` - The integer value (e.g., 'I' -> 1, 'V' -> 5)
        fn val(c: char) -> i32 {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        }

        // Iterate through each character, adding or subtracting its value
        // depending on whether it is followed by a larger numeral.
        for i in 0..s_chars.len() {
            let current = val(s_chars[i]);
            let next = if i + 1 < s_chars.len() { val(s_chars[i + 1]) } else { 0 };

            if current < next {
                res -= current;
            } else {
                res += current;
            }
        }

        res
    }
}
