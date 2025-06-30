using System;
using System.Collections.Generic;
using AE = System.ArgumentException;

/// <summary>
/// Solution class for LeetCode Roman to Integer problem.
/// Converts a Roman numeral string to its integer representation.
/// </summary>
public class Solution
{
    /// <summary>
    /// Converts a Roman numeral string to an integer.
    /// </summary>
    /// <param name="s">Roman numeral string</param>
    /// <returns>Integer value</returns>
    /// <exception cref="ArgumentException">Thrown if result exceeds valid range</exception>
    public int RomanToInt(string s)
    {
        Dictionary<char, int> rn = new Dictionary<char, int>
        {
            {'I', 1},
            {'V', 5},
            {'X', 10},
            {'L', 50},
            {'C', 100},
            {'D', 500},
            {'M', 1000}
        };

        int returnValue = 0;
        char prevDom = 'I';
        for (int i = s.Length - 1; i >= 0; i--)
        {
            if (rn[s[i]] < rn[prevDom])
            {
                returnValue -= rn[s[i]];
            }
            else
            {
                returnValue += rn[s[i]];
            }

            if (rn[s[i]] > rn[prevDom])
            {
                prevDom = s[i];
            }

            if (returnValue > 3999)
            {
                throw new AE("Value out of range.");
            }
        }
        return returnValue;
    }
}
