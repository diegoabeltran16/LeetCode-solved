package roman_to_integer_JAVA;
import java.util.Map;
import java.util.HashMap;

/**
 * Solution for LeetCode Problem 13: Roman to Integer.
 *
 * This class provides a method to convert a Roman numeral string into its integer value.
 * The conversion works from right to left, subtracting values when a smaller numeral precedes a larger one,
 * and adding otherwise, according to Roman numeral rules.
 */
class Solution {
    /**
     * Converts a Roman numeral string to its integer representation.
     *
     * @param s the Roman numeral string (e.g., "MCMXCIV")
     * @return the integer value of the Roman numeral
     */
    public int romanToInt(String s) {
        // Map each Roman numeral to its integer value
        Map<Character, Integer> romanMap = new HashMap<>();
        romanMap.put('I', 1);
        romanMap.put('V', 5);
        romanMap.put('X', 10);
        romanMap.put('L', 50);
        romanMap.put('C', 100);
        romanMap.put('D', 500);
        romanMap.put('M', 1000); 

        int result = 0;
        int prevValue = 1; // Previous numeral value (start with 1 for the rightmost character)

        // Iterate from right to left through the string
        for (int i = s.length() - 1; i >= 0; i--) {
            int currValue = romanMap.get(s.charAt(i));
            if (currValue < prevValue) {
                // If current value is less than previous, subtract it
                result -= currValue; 
            } else {
                // Otherwise, add it
                result += currValue; 
            }
            prevValue = currValue; 
        } 
        return result;
    } 
}