package longest_common_prefix_JAVA;

class Solution {
    public String longestCommonPrefix(String[] strs) {
        if (strs.length == 0) return "";

        String prefix = strs[0];

        for (int i = 1; i < strs.length; i++) {
            while (!strs[i].startsWith(prefix)) {
                prefix = prefix.substring(0, prefix.length() - 1);
                if (prefix.isEmpty()) return "";
            }
        }

        return prefix;
    }
}

/**
 * Minimal Main class for local compilation testing
 * Note: LeetCode runs only the Solution class. This Main is for CI build pass.
 */
public class LongestCommonPrefix {
    public static void main(String[] args) {
        Solution sol = new Solution();

        String[] example1 = {"flower", "flow", "flight"};
        String[] example2 = {"dog", "racecar", "car"};

        System.out.println("Example 1 Output: " + sol.longestCommonPrefix(example1)); // "fl"
        System.out.println("Example 2 Output: " + sol.longestCommonPrefix(example2)); // ""
    }
}
