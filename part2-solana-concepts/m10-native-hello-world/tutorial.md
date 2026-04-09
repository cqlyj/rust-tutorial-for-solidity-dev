# Module 10: Your First Native Solana Program — Hello World

## Overview

This module walks you through writing a native Solana program from scratch — no Anchor, no frameworks, just raw Rust against the `solana-program` crate. If you've written Solidity contracts, think of this as the equivalent of writing your first `contract HelloWorld {}` — except on Solana, the mechanics are fundamentally different.

By the end of this module you will:
- Understand how Solana programs are structured at the lowest level
- Know the three parameters every instruction receives
- Be able to build and deploy a program to a local validator
- See exactly where Solana concepts map to (and diverge from) Solidity

---

## Creating the Project from Scratch

```bash
cargo new m10-native-hello-world --lib
cd m10-native-hello-world
```

### Why `--lib`?

Solana programs are **libraries**, not binaries. They don't have a `main()` function. Instead, the Solana runtime loads your compiled code as a shared library and calls a well-known entrypoint function. This is conceptually similar to how the EVM loads contract bytecode — your contract doesn't "run" on its own; the runtime invokes specific functions in it.

If you used `cargo new` without `--lib`, Cargo would create a `src/main.rs` with a `fn main()`. That's wrong for Solana — you need `src/lib.rs` with an exported entrypoint.

### Edit `Cargo.toml`

After creating the project, edit `Cargo.toml`:

```toml
[package]
name = "m10-native-hello-world"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "lib"]

[dependencies]
solana-program = "2"
```

### Why `cdylib`?

The `crate-type = ["cdylib", "lib"]` line tells Cargo to produce two outputs:

| Crate type | Purpose |
|-----------|---------|
| `cdylib` | A C-compatible dynamic library (`.so` file). This is what the Solana runtime actually loads and executes. Think of it like compiling Solidity to EVM bytecode — `cdylib` is the "bytecode" output for Solana. |
| `lib` | A standard Rust library. This lets other Rust code (like your tests) import and use your program's types and functions. |

Without `cdylib`, `cargo build-sbf` would have nothing to produce for deployment. Without `lib`, you couldn't write Rust-level integration tests.

### Why `solana-program = "2"`?

Version `"2"` is the latest stable release of the `solana-program` crate. It's the base crate for all native Solana programs and provides the fundamental types (`Pubkey`, `AccountInfo`, `ProgramResult`) and macros (`entrypoint!`, `msg!`) you need.

---

## The Solidity Mental Model

