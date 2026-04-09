# Module 01: Variables, Types, and Functions

## Creating This Project From Scratch

If you were starting from zero, you'd run:

```bash
cargo new m01-variables-types-functions
```

Cargo generates the following file structure:

```
m01-variables-types-functions/
├── Cargo.toml    # The manifest — like package.json or Cargo's equivalent of Hardhat config
├── src/
│   └── main.rs   # Entry point — contains fn main(), like Solidity's constructor but for CLI programs
```

### What Each File Does

**`Cargo.toml`** — The package manifest (think `package.json` + `hardhat.config.ts` combined):
```toml
[package]
name = "m01-variables-types-functions"   # Crate name — like a Solidity project name
version = "0.1.0"                        # Semantic versioning
edition = "2021"                         # Rust edition — like Solidity's pragma version
```

**`src/main.rs`** — Your program entry point. Every Rust binary needs a `fn main()`. This is where execution begins — there's no "deploy" step like Solidity. You compile and run directly.

---

## 1. Variable Bindings and Immutability

### The Big Difference: Immutable By Default

In Solidity, every variable is mutable:

```solidity
// Solidity — mutable by default
uint256 x = 5;
x = 10;  // Fine
```

In Rust, variables are **immutable by default**:

```rust
let x = 5;     // Immutable — like Solidity's `constant` or `immutable`
// x = 10;     // ERROR: cannot assign twice to immutable variable

let mut y = 5; // `mut` makes it mutable — this is Solidity's default behavior
y = 10;        // Fine
```

**Why?** Immutability by default prevents accidental state mutations — a huge source of smart contract bugs. Rust forces you to be explicit about what can change.

### Naming Conventions

| Context | Rust | Solidity |
|---------|------|----------|
| Variables | `snake_case` | `camelCase` |
| Functions | `snake_case` | `camelCase` |
| Constants | `SCREAMING_SNAKE_CASE` | `SCREAMING_SNAKE_CASE` |
| Types/Structs | `PascalCase` | `PascalCase` |

```rust
let my_variable = 42;          // Rust: snake_case
// uint256 myVariable = 42;    // Solidity: camelCase

fn calculate_total() { }       // Rust: snake_case
// function calculateTotal()   // Solidity: camelCase
```

---

## 2. Primitive Types

### Integer Types

Rust gives you explicit control over integer sizes. In Solidity, you're usually working with `uint256` because the EVM is a 256-bit machine. Rust targets real hardware, so you pick the size you need.

| Rust | Size | Solidity Equivalent |
|------|------|-------------------|
| `u8` | 8-bit unsigned | `uint8` |
| `u16` | 16-bit unsigned | `uint16` |
| `u32` | 32-bit unsigned | `uint32` |
| `u64` | 64-bit unsigned | `uint64` |
| `u128` | 128-bit unsigned | `uint128` |
| `i8` | 8-bit signed | `int8` |
| `i16` | 16-bit signed | `int16` |
| `i32` | 32-bit signed | `int32` (Rust default) |
| `i64` | 64-bit signed | `int64` |
| `i128` | 128-bit signed | `int128` |
| `usize` | pointer-sized unsigned | No equivalent — used for indexing |
| `isize` | pointer-sized signed | No equivalent |

**Key difference:** Rust has no `uint256`. For Solana, you'll mostly use `u64` (for lamports/token amounts) and `u8` (for bumps/flags). Solana's maximum integer is `u128`.

**Default inference:** When you write `let x = 42;`, Rust infers `i32` — not `u256` like Solidity would default to.

### Floating-Point Types

Solidity has **no native floats** — you use fixed-point math libraries. Rust has `f32` and `f64`:

```rust
let price: f64 = 3.14;  // 64-bit float — no Solidity equivalent
let ratio: f32 = 0.5;   // 32-bit float
```

**Caution for Solana:** Avoid floats in on-chain programs. Use integer math with a known decimal scale, just like in Solidity. Floats are non-deterministic across hardware.

### Boolean

Same concept, different syntax:

```rust
let is_active: bool = true;   // Rust
// bool isActive = true;      // Solidity — identical concept
```

### Character

Rust's `char` is a 4-byte Unicode scalar value. Solidity has no character type — only `string` and `bytes`.

```rust
let letter: char = 'A';      // Single quotes for char
let emoji: char = '🦀';      // Unicode support — 4 bytes
```

---

## 3. Type Annotations vs Inference

