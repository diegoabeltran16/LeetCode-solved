#include <stdio.h>

/**
 * Finds the longest common prefix among the given strings.
 *
 * @param strs An array of strings
 * @param strsSize The number of strings in the array
 * @return The longest common prefix string
 */
char * longestCommonPrefix(char ** strs, int strsSize){
    if (strsSize == 0) return "";

    // Start with the first string as the initial prefix
    char *prefix = strs[0];

    for (int i = 1; i < strsSize; i++) {
        // Compare prefix with each string character by character
        int j = 0;
        while (prefix[j] && strs[i][j] && prefix[j] == strs[i][j]) {
            j++;
        }
        prefix[j] = '\0'; // Cut the prefix where it no longer matches

        // If prefix becomes empty, no common prefix exists
        if (prefix[0] == '\0') return "";
    }

    return prefix;
}