In Solidity, a minimal contract looks like:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract HelloWorld {
    event Hello(string message);

    function hello() public {
        emit Hello("Hello, World!");
    }
}
```

Key things happening implicitly:
- The EVM routes calls to the right function via the **function selector** (first 4 bytes of calldata)
- `msg.sender`, `msg.value`, etc. are available as globals
- State lives inside the contract at its address
- Events are logged via `emit`

In native Solana, the equivalent is:

```rust
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello, World!");
    Ok(())
}
```

Here's the mapping:

| Solidity | Native Solana | Notes |
|----------|--------------|-------|
| `address(this)` | `program_id: &Pubkey` | The program's own address |
| `msg.sender` + other accounts | `accounts: &[AccountInfo]` | ALL accounts are passed explicitly |
| `msg.data` (calldata) | `instruction_data: &[u8]` | Raw bytes, you parse them yourself |
| Function selector routing | You parse `instruction_data[0]` manually | No built-in function dispatch |
| `emit Event(...)` | `msg!("...")` | Logging (appears in transaction logs) |
| Implicit state in storage slots | Accounts with `data` field | State is external to the program |
| `require(...)` / `revert(...)` | Return `Err(ProgramError::...)` | Error handling |

The biggest conceptual shift: **Solana programs are stateless**. All state lives in accounts that are passed *into* the program. The program reads and writes account data but doesn't "own" storage the way a Solidity contract does.

---

## Anatomy of a Native Solana Program

### 1. The Entrypoint

```rust
entrypoint!(process_instruction);
```

The `entrypoint!` macro generates the low-level boilerplate that the Solana runtime expects. It:
- Defines a C-compatible `entrypoint` function (the actual symbol the runtime calls)
- Deserializes raw byte buffers into `Pubkey`, `AccountInfo`, and `instruction_data`
- Calls your `process_instruction` function with those deserialized values
- Converts your `ProgramResult` back into a status code

You never call this function yourself. The Solana runtime calls it when a transaction includes an instruction targeting your program.

### 2. The `process_instruction` Function

```rust
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Your logic here
    Ok(())
}
```

This is your program's "main" function. Every instruction that targets your program enters here. Let's break down each parameter:

#### `program_id: &Pubkey`

The public key (address) of your deployed program. Equivalent to `address(this)` in Solidity. You'll use this to verify that accounts are owned by your program.

```rust
// Solidity:  require(msg.sender == owner);
// Solana:    check that an account's owner matches your program_id
if account.owner != program_id {
    return Err(ProgramError::IncorrectProgramId);
}
```

#### `accounts: &[AccountInfo]`

A slice of all accounts passed to this instruction. This is the most important conceptual difference from Solidity:

- In Solidity, `msg.sender` is implicit. Storage is implicit. Other contract addresses are hardcoded or passed as arguments.
- In Solana, **every** account the instruction touches must be explicitly listed by the client and passed in. The runtime enforces that the accounts match what the client declared.

The order matters — your program identifies accounts by their position in the slice:
```rust
let accounts_iter = &mut accounts.iter();
let payer = next_account_info(accounts_iter)?;      // accounts[0]
let data_account = next_account_info(accounts_iter)?; // accounts[1]
```

#### `instruction_data: &[u8]`

Raw bytes of arbitrary data. This is like `msg.data` in Solidity (the calldata), except there's no built-in ABI encoding. You parse it yourself:

```rust
// Common pattern: first byte is the instruction variant
let instruction = instruction_data[0];
match instruction {
    0 => initialize(program_id, accounts, &instruction_data[1..])?,
    1 => transfer(program_id, accounts, &instruction_data[1..])?,
    _ => return Err(ProgramError::InvalidInstructionData),
}
```

This is what Anchor automates for you — but understanding the raw version is essential.

### 3. `AccountInfo` Fields

The `AccountInfo` struct gives you everything about an account:

| Field | Type | Solidity Equivalent | Description |
|-------|------|-------------------|-------------|
| `key` | `&Pubkey` | `address` | The account's public key (address) |
| `lamports` | `Rc<RefCell<&mut u64>>` | `address.balance` | Balance in lamports (1 SOL = 1B lamports) |
| `data` | `Rc<RefCell<&mut [u8]>>` | Contract storage | Raw byte array of account data |
| `owner` | `&Pubkey` | N/A (implicit) | The program that owns this account |
| `is_signer` | `bool` | Checked via `msg.sender` | Did this account sign the transaction? |
| `is_writable` | `bool` | N/A | Can this instruction modify the account? |
| `executable` | `bool` | `isContract()` | Is this account an executable program? |

Note the `Rc<RefCell<>>` wrappers on `lamports` and `data` — these enable interior mutability, allowing the runtime to enforce borrow rules at runtime rather than compile time.

### 4. Logging with `msg!`

```rust
msg!("Hello, World!");
msg!("Program ID: {}", program_id);
msg!("Account key: {}, signer: {}", account.key, account.is_signer);
```

`msg!` writes to the transaction log. It works like `println!` but writes to Solana's logging system instead of stdout. Logs are visible in:
- `solana logs` (CLI, when connected to a validator)
- Transaction details in explorers
- Test output from `solana-program-test`

In Solidity terms, `msg!` is closest to `emit` events or `console.log` (in Hardhat). However, Solana logs are simpler — just strings, not structured events.

### 5. `ProgramResult` and Error Handling

```rust
pub type ProgramResult = Result<(), ProgramError>;
```

Your function returns either `Ok(())` (success) or `Err(ProgramError::...)` (failure). When you return an error, the entire transaction is rolled back — just like `revert` in Solidity.

Common `ProgramError` variants:

| Variant | When to use |
|---------|------------|
| `InvalidInstructionData` | Malformed or unexpected instruction bytes |
| `InvalidAccountData` | Account data doesn't match expected format |
| `IncorrectProgramId` | An account isn't owned by the expected program |
| `MissingRequiredSignature` | A required signer didn't sign |
| `InsufficientFunds` | Not enough lamports |
| `AccountAlreadyInitialized` | Trying to init an account that already has data |
| `Custom(u32)` | Your own error code (like custom errors in Solidity) |

---

## Building Your Program

### `cargo build-sbf`

```bash
cargo build-sbf
```

This compiles your Rust code into **SBF** (Solana Bytecode Format). SBF is to Solana what EVM bytecode is to Ethereum:

| Concept | Ethereum | Solana |
|---------|----------|--------|
| Source language | Solidity / Vyper | Rust / C |
| Bytecode format | EVM bytecode | SBF (based on eBPF) |
| Build tool | `solc` | `cargo build-sbf` |
| Output | `.bin` / ABI JSON | `.so` (shared object) |

The output lands in `target/deploy/m10_native_hello_world.so`. Note the underscores — Cargo converts hyphens in crate names to underscores.

> **Note:** You need the Solana CLI tools installed for `cargo build-sbf`. Install with:
> ```bash
> sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
> ```

### For local checking (without Solana CLI)

If you just want to verify your code compiles:

```bash
cargo check
```

This type-checks everything without producing the final SBF binary. Useful during development.

---

## Deploying Your Program

### Start a local validator

```bash
solana-test-validator
```

This starts a local Solana cluster (like Hardhat's local node or Ganache).

### Deploy

```bash
solana program deploy target/deploy/m10_native_hello_world.so
```

This uploads your compiled program to the local validator. You'll get back a **Program ID** — the on-chain address of your deployed program.

In Solidity terms, this is like running `forge create` or `hardhat deploy`. The key difference: Solana programs are deployed to a specific account address, and that address is deterministic based on the deployer's keypair.

### Interacting with your program

Unlike Solidity where you call functions by name, interacting with a Solana program requires building a raw `Instruction`:

```rust
let instruction = Instruction {
    program_id: my_program_id,
    accounts: vec![
        AccountMeta::new(payer.pubkey(), true),  // writable + signer
    ],
    data: vec![],  // no instruction data for hello world
};
```

You'd typically do this from a TypeScript client using `@solana/web3.js`, or from a Rust test.

---

## Testing with `solana-program-test`

The `solana-program-test` crate gives you a lightweight local runtime for testing — like Hardhat's built-in network but for Solana:

```rust
#[cfg(test)]
mod tests {
    use solana_program_test::*;
    use solana_sdk::{
        instruction::{AccountMeta, Instruction},
        signer::Signer,
        transaction::Transaction,
    };