Rust has powerful type inference. You can annotate types explicitly or let the compiler figure it out:

```rust
let x = 5;            // Compiler infers i32
let x: u64 = 5;       // Explicit: unsigned 64-bit
let x: i128 = 5;      // Explicit: signed 128-bit
```

In Solidity, you **always** specify the type: `uint256 x = 5;`. In Rust, you only need annotations when the compiler can't infer the type or when you want a specific type that differs from the default.

---

## 4. Strings: `String` vs `&str`

This is one of the biggest "gotchas" for developers coming from Solidity (or any garbage-collected language). Rust has **two** string types:

| Type | Ownership | Mutable? | Analogy |
|------|-----------|----------|---------|
| `String` | Owned, heap-allocated | Yes (with `mut`) | Like Solidity's `string memory` |
| `&str` | Borrowed reference (slice) | No | Like a read-only view into a string |

```rust
let owned: String = String::from("hello");  // Heap-allocated, owned
let borrowed: &str = "hello";               // String literal, stored in binary

let also_owned: String = "hello".to_string(); // Another way to create String
```

**Why two types?** Rust doesn't have a garbage collector. `String` owns its data and cleans it up when it goes out of scope. `&str` is a lightweight reference — it doesn't own anything, so it's cheaper to pass around.

**Rule of thumb:**
- Use `&str` for function parameters (accepting strings)
- Use `String` when you need to own or modify the string
- String literals (`"hello"`) are always `&str`

---

## 5. Tuples and Arrays

### Tuples

Tuples group values of **different types** together. Solidity doesn't have tuples as a standalone type, but function return values work similarly:

```rust
let point: (i32, f64, char) = (10, 3.14, 'A');  // Mixed types
let (x, y, z) = point;                           // Destructuring — like Solidity's multi-return
let first = point.0;                              // Access by index (0-based)
```

Solidity comparison:
```solidity
// Solidity multi-return is the closest equivalent
function getPoint() returns (int32, string memory, bool) {
    return (10, "hello", true);
}
(int32 x, string memory y, bool z) = getPoint();
```

### Arrays (Fixed-Size)

Rust arrays are fixed-size and stack-allocated, similar to Solidity's fixed arrays:

```rust
let numbers: [i32; 5] = [1, 2, 3, 4, 5];  // [type; length]
let zeros = [0; 10];                        // [value; count] — creates [0,0,0,...,0]
let first = numbers[0];                     // Zero-indexed access
```

Solidity equivalent:
```solidity
uint256[5] memory numbers = [1, 2, 3, 4, 5];
```

**Important:** Rust arrays have bounds checking at runtime (panics on out-of-bounds). Solidity arrays revert.

---

## 6. Functions

### Basic Syntax

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = this is an expression (the return value)
}
```

Solidity equivalent:
```solidity
function add(int32 a, int32 b) public pure returns (int32) {
    return a + b;
}
```

Key differences:
- `fn` instead of `function`
- Parameters: `name: Type` (Rust) vs `Type name` (Solidity) — reversed order
- `-> ReturnType` instead of `returns (Type)`
- No visibility keywords (`public`, `private`) at the module level
- No `pure`/`view`/`payable` modifiers

### Expressions vs Statements

This is critical in Rust. The last expression in a block (without a semicolon) is the return value:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b       // Expression — returned (no semicolon)
}

fn add_v2(a: i32, b: i32) -> i32 {
    return a + b;  // Explicit return — works but not idiomatic
}

fn do_nothing() {
    let _x = 5;   // Statement (has semicolon) — returns () implicitly
}
```

**Rule:** Prefer the expression form (no `return`, no semicolon) for the final value. Use `return` only for early returns.

### Functions That Return Nothing

In Solidity, functions that return nothing just omit the `returns` clause. In Rust, they return the **unit type** `()`:

```rust
fn greet(name: &str) {       // No `-> Type` means `-> ()`
    println!("Hello, {}!", name);
}

fn explicit_unit() -> () {    // Same as above, just explicit
    println!("I return nothing");
}
```

---

## 7. Type Casting with `as`

Solidity uses explicit type conversion functions. Rust uses the `as` keyword:

```rust
let x: i32 = 42;
let y: u64 = x as u64;         // i32 → u64 (widening, safe)
let z: u8 = x as u8;           // i32 → u8 (narrowing, may truncate!)

let float_val: f64 = 3.99;
let truncated: i32 = float_val as i32;  // 3 (truncates, doesn't round)
```

