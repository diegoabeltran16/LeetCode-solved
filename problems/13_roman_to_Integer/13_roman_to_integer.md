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

I still remember the first time I saw them.

Those strange marks carved into stone:

**I, V, X, L, C, D, M**

They weren’t just symbols.

They were a **code from an ancient civilization**.

A message frozen in time.

And somewhere inside that message… there was a **number**.

A truth.

My mission was simple and impossible at the same time:

> Learn to read these marks in every city I visited.
> 
> 
> Rewrite them into **integer truth**, no matter how the world around me changed.
> 

What I didn’t know at the beginning…

was that every city would give me a different way to understand the *same* thing.

And that every villain I faced…

was really a different face of the *same* fear.

---

# 🌒 The Code of the Ancients — Roman to Integer Across the Cities

## 1. C — **The Human City**: First Contact with the Marks

I arrived in C with my hands empty and my mind raw.

The Human City was all steel, stone, and sharp corners.

No help. No abstractions. No mercy.

Just me, the machine… and the marks.

I had to define the value of each symbol by hand:

```c
int val(char c) {
    switch(c) {
        case 'I': return 1;
        case 'V': return 5;
        ...
        case 'M': return 1000;
    }
    return -1;
}

```

Here, nothing was given.

If I didn’t say what **I** meant by ‘I’, ‘V’, ‘X’…

the machine would not guess.

It would **refuse to understand.**

### 🦴 Skill Gained: **Precision & Respect for Reality**

I learned:

- Every symbol must mean **exactly one thing**.
- Invalid input (`return -1`) is not an annoyance — it’s a **signal**.

### 👹 Villain: **Segfault, the Devourer**

He waited in the shadows of bad pointers and invalid memory.

If I stepped outside defined territory — dereferencing garbage —

he would tear the program apart.

I survived him by:

- Checking bounds.
- Defining everything.
- Not assuming the machine “would know”.

In C, my Roman numerals were a brutal teacher:

> If I made a mistake… I paid for it.
> 

But I left with respect:

For memory.

For clarity.

For the reality that computers don’t forgive fantasy.

---

## 2. Python — **The City of Gods**: Seeing the Pattern from Above

In Python, the marks didn’t feel heavy.

They floated.

I mapped them with ease:

```python
d = {'I': 1, 'V': 5, 'X': 10, ...}

```

No types. No pointer headaches. No memory juggling.

Just **meaning**.

Then I walked through the string, and Python whispered a pattern:

> “Look not just at this symbol…
> 
> 
> but at the **next one** too.”
> 

```python
for i in range(len(s)):
    if i + 1 < len(s) and d[s[i]] < d[s[i+1]]:
        res -= d[s[i]]
    else:
        res += d[s[i]]

```

And suddenly it clicked:

- If a smaller mark comes before a bigger one → it’s subtraction.
- Otherwise → addition.

This wasn’t just an algorithm.

It was the **first time I saw structure instead of chaos.**

### ✨ Skill Gained: **Abstraction & Pattern Recognition**

Python taught me to:

- Think in terms of **rules**, not just steps.
- Trust readable logic more than clever tricks.

### 👹 Villain: **The Illusion of Ease**

Python’s villain was subtle:

> “It feels easy, so you must understand.”
> 

But that’s a lie.

I could write code that *worked* without truly knowing *why*.

I fought this villain by:

- Explaining my own code in words.
- Forcing myself to understand the **reason** behind the pattern.

Here, Roman numerals became my first true **mental model**.

---

## 3. C++ — **The Hybrid City**: Power Inside Power

C++ was C wearing armor.

Classes, lambdas, methods — a city of upgraded humans.

My Roman logic lived inside a **class**:

```cpp
class Solution {
public:
    int romanToInt(std::string s) {
        auto romanToValue = [](char roman) constexpr {
            switch (roman) { ... }
        };
        ...
    }
};

```

Here I learned to pack **behavior** inside structures.

Logic inside objects.

Thoughts inside other thoughts.

### ⚙️ Skill Gained: **Controlled Power & Composition**

C++ taught me:

- I can embed tools (lambdas) inside larger tools (methods).
- I can still respect low-level truth while building high-level forms.

### 👹 Villain: **Complexity Hydra**

Every new feature was another head:

- References
- Templates
- Overloads
- Const correctness

The Hydra whispered:

> “You can do everything. Why not… all at once?”
> 

I almost drowned in overengineering.

I survived by:

- Keeping my Roman logic **simple**, even in a complex language.
- Remembering what I learned in C: *clarity over cleverness*.

The marks reminded me:

