# Module 02: Ownership, Borrowing, and Lifetimes

## The Most Important Module in This Entire Course

If you learn nothing else about Rust, learn this. Ownership is Rust's central innovation — the feature that makes the language unique among all mainstream programming languages. There is **nothing like it in Solidity**, JavaScript, Python, C, or C++.

Every bug you'll fight in Rust for the next few weeks will trace back to ownership. Every "why won't this compile?" moment is the ownership system protecting you. Once it clicks, you'll wonder how you ever wrote code without it.

---

## Creating This Project

```bash
cargo new m02-ownership-borrowing
cd m02-ownership-borrowing
```

This creates the standard Rust project layout with `Cargo.toml` and `src/main.rs`.

---

## Why Ownership Matters

Every program needs to manage memory — allocating it when you need data, freeing it when you're done. Languages handle this in three ways:

| Strategy | Languages | Trade-off |
|---|---|---|
| **Garbage collector** | JavaScript, Python, Go, Java | Runtime overhead, unpredictable pauses |
| **Manual management** | C, C++ | Fast, but use-after-free bugs, memory leaks, segfaults |
| **Ownership system** | Rust | Zero runtime cost, compiler-enforced safety |

Rust chose a third path: the compiler tracks who owns every piece of data and inserts the cleanup code (`drop`) at compile time. No GC pauses. No forgotten `free()`. No dangling pointers. Ever.

### The Solidity Developer's Perspective

In Solidity, you never think about memory management:

```solidity
// Solidity: The EVM handles everything
function example() public {
    string memory name = "Alice";   // EVM allocates
    string memory copy = name;      // EVM copies — you don't care how
    // EVM cleans up when the function returns — you never think about it
}
```

The EVM is a managed environment. It allocates memory, copies values, and cleans up automatically. You pay gas for it, but you never see the machinery.

**Rust is different.** There is no runtime managing memory for you. When you write a Solana program in Rust, you're working with raw bytes — account data is literally a `&[u8]` byte slice that you deserialize, modify, and serialize back. You need to understand ownership to know:

- Who owns that deserialized account data?
- Can you hold a reference to it while modifying something else?
- When does that data get dropped (freed)?

---

## The Stack vs The Heap

You don't need a deep understanding of memory layout, but you need to know the practical difference:

### The Stack
- **Fast** — just moves a pointer up/down
- **Fixed-size data only** — the compiler must know the size at compile time
- Lives in: integers, booleans, floats, fixed-size arrays, tuples, references
- Automatically cleaned up when a function returns

### The Heap
- **Flexible** — can allocate any amount of data at runtime
- **Slower** — requires asking the OS for memory, following pointers
- Lives in: `String`, `Vec<T>`, `Box<T>`, anything dynamically sized
- Must be explicitly freed (Rust does this via ownership/drop)

**The key insight:** Ownership rules primarily matter for heap-allocated data. Stack data is cheap to copy, so Rust just copies it. Heap data is expensive to copy, so Rust *moves* it instead.

```
Stack (fast, fixed)          Heap (flexible, dynamic)
┌──────────────┐            ┌──────────────────────┐
│ i32: 42      │            │ "hello world" bytes   │
│ bool: true   │            │ Vec data: [1,2,3,...] │
│ ptr ─────────│──────────> │ String data           │
│ len: 5       │            └──────────────────────┘
│ capacity: 8  │
└──────────────┘
```

A `String` in Rust is actually three values on the stack: a pointer to heap data, a length, and a capacity. When the `String` goes out of scope, Rust frees the heap allocation.

---

## The Three Rules of Ownership

Memorize these. They are simple, absolute, and the source of most compiler errors you'll see:

1. **Each value in Rust has exactly one owner** (a variable)
2. **There can only be one owner at a time**
3. **When the owner goes out of scope, the value is dropped** (memory freed)

```rust
{
    let s = String::from("hello");  // s is the owner of this String
    // s is valid here, you can use it
}   // s goes out of scope — Rust calls `drop`, heap memory is freed
```

There's no `free()`, no `delete`, no garbage collector. The compiler inserts the cleanup at the closing `}` brace. This is deterministic, zero-cost, and impossible to get wrong.

---

## Move Semantics

This is where Solidity developers get their first surprise.

### In Solidity: Assignment Copies

```solidity
uint256 x = 42;
uint256 y = x;  // y is a copy. Both x and y are valid and independent.
```

### In Rust: Assignment Moves (for heap data)

```rust
let s1 = String::from("hello");
let s2 = s1;       // s1 is MOVED to s2. s1 is now invalid!
// println!("{}", s1);  // COMPILE ERROR: value used after move
println!("{}", s2);     // Works — s2 is the new owner
```

Why? Because `String` stores data on the heap. If Rust just copied the stack data (pointer, length, capacity), you'd have two variables pointing to the same heap memory. When both go out of scope, Rust would free the same memory twice — a **double-free bug**, one of the most dangerous memory errors in C/C++.

Rust's solution: when you assign a heap value, ownership *transfers*. The old variable becomes invalid. One owner, one cleanup. Always.

