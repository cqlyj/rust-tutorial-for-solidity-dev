# Module 08: Macros and Attributes

## Creating This Project From Scratch

If you were starting from zero, you'd run:

```bash
cargo new m08-macros-attributes
```

Cargo generates the following file structure:

```
m08-macros-attributes/
├── Cargo.toml    # The manifest — like package.json
├── src/
│   └── main.rs   # Entry point — contains fn main()
```

---

## Why This Matters for Solana

The Anchor framework is roughly **90% macros**. When you write an Anchor program, almost every line involves a macro or attribute:

```rust
// Every one of these is a macro or attribute:
declare_id!("YourProgramPublicKeyHere11111111111111");  // macro!

#[program]                          // attribute macro
mod my_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initialized!");       // macro!
        Ok(())
    }
}

#[derive(Accounts)]                 // derive macro
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]  // attribute macro
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]                 // attribute macro
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]                          // attribute macro
pub struct MyAccount {
    pub data: u64,
}
```

Understanding macros means understanding **what Anchor does behind the scenes**. When something goes wrong, you need to know what code Anchor generated for you. This module gives you that foundation.

---

## Solidity Comparison

Solidity has **no macro system**. The closest things are:

| Solidity Concept | Rust Equivalent | Power Level |
|-----------------|-----------------|-------------|
| Inheritance (`is`) | Derive macros (`#[derive(...)]`) | Macros are more flexible |
| Modifiers (`onlyOwner`) | Attribute macros (`#[account(mut)]`) | Macros generate arbitrary code |
| Events (`emit`) | `msg!()` macro | Similar logging concept |
| `pragma solidity ^0.8.0` | `#[cfg()]` attributes | Conditional compilation |
| No equivalent | `macro_rules!` | Full code generation |

**Key insight:** In Solidity, the compiler does one thing with your code. In Rust, macros transform your code *before* the compiler sees it. This is strictly more powerful — macros can generate entire function bodies, validate data structures at compile time, and automate boilerplate that Solidity forces you to write by hand.

---

## 1. Macros You Already Use

Before we write macros, recognize that you've been using them since Module 01. Every `!` after a name means "this is a macro":

```rust
println!("Hello");           // Macro — formats and prints to stdout
vec![1, 2, 3];               // Macro — creates a Vec with initial values
format!("x = {}", 42);       // Macro — returns a formatted String
assert!(true);               // Macro — panics if condition is false
assert_eq!(1 + 1, 2);        // Macro — panics if values aren't equal
todo!();                      // Macro — panics with "not yet implemented"
unreachable!();               // Macro — panics with "unreachable code"
panic!("something broke");    // Macro — immediately crashes the program
```

**Why are these macros and not functions?**

Functions in Rust take a fixed number of arguments with fixed types. `println!` accepts *any* number of arguments with *any* types — that's only possible with macros. Macros see your code as tokens and can generate different code depending on what you pass.

```rust
println!("no args");                    // 1 arg
println!("{} + {} = {}", 1, 2, 3);      // 4 args — a function can't do this
println!("{x}");                         // Captures variable from scope!
```

---

## 2. Declarative Macros with `macro_rules!`

Declarative macros use pattern matching to generate code. Think of them as "find-and-replace on steroids."

### Basic Syntax

```rust
macro_rules! say_hello {
    () => {
        println!("Hello from a macro!");
    };
}

// Usage:
say_hello!();  // Expands to: println!("Hello from a macro!");
```

The structure:
- `macro_rules! name` — declares the macro
- `() =>` — the pattern to match (empty parens = no arguments)
- `{ ... }` — the code to generate when the pattern matches

### Pattern Matching with Arguments

Macros match on **token patterns**, not types. The `$name:kind` syntax captures tokens:

```rust
macro_rules! create_greeting {
    ($name:expr) => {
        format!("Hello, {}!", $name)
    };
}

let greeting = create_greeting!("Rustacean");  // "Hello, Rustacean!"
```

Common capture kinds (called "fragment specifiers"):

| Specifier | What It Matches | Example |
|-----------|----------------|---------|
| `$x:expr` | Any expression | `5`, `a + b`, `foo()` |
| `$x:ident` | An identifier | `my_var`, `foo` |
| `$x:ty` | A type | `u64`, `String`, `Vec<u8>` |
| `$x:pat` | A pattern | `Some(x)`, `_`, `1..=5` |
| `$x:stmt` | A statement | `let x = 5;` |
| `$x:literal` | A literal value | `42`, `"hello"`, `true` |
| `$x:tt` | A single token tree | Anything — the catch-all |
| `$x:block` | A block `{ ... }` | `{ let x = 1; x + 2 }` |

