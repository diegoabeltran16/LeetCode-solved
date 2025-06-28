/**
 * LeetCode Problem 13: Roman to Integer
 * 
 * Converts a Roman numeral string to its integer representation.
 * Handles standard Roman numeral notation, including subtractive combinations
 * like IV (4), IX (9), XL (40), XC (90), CD (400), and CM (900).
 *
 * @param {string} s - A string containing the Roman numeral (e.g., "MCMXCIV").
 * @return {number} - The integer value corresponding to the Roman numeral.
 *
 * @example
 * console.log(romanToInt("III"));      // Output: 3
 * console.log(romanToInt("LVIII"));    // Output: 58
 * console.log(romanToInt("MCMXCIV"));  // Output: 1994
 */
var romanToInt = function (s) {
    // Mapping of Roman numerals to integer values
    let o = {
        'I': 1,
        'V': 5,
        'X': 10,
        'L': 50,
        'C': 100,
        'D': 500,
        'M': 1000
    };

    let ans = 0;

    // Iterate through the string, subtracting when a smaller numeral precedes a larger one
    for (let i = 0; i < s.length - 1; i++) {
        if (o[s[i]] < o[s[i + 1]]) {
            ans -= o[s[i]];
        } else {
            ans += o[s[i]];
        }
    }

    // Add the value of the last numeral
    return ans + o[s[s.length - 1]];
};

// Example execution for CI test
console.log(romanToInt("MCMXCIV")); // Output: 1994