Solidity comparison:
```solidity
uint256 x = 42;
uint8 y = uint8(x);  // Explicit downcast — same truncation risk
```

**Warning:** `as` can silently truncate. In production Solana code, use `.try_into()` for checked conversions that return `Result` instead of silently truncating.

---

## 8. Constants and Statics

### `const` — Compile-Time Constants

Like Solidity's `constant`:

```rust
const MAX_SUPPLY: u64 = 1_000_000_000;  // Must have type annotation
                                          // Underscores for readability (like 1000000000)
```

Solidity equivalent:
```solidity
uint64 constant MAX_SUPPLY = 1_000_000_000;
```

Rules for `const`:
- Must have an explicit type annotation
- Value must be known at compile time
- Naming convention: `SCREAMING_SNAKE_CASE`
- Inlined everywhere they're used (no fixed memory location)

### `static` — Global Variables with a Fixed Address

Like Solidity's state variables, but for off-chain Rust:

```rust
static PROGRAM_VERSION: &str = "1.0.0";         // Immutable global
static mut COUNTER: u32 = 0;                     // Mutable global (unsafe to access!)
```

**For Solana:** You won't use `static mut` in on-chain programs. Account data replaces global mutable state.

---

## 9. Shadowing

Rust allows you to **re-declare** a variable with the same name. This creates a completely new variable — the old one is "shadowed":

```rust
let x = 5;           // x is i32 with value 5
let x = x + 1;       // New x, also i32, value 6 — old x is gone
let x = "hello";     // New x, now a &str! Type can change with shadowing
```

Solidity has **no equivalent** — you can't re-declare a variable in the same scope.

**Why is this useful?**
- Transform a value through stages without needing `mut`
- Change a value's type (e.g., parse a string to a number) while keeping the name
- Common pattern: shadow a mutable value with its final immutable form

---

## 10. The Unit Type `()`

The unit type `()` is Rust's equivalent of `void` in C or a Solidity function with no return value. It has exactly one value: `()`.

```rust
fn log_message(msg: &str) -> () {  // Explicit unit return
    println!("{}", msg);
}

let result: () = log_message("hi");  // result is ()
```

You'll see `()` frequently in Rust, especially in:
- Functions that perform side effects (printing, writing to accounts)
- `Result<(), Error>` — success with no return value, or an error (very common in Solana)

---

## 11. Printing with `println!` and Format Strings

`println!` is a **macro** (note the `!`). It's similar to Solidity's `console.log` from Hardhat:

```rust
let name = "Rustacean";
let age = 30;

println!("Hello!");                           // Simple string
println!("Name: {}", name);                   // {} is a placeholder
println!("Name: {name}");                     // Inline variable (Rust 1.58+)
println!("{} is {} years old", name, age);     // Multiple placeholders
println!("Debug: {:?}", (1, 2, 3));           // {:?} for debug formatting
println!("Hex: {:x}", 255);                   // ff — hex format
println!("Binary: {:b}", 255);                // 11111111 — binary format
println!("Padded: {:>10}", "right");           // Right-aligned, 10 chars wide
```

Solidity comparison:
```solidity
// Hardhat console.log — much more limited
console.log("Name: %s, Age: %d", name, age);
```

---

## Summary: Rust vs Solidity Quick Reference

| Concept | Rust | Solidity |
|---------|------|----------|
| Variable declaration | `let x: u64 = 5;` | `uint64 x = 5;` |
| Mutable variable | `let mut x = 5;` | `uint256 x = 5;` (default) |
| Constant | `const X: u64 = 5;` | `uint64 constant X = 5;` |
| Function | `fn foo(x: u64) -> u64` | `function foo(uint64 x) returns (uint64)` |
| No return value | `fn foo()` (returns `()`) | `function foo()` |
| String (owned) | `String` | `string memory` |
| String (reference) | `&str` | `string calldata` (closest) |
| Type cast | `x as u64` | `uint64(x)` |
| Print/log | `println!("{}", x)` | `console.log(x)` / `emit Event(x)` |
| Default integer | `i32` | `uint256` |
| Naming | `snake_case` | `camelCase` |

---

## Next Steps

In the next module, we'll cover **Ownership and Borrowing** — Rust's most unique feature and the reason it doesn't need a garbage collector. This concept has no equivalent in Solidity but is essential for writing Solana programs.

Now open `src/main.rs` to see all these concepts in runnable code, then try the exercises in `exercises/src/main.rs`.
