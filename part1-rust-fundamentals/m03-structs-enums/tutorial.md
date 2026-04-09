# Module 03: Structs, Enums, and Pattern Matching

## Creating This Project

```bash
cargo new m03-structs-enums
cd m03-structs-enums
```

This generates a new Rust project with `Cargo.toml` and `src/main.rs`. Open it in your editor and follow along.

---

## Overview

This module covers Rust's primary tools for modeling data: **structs**, **enums**, and **pattern matching**. If you're coming from Solidity, you already know structs and enums — but Rust's versions are significantly more powerful.

| Concept | Solidity | Rust |
|---|---|---|
| Structs | Data-only containers | Data + methods via `impl` blocks |
| Enums | Numbered variants (0, 1, 2…) | Variants that carry data (tagged unions) |
| Null handling | `address(0)`, unchecked | `Option<T>` — compiler-enforced |
| Pattern matching | `if/else` chains | `match` — exhaustive, compiler-checked |

**Why this matters for Solana**: Account data is modeled as structs. Instruction types are modeled as enums. Pattern matching dispatches instructions. These are the building blocks of every Solana program.

---

## 1. Struct Definition and Instantiation

A struct groups related data under a single type. In Solidity, struct fields are your contract's state variables. In Rust, structs are standalone types.

```rust
struct TokenAccount {
    owner: String,
    balance: u64,
    is_frozen: bool,
}

fn main() {
    let account = TokenAccount {
        owner: String::from("Alice"),
        balance: 1_000_000,
        is_frozen: false,
    };

    println!("Owner: {}", account.owner);
}
```

**Solidity comparison:**
```solidity
struct TokenAccount {
    address owner;
    uint256 balance;
    bool isFrozen;
}
// In Solidity, you'd store this in a mapping: mapping(address => TokenAccount)
// In Rust/Solana, each account's data IS this struct, serialized into bytes.
```

### Field Init Shorthand

When a variable name matches a field name, you can skip the repetition:

```rust
let owner = String::from("Alice");
let balance = 1_000_000;

let account = TokenAccount {
    owner,       // shorthand for owner: owner
    balance,     // shorthand for balance: balance
    is_frozen: false,
};
```

This is identical to JavaScript's object shorthand (`{ owner, balance }`). Solidity doesn't have this.

### Struct Update Syntax

Create a new struct based on an existing one, overriding specific fields:

```rust
let account2 = TokenAccount {
    owner: String::from("Bob"),
    ..account  // copy remaining fields from `account`
};
```

> **Note:** This *moves* any non-Copy fields (like `String`). After this, `account.owner` is no longer accessible, but `account.balance` and `account.is_frozen` still are (because `u64` and `bool` implement `Copy`).

---

## 2. Tuple Structs (Unnamed Fields)

Tuple structs have types but no field names. They're used for the **newtype pattern** — wrapping an existing type to give it a distinct identity.

```rust
struct Lamports(u64);
struct TokenAmount(u64);

let fee = Lamports(5000);
let amount = TokenAmount(100);

// fee == amount  // Compile error! Different types, even though both wrap u64.
```

**Why this matters for Solana:** Solana uses this pattern extensively. `Pubkey` is essentially a wrapper around `[u8; 32]`. The newtype pattern prevents you from accidentally passing a token amount where a lamport amount is expected — the compiler catches it.

**Solidity comparison:** Solidity has no equivalent. You'd use `uint256` for both and hope you don't mix them up. Rust's type system prevents the mistake at compile time.

---

## 3. Unit Structs (No Fields)

Unit structs have no fields at all. They're used as markers or type-level tokens.

```rust
struct Initialize;
struct Finalize;
```

You'll see these in trait implementations and type-state patterns. They carry no data, just identity.

---

## 4. Methods with `impl` Blocks

This is where Rust structs diverge sharply from Solidity structs. In Solidity, functions live at the contract level and operate on storage. In Rust, methods are attached to structs via `impl` blocks.