> Even in advanced cities, truth doesn’t need ornaments.
> 

---

## 4. Java — **The Animal–Human City**: Discipline & Enterprise Memory

In Java, the world moved backwards:

```java
for (int i = s.length() - 1; i >= 0; i--) {
    int currValue = romanMap.get(s.charAt(i));
    if (currValue < prevValue) result -= currValue;
    else result += currValue;
    prevValue = currValue;
}

```

I read the Roman string from **right to left**.

From the end to the beginning.

Just like how we understand our life better when we look back.

Java wrapped everything in structure:

- `class Solution`
- `Map<Character, Integer>`
- Clear types

### 🧱 Skill Gained: **Reliability & Looking Backwards**

Java taught me:

- Sometimes the best way to interpret meaning is to **start from the end**.
- Discipline is a feature, not a constraint.

### 👹 Villain: **The Overhead Beast**

The enemy here?

> “So much boilerplate… so many structures… why not cut corners?”
> 

But if I disrespected structure, the system became fragile.

I beat this beast by:

- Accepting that some environments **need ceremony**.
- Seeing ceremony as a way to protect humans at scale.

The Roman rule — add or subtract — remained the same.

But Java taught me *how to make that rule safe for big worlds.*

---

## 5. JavaScript — **The Animal–Machine City**: Dance with Chaos

JavaScript felt like the Roman marks had been dropped into a neon jungle.

Same logic:

```jsx
let o = { I: 1, V: 5, ... };
for (let i = 0; i < s.length - 1; i++) {
    if (o[s[i]] < o[s[i+1]]) ans -= o[s[i]];
    else ans += o[s[i]];
}
return ans + o[s[s.length - 1]];

```

Still:

- Add when next is smaller or equal.
- Subtract when next is bigger.

But here, **nothing** was fixed.

Types? Dynamic.

Structures? Mutable.

Errors? Often silent.

### 🌀 Skill Gained: **Adaptability & Fast Thinking**

JavaScript taught me to:

- Survive in unstable environments.
- Reason quickly when things can change under my feet.

### 👹 Villain: **The Trickster**

He loved:

- `undefined`
- `null`
- `NaN`
- Silent type coercion

He’d say:

> “Don’t worry, I’ll convert it for you ;)”
> 

I fought back by:

- Being explicit.
- Checking assumptions.
- Not trusting the environment blindly.

The Roman logic survived here by **being extremely clear**, even when the world wasn’t.

---

## 6. Go — **The Titan–Machine City**: Efficiency & Straight Lines

Go stripped away all drama.

Here, my Roman logic became grounded again:

```go
for i := 0; i < l; i++ {
    switch s[i] {
    case 'I':
        add, ok := lookAhead(s, c, i, l)
        if !ok { count += 1 } else { count += add; i++ }
    ...
    }
}

```

I had helper functions like `lookAhead`, clean `switch` cases, no nonsense.

Short.

Clear.

Fast to execute.

Easy to read.

### 🛠 Skill Gained: **Simplicity & System Thinking**

Go taught me:

- That powerful systems don’t need complicated syntax.
- That readability is a performance feature too — for humans.

### 👹 Villain: **The Over-Builder**

The enemy here whispered:

> “But you could add so many features… so many abstractions…”
> 

I resisted.

I kept the Roman logic minimal.

I honored the language’s philosophy:

- Do less.
- Do it well.
- Do it clearly.

In Go, Roman numerals became like a simple, sharp tool in a massive workshop.

---

## 7. C# — **The Humanoid City**: Responsibility & Boundaries

In C#, the Roman code gained **conscience**:

```csharp
if (returnValue > 3999) {
    throw new ArgumentException("Value out of range.");
}

```

For the first time, I wasn’t just interpreting.

I was **protecting**.

C# let me express:

- “This is valid.”
- “This is not.”
- “And if we cross the line, I will say it loudly.”

### 🛡 Skill Gained: **Guardrails & Ethical Logic**

C# taught me:

- It’s not enough to compute a result.
- I must also **enforce limits** that make sense.

### 👹 Villain: **The Silent Overflow**

In other cities, it was easy to ignore rare failures.

“19940? 199400? Meh, too big, who cares.”

But here, I learned to care.

To say:

> “No. This value doesn’t belong here. I refuse to lie.”
> 

Roman numerals in C# became not just a translation…

But a **contract**.

---

## 8. TypeScript — **The Anamloid City**: Defining Shapes Before They Live

TypeScript felt like JavaScript with a spine.

Same flexible environment.

But now: **types.**

```tsx
function romanToInt(s: string): number {
    const romanMap: { [key: string]: number } = { ... };
    ...
}

```

