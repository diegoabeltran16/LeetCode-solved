/*
 * - Contains two approaches with XML doc comments and Castle Gatekeeper Story.
 */

using System;
using System.Collections.Generic;

namespace ValidParenthesesSolution
{
    public class Solution
    {
        /// <summary>
        /// Scholar Gatekeeper (Dictionary-based approach).
        /// Uses a Dictionary to map closing brackets to opening brackets.
        /// </summary>
        /// <param name="s">Input string containing '()[]{}'</param>
        /// <returns>True if valid, false otherwise.</returns>
        public bool IsValidScholar(string s)
        {
            Stack<char> stack = new Stack<char>();
            Dictionary<char, char> pairs = new Dictionary<char, char>
            {
                {')', '('},
                {']', '['},
                {'}', '{'}
            };

            foreach (char ch in s)
            {
                if (pairs.ContainsKey(ch))
                {
                    if (stack.Count == 0 || stack.Pop() != pairs[ch])
                        return false;
                }
                else
                {
                    stack.Push(ch);
                }
            }
            return stack.Count == 0;
        }

        /// <summary>
        /// Preemptive Gatekeeper (Push expected closer approach).
        /// Pushes expected closing brackets onto the stack for direct matching.
        /// </summary>
        /// <param name="s">Input string containing '()[]{}'</param>
        /// <returns>True if valid, false otherwise.</returns>
        public bool IsValidPreemptive(string s)
        {
            Stack<char> stack = new Stack<char>();

            foreach (char c in s)
            {
                if (c == '(') { stack.Push(')'); }
                else if (c == '[') { stack.Push(']'); }
                else if (c == '{') { stack.Push('}'); }
                else
                {
                    if (stack.Count == 0 || stack.Pop() != c) return false;
                }
            }
            return stack.Count == 0;
        }
    }

    class Program
    {
        static void Main()
        {
            Solution sol = new Solution();
            string[] testCases = { "()", "()[]{}", "(]", "([])", "([)]", "{[]}" };

            Console.WriteLine("Testing IsValidScholar (Dictionary-based):");
            foreach (var s in testCases)
            {
                Console.WriteLine($"Input: {s}, Output: {sol.IsValidScholar(s)}");
            }

            Console.WriteLine("\nTesting IsValidPreemptive (Push expected closer):");
            foreach (var s in testCases)
            {
                Console.WriteLine($"Input: {s}, Output: {sol.IsValidPreemptive(s)}");
            }
        }
    }
}

/*
 * Story:
 * Scholar Gatekeeper – Uses a magic dictionary to map closers to openers.
 * Preemptive Gatekeeper – Writes expected closing gates on the scroll when opening gates.
 * Both keep the castle safe if their strategy fits the traveller patterns.
 */
