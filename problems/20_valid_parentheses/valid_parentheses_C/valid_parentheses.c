/*
 * - Uses stack implemented as array with top pointer.
 * - Includes documentation, explanation, and CI/CD compatibility.
 *
 * Story (Castle Gatekeeper – Direct Comparison):
 * The gatekeeper memorizes each gate type directly, checking manually for round '()', square '[]', and curly '{}'.
 * Efficient when only a few gate types exist in the kingdom.
 */

#include <stdbool.h>
#include <string.h>
#include <stdio.h>

/**
 * Checks if the input string has valid parentheses.
 *
 * @param s input string containing only '(', ')', '{', '}', '[', ']'
 * @return true if valid, false otherwise
 */
bool isValid(char* s) {
    char stack[10000]; // maximum input length per problem constraints
    int top = -1;

    for (int i = 0; s[i] != '\0'; i++) {
        char ch = s[i];
        if (ch == '(' || ch == '[' || ch == '{') {
            stack[++top] = ch; // push onto stack
        } else {
            if (top == -1) return false; // stack empty when closing found
            if ((ch == ')' && stack[top] != '(') ||
                (ch == ']' && stack[top] != '[') ||
                (ch == '}' && stack[top] != '{')) {
                return false; // mismatched closing bracket
            }
            top--; // pop from stack
        }
    }
    return top == -1; // true if all brackets matched and stack empty
}

#ifdef TEST_VALID_PARENTHESIS
int main() {
    char *testCases[] = {"()", "()[]{}", "(]", "([])", "([)]", "{[]}"};
    int expected[] = {1, 1, 0, 1, 0, 1};

    printf("Testing isValid:\n");
    for (int i = 0; i < 6; i++) {
        bool result = isValid(testCases[i]);
        printf("Input: %s, Output: %s, Expected: %s\n", testCases[i], result ? "true" : "false", expected[i] ? "true" : "false");
    }
    return 0;
}
#endif