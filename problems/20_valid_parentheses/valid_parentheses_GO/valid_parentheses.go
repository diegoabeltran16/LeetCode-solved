// problems/20_valid_parentheses/valid_parentheses.go
// Problem: 20. Valid Parentheses (Go Repository Version)
// Contains stack-based implementation with documentation and usage examples.

package valid_parentheses

// IsValid checks whether the input string has valid parentheses.
//
// Approach:
// - Uses a stack to track opening brackets.
// - Matches closing brackets against the top of the stack.
//
// Time Complexity: O(n)
// Space Complexity: O(n)
func IsValid(s string) bool {
	stack := []rune{}
	pairs := map[rune]rune{
		')': '(',
		']': '[',
		'}': '{',
	}

	for _, ch := range s {
		if ch == '(' || ch == '[' || ch == '{' {
			stack = append(stack, ch)
		} else if ch == ')' || ch == ']' || ch == '}' {
			if len(stack) == 0 || stack[len(stack)-1] != pairs[ch] {
				return false
			}
			stack = stack[:len(stack)-1]
		}
	}
	return len(stack) == 0
}

/*
Usage Example:

package main

import (
	"fmt"
	"problems/20_valid_parentheses"
)

func main() {
	testCases := []string{"()", "()[]{}", "(]", "([])", "([)]", "{[]}"}

	for _, s := range testCases {
		fmt.Printf("Input: %s, Output: %v\n", s, valid_parentheses.IsValid(s))
	}
}
*/