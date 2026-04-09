# Module 13: Your First Anchor Program — A Simple Counter

Welcome to the Anchor framework! If you've made it through the raw Solana program modules, you know firsthand how much boilerplate goes into deserializing accounts, checking owners, and validating signers. Anchor eliminates all of that. Think of it as the **Hardhat + OpenZeppelin** of Solana — a framework that generates the tedious code so you can focus on business logic.

In this module you'll build a **Counter program** — the "Hello World" of Anchor — and learn every concept by mapping it back to Solidity patterns you already know.

---

## Table of Contents

1. [Creating the Project from Scratch](#1-creating-the-project-from-scratch)
2. [Solidity vs Anchor — Side by Side](#2-solidity-vs-anchor--side-by-side)
3. [Deep Dive into the Anchor Counter](#3-deep-dive-into-the-anchor-counter)
4. [Account Space and the 8-Byte Discriminator](#4-account-space-and-the-8-byte-discriminator)
5. [Building, Testing, and Deploying](#5-building-testing-and-deploying)
6. [Best Practices](#6-best-practices)
7. [Key Takeaways](#7-key-takeaways)

---

## 1. Creating the Project from Scratch

### Prerequisites

Install the Anchor CLI (requires Rust and Solana CLI already installed):

```bash
# Install Anchor Version Manager (avm)
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force

# Install Anchor 0.30.x
avm install 0.30.1
avm use 0.30.1

# Verify
anchor --version
# anchor-cli 0.30.1
```

### Scaffold a New Project

```bash
anchor init counter
cd counter
```

This generates the following file structure:

```
counter/
├── Anchor.toml              # Project configuration (like hardhat.config.js)
├── Cargo.toml               # Rust workspace root
├── package.json             # Node.js deps for tests
├── tsconfig.json            # TypeScript config for tests
├── app/                     # (empty) Frontend placeholder
├── migrations/
│   └── deploy.ts            # Migration script run on `anchor migrate`
├── programs/
│   └── counter/
│       ├── Cargo.toml       # Rust package for your program
│       └── src/
│           └── lib.rs        # YOUR PROGRAM CODE LIVES HERE
└── tests/
    └── counter.ts           # TypeScript integration tests
```

### What Each File Does

| File/Directory | Purpose | Solidity Equivalent |
|---|---|---|
| `Anchor.toml` | Declares program IDs, cluster (devnet/localnet), wallet path, test command | `hardhat.config.js` |
| `programs/counter/src/lib.rs` | Your on-chain program | `contracts/Counter.sol` |
| `programs/counter/Cargo.toml` | Rust dependencies for the program | Nothing direct (managed by framework) |
| `tests/counter.ts` | Integration tests using `@coral-xyz/anchor` TypeScript SDK | `test/Counter.test.js` with ethers.js |
| `migrations/deploy.ts` | Deployment script | `scripts/deploy.js` |
| `Cargo.toml` (root) | Rust workspace — groups all programs | Monorepo `package.json` |

### Anchor.toml Explained

```toml
[features]
seeds = false
skip-lint = false

[programs.localnet]
counter = "YourProgramId111111111111111111111111111111"

[registry]
url = "https://api.apr.dev"

[provider]
cluster = "Localnet"
wallet = "~/.config/solana/id.json"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
```

Key fields:
- **`[programs.localnet]`** — Maps program names to their deployed addresses (like a deployment registry).
- **`[provider]`** — Which cluster to deploy to and which wallet pays for transactions.
- **`[scripts]`** — The command `anchor test` runs. Default uses `ts-mocha`.

> **Solidity parallel**: This is your `hardhat.config.js` — it defines networks, accounts, and where your contracts live.

---

## 2. Solidity vs Anchor — Side by Side

Here's the complete Counter in both languages:

### Solidity Version

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Counter {
    uint64 public count;

    function initialize() public {
        count = 0;
    }

    function increment() public {
        count += 1;
    }

    function decrement() public {
        require(count > 0, "Counter: cannot go below zero");
        count -= 1;
    }
}
```

### Anchor/Rust Version

```rust
use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        require!(counter.count > 0, ErrorCode::BelowZero);
        counter.count -= 1;
        Ok(())
    }
}

// Account data struct
#[account]
pub struct Counter {
    pub count: u64,
}

// Account validation for each instruction...
// (see full code in src/lib.rs)
```

### Mapping the Concepts

| Solidity | Anchor | Notes |
|---|---|---|
| `contract Counter` | `#[program] pub mod counter` | Module = contract |
| Contract address | `declare_id!(...)` | Known at compile time |
| `uint64 public count` | `#[account] struct Counter { count: u64 }` | State lives in a *separate account* |
| `function initialize()` | `pub fn initialize(ctx: Context<Initialize>)` | Each fn gets a typed Context |
| `msg.sender` | `Signer<'info>` in the Accounts struct | Must be explicitly declared |
| `require(...)` | `require!(...)` macro | Returns an error instead of reverting |
| Contract storage | Account data (serialized with Borsh) | Allocated once with `init` |
| `msg.value` (ETH payment) | `payer` constraint | Who pays for account rent |
| None — automatic | `space` constraint | You must calculate storage size |

### The Fundamental Difference

In Solidity, **state lives inside the contract**. In Solana/Anchor, **state lives in separate accounts** that the program reads and writes. This is why every instruction needs an "Accounts struct" — it declares which accounts the instruction will touch and how they should be validated.

Think of it like this: A Solidity contract is a **single object** with code + state. An Anchor program is a **stateless function** that operates on **external state accounts** passed to it.

---

## 3. Deep Dive into the Anchor Counter

Let's walk through every concept in detail.

### 3.1 `declare_id!` — The Program Address

```rust
declare_id!("11111111111111111111111111111111");
```

- Sets the program's on-chain address (public key).
- When you run `anchor build`, the CLI generates a keypair in `target/deploy/counter-keypair.json` and updates this macro with the real public key.
- The Anchor framework verifies at runtime that instructions are being sent to this program ID.

**Solidity parallel**: Like knowing your contract's deployed address at compile time. In Solidity you don't need this because the EVM routes calls by address automatically. On Solana, the runtime needs to verify the program ID explicitly.

### 3.2 `#[program]` Module — Instruction Handlers

```rust
#[program]
pub mod counter {
    use super::*;
    // instruction handlers go here
}
```

The `#[program]` attribute macro generates:
1. An **entrypoint** function that the Solana runtime calls.
2. A **dispatcher** that reads the first 8 bytes of instruction data (the "discriminator") and routes to the correct function.
3. **Account deserialization** — before your function runs, Anchor deserializes all accounts specified in the Context type.
4. **Account serialization** — after your function returns, Anchor serializes modified account data back.

**Solidity parallel**: The generated code is like Solidity's ABI dispatcher — the EVM reads the first 4 bytes of calldata (function selector) and jumps to the right function. Anchor does the same with 8 bytes.

### 3.3 `Context<T>` — Accessing Accounts

```rust
pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    // ...
}
```

`Context<T>` provides:
- `ctx.accounts` — the validated, deserialized accounts (type `T`).
- `ctx.program_id` — the program's public key.
- `ctx.remaining_accounts` — any extra accounts not declared in `T`.
- `ctx.bumps` — PDA bump seeds (we'll cover PDAs in a later module).

The generic parameter `T` (e.g., `Initialize`) is the Accounts struct that defines which accounts this instruction expects. Anchor validates them *before* your function body runs.

**Solidity parallel**: It's like if Solidity gave you a pre-validated struct of all your function's inputs plus `msg.sender`, `msg.value`, etc. — all checked before your code executes.

### 3.4 `#[derive(Accounts)]` — Account Validation

```rust
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

This is where Anchor truly shines. The `#[derive(Accounts)]` macro generates code to:
1. **Deserialize** each account from the raw `AccountInfo` array.
2. **Validate** ownership, signers, mutability, and custom constraints.
3. **Create** accounts if the `init` constraint is present.

Each field has a type that enforces specific checks:

| Type | What It Validates | Solidity Equivalent |
|---|---|---|
| `Account<'info, T>` | Owned by this program, deserializable as `T` | A typed storage pointer |
| `Signer<'info>` | The account signed the transaction | `msg.sender` |
| `Program<'info, System>` | It's a valid program account | N/A (implicit in EVM) |
| `SystemAccount<'info>` | Just a system-owned account | An address with ETH balance |
| `UncheckedAccount<'info>` | No validation (⚠️ dangerous) | Raw `address` |

**The `'info` lifetime**: This is Rust's way of saying "these references borrow from the same source" (the transaction's account data). Don't overthink it — just include `<'info>` on every Accounts struct.

### 3.5 Account Constraints

Constraints are the `#[account(...)]` attributes on each field:

#### `init` — Create a New Account

```rust
#[account(init, payer = user, space = 8 + 8)]
pub counter: Account<'info, Counter>,
```

- Calls `system_program::create_account` behind the scenes.
- Allocates `space` bytes of storage.
- Transfers rent from `payer` to the new account.
- Sets the owner to this program.

**Solidity parallel**: Like deploying a new storage contract and paying gas. In Solidity, storage is "free" (you just use it), but on Solana you must explicitly allocate and pay rent.

#### `payer` — Who Pays

```rust
payer = user
```

The account that funds the new account's rent. Must be a `Signer` marked `#[account(mut)]` (mutable because SOL is deducted).

**Solidity parallel**: `msg.sender` paying gas + `msg.value` for contract creation.

#### `space` — Storage Allocation

```rust
space = 8 + 8
```

How many bytes to allocate. This is **fixed at creation** — you can't resize later (without `realloc`).

- `8` = Anchor discriminator (always required).
- `+ 8` = our `u64` field (`count`).

**Solidity parallel**: In Solidity you never think about this because the EVM manages storage slots dynamically. On Solana, you must plan your storage layout upfront.

#### `mut` — Mutable Access

```rust
#[account(mut)]
pub user: Signer<'info>,
```

Marks an account as writable. Without `mut`, trying to modify the account will fail at the runtime level.

### 3.6 `#[account]` — Program Account Data

```rust
#[account]
pub struct Counter {
    pub count: u64,
}
```

The `#[account]` attribute macro generates:
1. **Borsh serialization** (`BorshSerialize`, `BorshDeserialize`) — binary encoding for on-chain storage.
2. An **8-byte discriminator** — a hash of the account type name, used to verify the account contains the expected data type.
3. **Owner check** — ensures the account is owned by this program.

**Solidity parallel**: Like a Solidity struct used for storage, except it's explicitly serialized/deserialized on every instruction call.

### 3.7 `Signer<'info>` — Transaction Signer

```rust
pub user: Signer<'info>,
```

Validates that this account signed the transaction. If the account didn't sign, the instruction fails before your code runs.

**Solidity parallel**: This is `msg.sender`. But in Solidity it's implicit — you always have access to `msg.sender`. In Anchor, you must explicitly declare which accounts must sign.

### 3.8 `SystemProgram` — The System Program

```rust
pub system_program: Program<'info, System>,
```

Required whenever you use the `init` constraint, because account creation calls the System Program under the hood. It's like a built-in "factory" that creates accounts.

**Solidity parallel**: No direct equivalent. The EVM handles account creation internally. On Solana, the System Program is an explicit, separate program that your program calls via Cross-Program Invocation (CPI).

---

## 4. Account Space and the 8-Byte Discriminator

### The Discriminator

Every Anchor account starts with an 8-byte discriminator (a truncated SHA-256 hash of `"account:<TypeName>"`). This prevents accidentally deserializing the wrong account type.

```
[8 bytes: discriminator][rest: your data]
```

### Space Calculation

When you specify `space`, always start with `8` for the discriminator:

```
space = 8 + <your fields>
```

### Common Type Sizes

| Rust Type | Size (bytes) | Solidity Equivalent |
|---|---|---|
| `bool` | 1 | `bool` |
| `u8` / `i8` | 1 | `uint8` / `int8` |
| `u16` / `i16` | 2 | `uint16` / `int16` |
| `u32` / `i32` | 4 | `uint32` / `int32` |
| `u64` / `i64` | 8 | `uint64` / `int64` |
| `u128` / `i128` | 16 | `uint128` / `int128` |
| `Pubkey` | 32 | `address` (20 bytes in EVM) |
| `String` | 4 + len | `string` (dynamic in EVM) |
| `Vec<T>` | 4 + (len × sizeof(T)) | Dynamic array |
| `Option<T>` | 1 + sizeof(T) | N/A |

### Example: Complex Account

```rust
#[account]
pub struct GameState {
    pub authority: Pubkey,   // 32 bytes
    pub score: u64,          // 8 bytes
    pub name: String,        // 4 + max_len bytes (let's say max 32 chars)
    pub is_active: bool,     // 1 byte
}

// space = 8 (discriminator) + 32 + 8 + (4 + 32) + 1 = 85
```

> **Tip**: Always double-check your space calculation. Too little space causes a runtime error. Too much wastes rent (SOL).

---

## 5. Building, Testing, and Deploying

### Building

```bash
anchor build
```

This:
1. Compiles your Rust program to BPF bytecode (the format Solana's runtime executes).
2. Generates an IDL (Interface Description Language) file at `target/idl/counter.json` — like Solidity's ABI.
3. Generates TypeScript types at `target/types/counter.ts`.
4. Creates a program keypair at `target/deploy/counter-keypair.json`.
5. Updates `declare_id!` with the keypair's public key.

**Solidity parallel**: `npx hardhat compile` — produces ABI + bytecode. Anchor's IDL is the equivalent of Solidity's ABI JSON.

### Testing

```bash
anchor test
```

This:
1. Starts a local Solana validator (`solana-test-validator`).
2. Builds and deploys your program to it.
3. Runs your TypeScript tests.
4. Shuts down the validator.

A typical test file (`tests/counter.ts`):

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { expect } from "chai";

describe("counter", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Counter as Program<Counter>;

  // Generate a new keypair for the counter account
  const counter = anchor.web3.Keypair.generate();

  it("Initializes the counter", async () => {
    await program.methods
      .initialize()                           // Call the initialize instruction
      .accounts({
        counter: counter.publicKey,           // The counter account to create
        user: provider.wallet.publicKey,      // The payer
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([counter])                     // counter keypair must sign (it's being created)
      .rpc();                                 // Send the transaction

    // Fetch the account and check the data
    const account = await program.account.counter.fetch(counter.publicKey);
    expect(account.count.toNumber()).to.equal(0);
  });

  it("Increments the counter", async () => {
    await program.methods
      .increment()
      .accounts({
        counter: counter.publicKey,
      })
      .rpc();

    const account = await program.account.counter.fetch(counter.publicKey);
    expect(account.count.toNumber()).to.equal(1);
  });
});
```

**Solidity parallel**: This is very similar to a Hardhat/ethers.js test. Instead of `contract.increment()`, you call `program.methods.increment()`. Instead of getting return values, you fetch account data after the transaction.

### Deploying

```bash
# Deploy to devnet
anchor deploy --provider.cluster devnet

# Or configure Anchor.toml and just:
anchor deploy
```

After deployment:
- Your program is live at the address in `declare_id!`.
- Anyone can call its instructions by sending transactions with the correct accounts.

**Solidity parallel**: `npx hardhat run scripts/deploy.js --network goerli`

---

## 6. Best Practices

### 1. Use Descriptive Account Names

```rust
// ❌ Bad
pub struct DoThing<'info> {
    pub a: Account<'info, Counter>,
    pub b: Signer<'info>,
}

// ✅ Good
pub struct Increment<'info> {
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}
```

### 2. Always Validate with Constraints

Don't rely on runtime checks in your instruction body when Anchor constraints can do it:

```rust
// ❌ Manual check
pub fn increment(ctx: Context<Increment>) -> Result<()> {
    if ctx.accounts.counter.owner != ctx.program_id {
        return Err(ErrorCode::InvalidOwner.into());
    }
    // ...
}

// ✅ Anchor does this automatically with Account<'info, T>
// The Account type checks ownership for you
```

### 3. Calculate Space Correctly

```rust
// Define a constant for clarity
const COUNTER_SIZE: usize = 8 + 8; // discriminator + u64

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = COUNTER_SIZE)]
    pub counter: Account<'info, Counter>,
    // ...
}
```

### 4. Use `require!` for Business Logic Checks

```rust
pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    require!(counter.count > 0, ErrorCode::BelowZero);
    counter.count -= 1;
    Ok(())
}
```

### 5. Define Custom Errors

```rust
#[error_code]
pub enum ErrorCode {
    #[msg("Cannot decrement below zero")]
    BelowZero,
}
```

Anchor's `#[error_code]` generates error types with unique codes — much better than opaque error messages.

---

## 7. Key Takeaways

| Concept | What You Learned |
|---|---|
| `declare_id!` | Sets the program's on-chain address |
| `#[program]` | Marks the module containing instruction handlers |
| `Context<T>` | Provides validated accounts to each instruction |
| `#[derive(Accounts)]` | Defines account validation rules (the real power of Anchor) |
| `#[account]` | Marks a struct as serializable on-chain data |
| `init` constraint | Creates and initializes a new account |
| `payer` / `space` | Who pays and how much storage to allocate |
| `Account<'info, T>` | Typed, validated account access |
| `Signer<'info>` | Verified transaction signer (like `msg.sender`) |
| Discriminator | 8-byte type tag at the start of every Anchor account |

### What's Next

In Module 14, we'll explore **PDAs (Program Derived Addresses)** — Solana's equivalent of deterministic contract addresses. PDAs let you create accounts at predictable addresses without a private key, enabling patterns like per-user storage, escrows, and more.

---

## Exercises

Check the `exercises/` directory for hands-on practice:

1. **Complete the Initialize struct** — Fill in the missing account constraints.
2. **Add a Reset instruction** — Create a new instruction that sets count back to 0.
3. **Add Authority control** — Add an `authority` field to Counter and validate only the authority can increment.
4. **Calculate complex space** — Figure out the correct `space` for a multi-field account.
5. **Safe decrement with errors** — Implement decrement with proper custom error handling.

Solutions are in the `solutions/` directory.