    #[tokio::test]
    async fn test_hello_world() {
        let program_id = Pubkey::new_unique();
        let (mut banks_client, payer, recent_blockhash) =
            ProgramTest::new("m10_native_hello_world", program_id, None)
                .start()
                .await;

        let instruction = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(payer.pubkey(), true),
            ],
            data: vec![],
        };

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();
    }
}
```

We'll cover testing in depth in a later module. For now, just know that `solana-program-test` lets you run your program without spinning up a full validator.

---

## Best Practices (The Security Checklist)

These are the Solana equivalents of Solidity's common security checks:

### 1. Always verify account ownership

```rust
// Solidity: Built into the contract model — storage belongs to the contract
// Solana:   You MUST check manually
if account.owner != program_id {
    return Err(ProgramError::IncorrectProgramId);
}
```

If you skip this, an attacker can pass in an account owned by a different program with crafted data. This is like a Solidity contract blindly reading from an arbitrary storage slot at a user-provided address.

### 2. Always validate signers

```rust
// Solidity: require(msg.sender == owner)
// Solana:   check the is_signer flag
if !authority_account.is_signer {
    return Err(ProgramError::MissingRequiredSignature);
}
```

The `is_signer` flag is set by the runtime and cannot be forged. It's the equivalent of checking `msg.sender`.

### 3. Check writable flags

```rust
if !data_account.is_writable {
    return Err(ProgramError::InvalidAccountData);
}
```

Even though the runtime enforces writability at a low level, checking it in your program is defense-in-depth.

### 4. Validate instruction data length

```rust
if instruction_data.len() < 1 {
    return Err(ProgramError::InvalidInstructionData);
}
```

Never assume instruction data has a certain length. Always validate before indexing.

### 5. Use checked arithmetic

```rust
let new_balance = current_balance
    .checked_add(amount)
    .ok_or(ProgramError::InvalidArgument)?;
```

Just like Solidity 0.8+ checks for overflow by default, you should use Rust's `checked_*` methods to prevent arithmetic bugs.

---

## Summary

| What | How |
|------|-----|
| Create project | `cargo new my-program --lib` |
| Set crate type | `crate-type = ["cdylib", "lib"]` in Cargo.toml |
| Add dependency | `solana-program = "2"` |
| Define entrypoint | `entrypoint!(process_instruction)` |
| Log messages | `msg!("Hello!")` |
| Return success | `Ok(())` |
| Return error | `Err(ProgramError::InvalidInstructionData)` |
| Build for Solana | `cargo build-sbf` |
| Deploy | `solana program deploy target/deploy/*.so` |
| Type-check locally | `cargo check` |

You now have the mental model for how every native Solana program works. In the next modules, we'll build on this foundation to work with accounts, state, and cross-program invocations.
