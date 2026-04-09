# Module 07: Modules, Crates, and Cargo

## Creating This Project

```bash
cargo new m07-modules-crates
cd m07-modules-crates
```

This generates:
```
m07-modules-crates/
├── Cargo.toml
└── src/
    └── main.rs
```

We'll expand this into a multi-file project that demonstrates Rust's module system.

---

## Solidity vs Rust: Organizing Code

In Solidity, you organize code with files and imports:

```solidity
// In Solidity
import "./Token.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

library MathUtils {
    function add(uint a, uint b) internal pure returns (uint) {
        return a + b;
    }
}
```

Rust's approach is fundamentally different and more structured:

| Concept | Solidity | Rust |
|---------|----------|------|
| Code organization | Files + imports | Module tree (hierarchical) |
| External deps | npm / GitHub URLs | crates.io (Cargo.toml) |
| Default visibility | `public` | **private** |
| Restrict to file/crate | `internal` | `pub(crate)` |
| Import syntax | `import "./File.sol"` | `use crate::module::Item` |
| Library | `library Foo { }` | Module or crate |
| Build tool | Hardhat / Foundry | **Cargo** (built-in) |

The biggest mindset shift: **Rust modules form a tree rooted at `main.rs` (or `lib.rs`)**. You don't just import files — you declare modules and Rust resolves them through a hierarchy.

---

## Cargo: Rust's Build System and Package Manager

Cargo is to Rust what Foundry + npm combined are to Solidity. It handles building, testing, dependency management, linting, formatting, and publishing — all in one tool.

### Cargo.toml Anatomy

Every Rust project has a `Cargo.toml` at its root. Here's a complete breakdown:

```toml
[package]
name = "my-project"          # Crate name (used on crates.io)
version = "0.1.0"            # Semantic versioning
edition = "2021"             # Rust edition (2015, 2018, 2021, 2024)
authors = ["You <you@dev>"]  # Optional
description = "A cool crate" # Optional, needed for crates.io
license = "MIT"              # Optional, needed for crates.io

[dependencies]
# Production dependencies — like npm "dependencies"
serde = "1.0"                # From crates.io, version ^1.0
serde_json = { version = "1.0", features = ["raw_value"] }
my_local = { path = "../my_local" }  # Local crate

[dev-dependencies]
# Test-only dependencies — like npm "devDependencies"
tokio-test = "0.4"

[features]
# Conditional compilation flags — like Solidity's compile-time constants
default = ["json"]
json = ["serde_json"]        # Enabling "json" pulls in serde_json

[workspace]
# For monorepos with multiple crates (used in Anchor projects)
members = ["programs/*", "sdk"]
```

### Essential Cargo Commands

```bash
# Creating projects
cargo new my-project        # New project with git init
cargo new my-lib --lib      # New library crate (lib.rs instead of main.rs)
cargo init                  # Initialize in existing directory

# Adding dependencies (like `npm install`)
cargo add serde             # Add latest serde to [dependencies]
cargo add tokio --features full  # Add with features
cargo add --dev mockall     # Add to [dev-dependencies]

# Building and running
cargo build                 # Compile (debug mode, fast compile, slow binary)
cargo build --release       # Compile with optimizations (slow compile, fast binary)
cargo run                   # Build + run
cargo run --release         # Build + run optimized

# Checking and testing
cargo check                 # Type-check without producing binary (fastest feedback)
cargo test                  # Run all tests
cargo test wallet            # Run tests matching "wallet"

# Code quality
cargo clippy                # Linter — catches common mistakes and suggests improvements
cargo fmt                   # Auto-format code (like prettier for Rust)
cargo fmt -- --check        # Check formatting without changing files (CI-friendly)
```

### Cargo.lock — When to Commit It

| Project type | Commit Cargo.lock? | Why |
|-------------|-------------------|-----|
| Binary / Application | **Yes** | Ensures reproducible builds across machines |
| Library | **No** (.gitignore it) | Consumers should resolve their own versions |
| Solana program (Anchor) | **Yes** | It's a deployable binary |

`Cargo.lock` pins exact dependency versions. It's auto-generated — never edit it by hand.

### Workspaces — Multiple Crates in One Repo

Anchor projects use workspaces. A workspace shares one `Cargo.lock` and one `target/` directory across multiple crates:

```
my-anchor-project/
├── Cargo.toml              # Workspace root
├── programs/
│   └── my-program/
│       ├── Cargo.toml      # Individual crate
│       └── src/
│           └── lib.rs
├── sdk/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── tests/
```

Workspace root `Cargo.toml`:
```toml
[workspace]
members = [
    "programs/my-program",
    "sdk",
]
resolver = "2"
```

Benefits: shared compilation cache, consistent dependency versions, build everything with one `cargo build`.

---

## The Module System

### Key Terminology

- **Crate**: A compilation unit. Either a binary (has `main.rs`) or a library (has `lib.rs`).
- **Module**: A namespace within a crate. Defined with `mod`.
- **Path**: How you refer to items: `crate::wallet::Wallet`.

