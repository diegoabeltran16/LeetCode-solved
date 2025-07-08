// Contains two approaches: direct comparison and map-based matching.
// Note: Class is package-private to allow filename mismatch without 'public'.

import java.util.Stack;
import java.util.Map;
import java.util.HashMap;

class ValidParentheses {

    /**
     * Direct comparison approach.
     * Uses if-else conditions for each bracket type.
     * Faster due to avoiding HashMap lookup.
     *
     * @param s input string containing only '()[]{}'
     * @return true if valid, false otherwise
     */
    public static boolean isValidDirect(String s) {
        Stack<Character> st = new Stack<>();
        for (int i = 0; i < s.length(); i++) {
            char ch = s.charAt(i);
            if (ch == '[' || ch == '{' || ch == '(') {
                st.push(ch);
            } else {
                if (st.isEmpty()) return false;
                if ((ch == ']' && st.peek() == '[') ||
                    (ch == '}' && st.peek() == '{') ||
                    (ch == ')' && st.peek() == '(')) {
                    st.pop();
                } else {
                    return false;
                }
            }
        }
        return st.isEmpty();
    }

    /**
     * Map-based approach.
     * Uses HashMap to match closing to opening brackets.
     * More flexible for extended bracket types.
     *
     * @param s input string containing only '()[]{}'
     * @return true if valid, false otherwise
     */
    public static boolean isValidMap(String s) {
        Stack<Character> stack = new Stack<>();
        Map<Character, Character> pairs = new HashMap<>();
        pairs.put(')', '(');
        pairs.put(']', '[');
        pairs.put('}', '{');

        for (char ch : s.toCharArray()) {
            if (pairs.containsKey(ch)) {
                if (stack.isEmpty() || stack.pop() != pairs.get(ch)) {
                    return false;
                }
            } else {
                stack.push(ch);
            }
        }
        return stack.isEmpty();
    }

    /**
     * Usage examples for testing and documentation clarity.
     */
    public static void main(String[] args) {
        String[] testCases = {"()", "()[]{}", "(]", "([])", "([)]", "{[]}"};

        System.out.println("Testing isValidDirect");
        for (String s : testCases) {
            System.out.printf("Input: %s, Output: %b\n", s, isValidDirect(s));
        }

        System.out.println("\nTesting isValidMap");
        for (String s : testCases) {
            System.out.printf("Input: %s, Output: %b\n", s, isValidMap(s));
        }
    }
}


// 📝 **Story for Understanding Both Approaches**
// Imagine you are a **castle gatekeeper**.
// In the **Direct Comparison approach**, you have **three separate keys hanging on your belt**: one for round gates, one for curly gates, one for square gates. Every time someone wants to leave, you check manually which key matches the gate they are at.
// In the **Map-Based approach**, you keep a **magic book** that tells you *“for every closing gate symbol, here is its matching opening symbol”*. So when someone wants to leave, you open your book, look up the closing symbol, and confirm it matches the last gate opened.
// - **Direct comparison is faster if you only guard three types of gates**, because you don't need to consult the book.
// - **Map-based approach is better if the kingdom adds many new gate types**, because your book scales easily, and you don’t need to write new if-else rules for each gate.

