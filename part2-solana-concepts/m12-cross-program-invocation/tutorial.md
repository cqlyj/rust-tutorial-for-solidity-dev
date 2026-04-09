# Module 12: Cross-Program Invocation (CPI)

## Creating This Project

```bash
cargo new m12-cross-program-invocation --lib
```

Then update `Cargo.toml`:

```toml
[package]
name = "m12-cross-program-invocation"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "lib"]

[dependencies]
solana-program = "2"
borsh = "1"
```

The `cdylib` crate type compiles to a shared library that the Solana runtime can load as an on-chain program. The `lib` target lets other Rust crates import your types and helpers.

---

## What Is CPI?

Cross-Program Invocation is one program calling another program's instruction while executing on-chain. A single transaction can touch many programs, but CPI lets a *program* (not just the client) route logic through other programs at runtime.

### Solidity Comparison

In Solidity you call another contract like this:

```solidity
// Direct call — the compiler knows the interface
otherContract.someFunction(arg1, arg2);

// Low-level call — you encode calldata yourself
(bool success, bytes memory data) = address(other).call(
    abi.encodeWithSignature("someFunction(uint256)", 42)
);
```

In both cases the EVM automatically sets `msg.sender` to the calling contract's address. The callee can trust that value.

**Solana is fundamentally different:**

| Concept | Solidity (EVM) | Solana (SVM) |
|---|---|---|
| Calling mechanism | `contract.fn()` or `.call()` | `invoke()` / `invoke_signed()` |
| Caller identity | Automatic `msg.sender` | Accounts + signers passed explicitly |
| Storage model | Callee owns its storage slots | Accounts passed in; ownership checked by runtime |
| `delegatecall` | Yes — execute callee code in caller's storage context | **No equivalent** — programs never share storage context |
| Call depth limit | 1024 | **4** (CPI depth, not counting the top-level call) |

The biggest mental shift: on Solana, the caller must forward every account the callee needs. There is no implicit context.

---

## The Building Blocks

### The `Instruction` Struct

Every CPI starts by building an `Instruction`:

```rust
pub struct Instruction {
    /// The program to invoke.
    pub program_id: Pubkey,
    /// Accounts required by the callee, in order.
    pub accounts: Vec<AccountMeta>,
    /// Serialized instruction data (opcode + args).
    pub data: Vec<u8>,
}
```

This is analogous to encoding calldata in Solidity's low-level `.call()`.

### `AccountMeta`

Each account entry describes permissions:

```rust
pub struct AccountMeta {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}
```

Convenience constructors:

```rust
AccountMeta::new(pubkey, is_signer)           // writable
AccountMeta::new_readonly(pubkey, is_signer)   // read-only
```

This is unique to Solana. In Solidity you never declare which storage slots a call touches — the EVM figures it out at runtime. On Solana, the runtime **requires** you to declare everything up front so it can parallelize transactions.

---

## `invoke()` — Basic CPI

Use `invoke()` when the accounts that need to sign have **already** signed in the original transaction:

```rust
use solana_program::program::invoke;

invoke(
    &instruction,          // The Instruction to execute
    &[account1, account2], // AccountInfo slice — must include every account the callee needs
)?;
```

**Example — Transfer SOL via System Program:**

```rust
use solana_program::system_instruction;

let transfer_ix = system_instruction::transfer(
    from_pubkey,   // source (must be a signer)
    to_pubkey,     // destination
    lamports,      // amount
);

invoke(
    &transfer_ix,
    &[from_account.clone(), to_account.clone()],
)?;
```

The System Program's `transfer` helper builds the `Instruction` for you, including the correct `program_id`, `AccountMeta` entries, and serialized data. This is the preferred pattern — use the target program's crate helpers whenever possible rather than hand-encoding data.

### Solidity Equivalent

```solidity
// Sending ETH to another address
payable(recipient).transfer(amount);
// or
(bool ok, ) = recipient.call{value: amount}("");
```