### Moves happen in three places:
1. **Assignment:** `let s2 = s1;`
2. **Function calls:** `some_function(s1);` — s1 is moved into the function parameter
3. **Return values:** the function moves the value back to the caller

```rust
fn take_ownership(s: String) {     // s takes ownership of the String
    println!("{}", s);
}   // s goes out of scope, String is dropped

let name = String::from("Alice");
take_ownership(name);              // name is moved into the function
// println!("{}", name);           // ERROR: name was moved
```

---

## Clone and Copy

### Clone: Explicit Deep Copy

When you *want* a full copy of heap data, call `.clone()`:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();       // Deep copy — new heap allocation
println!("{} {}", s1, s2); // Both valid! s1 was not moved.
```

`.clone()` is explicit and potentially expensive. Rust wants you to know when you're duplicating heap data.

### Copy: Implicit Stack Copy

Simple stack types implement the `Copy` trait. Assignment copies them automatically — no move:

```rust
let x: i32 = 42;
let y = x;         // x is COPIED (not moved), because i32 implements Copy
println!("{} {}", x, y); // Both valid!
```

**Types that implement `Copy`:** all integers, `f32`, `f64`, `bool`, `char`, tuples of Copy types, references.

**Types that do NOT implement `Copy`:** `String`, `Vec<T>`, any type that allocates heap memory.

**Rule of thumb:** If it lives entirely on the stack and is cheap to copy, it's `Copy`. If it owns heap data, it moves.

---

## References and Borrowing

Moving values everywhere is impractical. You often want to *use* a value without taking ownership of it. That's what references are for.

A reference **borrows** a value — it lets you access the data without owning it:

```rust
fn calculate_length(s: &String) -> usize {  // s is a REFERENCE to a String
    s.len()                                   // We can read it
}   // s goes out of scope, but since it doesn't own the String, nothing is dropped

let name = String::from("hello");
let len = calculate_length(&name);  // &name creates a reference — name is BORROWED
println!("{} has length {}", name, len); // name is still valid!
```

The `&` symbol creates a reference. The function *borrows* the value — it can read it but doesn't own it and can't drop it.

### Mutable References

By default, references are immutable — you can look but not touch:

```rust
fn add_world(s: &mut String) {  // Mutable reference — can modify the data
    s.push_str(", world!");
}

let mut greeting = String::from("hello");  // Variable must be declared mut
add_world(&mut greeting);                   // Pass a mutable reference
println!("{}", greeting);                   // "hello, world!"
```

### The Borrowing Rules

These rules prevent data races at compile time:

1. **You can have MANY immutable references** (`&T`) — multiple readers are safe
2. **OR you can have ONE mutable reference** (`&mut T`) — one writer is safe
3. **But NEVER both at the same time** — reading while writing is a data race

```rust
let mut s = String::from("hello");

let r1 = &s;       // OK — first immutable reference
let r2 = &s;       // OK — second immutable reference (many readers allowed)
// let r3 = &mut s; // ERROR! Can't have mutable ref while immutable refs exist

println!("{} {}", r1, r2); // r1 and r2 are used here, then their lifetimes end

let r3 = &mut s;   // OK now — no more immutable references active
r3.push_str("!");
```

### Why These Rules Matter for Solana

In Solana programs, you work with account data through references. The runtime enforces similar rules — you can't have two mutable references to the same account in one instruction. Rust's borrow checker trains you for this:

```rust
// Conceptual Solana pattern:
// let account_data: &[u8] = &ctx.accounts.my_account.data.borrow();
// You can have many &[u8] readers, or one &mut [u8] writer.
```

---

## Dangling References

A dangling reference points to memory that has been freed. In C, this is a common and devastating bug. In Rust, it's **impossible** — the compiler prevents it:

```rust
// fn dangle() -> &String {       // ERROR: returns a reference to dropped data
//     let s = String::from("hello");
//     &s                          // s is dropped at the end of this function!
// }                               // The reference would point to freed memory

fn no_dangle() -> String {         // Instead, return the owned value
    let s = String::from("hello");
    s                               // Ownership is moved to the caller
}
```

The compiler catches this at compile time. No segfaults. No undefined behavior. Ever.

---

## String vs &str

This distinction confuses every new Rustacean. Here's the definitive explanation:

| | `String` | `&str` |
|---|---|---|
| **Ownership** | Owned — the variable owns the heap data | Borrowed — a reference to string data |
| **Mutability** | Growable, modifiable | Read-only view |
| **Storage** | Heap-allocated | Points to data anywhere (heap, stack, binary) |
| **Size** | Dynamic (ptr + len + capacity) | Fixed (ptr + len) |
| **Analogy** | You own the house | You have a window to look inside |

```rust
let owned: String = String::from("hello");  // Heap-allocated, you own it
let borrowed: &str = "hello";               // Points to data baked into the binary
let slice: &str = &owned[0..3];             // Points into the String's heap data
```

### When to use which:
- **Function parameters:** prefer `&str` — accepts both `String` and `&str`
- **Struct fields that own data:** use `String`
- **String literals:** always `&str` (type `&'static str`)