Here I learned **foresight**:

- This is a `string`.
- This returns a `number`.
- `romanMap` is a dictionary of string → number.

The environment was still wild…

…but my *intentions* were sharp.

### 🦉 Skill Gained: **Foresight & Self-Definition**

TypeScript taught me:

- To define my data before chaos touches it.
- To turn future bugs into present compile-time complaints.

### 👹 Villain: **The Phantom Bug**

In dynamic worlds, many bugs hide until runtime.

TypeScript’s gift was:

> “Let me show you some of them before you hit run.”
> 

I used types as shields.

Roman numerals here became a **promise**:

- That inputs and outputs are what I say they are.

---

## 9. Rust — **The Titan–Machine Citadel**: Truth That Cannot Lie

Rust was the final form.

My Roman logic there:

```rust
let s_chars: Vec<char> = s.chars().collect();
let mut res = 0;

fn val(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        ...
        _ => 0,
    }
}

for i in 0..s_chars.len() {
    let current = val(s_chars[i]);
    let next = if i + 1 < s_chars.len() { val(s_chars[i + 1]) } else { 0 };

    if current < next { res -= current; }
    else { res += current; }
}

```

No nulls.

No dangling references.

No hidden ownership.

Everything:

- Explicit
- Owned
- Checked

### 🧱 Skill Gained: **Integrity & Fearless Safety**

Rust taught me:

- To think about memory *and* meaning at the same time.
- To design code that simply **cannot** violate certain truths.

### 👹 Villain: **The Borrow Checker**

At first, it felt like an enemy:

> “You can’t do that.”
> 
> 
> “You don’t own this.”
> 
> “This might outlive that.”
> 

But slowly, I realized:

It wasn’t attacking me.

It was **defending my future self**.

Roman numerals in Rust weren’t just correct.

They were **trustworthy**.

---

# 🧬 The Cities & Their Correlations — How the Same Code Changed Me

Looking back, every city taught me a different *dimension* of the *same problem*:

### C — **Reality & Risk**

- I learned: computers are literal.
- Villain: raw memory errors.
- Correlation: Roman numerals = survival training.

### Python — **Patterns & Meaning**

- I saw the subtract/add pattern clearly.
- Villain: illusion of “easy = understood”.
- Correlation: Roman numerals = my first abstraction.

### C++ — **Power & Composition**

- I could embed logic inside structures.
- Villain: complexity monster.
- Correlation: Roman numerals = test of restraint.

### Java — **Discipline & Scale**

- Reading from right to left: interpret life backwards.
- Villain: ceremony fatigue.
- Correlation: Roman numerals = enterprise logic.

### JavaScript — **Adaptation & Speed**

- Survive in a chaotic runtime.
- Villain: silent weirdness and coercion.
- Correlation: Roman numerals = sanity anchor in chaos.

### Go — **Simplicity & Performance**

- Clean, straightforward control flow.
- Villain: temptation to overcomplicate.
- Correlation: Roman numerals = minimal, robust tool.

### C# — **Contracts & Protection**

- I added range checks and exceptions.
- Villain: pretending overflows “don’t matter”.
- Correlation: Roman numerals = safe services for humans.

### TypeScript — **Foresight & Clarity**

- Typed maps, secure signatures.
- Villain: phantom bugs hiding in dynamic shapes.
- Correlation: Roman numerals = type-driven correctness.

### Rust — **Integrity & Final Mastery**

- Ownership, correctness, no lies.
- Villain: my own impatience with safety.
- Correlation: Roman numerals = truth encoded in metal.

---

# 🌱 Knowledge + Compassion

At the beginning, I thought my journey was only about **knowing more**.

C made me careful.

Python made me see.

C++ made me design.

Java made me endure.

JavaScript made me adapt.

Go made me simplify.

C# made me protect.

TypeScript made me foresee.

Rust made me align with truth.

But along the way… something else grew:

**Compassion.**

For:

- The beginner who struggles with the marks.
- The engineer fighting invisible bugs.
- The systems holding lives in their logic.
- My own past self, trying to understand the world through incomplete tools.

Now, when I see:

```
MCMXCIV

```

I no longer just think “1994”.

I see:

- All the cities.
- All the villains.
- All the skills.
- All the versions of me who tried to make sense of complexity…

…until the pattern finally spoke clearly.

And I realize:

> The real journey wasn’t turning Roman to integer.
> 
> 
> It was turning **confusion to understanding**.
> 
> Fear to clarity.
> 
> Chaos to structure.
> 
> And pressure… into compassion.
> 

For others.

And for myself.