The Solana version is more explicit: you build an instruction, list the accounts, and call `invoke`.

---

## `invoke_signed()` — CPI With PDA Signers

Program Derived Addresses (PDAs) have no private key, so no external wallet can sign for them. Instead, the program that derived the PDA can prove ownership by providing the seeds:

```rust
use solana_program::program::invoke_signed;

invoke_signed(
    &instruction,
    &[account1, account2],
    &[&[seed1, seed2, &[bump]]],  // signer_seeds: &[&[&[u8]]]
)?;
```

The runtime re-derives the PDA from the seeds and verifies it matches one of the accounts marked `is_signer`. If it matches, the runtime grants signer privilege for that CPI.

**Why this matters:** PDAs let programs own accounts and authorize actions without any private key. This is how token vaults, escrows, and protocol treasuries work on Solana.

### Solidity Equivalent

There is no direct equivalent. In Solidity, a contract *is* its own authority — `msg.sender == address(this)` is always true inside the contract. On Solana, a program doesn't have an "address that can sign." It uses PDAs instead.

---

## Common CPI Targets

### 1. System Program

The System Program manages SOL accounts:

```rust
use solana_program::system_instruction;

// Create a new account
let create_ix = system_instruction::create_account(
    payer_pubkey,       // funds the new account
    new_account_pubkey, // the account to create
    lamports,           // rent-exempt balance
    space as u64,       // data size in bytes
    owner_program_id,   // which program owns the new account
);

// Transfer SOL
let transfer_ix = system_instruction::transfer(
    from_pubkey,
    to_pubkey,
    lamports,
);
```

### 2. SPL Token Program

The SPL Token program manages fungible tokens:

```rust
// These come from the spl-token crate, but you can build them manually too.
// Transfer tokens between token accounts:
let transfer_ix = spl_token::instruction::transfer(
    token_program_id,
    source_token_account,
    destination_token_account,
    authority,         // owner of source account
    &[],              // multisig signers (usually empty)
    amount,
)?;
```

### 3. Associated Token Account (ATA) Program

Creates the canonical token account for a wallet + mint pair:

```rust
let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
    payer,
    wallet,
    mint,
    token_program_id,
);
```

---

## Privilege Escalation

When your program calls `invoke()`, any signer privileges that the **original transaction** granted are automatically forwarded to the callee. This is called privilege escalation:

```
Transaction signer: Alice
  → Your Program (Alice is a signer here)
    → invoke(System Program transfer from Alice)
      → System Program sees Alice as signer ✓
```

Alice signed the transaction, so your program can pass her `AccountInfo` (with `is_signer = true`) into a CPI and the callee trusts it.

For PDAs, `invoke_signed()` grants signer privilege only for the specific PDA whose seeds you provide. The runtime enforces this — you cannot forge a signature for an arbitrary PDA.

---

## CPI Depth Limit

Solana allows a maximum of **4 levels** of CPI nesting:

```
Transaction
  → Program A (depth 0)
    → CPI to Program B (depth 1)
      → CPI to Program C (depth 2)
        → CPI to Program D (depth 3)
          → CPI to Program E (depth 4) ← maximum
            → CPI to Program F ← FAILS
```

In Solidity, the call stack limit is 1024, so you rarely hit it. On Solana the limit is intentionally low to keep execution bounded. Design your architecture so that deep CPI chains are unnecessary.

---

## Return Data

Programs can pass data back through CPI using `set_return_data` and `get_return_data`:

```rust
use solana_program::program::{set_return_data, get_return_data};

// In the callee — set return data after processing
set_return_data(&result_bytes);

// In the caller — read return data after invoke
invoke(&ix, &accounts)?;
if let Some((program_id, data)) = get_return_data() {
    // Verify program_id matches the callee you invoked
    // Deserialize data
}
```

This is like Solidity's `abi.decode(returndata)` after a low-level `.call()`. The important difference: you must verify the `program_id` in the return data to ensure it came from the program you expected.

---

## Security Considerations