```rust
fn greet(name: &str) {            // Accepts &str AND &String (auto-deref)
    println!("Hello, {}!", name);
}

greet("world");                    // &str literal — works
greet(&String::from("world"));    // &String auto-derefs to &str — works
```

---

## Slices

A slice is a reference to a contiguous section of a collection. You've already seen `&str` — that's a string slice. The general form is `&[T]`:

```rust
let numbers = vec![1, 2, 3, 4, 5];

let all: &[i32] = &numbers;        // Slice of the entire vector
let middle: &[i32] = &numbers[1..4]; // Slice of elements at index 1, 2, 3
let first_two: &[i32] = &numbers[..2]; // First two elements
let last_two: &[i32] = &numbers[3..];  // Last two elements

println!("{:?}", middle); // [2, 3, 4]
```

Slices are **borrowed** — they don't own the data. They're just a pointer + length, making them very cheap to pass around.

### Slices in Solana

Account data in Solana is accessed as byte slices:

```rust
// In Anchor/Solana:
// account.data.borrow()  returns Ref<&[u8]> — a byte slice
// You parse this slice to read/write account data
```

Understanding slices is essential for working with raw account data.

---

## Lifetimes

Lifetimes are Rust's way of ensuring references are always valid. Most of the time, the compiler infers them automatically. But sometimes you need to annotate them explicitly.

### What is a lifetime?

A lifetime is the scope for which a reference is valid. Every reference has a lifetime, even if you don't write it:

```rust
let r;                          // r declared here — lifetime starts
{
    let x = 5;
    r = &x;                     // ERROR: x doesn't live long enough
}                               // x is dropped here
// println!("{}", r);           // r would be a dangling reference
```

### When you need lifetime annotations

When a function returns a reference, the compiler needs to know: "which input does this reference point to?" If there's only one reference input, Rust infers it. If there are multiple, you must be explicit:

```rust
// The compiler can't figure out if the return value borrows from x or y.
// You annotate with 'a to say: "the return value lives as long as BOTH inputs."
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

The `'a` (read: "lifetime a") says: the returned reference will be valid for as long as both `x` and `y` are valid. The compiler uses this to prevent dangling references.

### Lifetime syntax

```rust
&'a T           // A reference with lifetime 'a
&'a mut T       // A mutable reference with lifetime 'a
&'static str    // A reference that lives for the entire program
```

### Lifetime elision rules

Rust has three rules that let you skip writing lifetimes in common cases:

1. Each reference parameter gets its own lifetime
2. If there's exactly one input lifetime, it's assigned to all output lifetimes
3. If one parameter is `&self` or `&mut self`, its lifetime is assigned to all output lifetimes

That's why most functions don't need explicit lifetime annotations.

### The `'info` Lifetime in Solana

In Anchor (Solana's main framework), you'll see `'info` everywhere:

```rust
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

The `'info` lifetime says: "all these account references are valid for the duration of instruction processing." Every account reference in a Solana instruction shares the same lifetime — they all come from the same transaction context and are all valid for the same scope.

Understanding lifetimes isn't just academic — it's essential for writing Solana programs.

---

## Best Practices

1. **Prefer borrowing over cloning.** If a function only needs to read data, take `&T`, not `T`.
2. **Use `&str` in function parameters** instead of `&String` — it's more flexible.
3. **Clone only when necessary.** Each `.clone()` is a heap allocation. Ask yourself: "can I borrow instead?"
4. **Keep mutable borrows short.** The shorter the `&mut` borrow, the less likely you'll hit borrow checker errors.
5. **Return owned types from functions** that create new data. Let the caller decide whether to borrow.
6. **Don't fight the borrow checker.** If the compiler rejects your code, it's usually because your design has a real problem. Restructure instead of reaching for `clone()` or `unsafe`.

---

## Summary

| Concept | What it means |
|---|---|
| **Ownership** | Every value has exactly one owner; dropped when owner goes out of scope |
| **Move** | Heap values transfer ownership on assignment; original becomes invalid |
| **Clone** | Explicit deep copy of heap data |
| **Copy** | Implicit bitwise copy for simple stack types |
| **`&T`** | Immutable/shared reference — read-only borrow |
| **`&mut T`** | Mutable/exclusive reference — read-write borrow |
| **Borrowing rules** | Many `&T` OR one `&mut T`, never both simultaneously |
| **Slices** | Borrowed view into a contiguous sequence (`&[T]`, `&str`) |
| **Lifetimes** | Compiler-tracked scope of reference validity |

---

## Next Steps

In the `src/main.rs` file, you'll find runnable code demonstrating every concept from this tutorial. In the `exercises/` directory, you'll find 10 exercises to test your understanding. Solutions are in `solutions/`.

Move on to Module 03 once you can comfortably:
- Explain why `let s2 = s1;` invalidates `s1` for `String` but not for `i32`
- Write functions that borrow (`&T`) vs take ownership (`T`)
- Fix common borrow checker errors
- Read basic lifetime annotations like `'a` and `'info`
