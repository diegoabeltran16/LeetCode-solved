struct Solution;
impl Solution {
    /// Returns the longest common prefix among a vector of strings.
    ///
    /// # Arguments
    /// * `strs` - A vector of `String` containing input strings.
    ///
    /// # Returns
    /// * A `String` representing the longest common prefix. Returns empty if none exists.
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        // If the input vector is empty, return an empty string immediately
        if strs.is_empty() {
            return String::new();
        }

        // Start by assuming the first string is the common prefix
        let mut prefix = strs[0].clone();

        // Iterate through each string after the first
        for s in strs.iter().skip(1) {
            // While the current string does not start with prefix
            while !s.starts_with(&prefix) {
                // Remove the last character from prefix
                prefix.pop();

                // If prefix becomes empty, there is no common prefix
                if prefix.is_empty() {
                    return String::new();
                }
            }
        }

        prefix
    }
}