### 1. Always Verify the Program ID

Never assume an account passed as "the token program" is actually the token program:

```rust
// BAD — trusts whatever the client passes
invoke(&ix, &accounts)?;

// GOOD — verify before CPI
if token_program.key != &spl_token::id() {
    return Err(ProgramError::IncorrectProgramId);
}
```

In Solidity the compiler resolves contract addresses at deploy time (for direct calls) or you validate them. On Solana, **every** program ID arrives as an account, so you must check it.

### 2. Check Account Ownership Before CPI

Before reading or trusting an account's data, verify who owns it:

```rust
if account.owner != &expected_program_id {
    return Err(ProgramError::IllegalOwner);
}
```

### 3. PDA Signing Prevents Unauthorized Access

A PDA can only be "signed for" by the program that derived it. If your program creates a PDA with specific seeds, no other program can produce a valid signature for that PDA. This is the foundation of Solana's authorization model for program-owned accounts.

### 4. No `delegatecall` — And That's Good

Solidity's `delegatecall` executes another contract's code in the caller's storage context. This enables proxy patterns but has caused catastrophic bugs (Parity wallet hack, etc.). Solana has **no equivalent**. Every program operates only on the accounts it owns. This eliminates an entire class of vulnerabilities.

---

## Building an Instruction Manually

Sometimes you need to build an `Instruction` by hand instead of using a helper crate:

```rust
use solana_program::instruction::{AccountMeta, Instruction};

let instruction = Instruction {
    program_id: target_program_id,
    accounts: vec![
        AccountMeta::new(writable_account, true),   // writable + signer
        AccountMeta::new(writable_account2, false),  // writable, not signer
        AccountMeta::new_readonly(read_only, false), // read-only, not signer
    ],
    data: {
        let mut buf = Vec::new();
        buf.push(instruction_discriminator);  // first byte = which instruction
        buf.extend_from_slice(&amount.to_le_bytes());
        buf
    },
};
```

This is equivalent to manually encoding calldata with `abi.encodePacked(...)` in Solidity.

---

## Best Practices

1. **Use helper crates.** If the target program publishes a Rust crate (like `system_instruction`, `spl_token`), use its builder functions. They handle serialization and `AccountMeta` ordering correctly.

2. **Verify every program ID.** Compare against known constants before CPI. A malicious client could substitute a fake program.

3. **Validate accounts before CPI.** Check ownership, key derivation, and data integrity before forwarding accounts into a CPI.

4. **Minimize CPI depth.** The 4-level limit is strict. Prefer flat architectures where one orchestrator program makes multiple independent CPIs rather than chaining through intermediaries.

5. **Clone `AccountInfo` for CPI.** The `invoke` functions take `&[AccountInfo]` — use `.clone()` on each `AccountInfo` you pass. This is a shallow clone (Rc pointers), not an expensive copy.

6. **Check return data provenance.** After `get_return_data()`, always verify the `program_id` matches the program you invoked.

7. **Handle errors.** CPI can fail for many reasons — insufficient funds, wrong signer, account not writable. Always use `?` to propagate and consider mapping errors to your program's custom error codes.

---

## Summary

| Concept | What It Does |
|---|---|
| `invoke()` | CPI where all required signers already signed the transaction |
| `invoke_signed()` | CPI where a PDA acts as signer (program provides seeds) |
| `Instruction` | Target program + accounts + serialized data |
| `AccountMeta` | Per-account permissions (signer, writable) |
| Privilege escalation | Signer status flows from transaction through CPI chain |
| Depth limit | Max 4 CPI levels |
| Return data | `set_return_data` / `get_return_data` for CPI return values |
| No `delegatecall` | Programs cannot execute in another program's context |

CPI is how Solana programs compose. Where Solidity contracts call each other through interfaces and `msg.sender` propagates automatically, Solana requires you to explicitly declare every account, every permission, and every signer. It is more verbose but eliminates ambiguity — the runtime knows exactly what every instruction touches before it executes.
