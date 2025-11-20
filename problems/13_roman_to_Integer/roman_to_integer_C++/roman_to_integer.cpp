/**
 * LeetCode Problem 13: Roman to Integer
 *
 * This class provides a method to convert a Roman numeral string
 * into its corresponding integer value.
 *
 * The implementation handles standard Roman numeral notation,
 * including subtractive combinations (e.g., IV for 4, IX for 9, etc.).
 *
 * Example usage:
 *   Solution sol;
 *   int result = sol.romanToInt("MCMXCIV"); // result = 1994
 */

#include <string>

class Solution {
public:
    /**
     * Converts a Roman numeral string to its integer representation.
     *
     * @param s A string containing the Roman numeral (e.g., "MCMXCIV").
     * @return The integer value corresponding to the Roman numeral.
     */
    int romanToInt(std::string s)
    {
        int sum {0};

        // Lambda function to return the integer value of a Roman numeral character.
        auto romanToValue = [](char roman) constexpr {
            switch (roman)
            {
                case 'I': return 1;
                case 'V': return 5;
                case 'X': return 10;
                case 'L': return 50;
                case 'C': return 100;
                case 'D': return 500;
                case 'M': return 1000;
                default: return 0;
            }
        };

        // Iterate through the string, adding or subtracting values based on Roman numeral rules.
        for (int i = 0; i < s.length(); ++i)
        {
            if (i + 1 < s.length() && romanToValue(s[i]) < romanToValue(s[i + 1]))
            {
                // Subtractive combination detected, subtract current value.
                sum -= romanToValue(s[i]);
            }
            else
            {
                // Regular addition of numeral value.
                sum += romanToValue(s[i]);
            }
        }

        return sum;
    }
};
