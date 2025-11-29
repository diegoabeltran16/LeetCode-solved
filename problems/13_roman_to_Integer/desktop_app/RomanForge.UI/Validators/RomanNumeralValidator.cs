using System.Linq;
using System.Text.RegularExpressions;

namespace RomanForge.UI.Validators
{
    /// <summary>
    /// Roman Numeral Validator
    /// The Humanoid City's first line of defense
    /// </summary>
    public static class RomanNumeralValidator
    {
        private static readonly char[] ValidChars = { 'I', 'V', 'X', 'L', 'C', 'D', 'M' };
        
        // Basic regex pattern for Roman numerals
        // This could be made more sophisticated to catch invalid patterns
        private static readonly Regex RomanPattern = new Regex(
            @"^M{0,3}(CM|CD|D?C{0,3})(XC|XL|L?X{0,3})(IX|IV|V?I{0,3})$",
            RegexOptions.Compiled | RegexOptions.IgnoreCase
        );

        /// <summary>
        /// Validates if a string is a valid Roman numeral
        /// </summary>
        public static bool IsValidRoman(string input)
        {
            if (string.IsNullOrWhiteSpace(input))
                return false;

            // Must contain only valid Roman numeral characters
            if (!input.All(c => ValidChars.Contains(char.ToUpper(c))))
                return false;

            // Must match valid Roman numeral pattern
            return RomanPattern.IsMatch(input);
        }

        /// <summary>
        /// Validates if an integer is in valid Roman numeral range
        /// </summary>
        public static bool IsValidRange(int value)
        {
            return value >= 1 && value <= 3999;
        }
    }
}