```rust
struct TokenAccount {
    owner: String,
    balance: u64,
    is_frozen: bool,
}

impl TokenAccount {
    // Method that borrows self immutably — read-only access
    fn balance(&self) -> u64 {
        self.balance
    }

    // Method that borrows self mutably — can modify fields
    fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }

    // Method that takes ownership of self — consumes the struct
    fn close(self) -> u64 {
        self.balance  // after this, `self` is dropped
    }
}
```

**Solidity comparison:**
```solidity
// In Solidity, these would be contract functions:
function balance() public view returns (uint256) { ... }  // view = &self
function deposit(uint256 amount) public { ... }           // mutating = &mut self
// There's no Solidity equivalent of consuming self.
```

### Choosing the Right Receiver

| Receiver | Meaning | Solidity analogy |
|---|---|---|
| `&self` | Immutable borrow — read only | `view` function |
| `&mut self` | Mutable borrow — can modify | Regular state-changing function |
| `self` | Takes ownership — consumes | No direct equivalent (close/destroy) |

**Rule of thumb:** Use `&self` by default. Use `&mut self` when you need to modify. Use `self` only when the method logically "consumes" the value (like closing an account).

---

## 5. Associated Functions (No `self`)

Functions inside `impl` that don't take `self` are **associated functions**. They're called with `::` syntax, not dot syntax. The most common use is constructors.

```rust
impl TokenAccount {
    fn new(owner: String) -> Self {
        Self {
            owner,
            balance: 0,
            is_frozen: false,
        }
    }
}

let account = TokenAccount::new(String::from("Alice"));
```

**Solidity comparison:** This is like a Solidity `constructor()` — it creates a new instance. The `Self` keyword refers to the type being implemented (here, `TokenAccount`).

---

## 6. Enums: Rust's Superpower

Solidity enums are numbered variants — they're basically labeled integers:

```solidity
enum Status { Active, Paused, Closed }  // 0, 1, 2
```

Rust enums can **carry data** with each variant. This makes them tagged unions — each variant can hold different types and amounts of data.

### Variant Types

```rust
enum Instruction {
    // Unit variant — no data (like Solidity enums)
    Initialize,

    // Tuple variant — unnamed data
    Transfer(u64),

    // Struct variant — named fields
    CreateAccount {
        owner: String,
        initial_balance: u64,
    },
}
```

**This is impossible in Solidity.** In Solidity, if `Transfer` needs an amount, you'd pass it as a separate function parameter. In Rust (and Solana), the data rides with the variant.

### Why This Matters for Solana

Every Solana program defines an instruction enum:

```rust
enum TokenInstruction {
    InitializeMint { decimals: u8 },
    Transfer { amount: u64 },
    Approve { amount: u64 },
    Burn { amount: u64 },
    CloseAccount,
}
```

The program deserializes incoming bytes into this enum, then pattern-matches to dispatch the right handler. This is the core architecture of every Solana program.

---

## 7. `Option<T>` — Rust's Null Safety

There is no `null` in Rust. Instead, Rust uses the `Option<T>` enum:

```rust
enum Option<T> {
    Some(T),  // a value is present
    None,     // no value
}
```

**Solidity comparison:** In Solidity, an uninitialized `address` is `address(0)`. An uninitialized `uint` is `0`. There's no way to distinguish "intentionally zero" from "not set." This causes bugs.

In Rust, the compiler **forces** you to handle the `None` case before using the value:

```rust
let maybe_balance: Option<u64> = Some(100);

// You can't just use maybe_balance as a u64. You must unwrap it:
match maybe_balance {
    Some(b) => println!("Balance: {}", b),
    None => println!("No balance set"),
}
```

### Common `Option` Methods

```rust
let x: Option<u64> = Some(42);

x.unwrap();           // Returns 42, but panics on None — use sparingly
x.unwrap_or(0);       // Returns 42, or 0 if None
x.is_some();          // true
x.is_none();          // false
x.map(|v| v * 2);     // Some(84) — transforms the inner value
```

---

## 8. Pattern Matching with `match`

`match` is Rust's most powerful control flow tool. It's like a `switch` statement, but:

1. It's **exhaustive** — the compiler ensures you handle every variant.
2. It can **destructure** data out of enums and structs.
3. Each arm can **bind variables** to the extracted data.