### Multiple Pattern Arms

Like `match`, macros can have multiple patterns:

```rust
macro_rules! math {
    (add $a:expr, $b:expr) => { $a + $b };
    (mul $a:expr, $b:expr) => { $a * $b };
}

let sum = math!(add 5, 3);    // 8
let product = math!(mul 5, 3); // 15
```

### Repetition with `$(...)*` and `$(...)+`

This is where macros become powerful. You can match repeated patterns:

- `$(...)*` — zero or more repetitions
- `$(...)+` — one or more repetitions

```rust
macro_rules! make_vec {
    ( $( $element:expr ),* ) => {
        {
            let mut v = Vec::new();
            $( v.push($element); )*
            v
        }
    };
}

let v = make_vec![1, 2, 3, 4, 5];  // Creates vec![1, 2, 3, 4, 5]
```

Breaking down `$( $element:expr ),*`:
- `$( ... )` — the pattern to repeat
- `$element:expr` — capture each expression
- `,` — the separator between repetitions
- `*` — zero or more times

The expansion of `make_vec![1, 2, 3]` is:

```rust
{
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v
}
```

### A Practical Example: HashMap Literal

Rust has no built-in HashMap literal syntax. Macros fix that:

```rust
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $( map.insert($key, $value); )*
            map
        }
    };
}

let scores = hashmap! {
    "Alice" => 100,
    "Bob" => 85,
    "Charlie" => 92,
};
```

The `$(,)?` at the end optionally matches a trailing comma — a nice ergonomic touch.

---

## 3. Procedural Macros (Conceptual)

Procedural macros are more powerful than declarative macros. They receive Rust code as input, transform it, and output new Rust code. You **use** them constantly but **write** them rarely.

There are three kinds:

### 3a. Derive Macros — `#[derive(...)]`

The most common proc macro. Auto-implements traits for your types:

```rust
#[derive(Debug, Clone, PartialEq)]
struct Token {
    symbol: String,
    decimals: u8,
}
```

This generates an implementation of `Debug`, `Clone`, and `PartialEq` for `Token`. Without derive, you'd write:

```rust
impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Token")
            .field("symbol", &self.symbol)
            .field("decimals", &self.decimals)
            .finish()
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Token {
            symbol: self.symbol.clone(),
            decimals: self.decimals.clone(),
        }
    }
}

// ... and PartialEq too. That's a lot of boilerplate!
```

**Solana relevance:** Anchor's `#[derive(Accounts)]` is a derive macro that generates all the account validation, deserialization, and security checking logic.

### 3b. Attribute Macros

Attribute macros attach to items and can completely rewrite them:

```rust
#[test]           // Marks a function as a test — only compiled in test builds
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[cfg(test)]      // Only compile this module in test builds
mod tests { }

#[allow(dead_code)]   // Suppress "unused" warnings for this item
fn unused_helper() { }
```

**Solana relevance:**
- `#[program]` rewrites your module to add Solana entrypoint routing
- `#[account]` adds serialization/deserialization and type discriminators
- `#[account(init, payer = user, space = 8 + 8)]` generates account initialization logic

### 3c. Function-Like Proc Macros

These look like function calls with `!` but are proc macros under the hood:

```rust
// Anchor's declare_id! is a function-like proc macro
declare_id!("YourProgramPublicKeyHere11111111111111");

// Solana's msg! is another example
msg!("Transfer of {} lamports complete", amount);
```

---

## 4. Attributes in Rust

Attributes are metadata attached to code items. They come in two forms:

### Outer Attributes `#[...]`

Applied to the item that follows them:

```rust
#[derive(Debug)]          // Applies to the struct below
struct MyStruct { }

#[test]                   // Applies to the function below
fn my_test() { }

#[allow(unused_variables)] // Applies to the function below
fn example() {
    let x = 5;            // No warning about unused x
}
```

### Inner Attributes `#![...]`

Applied to the enclosing item (usually the whole file/crate):

```rust
#![allow(dead_code)]       // Suppress dead_code warnings for entire file
#![cfg_attr(not(test), no_std)]  // Use no_std when not testing
```

