"""
Problem: 20. Valid Parentheses
Repository: problems/20_valid_parentheses/valid_parentheses.py
Contains two approaches for clarity and learning.
"""

def isValid_concise(s):
    """
    Stack-based concise solution.

    Args:
        s (str): Input string containing '()[]{}'.

    Returns:
        bool: True if valid, False otherwise.

    Approach:
        Uses stack and direct mapping.
    """
    stack = []
    pairs = {')': '(', ']': '[', '}': '{'}

    for ch in s:
        if ch in pairs:
            if not stack or stack.pop() != pairs[ch]:
                return False
        else:
            stack.append(ch)

    return not stack


def isValid_explicit(s):
    """
    Stack-based explicit solution with early length check.

    Args:
        s (str): Input string containing '()[]{}'.

    Returns:
        bool: True if valid, False otherwise.

    Approach:
        - Early return if length <= 1 (invalid).
        - Push open brackets explicitly.
        - On closing bracket, checks stack top before popping.
    """
    stack = []
    pairs = {')': '(', ']': '[', '}': '{'}

    if len(s) <= 1:
        return False

    for char in s:
        if char in "([{":
            stack.append(char)
        elif char in ")]}":
            if not stack or stack[-1] != pairs[char]:
                return False
            stack.pop()

    return len(stack) == 0


if __name__ == "__main__":
    # Usage examples for testing and documentation clarity
    test_cases = ["()", "()[]{}", "(]", "([])", "([)]", "{[]}" ]

    print("Testing isValid_concise")
    for s in test_cases:
        print(f"Input: {s}, Output: {isValid_concise(s)}")

    print("\nTesting isValid_explicit")
    for s in test_cases:
        print(f"Input: {s}, Output: {isValid_explicit(s)}")

"""
Both functions are optimal (O(n) time, O(n) space). The concise version is shorter;
the explicit version is clearer for beginners.
Choose per context: interviews (explicit), production (concise).
"""
