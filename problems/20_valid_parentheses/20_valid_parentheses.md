# 📝 Problem 20: Valid Parentheses

## **Problem Description**

Given a string `s` containing just the characters `'('`, `')'`, `'{'`, `'}'`, `'['`, and `']'`, determine if the input string is **valid**.

### **Validity Rules**

- Open brackets must be closed by **the same type of brackets**.  
- Open brackets must be closed in the **correct order**.  
- Every close bracket has a **corresponding open bracket** of the same type.

---

## **Examples**

| Input | Output | Explanation |
|--|--|--|
| `"()"` | `true` | Single pair matched correctly |
| `"()[]{}"` | `true` | Multiple types matched correctly |
| `"(]"` | `false` | Different types closed incorrectly |
| `"([])"` | `true` | Nested matching brackets |

---

## **Constraints**

- `1 <= s.length <= 10^4`
- `s` consists of only `'('`, `')'`, `'{'`, `'}'`, `'['`, and `']'`.

---

## **Input**

- String `s`.

## **Output**

- Boolean value (`true` if valid, else `false`).

---

## **Approach Summary**

✅ **Stack-Based Matching (Optimal)**  
Push open brackets to a stack, pop and check match on closing brackets.  
- **Time Complexity:** O(n)  
- **Space Complexity:** O(n) stack

---

## **Why is this problem important?**

- Fundamental stack usage in parsing.  
- Basis for compiler syntax validation and interpreters.  
- Real-world analogy: **Gatekeeper ensuring gates are closed in correct order and type**.