```rust
enum Instruction {
    Initialize,
    Transfer(u64),
    CreateAccount { owner: String, initial_balance: u64 },
}

fn process(ix: Instruction) {
    match ix {
        Instruction::Initialize => {
            println!("Initializing...");
        }
        Instruction::Transfer(amount) => {
            println!("Transferring {} tokens", amount);
        }
        Instruction::CreateAccount { owner, initial_balance } => {
            println!("Creating account for {} with {}", owner, initial_balance);
        }
    }
}
```

If you forget a variant, the compiler tells you:

```
error[E0004]: non-exhaustive patterns: `CreateAccount { .. }` not covered
```

**Solidity comparison:** Solidity has no equivalent. You'd use `if/else` chains that the compiler doesn't check. Missing a case is a runtime bug (or worse, a security exploit).

### Match Guards and Wildcards

```rust
match amount {
    0 => println!("Zero transfer"),
    1..=100 => println!("Small transfer"),
    n if n > 1_000_000 => println!("Whale alert: {}", n),
    _ => println!("Normal transfer"),  // _ matches anything
}
```

---

## 9. `if let` — Single-Pattern Matching

When you only care about one variant, `if let` is more concise than a full `match`:

```rust
let maybe_owner: Option<String> = Some(String::from("Alice"));

// Instead of a full match:
if let Some(owner) = maybe_owner {
    println!("Owner is {}", owner);
}
// If it's None, we just skip the block.
```

**Use `match` when:** You need to handle multiple variants.
**Use `if let` when:** You only care about one variant and want to ignore the rest.

---

## 10. `#[derive()]` — Automatic Trait Implementations

The `#[derive()]` attribute tells the compiler to auto-generate implementations of common traits:

```rust
#[derive(Debug, Clone, PartialEq)]
struct TokenAccount {
    owner: String,
    balance: u64,
    is_frozen: bool,
}
```

| Derive | What it does | Why you need it |
|---|---|---|
| `Debug` | Enables `{:?}` formatting | Print structs for debugging |
| `Clone` | Enables `.clone()` | Create explicit copies |
| `Copy` | Implicit copy on assignment | Only for small, stack-only types |
| `PartialEq` | Enables `==` comparison | Compare struct instances |
| `Default` | Enables `Default::default()` | Zero/empty initialization |

**Best practice:** Derive `Debug` on everything. You will always want to print your types during development.

**Solana note:** Solana programs use `#[derive(BorshSerialize, BorshDeserialize)]` to serialize account data. Same mechanism, different traits.

---

## Best Practices

1. **Derive `Debug` on everything.** There's no reason not to. `{:?}` formatting is invaluable during development.

2. **Use enums for state machines.** If something can be in one of several states, model it as an enum. The compiler will force you to handle every state.

   ```rust
   enum AccountState {
       Uninitialized,
       Active { balance: u64 },
       Frozen { balance: u64, reason: String },
       Closed,
   }
   ```

3. **Prefer `match` over `if-else` chains.** `match` is exhaustive — when you add a new variant, the compiler tells you everywhere you need to update.

4. **Use `Option<T>` instead of sentinel values.** Don't use `0` or empty strings to mean "not set." Use `Option::None`.

5. **Use the newtype pattern** to distinguish values with the same underlying type. `Lamports(u64)` and `TokenAmount(u64)` are different types — the compiler prevents mixing them.

6. **Put related methods in `impl` blocks.** Keep your data (struct) and behavior (methods) together.

---

## Summary

| Rust Concept | Solidity Equivalent | Key Difference |
|---|---|---|
| `struct` | `struct` | Rust structs have methods via `impl` |
| `impl` block | Contract functions | Methods are scoped to a type |
| Tuple struct | No equivalent | Newtype pattern for type safety |
| `enum` | `enum` (numbered) | Rust enums carry data |
| `Option<T>` | `address(0)`, `0` | Compiler-enforced null safety |
| `match` | `if/else` chains | Exhaustive, compiler-checked |
| `#[derive()]` | No equivalent | Auto-generate trait impls |

**Next module:** We'll explore ownership and borrowing — the heart of Rust's memory safety model, and the concept that makes Solana's account model possible.
