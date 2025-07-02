/**
 * ✅ Efficient because it:
 * - Starts with the first word as the initial prefix.
 * - Iterates through each word, and while the current word does NOT start with the prefix,
 *   it removes the last character from the prefix until a common prefix is found.
 * - Returns early if the prefix becomes empty.
 *
 * @param {string[]} strs - Array of input strings.
 * @return {string} The longest common prefix, or "" if none exists.
 * 
 * Example:
 * Input: ["flower","flow","flight"]
 * Output: "fl"
 */
var longestCommonPrefix = function(strs) {
    if (strs.length === 0) return "";

    let prefix = strs[0];

    for (let word of strs) {
        // Skip checking if word is exactly the current prefix
        if (word === prefix) continue;

        // While the current word does not start with the prefix
        while (word.indexOf(prefix) !== 0) {
            // Remove the last character from prefix
            prefix = prefix.slice(0, -1);
            // If prefix becomes empty, return ""
            if (!prefix) return "";
        }
    }

    return prefix;
};
