# Rust-to-Solana: A Complete Tutorial for Solidity Developers

A hands-on, heavily-commented tutorial that takes you from Rust fundamentals to building Solana smart contracts with Anchor -- designed specifically for experienced Solidity/EVM developers.

## Who Is This For?

You already know Solidity and the EVM. You want to build on Solana. This tutorial bridges the gap by:

- Teaching Rust through the lens of concepts you already know
- Comparing EVM and Solana architecture side-by-side
- Building progressively from "Hello World" to a full Escrow program
- Explaining **every single line of code** so you can reproduce it from memory

## Prerequisites

- Comfortable with Solidity and smart contract development
- Basic terminal/command-line skills
- A Linux or macOS machine (or WSL2 on Windows)

## Setup

### Quick Install (Everything at Once)

```bash
curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash
```

This installs Rust, Solana CLI, Anchor CLI, Node.js, and Yarn.

### Manual Install (Step by Step)

**1. Install Rust**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
```

Verify:
```bash
rustc --version   # Should show 1.79+ 
cargo --version   # Rust's package manager (like npm/hardhat)
```

**2. Install Solana CLI** (needed for Part 2+)

```bash
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
```

Verify:
```bash
solana --version
solana-keygen new   # Create a local keypair for development
solana config set -u localhost   # Point to local validator
```

**3. Install Anchor CLI** (needed for Part 3)

```bash
cargo install --git https://github.com/coral-xyz/anchor avm --force
avm install latest
avm use latest
```

Verify:
```bash
anchor --version   # Should show 0.32+
```

**4. Linux Dependencies** (if not already installed)

```bash
sudo apt-get update && sudo apt-get install -y \
  build-essential pkg-config libudev-dev llvm libclang-dev \
  protobuf-compiler libssl-dev
```

## Learning Path

### Part 1: Rust Fundamentals (Do This First)

No Solana tools needed -- just Rust. Each module is a standalone project you can `cargo run`.

| # | Module | What You Learn | Solidity Parallel |
|---|--------|---------------|-------------------|
| 01 | [Variables, Types & Functions](part1-rust-fundamentals/m01-variables-types-functions/) | `let`, `mut`, types, `fn` | `uint256`, `address`, `function` |
| 02 | [Ownership & Borrowing](part1-rust-fundamentals/m02-ownership-borrowing/) | Rust's killer feature | No equivalent -- biggest mental shift |
| 03 | [Structs & Enums](part1-rust-fundamentals/m03-structs-enums/) | Data modeling, `impl`, `match` | `struct`, `enum` but richer |
| 04 | [Error Handling](part1-rust-fundamentals/m04-error-handling/) | `Result`, `Option`, `?` operator | `require()`, `revert()` |
| 05 | [Traits & Generics](part1-rust-fundamentals/m05-traits-generics/) | Polymorphism, trait bounds | `interface` but more powerful |
| 06 | [Collections & Iterators](part1-rust-fundamentals/m06-collections-iterators/) | `Vec`, `HashMap`, functional chains | `mapping`, arrays |
| 07 | [Modules & Crates](part1-rust-fundamentals/m07-modules-crates/) | Project organization, dependencies | `import`, `library` |
| 08 | [Macros & Attributes](part1-rust-fundamentals/m08-macros-attributes/) | Metaprogramming | Needed for Anchor's magic |

### Part 2: Solana Core Concepts

Understand the Solana runtime before using frameworks. Requires Solana CLI.

| # | Module | What You Learn |
|---|--------|---------------|
| 09 | [Solana vs EVM Architecture](part2-solana-concepts/m09-solana-vs-evm/) | Account model, rent, programs vs contracts |
| 10 | [Native Hello World](part2-solana-concepts/m10-native-hello-world/) | `entrypoint!`, `process_instruction`, building programs |
| 11 | [Program Derived Addresses](part2-solana-concepts/m11-program-derived-addresses/) | PDAs, seeds, bumps (like CREATE2) |
| 12 | [Cross-Program Invocation](part2-solana-concepts/m12-cross-program-invocation/) | CPI, `invoke`, `invoke_signed` |

### Part 3: Anchor Framework

The standard framework for Solana development. Requires Anchor CLI.

| # | Module | What You Build |
|---|--------|---------------|
| 13 | [Anchor Counter](part3-anchor-programs/m13-anchor-counter/) | Your first Anchor program |
| 14 | [Account Constraints](part3-anchor-programs/m14-account-constraints/) | Validation macros deep dive |
| 15 | [Todo App](part3-anchor-programs/m15-anchor-todo-app/) | Full CRUD dApp |
| 16 | [Token Operations](part3-anchor-programs/m16-token-operations/) | SPL tokens (like ERC-20) |
| 17 | [Escrow Program](part3-anchor-programs/m17-escrow-program/) | Capstone: trustless escrow |

## How to Use Each Module

```bash
# 1. Read the tutorial
cat m01-variables-types-functions/tutorial.md

# 2. Study the commented tutorial code
cat m01-variables-types-functions/src/main.rs

# 3. Run the tutorial code
cd m01-variables-types-functions
cargo run

# 4. Try the exercises (fill in the TODOs)
cd exercises
cargo run   # Won't compile until you fill in TODOs

# 5. Check your answers
cd ../solutions
cargo run
```

## How to Create a New Rust Project from Scratch

This is a skill you'll use constantly. Here's the process:

```bash
# Create a new binary project
cargo new my-project
cd my-project

# What cargo created:
# my-project/
# ├── Cargo.toml    # Project manifest (like package.json)
# └── src/
#     └── main.rs   # Entry point (like your contract's constructor)

# Add a dependency
cargo add serde     # Adds serde to Cargo.toml automatically

# Build and run
cargo build         # Compile (like `forge build`)
cargo run           # Compile + run (like `forge script`)
cargo test          # Run tests (like `forge test`)

# Create a library project instead
cargo new my-lib --lib
# This creates src/lib.rs instead of src/main.rs
```

## Best Practices

1. **Run `cargo clippy`** after writing code -- it catches common mistakes
2. **Run `cargo fmt`** to auto-format your code -- the community standard
3. **Use `cargo doc --open`** to read documentation for your dependencies
4. **Commit Cargo.lock** for binary projects, ignore it for libraries
5. **Test early and often** with `cargo test` -- Rust's test framework is built-in
