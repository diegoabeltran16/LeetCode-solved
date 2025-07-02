package main

import (
    "fmt"
)

/*
LeetCode Problem 14: Longest Common Prefix

This solution uses the **prefix reduction approach**:
- Initialize the prefix as the first string.
- Iterate through each subsequent string:
  - Reduce the prefix until it matches the current string's prefix.
  - If it becomes empty, return "" immediately.

Time Complexity: O(S), where S is the sum of all characters.
Space Complexity: O(1) extra space.
*/

// longestCommonPrefix returns the longest common prefix among an array of strings.
func longestCommonPrefix(strs []string) string {
    if len(strs) == 0 {
        return ""
    }

    prefix := strs[0]

    for _, s := range strs[1:] {
        // Reduce prefix while it does not match current string
        for len(prefix) > 0 && (len(s) < len(prefix) || s[:len(prefix)] != prefix) {
            prefix = prefix[:len(prefix)-1]
        }

        if prefix == "" {
            return ""
        }
    }

    return prefix
}

// Example usage for local testing
func main() {
    fmt.Println(longestCommonPrefix([]string{"flower", "flow", "flight"})) // Output: "fl"
    fmt.Println(longestCommonPrefix([]string{"dog", "racecar", "car"}))    // Output: ""
}
