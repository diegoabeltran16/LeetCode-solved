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

| Language   | Runtime | Memory  | Approach                    | Conductor Type     |
| ---------- | ------- | ------- | --------------------------- | ------------------ |
| **Python** | 1 ms    | 12.4 MB | Dummy node, iterative merge | Cautious Conductor |
| **Go**     | 0 ms    | 4.35 MB | Dummy node, iterative merge | Nimble Conductor   |
| **Rust**   | 0 ms    | 2.3 MB  | Dummy node, iterative merge | Fearless Conductor |

---

## 💡 **Analytical Summary**

✅ **Fastest Runtime (0 ms)**: Go, Rust.

* **Rust (2.3 MB)** demonstrates superior memory efficiency.

✅ **Best Memory Usage**: Rust < Go < Python.

✅ **Python** excels in simplicity and readability but has slightly higher overhead.

---

## ⚔️ **Approach Strengths & Trade-offs**

| Approach                       | Strength                         | Trade-off                            |
| ------------------------------ | -------------------------------- | ------------------------------------ |
| **Dummy node (Iterative)**     | Simple, efficient, no edge cases | Slight overhead due to extra pointer |
| **Recursive (Not shown here)** | Elegant and clear logic          | Higher stack usage, slower in Python |

---

## 🎓 **Mentor Takeaways**

✅ All provided implementations are **O(n + m)** time complexity, with **O(1)** additional space.

✅ **Rust and Go** are ideal for **performance-critical contexts**.

✅ Python remains excellent for **prototyping and clarity**.

---

### 🏷️ **Final Comparative Table**

| Language | Runtime | Memory  | Conductor Type |
| -------- | ------- | ------- | -------------- |
| Rust     | 0 ms    | 2.3 MB  | Fearless       |
| Go       | 0 ms    | 4.35 MB | Nimble         |
| Python   | 1 ms    | 12.4 MB | Cautious       |

---

## 🔑 **Conclusion**

Choosing the right approach depends on **contextual needs**—whether you prioritize readability (Python), rapid performance (Go), or unbeatable efficiency and safety (Rust). Mastering linked-list merging techniques equips you with tools essential for a variety of algorithmic challenges and real-world applications.

---

# 🎩 The Merge Conductors:

🐍 Python Express, led by the Cautious Conductor
Thoughtful, deliberate, and safe. He examined each pair of carriages, ensuring nothing went out of place. Ideal for prototyping new merge routes under pressure.

🦫 Go Sprinter, driven by the Nimble Conductor
Ultra-fast and efficient. She zipped through merge tasks like slicing through air — minimal memory, no fuss. Go was built for raw performance under strict time windows.

⚙️ Rust Bullet, operated by the Fearless Conductor
Precision meets bravery. He didn’t just merge — he did it with zero waste and absolute safety. Memory was sacred, and nothing moved unless it was provably correct.

☕ Java Express, commanded by the Disciplined Conductor
Structured, verbose, reliable. He kept logs, wrote contracts, and ensured every merge ran smoothly in high-scale environments. Java may have more ceremony, but it delivered under pressure.

🌐 JavaScript Metro, helmed by the Pragmatic Conductor
Fast, flexible, and unopinionated. He got the job done quickly with fewer rules — perfect for rapid deployments, quick fixes, and living inside browsers across the world.

🏆 The Wisdom of the Merge
Python: perfect for clarity and quick validation.

Go: built for blazing-fast, low-memory environments.

Rust: the king of correctness and performance.

Java: the enterprise giant, dependable and clear.

JavaScript: the web-native sprinter, always ready to adapt.