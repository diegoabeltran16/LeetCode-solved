/*
 * - Uses stack with unordered_map for mapping closing brackets to openers.
 * - Includes full documentation, explanation, and usage examples.
 *
 * Story (Castle Gatekeeper – Map-Based Approach):
 * The gatekeeper keeps a **magic book (unordered_map)** mapping each closing gate to its opening gate.
 * They check each closing gate against the last opened gate in their **scroll (stack)** to maintain kingdom safety.
 */

#include <iostream>
#include <stack>
#include <unordered_map>
#include <vector>

/**
 * Checks if the input string has valid parentheses.
 *
 * @param s input string containing only '()[]{}'
 * @return true if valid, false otherwise
 */
bool isValid(std::string s) {
    std::stack<char> stack;
    std::unordered_map<char, char> pairs = {
        {')', '('},
        {']', '['},
        {'}', '{'}
    };

    for (char ch : s) {
        if (pairs.count(ch)) {
            if (stack.empty() || stack.top() != pairs[ch]) {
                return false;
            }
            stack.pop();
        } else {
            stack.push(ch);
        }
    }
    return stack.empty();
}

#ifdef TEST_VALID_PARENTHESIS
int main() {
    std::vector<std::pair<std::string, bool>> testCases = {
        {"()", true},
        {"()[]{}", true},
        {"(]", false},
        {"([])", true},
        {"([)]", false},
        {"{[]}", true}
    };

    std::cout << "Testing isValid:" << std::endl;
    for (auto& [input, expected] : testCases) {
        bool result = isValid(input);
        std::cout << "Input: " << input << ", Output: " << (result ? "true" : "false")
                  << ", Expected: " << (expected ? "true" : "false") << std::endl;
    }
    return 0;
}
#endif
