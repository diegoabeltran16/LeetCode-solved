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

/**
 * Story for JavaScript approaches:
 *
 * **Concise Mapping-Based Approach Story**:
 * Imagine you have a **magic dictionary scroll** that tells you exactly which opening bracket matches each closing bracket.
 * Whenever you see a closing bracket, you quickly look it up in your scroll to see the expected opener and check the last opened gate on your stack.
 *
 * **Why use it?**
 * Fast when you have a neat mapping prepared – like having pre-written notes during an open-book exam.
 *
 * ---
 *
 * **Explicit Direct Comparison Approach Story**:
 * Here, you are the **gatekeeper memorizing each gate type’s match manually**.
 * You check directly:
 * - “If I see ')', is the top '(' ?”
 * - “If I see ']', is the top '[' ?”
 * - “If I see '}', is the top '{' ?”
 *
 * **Why use it?**
 * You don’t rely on external mappings. It’s simple, direct, and fast when only a few types exist.
 *
 * ---
 *
 * ⚖️ **Comparison with Java Approaches:**
 * The logic is identical – only syntax differs. Both languages use the same stack-based reasoning and pattern.
 *
 */