You'll see `#![...]` at the top of files, especially in Solana programs:
```rust
#![allow(clippy::result_large_err)]  // Common in Anchor programs
```

### Common Attributes Reference

| Attribute | Purpose | Solana Use |
|-----------|---------|------------|
| `#[derive(Debug)]` | Auto-implement Debug trait | Debugging account data |
| `#[derive(Clone)]` | Auto-implement Clone trait | Copying data structures |
| `#[derive(PartialEq)]` | Auto-implement equality | Comparing account states |
| `#[cfg(test)]` | Compile only in test mode | Test modules |
| `#[cfg(feature = "...")]` | Compile if feature is enabled | Feature flags |
| `#[test]` | Mark function as a test | Unit tests |
| `#[allow(unused)]` | Suppress unused warnings | Development convenience |
| `#[warn(missing_docs)]` | Warn if docs are missing | Code quality |
| `#[repr(C)]` | Use C memory layout | Solana account data layout |
| `#[inline]` | Suggest function inlining | Performance optimization |

### `#[repr(C)]` — Memory Layout Control

This is important for Solana. By default, Rust can reorder struct fields for optimal alignment. `#[repr(C)]` forces C-compatible field ordering:

```rust
#[repr(C)]
struct AccountData {
    is_initialized: bool,  // 1 byte — stays first
    balance: u64,          // 8 bytes — stays second
    owner: [u8; 32],       // 32 bytes — stays third
}
```

In Solana, account data is serialized to bytes and stored on-chain. Predictable field order means you can safely read this data from any language. Anchor handles this for you with `#[account]`, but understanding `#[repr(C)]` helps when doing raw account manipulation.

---

## 5. Anchor Macros Preview

Here's a complete Anchor program with every macro explained. **You can't run this code yet** — it requires the Anchor framework — but you should understand what each macro does:

```rust
use anchor_lang::prelude::*;

// declare_id!() — A function-like proc macro.
// Sets this program's on-chain address (public key).
// Solidity equivalent: the contract's deployed address (but explicit).
declare_id!("11111111111111111111111111111111");

// #[program] — An attribute macro.
// Marks this module as the program's instruction handlers.
// Anchor generates the entrypoint routing: when a transaction arrives,
// it reads the instruction name and dispatches to the right function.
// Solidity equivalent: the contract's public/external functions.
#[program]
mod my_program {
    use super::*;

    // Each pub fn becomes an instruction endpoint.
    // Solidity equivalent: a public function on a contract.
    pub fn initialize(ctx: Context<Initialize>, initial_value: u64) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.value = initial_value;

        // msg!() — Solana's logging macro.
        // Logs appear in transaction logs (like Solidity events or console.log).
        msg!("Account initialized with value: {}", initial_value);

        Ok(())
    }
}

// #[derive(Accounts)] — A derive macro (the most complex one in Anchor).
// Generates all the account validation logic:
//   - Deserializes accounts from raw bytes
//   - Checks account ownership
//   - Validates signers
//   - Initializes new accounts if needed
// Solidity equivalent: nothing — Solidity handles this implicitly.
#[derive(Accounts)]
pub struct Initialize<'info> {
    // #[account(...)] — An attribute macro on each field.
    // init: create this account
    // payer = user: the `user` account pays for creation
    // space = 8 + 8: allocate 8 bytes (discriminator) + 8 bytes (u64)
    #[account(init, payer = user, space = 8 + 8)]
    pub my_account: Account<'info, MyAccount>,

    // #[account(mut)] — This account's lamports will change (it's paying).
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// #[account] — An attribute macro.
// Marks this struct as a Solana account data type.
// Generates: BorshSerialize, BorshDeserialize, and a type discriminator.
// Solidity equivalent: a contract's state variables, but as a serializable struct.
#[account]
pub struct MyAccount {
    pub value: u64,
}
```

### Anchor Macro Summary

| Macro | Kind | What It Does |
|-------|------|-------------|
| `declare_id!()` | Function-like | Sets program's public key |
| `#[program]` | Attribute | Generates instruction routing/entrypoint |
| `#[derive(Accounts)]` | Derive | Generates account validation/deserialization |
| `#[account(...)]` | Attribute (on fields) | Specifies account constraints |
| `#[account]` | Attribute (on structs) | Makes struct serializable as account data |
| `#[instruction()]` | Attribute | Passes instruction args to validation struct |
| `msg!()` | Function-like | Logs a message to transaction logs |

