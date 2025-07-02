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

## Comparassion table results
| **#** | **Language**   | **Runtime** | **Memory Usage** | **Approach Summary**                                                 | **Strengths**                                                | **Trade-offs**                                                 |
| ----- | -------------- | ----------- | ---------------- | -------------------------------------------------------------------- | ------------------------------------------------------------ | -------------------------------------------------------------- |
| 1     | **Rust**       | **0 ms**    | **2.2 MB**       | Prefix reduction loop with slice mutability                          | Fastest runtime and minimal memory; strong safety guarantees | Requires ownership and borrow checker understanding            |
| 2     | **Go**         | 0 ms        | **4.2 MB**       | Prefix trimming via range checks; efficient string slicing           | Low memory and fast; clean syntax for concurrency readiness  | Less ergonomic for advanced data structures                    |
| 3     | **C**          | 0 ms        | **8.1 MB**       | Character-by-character prefix comparison with null-termination logic | Direct memory access; simple compiled performance            | Verbose; manual memory safety responsibility                   |
| 4     | **C++**        | 0-3 ms      | \~12 MB          | String slicing with substr and iterative character matching          | Flexible standard library; competitive runtime               | More complex syntax; potential for performance variance        |
| 5     | **Python**     | 0 ms        | **12.6 MB**      | Prefix reduction via startswith in a while loop                      | Extremely readable; concise logic                            | Slower for larger inputs; interpreted overhead                 |
| 6     | **Java**       | 0-1 ms      | \~41 MB          | Prefix reduction with substring trimming                             | Strong typing; JVM optimizations; readable                   | Higher memory footprint due to JVM                             |
| 7     | **C#**         | 0-1 ms      | \~43 MB          | Prefix reduction with substring trimming; similar to Java            | Fast under .NET JIT; readable syntax                         | Higher memory usage due to .NET runtime                        |
| 8     | **JavaScript** | 0-1 ms      | \~54 MB          | Prefix slice reduction with while + indexOf logic                    | Fast for interpreted language; simple implementation         | Highest memory usage (V8 runtime); single-threaded limitations |

## 🔍 **Key Insights**

✅ **Rust leads** with minimal memory and fastest runtime, ideal for system-level optimizations.

✅ **Go and C** deliver optimal speed with low memory use and simplicity.

✅ **C++, Java, and C#** perform efficiently but come with higher runtime memory footprints due to their managed runtimes or language abstractions.

✅ **JavaScript** remains surprisingly performant but with **very high memory usage** due to the V8 engine.

✅ **Python is readable and elegant**, but interpreted performance remains a limitation for large-scale high-frequency calls.