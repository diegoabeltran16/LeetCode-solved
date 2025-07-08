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

## **Why is this problem important?**

- Fundamental stack usage in parsing.  
- Basis for compiler syntax validation and interpreters.  
- Real-world analogy: **Gatekeeper ensuring gates are closed in correct order and type**.

## 📊 **Performance Comparison Summary**

| Language                        | Runtime | Memory       | Approach                     | Gatekeeper Type       |
| ------------------------------- | ------- | ------------ | ---------------------------- | --------------------- |
| **C# (expected closer push)**   | 2 ms    | 42 MB        | Pushes expected closers      | Preemptive Gatekeeper |
| **C# (optimized)**              | 1 ms    | 41.7 MB      | Same as above                | Preemptive Gatekeeper |
| **C# (dictionary-based)**       | 5 ms    | 42.2 MB      | Map-based                    | Scholar Gatekeeper    |
| **C++**                         | 0 ms    | 9.1 MB       | Stack + unordered\_map       | Scholar Gatekeeper    |
| **C**                           | 0 ms    | 8.6 MB       | Stack with direct comparison | Memorizer Gatekeeper  |
| **Rust**                        | 0 ms    | 2.2 MB       | Vec + HashMap                | Scholar Gatekeeper    |
| **JavaScript**                  | 2 ms    | 55.4 MB      | Object mapping + stack       | Scholar Gatekeeper    |
| **Java (expected closer push)** | 2 ms    | 41.8 MB      | Pushes expected closers      | Preemptive Gatekeeper |
| **Java (map-based)**            | 3 ms    | 41.8 MB      | Map-based                    | Scholar Gatekeeper    |
| **Go**                          | 0 ms    | 4.2 MB       | Slice stack + map            | Scholar Gatekeeper    |
| **Python**                      | 0-3 ms  | 12.4-12.7 MB | Stack + dict mapping         | Scholar Gatekeeper    |

---

## 💡 **Analytical Summary**

✅ **Fastest Runtimes (0 ms)**: C, C++, Rust, Go, Python.

* **Rust (0 ms, 2.2 MB)** has the **best memory efficiency** due to zero-cost abstractions.

✅ **Best Memory Usage**: Rust < Go < C < C++.

✅ **C#/Java/JS**: Good runtimes but higher memory due to runtime environments.

---

## ⚔️ **Approach Strengths & Trade-offs**

| Approach                              | Strength                        | Trade-off                      |
| ------------------------------------- | ------------------------------- | ------------------------------ |
| **Direct Comparison (Memorizer)**     | Minimal overhead, fast          | Harder to extend for new types |
| **Map-based (Scholar)**               | Clean, scalable                 | Slight lookup overhead         |
| **Expected Closer Push (Preemptive)** | Fastest runtime, minimal checks | Less intuitive for beginners   |

---

## 🎓 **Mentor Takeaways**

✅ All are **O(n)** in time and space complexity – differences are **implementation-level**.

✅ **Rust and Go** excel in **memory efficiency**.

✅ **Preemptive Gatekeeper approach (push expected closers)** is often fastest in C# and Java.

✅ For **interviews**, use:

* **Preemptive Gatekeeper** for clarity and performance.
* **Scholar Gatekeeper (Map-based)** for scalability to new types.

---

### 🏷️ **Final Comparative Table**

| Language   | Runtime | Memory       | Gatekeeper           |
| ---------- | ------- | ------------ | -------------------- |
| Rust       | 0 ms    | 2.2 MB       | Scholar              |
| Go         | 0 ms    | 4.2 MB       | Scholar              |
| C          | 0 ms    | 8.6 MB       | Memorizer            |
| C++        | 0 ms    | 9.1 MB       | Scholar              |
| Python     | 0-3 ms  | 12.4-12.7 MB | Scholar              |
| Java       | 2-3 ms  | 41.8 MB      | Preemptive / Scholar |
| C#         | 1-5 ms  | 41.7-42.2 MB | Preemptive / Scholar |
| JavaScript | 2 ms    | 55.4 MB      | Scholar              |

---

## 🔑 **Conclusion**

The choice of approach (Memorizer, Scholar, Preemptive) depends on **problem constraints, extensibility needs, and performance goals**. Understanding these patterns deeply ensures confidence in interviews, production, and compiler-level parsing challenges.

# 🏰 **The Castle of Brackets: A Multi-Gatekeeper Tale**

In the kingdom of **Syntaxia**, there stood a grand **Castle of Brackets** with three ancient gates:

* **Round Gate `()`** guarded by Elves.
* **Square Gate `[]`** guarded by Dwarves.
* **Curly Gate `{}`** guarded by Wizards.

To protect the castle from intruders and syntax errors, three types of **Gatekeepers** served with distinct skills:

### 👑 **1. The Memorizer Gatekeeper (Direct Comparison)**

They carried **three keys on their belt** for each gate type, checking each closing gate manually against their memorised opener.

* **Strength:** Fastest and simplest when gate types are few and known.
* **Trade-off:** Hard to extend when new gates are introduced.

### 👑 **2. The Scholar Gatekeeper (Map-based)**

They wielded a **Magic Ledger (HashMap/Object/Dictionary)** listing every gate type and its match.

* **Strength:** Easily scalable to new gate types.
* **Trade-off:** Slightly slower due to reading their ledger each time.

### 👑 **3. The Preemptive Gatekeeper (Push Expected Closer)**

They wrote the **expected closing gate directly onto their scroll (stack)** each time they opened a gate.

* **Strength:** Fastest approach as they only check against expected closers.
* **Trade-off:** Less intuitive for beginners; requires thinking ahead.

---

### ⚔️ **The Tale of Multi-Gatekeeper Strategy**

On **peaceful days with few travellers**, the Memorizer excelled with quick manual checks.

During **busy festivals with diverse visitors**, the Scholar triumphed by consulting their ledger without memorising each type.

When the kingdom announced **new gates for dragons `< >` or fairies `<>`**, the Scholar easily updated their book, while the Memorizer struggled to remember new symbols.

Finally, in **high-speed tournaments where efficiency was critical**, the Preemptive Gatekeeper ruled, writing expected closers before any traveller attempted exit.

---

### 📝 **Mentor’s Wisdom**

✅ Use **Memorizer Gatekeeper** when gate types are known and minimal.
✅ Use **Scholar Gatekeeper** for scalable, maintainable solutions.
✅ Use **Preemptive Gatekeeper** when performance is paramount.

Thus, the Castle of Brackets remained protected, its gates closed in perfect order, as each Gatekeeper used their strength wisely.


