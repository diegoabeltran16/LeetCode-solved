class Solution(object):
    """
    LeetCode Problem 13: Roman to Integer

    This class provides a method to convert a Roman numeral string
    into its corresponding integer value.

    The implementation handles standard Roman numeral notation,
    including subtractive combinations (e.g., IV for 4, IX for 9, etc.).
    """

    def romanToInt(self, s):
        """
        Converts a Roman numeral string to its integer representation.

        :param s: str - A string containing the Roman numeral (e.g., "MCMXCIV").
        :return: int - The integer value corresponding to the Roman numeral.

        Example:
        >>> sol = Solution()
        >>> sol.romanToInt("MCMXCIV")
        1994
        """

        # Mapping Roman numerals to their integer values
        d = {'I': 1, 'V': 5, 'X': 10, 'L': 50,
             'C': 100, 'D': 500, 'M': 1000}
        res = 0

        # Iterate through the string, adding or subtracting based on value comparison
        for i in range(len(s)):
            if i + 1 < len(s) and (d[s[i]] < d[s[i + 1]]):
                # Subtractive combination detected (e.g. IV, IX)
                res -= d[s[i]]
            else:
                # Regular addition of numeral value
                res += d[s[i]]

        return res


# Example usage for CI test
if __name__ == "__main__":
    sol = Solution()
    print(sol.romanToInt("MCMXCIV"))  # Output: 1994