### Declaring Modules with `mod`

There are two ways to define a module:

**Inline (in the same file):**
```rust
mod wallet {
    pub struct Wallet {
        pub balance: u64,
    }
}
```

**File-based (separate file):**
```rust
// In main.rs or lib.rs
mod wallet;  // Tells Rust: look for wallet.rs or wallet/mod.rs
```

When Rust sees `mod wallet;`, it looks for:
1. `src/wallet.rs` (preferred, modern style)
2. `src/wallet/mod.rs` (older style, still used for modules with submodules)

### The Module Tree

Every crate has a module tree rooted at its crate root (`main.rs` or `lib.rs`):

```
crate (main.rs)
├── wallet          (src/wallet.rs)
├── transactions    (src/transactions.rs)
└── utils           (src/utils/mod.rs)
    └── helpers     (src/utils/helpers.rs)
```

This is the project structure we build in this module. The key insight: **`main.rs` must declare every top-level module**. Files don't become modules just by existing — they must be declared.

### File Layout

```
src/
├── main.rs             # Crate root — declares mod wallet, mod transactions, mod utils
├── wallet.rs           # wallet module
├── transactions.rs     # transactions module
└── utils/
    ├── mod.rs          # utils module — declares mod helpers
    └── helpers.rs      # utils::helpers submodule
```

When a module has submodules, use the directory form:
- `utils/mod.rs` is the `utils` module itself
- `utils/helpers.rs` is `utils::helpers`
- `utils/mod.rs` must contain `pub mod helpers;` to expose it

---

## Visibility: `pub` and Privacy

**Everything in Rust is private by default.** This is like Solidity's `internal`, except even stricter — child modules can't see parent's private items by default.

```rust
mod wallet {
    struct SecretKey(String);           // Private — only this module can use it
    pub struct Wallet {                 // Public — anyone can use it
        pub owner: String,             // Public field
        balance: u64,                  // Private field — even if struct is pub!
    }
    pub(crate) fn internal_check() {}  // Visible within this crate only
    pub fn create_wallet() -> Wallet { // Public function
        Wallet {
            owner: "Alice".into(),
            balance: 0,
        }
    }
}
```

### Visibility Modifiers

| Modifier | Scope | Solidity Equivalent |
|----------|-------|-------------------|
| (none) | Private to current module | `private` |
| `pub` | Public to everyone | `public` / `external` |
| `pub(crate)` | Visible within the crate | `internal` |
| `pub(super)` | Visible to parent module | No equivalent |
| `pub(in path)` | Visible to a specific ancestor | No equivalent |

### Best Practice

Prefer `pub(crate)` over `pub` when the item doesn't need to be part of your public API. This is especially important in Solana programs where you want to limit what's exposed:

```rust
// Good — only exposed within our crate
pub(crate) fn validate_transaction(tx: &Transaction) -> bool { /* ... */ }

// Only use pub when it's truly part of your public interface
pub fn process_instruction(/* ... */) { /* ... */ }
```

---

## The `use` Keyword — Bringing Items into Scope

Without `use`, you'd write full paths everywhere:

```rust
let w = crate::wallet::Wallet::new("Alice", 100);
```

`use` creates shortcuts:

```rust
use crate::wallet::Wallet;

let w = Wallet::new("Alice", 100);  // Much cleaner
```

### Importing Patterns

```rust
// Import a single item
use crate::wallet::Wallet;

// Import multiple items from the same module
use crate::transactions::{Transaction, TransactionType};

// Import everything (glob import) — use sparingly
use crate::utils::helpers::*;

// Rename to avoid conflicts
use crate::wallet::Wallet as SolWallet;

// Nested imports
use std::collections::{HashMap, HashSet};

// Import the module itself (then use module::Item)
use crate::wallet;
let w = wallet::Wallet::new("Alice", 100);
```

### Path Prefixes: `crate`, `self`, `super`

```rust
// crate:: — absolute path from the crate root
use crate::wallet::Wallet;

// self:: — relative to current module
use self::helpers::format_lamports;

// super:: — parent module (like ../ in file paths)
use super::wallet::Wallet;
```

Example with `super`:
```rust
mod outer {
    pub fn hello() -> String {
        "hello".to_string()
    }

    mod inner {
        pub fn greet() -> String {
            // super:: goes up to outer
            super::hello()
        }
    }
}
```

---

## External Crates

Adding external crates is like `npm install` but declared in `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
borsh = "1.0"
```

Then use them in your code:

```rust
use serde::{Serialize, Deserialize};
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct TokenAccount {
    pub mint: [u8; 32],
    pub owner: [u8; 32],
    pub amount: u64,
}
```

### Common Solana Crates (Preview)

You'll use these extensively in Part 2:

