# 📝 Problem 14. Longest Common Prefix

**Difficulty:** Easy  
**Topics:** Strings, Arrays

---

## 📖 Problem Statement

Write a function to find the **longest common prefix string** amongst an array of strings.

If there is **no common prefix**, return an **empty string `""`**.

---

### ✅ **Examples**

#### **Example 1**

**Input:**  
`strs = ["flower","flow","flight"]`

**Output:**  
`"fl"`

---

#### **Example 2**

**Input:**  
`strs = ["dog","racecar","car"]`

**Output:**  
`""`

**Explanation:**  
There is no common prefix among the input strings.

---

### 🔒 **Constraints**

- `1 <= strs.length <= 200`
- `0 <= strs[i].length <= 200`
- `strs[i]` consists of only **lowercase English letters** if it is non-empty.

---

## 💡 **Key insights**

- The longest common prefix must be a **prefix of the first word**.
- Comparing character by character is efficient to avoid unnecessary substring creation.
- If any string is empty, the result is automatically `""`.

---

### ✨ **Why is this problem important?**

✅ Trains **string manipulation fundamentals**  
✅ Builds intuition for **early stopping conditions** in scanning algorithms  
✅ Real-world applications:
  - Autocomplete systems
  - Command parsers
  - DNA sequence alignment (prefix similarity)

---

> **Learning in the AI era:**  
> Understanding string scanning logic across languages is more important than memorizing syntax.

