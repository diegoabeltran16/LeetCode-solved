
# 📝 Problem 21: Merge Two Sorted Lists

## **Problem Description**

Given the heads of two sorted linked lists, `list1` and `list2`, merge them into one sorted linked list. The list should be made by splicing together nodes of the first two lists without creating new nodes.

### **Merge Rules**

* Both input lists are sorted in non-decreasing order.
* The merged linked list should reuse existing nodes.
* Maintain the sorted order during merging.

---

## **Examples**

| Input                | Output          | Explanation                  |
| -------------------- | --------------- | ---------------------------- |
| `[1,2,4]`, `[1,3,4]` | `[1,1,2,3,4,4]` | Nodes merged in sorted order |
| `[]`, `[]`           | `[]`            | Both lists empty             |
| `[]`, `[0]`          | `[0]`           | One list empty               |

---

## **Constraints**

* Number of nodes in both lists: `[0, 50]`
* Node values: `-100 <= Node.val <= 100`
* Lists sorted in non-decreasing order.

---

## **Input**

* Heads of two singly-linked lists (`list1`, `list2`).

## **Output**

* Head of the merged sorted linked list.

---

## **Why is this problem important?**

* Fundamental problem for understanding linked list manipulation.
* Essential for mastering merge operations in algorithms like Merge Sort.
* Real-world analogy: **Merging sorted data streams efficiently**.

---

## 📊 **Performance Comparison Summary**

| Language   | Runtime | Memory   | Approach                    | Conductor Type       |
| ---------- | ------- | -------- | --------------------------- | -------------------- |
| Python     | 1 ms    | 12.4 MB  | Dummy node, iterative merge | Cautious Conductor   |
| Go         | 0 ms    | 4.35 MB  | Dummy node, iterative merge | Nimble Conductor     |
| Rust       | 0 ms    | 2.1 MB   | Dummy node, iterative merge | Fearless Conductor   |
| Java       | 0 ms    | 42.6 MB  | Dummy node, iterative merge | Disciplined Conductor|
| JavaScript | 0 ms    | 58.3 MB  | Dummy node, iterative merge | Pragmatic Conductor  |
| C          | 0 ms    | 10.7 MB  | Dummy node, iterative merge | Deterministic Conductor |
| C++        | 0 ms    | 19.6 MB  | Dummy node, iterative merge | Classicist Conductor |
| C#         | 0 ms    | 43.0 MB  | Dummy node, iterative merge | Methodical Conductor |

---

## 💡 **Analytical Summary**

✅ **Fastest Runtime (0 ms)**: Go, Rust, Java, JS, C, C++, C#

✅ **Best Memory Usage**: Rust < Go < C < C++

✅ **Python**: Still valuable for readability and prototyping

---

## ⚔️ **Approach Strengths & Trade-offs**

| Language   | Strength                            | Trade-off                         |
|------------|-------------------------------------|------------------------------------|
| Python     | Easy to read/write, ideal for test  | Slower, higher memory              |
| Go         | Efficient, low memory               | Less flexible types                |
| Rust       | High safety, low memory             | Steeper learning curve             |
| Java       | Enterprise ready, reliable          | Verbose, heavier runtime           |
| JS         | Web-native, quick prototyping       | Higher memory due to GC            |
| C          | Minimal overhead, raw performance   | Manual memory, error-prone         |
| C++        | Powerful STL, fast                  | Complex, potential memory issues   |
| C#         | Clear syntax, .NET integration      | More abstract, higher memory use   |

---

## 🎓 **Mentor Takeaways**

✅ All implementations use **O(n + m)** time, **O(1)** space

✅ Rust and Go are ideal for **systems-level performance**

✅ Python/JavaScript shine for **teaching and demo**

✅ Java and C# great for **enterprise structure**

✅ C/C++ critical for **embedded and legacy systems**

---

## 🛤️ **Polyglot Merge Train Tale: United Lines of Dataville**

In the legendary **Dataville Central Yard**, eight great conductors gathered to prove their mettle:

- 🐍 **Python Express**, led by the *Cautious Conductor*: clear but careful, best for planning and prototyping.
- 🦫 **Go Sprinter**, driven by the *Nimble Conductor*: direct, rapid, with minimal baggage.
- ⚙️ **Rust Bullet**, operated by the *Fearless Conductor*: safe, exact, never wasteful.
- ☕ **Java Express**, commanded by the *Disciplined Conductor*: systematic, always documented and structured.
- 🌐 **JavaScript Metro**, helmed by the *Pragmatic Conductor*: fast, browser-bound, ready to adapt.
- 🧱 **C Freightliner**, managed by the *Deterministic Conductor*: lean, mechanical, and brutally efficient.
- 🧭 **C++ Steamer**, run by the *Classicist Conductor*: master of legacy, elegance and optimization.
- 🧑‍💼 **C# Tramline**, supervised by the *Methodical Conductor*: structured, modern, and enterprise-ready.

> The Merge Track was conquered not by one—but by **all**. For every need, there was a train. And for every train, a track was forged.