| Crate | Purpose | Like in Solidity |
|-------|---------|-----------------|
| `solana-program` | Core Solana types and entrypoint | Built-in Solidity types |
| `anchor-lang` | Framework for Solana programs | Like OpenZeppelin + Hardhat |
| `borsh` | Binary serialization (Solana's format) | ABI encoding |
| `spl-token` | SPL Token program interface | ERC-20 interface |
| `solana-sdk` | Client-side Solana utilities | ethers.js |

```toml
# Typical Anchor program Cargo.toml
[dependencies]
anchor-lang = "0.30"
```

```rust
// The famous Anchor prelude import
use anchor_lang::prelude::*;
```

---

## Re-exporting with `pub use`

`pub use` re-exports an item, making it available from the current module's path. This is how libraries create clean public APIs:

```rust
// Inside src/lib.rs
mod wallet;
mod transactions;

// Re-export so users write `my_crate::Wallet` instead of `my_crate::wallet::Wallet`
pub use wallet::Wallet;
pub use transactions::Transaction;
```

This is exactly what Anchor does with its prelude:

```rust
// Inside anchor_lang/src/prelude.rs (simplified)
pub use crate::accounts::*;
pub use crate::context::Context;
pub use crate::error::*;
pub use borsh::{BorshDeserialize, BorshSerialize};
```

That's why `use anchor_lang::prelude::*` gives you access to `Context`, `Account`, `Program`, etc. — they're all re-exported into the prelude module.

### The Prelude Pattern

Many Rust crates (including the standard library) use a "prelude" module that re-exports the most commonly used items:

```rust
// Your crate's prelude
pub mod prelude {
    pub use crate::Wallet;
    pub use crate::Transaction;
    pub use crate::TransactionType;
}

// Users just write:
use my_crate::prelude::*;
```

---

## Our Project Structure

Here's the complete project we build in this module:

```
m07-modules-crates/
├── Cargo.toml
└── src/
    ├── main.rs             # Crate root: declares modules, runs demo
    ├── wallet.rs           # Wallet struct and methods
    ├── transactions.rs     # Transaction types
    └── utils/
        ├── mod.rs          # Utils module: declares helpers submodule
        └── helpers.rs      # Helper/utility functions
```

### How It All Connects

1. `main.rs` declares `mod wallet;`, `mod transactions;`, `mod utils;`
2. `main.rs` uses `use crate::wallet::Wallet;` to bring `Wallet` into scope
3. `wallet.rs` defines a `Wallet` struct with `pub` fields and methods
4. `transactions.rs` uses `use crate::wallet::Wallet;` to reference the wallet module
5. `utils/mod.rs` declares `pub mod helpers;` and re-exports key functions
6. `utils/helpers.rs` provides utility functions used by `main.rs`

---

## Best Practices

### 1. Keep Modules Focused
Each module should have a single responsibility, just like Solidity contracts:
```
src/
├── state.rs        # Account/state definitions
├── instructions.rs # Instruction handlers
├── errors.rs       # Custom errors
└── utils.rs        # Helper functions
```

### 2. Prefer `pub(crate)` Over `pub`
Only use `pub` for items that are part of your crate's public API. Use `pub(crate)` for everything else:
```rust
pub(crate) fn validate(data: &[u8]) -> bool { /* ... */ }  // Internal
pub fn process(data: &[u8]) -> Result<()> { /* ... */ }     // Public API
```

### 3. Use Re-exports for Clean APIs
Don't make users dig into your module hierarchy:
```rust
// In lib.rs
pub use errors::MyError;
pub use state::MyAccount;
```

### 4. Organize by Feature, Not by Type
```
// Good — organized by feature
src/
├── transfer/
│   ├── mod.rs
│   ├── handler.rs
│   └── validation.rs
└── mint/
    ├── mod.rs
    ├── handler.rs
    └── validation.rs

// Avoid — organized by type
src/
├── handlers/
├── validators/
└── types/
```

### 5. Be Explicit with Imports
```rust
// Good — clear where things come from
use crate::wallet::Wallet;
use crate::transactions::Transaction;

// Avoid in production — hides origins
use crate::wallet::*;
```

---

## Summary

| Concept | Syntax | Purpose |
|---------|--------|---------|
| Declare module | `mod foo;` | Add `foo.rs` to the module tree |
| Inline module | `mod foo { }` | Define module in same file |
| Make public | `pub` | Expose to other modules |
| Crate-only | `pub(crate)` | Expose within crate only |
| Import | `use crate::foo::Bar` | Bring item into scope |
| Rename import | `use foo as bar` | Avoid name conflicts |
| Re-export | `pub use foo::Bar` | Expose from current module |
| Glob import | `use foo::*` | Import everything (use sparingly) |
| Parent module | `super::` | Relative path to parent |
| Crate root | `crate::` | Absolute path from root |

---

## Next Steps

In Module 08, we'll cover **Error Handling** — Rust's `Result<T, E>` and `Option<T>` types, the `?` operator, and custom error types. You'll see why Rust developers rarely use exceptions and how this maps to Solana's error handling patterns.
