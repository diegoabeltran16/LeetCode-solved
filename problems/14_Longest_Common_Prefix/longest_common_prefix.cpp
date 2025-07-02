/**
 * @file longest_common_prefix.cpp
 * @brief Solution for LeetCode Problem 14: Longest Common Prefix
 *
 * This program finds the longest common prefix among an array of strings.
 * The approach uses prefix reduction by comparing each string with the current prefix.
 *
 * Compilation:
 *   g++ -std=c++17 longest_common_prefix.cpp -o longest_common_prefix
 */

#include <iostream>
#include <vector>
#include <string>

class Solution {
public:
    /**
     * @brief Finds the longest common prefix in an array of strings.
     *
     * @param strs Vector of input strings.
     * @return The longest common prefix string. Empty if none exists.
     */
    std::string longestCommonPrefix(std::vector<std::string>& strs) {
        if (strs.empty()) return "";

        std::string prefix = strs[0];

        for (size_t i = 1; i < strs.size(); ++i) {
            size_t j = 0;
            while (j < prefix.size() && j < strs[i].size() && prefix[j] == strs[i][j]) {
                ++j;
            }
            prefix = prefix.substr(0, j);

            // Early termination if prefix is empty
            if (prefix.empty()) {
                break;
            }
        }

        return prefix;
    }
};

#ifdef RUN_LOCAL_TEST
int main() {
    Solution sol;
    std::vector<std::string> test1 = {"flower", "flow", "flight"};
    std::vector<std::string> test2 = {"dog", "racecar", "car"};

    std::cout << "Test1 (Expected 'fl'): " << sol.longestCommonPrefix(test1) << std::endl;
    std::cout << "Test2 (Expected ''): " << sol.longestCommonPrefix(test2) << std::endl;

    return 0;
}
#endif
