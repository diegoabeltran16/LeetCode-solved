| **#** | **Language** | **Runtime** | **Memory Usage** | **Approach Summary** | **Strengths** | **Trade-offs** |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | **Rust** | **0 ms** | **2.4 MB** | Match statement mapping, right-to-left or left-to-right with state tracking | Fastest runtime and minimal memory; no GC overhead | Requires ownership and lifetime understanding |
| 2 | **C** | 1 ms | 10.6 MB | Switch-case mapping, left-to-right with explicit subtractive handling | Near-Rust performance; direct memory access | Manual memory management; verbose |
| 3 | **Go** | 1 ms | **4.7 MB** | Switch-case + lookAhead function; skips i++ on match | Excellent speed + low memory; clean concurrency language | Less flexible for complex pattern mapping |
| 4 | **C#** | 1 ms | **49 MB** | Dictionary lookup, right-to-left with prevDom state tracking | Fast due to JIT optimization; good readability | High memory usage due to .NET runtime and GC |
| 5 | **Java** | 2-4 ms | ~45 MB | HashMap lookup, right-to-left with previous value tracking | Consistent, reliable performance; strong typing | JVM memory overhead; slightly slower |
| 6 | **JavaScript** | 2-3 ms | ~61 MB | Object mapping, left-to-right with final addition optimization | Fast for interpreted language; easy syntax | Highest memory usage due to V8 runtime |
| 7 | **C++** | 3-8 ms | ~10 MB | Lambda constexpr switch-case mapping, left-to-right | Fast but performance varies by approach; flexible | More complex syntax; mixed performance based on implementation |
| 8 | **Python** | 15 ms | **12.5 MB** | Dict mapping, left-to-right with lookahead | Easiest to implement and read | Slowest runtime; interpreted and dynamic typing |

## 🔍 **Key Insights**

✅ **Rust leads** in both runtime and memory, validating its design for system-level performance-critical tasks.

✅ **Go, C, C#** achieve ~1ms runtime but **C# has high memory usage** due to .NET GC.

✅ **JavaScript is surprisingly fast**, but **memory usage is extremely high** (V8 runtime cost).

✅ **Java and C++** show solid performance with expected memory footprints for their runtime architectures.

✅ **Python remains slowest**, highlighting the cost of interpreted dynamic languages for low-level algorithmic tasks.