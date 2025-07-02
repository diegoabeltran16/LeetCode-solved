// LeetCode Problem 14: Longest Common Prefix
// Solution by prefix reduction approach
// Efficient and widely used for production

using System;

namespace LeetCodeProblems.LongestCommonPrefix
{
    public class Solution
    {
        /// <summary>
        /// Finds the longest common prefix among an array of strings.
        /// If no common prefix exists, returns an empty string.
        /// </summary>
        /// <param name="strs">Array of strings</param>
        /// <returns>Longest common prefix string</returns>
        public string LongestCommonPrefix(string[] strs)
        {
            if (strs == null || strs.Length == 0)
                return "";

            // Start with the first string as the prefix candidate
            string prefix = strs[0];

            // Compare the prefix with each string in the array
            foreach (string s in strs)
            {
                while (!s.StartsWith(prefix))
                {
                    // Reduce the prefix by one character from the end
                    prefix = prefix.Substring(0, prefix.Length - 1);

                    // If prefix becomes empty, no common prefix exists
                    if (prefix == "")
                        return "";
                }
            }

            return prefix;
        }
    }
}
