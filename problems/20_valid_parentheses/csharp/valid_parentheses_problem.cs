/*
 * Includes problem statement, constraints, examples, and The Castle of Brackets story.
 */

namespace ValidParenthesesProblem
{
    public static class ProblemDescription
    {
        public static void Show()
        {
            Console.WriteLine("Problem: 20. Valid Parentheses\n");

            Console.WriteLine("Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.\n");

            Console.WriteLine("An input string is valid if:\n" +
                "1. Open brackets are closed by the same type of brackets.\n" +
                "2. Open brackets are closed in the correct order.\n" +
                "3. Every closing bracket has a corresponding opening bracket of the same type.\n");

            Console.WriteLine("Examples:");
            Console.WriteLine("Input: s = \"()\" -> Output: true");
            Console.WriteLine("Input: s = \"()[]{}\" -> Output: true");
            Console.WriteLine("Input: s = \"(]\" -> Output: false");
            Console.WriteLine("Input: s = \"([])\" -> Output: true\n");

            Console.WriteLine("Constraints:");
            Console.WriteLine("1 <= s.length <= 10^4");
            Console.WriteLine("s consists of parentheses only '()[]{}'.\n");

            Console.WriteLine("🏰 Story: The Castle of Brackets\n");
            Console.WriteLine("Imagine you are a castle gatekeeper guarding three magical gates:\n" +
                "- Round Gate '()' for elves\n" +
                "- Square Gate '[]' for dwarves\n" +
                "- Curly Gate '{}' for wizards\n\n" +
                "You keep a scroll (stack) to record opened gates. When someone wants to close a gate,\n" +
                "you check if it matches the last one opened. If everything matches perfectly and your scroll is empty at the end,\n" +
                "the castle is safe.");
        }
    }
}


