#!/usr/bin/env node
/**
 * Problem: 20. Valid Parentheses (JavaScript Repository Version)
 * Contains two approaches: concise and explicit
 *
 * CI Workflow: JavaScript Run Check (javascript.yml) will execute this file via `node`.
 *
 * Usage:
 *   node valid_parentheses.js
 */

/**
 * Concise mapping-based approach.
 * @param {string} s
 * @return {boolean}
 */
function isValid(s) {
    const stack = [];
    const pairs = {')': '(', ']': '[', '}': '{'};

    for (const ch of s) {
        if (pairs[ch]) {
            if (stack.pop() !== pairs[ch]) return false;
        } else {
            stack.push(ch);
        }
    }
    return stack.length === 0;
}

/**
 * Explicit approach with direct condition checks.
 * @param {string} s
 * @return {boolean}
 */
function isValidExplicit(s) {
    const stack = [];
    for (const ch of s) {
        if (ch === '(' || ch === '[' || ch === '{') {
            stack.push(ch);
        } else if (ch === ')' || ch === ']' || ch === '}') {
            if (!stack.length) return false;
            const top = stack[stack.length - 1];
            if ((ch === ')' && top === '(') ||
                (ch === ']' && top === '[') ||
                (ch === '}' && top === '{')) {
                stack.pop();
            } else {
                return false;
            }
        }
    }
    return stack.length === 0;
}

// Usage examples for CI and manual testing
const testCases = ["()", "()[]{}", "(]", "([])", "([)]", "{[]}" ];
console.log("Testing isValid (concise)");
testCases.forEach(s => console.log(`Input: ${s}, Output: ${isValid(s)}`));

console.log("\nTesting isValidExplicit (explicit)");
testCases.forEach(s => console.log(`Input: ${s}, Output: ${isValidExplicit(s)}`));

// Export for external testing if required
module.exports = { isValid, isValidExplicit };