---

## 6. Conditional Compilation

`#[cfg(...)]` controls what code gets compiled based on conditions. This is like having an `#ifdef` preprocessor, but type-safe and integrated into the language.

### Test-Only Code

```rust
#[cfg(test)]                    // Only compiled when running `cargo test`
mod tests {
    use super::*;

    #[test]                     // Marks this function as a test case
    fn test_my_function() {
        assert_eq!(2 + 2, 4);
    }
}
```

**This is how all Rust projects organize tests.** The `#[cfg(test)]` ensures test code isn't included in release builds — it's zero cost.

### Feature Flags

```rust
#[cfg(feature = "verbose")]
fn log_details(data: &str) {
    println!("VERBOSE: {}", data);
}

#[cfg(not(feature = "verbose"))]
fn log_details(_data: &str) {
    // No-op in non-verbose mode
}
```

Enable with: `cargo run --features verbose`

### Platform-Specific Code

```rust
#[cfg(target_os = "linux")]
fn platform_name() -> &'static str { "Linux" }

#[cfg(target_os = "windows")]
fn platform_name() -> &'static str { "Windows" }

#[cfg(target_os = "macos")]
fn platform_name() -> &'static str { "macOS" }
```

### Combining Conditions

```rust
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn optimized_for_linux_x64() { }

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn unix_like() { }

#[cfg(not(debug_assertions))]  // Only in release builds
fn production_only() { }
```

---

## 7. Macro Hygiene and Scoping

Rust macros are "hygienic" — variables created inside a macro don't leak into surrounding code:

```rust
macro_rules! make_x {
    () => {
        let x = 42;
    };
}

fn main() {
    make_x!();
    // println!("{}", x);  // ERROR: x is not in scope here
    // The x created by the macro is scoped to the macro expansion
}
```

This prevents the nasty bugs that C preprocessor macros are infamous for. However, it also means you need to be intentional about what a macro exposes.

---

## 8. When to Use Each Kind of Macro

| Use Case | Tool | Example |
|----------|------|---------|
| Simple code generation | `macro_rules!` | Custom `vec!`-like syntax |
| Auto-implement traits | `#[derive(...)]` | `Debug`, `Clone`, Anchor's `Accounts` |
| Transform functions/modules | Attribute macros | `#[test]`, `#[program]` |
| Complex code generation | Proc macros | `declare_id!()`, serialization |
| You're writing an app | Use existing macros | Don't write proc macros |

---

## Best Practices

1. **Don't write proc macros unless you have to.** They're complex, hard to debug, and require a separate crate. Use `macro_rules!` for simple patterns.

2. **Understand the derive macros you use.** When you write `#[derive(Debug)]`, know that it generates a `fmt::Debug` implementation. When you write `#[derive(Accounts)]`, know that it generates validation logic.

3. **Use `#[cfg(test)]` for test modules.** Every Rust file with tests should wrap them in `#[cfg(test)] mod tests { ... }`.

4. **Read macro errors from the inside out.** Macro errors can be confusing. The innermost error is usually the real problem.

5. **Use `cargo expand` to see macro output.** Install with `cargo install cargo-expand`, then run `cargo expand` to see what your macros generate. This is invaluable for understanding Anchor.

6. **Prefer standard derives over manual implementations.** If `#[derive(Debug)]` works for your type, use it instead of implementing `Debug` by hand.

7. **Use `#[repr(C)]` when data crosses language boundaries.** This includes Solana account data that might be read by JavaScript clients or other programs.

---

## Summary: Macro System Comparison

| Aspect | Rust | Solidity |
|--------|------|----------|
| Code generation | `macro_rules!`, proc macros | None |
| Trait auto-implementation | `#[derive(Debug, Clone)]` | No equivalent |
| Conditional compilation | `#[cfg(test)]` | No equivalent |
| Logging | `println!()`, `msg!()` | `console.log()`, `emit Event()` |
| Test marking | `#[test]` | Hardhat/Foundry test conventions |
| Compile-time validation | Proc macros can validate | No equivalent |
| Metaprogramming | Full macro system | No equivalent |
| Boilerplate reduction | Derive macros | Inheritance, but limited |

---

## Next Steps

Now open `src/main.rs` to see these concepts as runnable code, then try the exercises in `exercises/src/main.rs`. The exercises focus on writing your own macros and reading Anchor-style macro code — skills you'll use daily in Solana development.
