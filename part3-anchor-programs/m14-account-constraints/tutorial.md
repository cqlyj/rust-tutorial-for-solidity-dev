# Module 14: Deep Dive into Anchor Account Constraints

## Overview

In Solidity, you validate inputs with imperative `require()` statements scattered throughout your function bodies. In Anchor, validation is **declarative** — you specify constraints directly on the accounts struct and Anchor generates all the runtime checks automatically. This is one of Anchor's most powerful features and the primary reason Solana programs written with Anchor are less error-prone than raw Solana programs.

```solidity
// Solidity: imperative validation
function transfer(address to, uint amount) public {
    require(msg.sender == owner, "Not owner");     // Anchor: has_one = authority
    require(amount > 0, "Zero amount");             // Anchor: constraint = ...
    require(balances[msg.sender] >= amount, "NSF"); // Anchor: constraint = ...
    require(to != address(0), "Zero address");      // Anchor: address = ...
    // ... actual logic
}
```

```rust
// Anchor: declarative validation
#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(
        mut,
        has_one = authority,                          // owner check
        constraint = vault.amount > 0 @ MyError::ZeroAmount,  // balance check
    )]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,                     // signer check is automatic
}
```

The Anchor approach has three major advantages:

1. **Checks can't be forgotten** — they're part of the struct definition, not buried in logic
2. **Checks run before your instruction body** — no partially-executed state on failure
3. **Anchor generates optimized code** — the constraint macros expand to efficient Rust

---

## Creating This Project

```bash
# If you have Anchor CLI installed:
anchor init account-constraints
cd account-constraints

# The key files to modify:
# - programs/account-constraints/src/lib.rs  (your program)
# - programs/account-constraints/Cargo.toml  (dependencies)
# - tests/account-constraints.ts             (TypeScript tests)

# For this tutorial module, we work with a standalone lib crate.
# See src/lib.rs for the complete program.
```

---

## Account Types

Every field in a `#[derive(Accounts)]` struct must have a type that implements the `Accounts` trait. Anchor provides several built-in types, each with different levels of validation.

### `Account<'info, T>` — Deserialized, Type-Checked Account

The workhorse type. Anchor automatically:
- Checks the account is owned by the expected program
- Deserializes the data into type `T`
- Verifies the 8-byte discriminator matches type `T`

```rust
#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub balance: u64,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    // Anchor verifies:
    // 1. vault is owned by this program
    // 2. First 8 bytes match Vault's discriminator
    // 3. Data deserializes into Vault
    #[account(mut)]
    pub vault: Account<'info, Vault>,
}
```

**Solidity parallel:** This is like having a typed storage pointer — you know the data shape is correct.

### `Signer<'info>` — Must Be a Transaction Signer

Verifies that the account signed the transaction. Does NOT deserialize any data.

```rust
#[derive(Accounts)]
pub struct Withdraw<'info> {
    // Anchor checks: this account's key is in the transaction's signers list
    pub authority: Signer<'info>,
}
```

**Solidity parallel:** Like `msg.sender` — you know the caller authenticated.

### `SystemAccount<'info>` — Any Account Owned by System Program

An account that is owned by the System Program. Useful for receiving SOL (wallet accounts).

```rust
#[derive(Accounts)]
pub struct CloseVault<'info> {
    // No deserialization; just verifies owner == System Program
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
}
```

### `UncheckedAccount<'info>` — No Checks (Dangerous!)

Performs zero validation. You MUST add a `/// CHECK:` doc comment explaining why it's safe, or Anchor will refuse to compile.

```rust
#[derive(Accounts)]
pub struct RawInstruction<'info> {
    /// CHECK: This account is validated manually in the instruction body.
    pub unchecked: UncheckedAccount<'info>,
}
```

**When to use:** Almost never. Only when you need an account that doesn't fit any other type and you're performing manual validation.

### `Program<'info, T>` — Verified Program Account

Ensures the account is a specific program (checks the program ID matches).

```rust
use anchor_lang::system_program::System;

#[derive(Accounts)]
pub struct CreateVault<'info> {
    // Anchor verifies: this account's key == system_program::ID
    pub system_program: Program<'info, System>,
}
```

**Solidity parallel:** Like verifying a contract address before calling it.

### `AccountInfo<'info>` — Raw Account

The lowest-level account type. Provides raw access but no automatic validation. Like `UncheckedAccount`, requires a `/// CHECK:` comment.

```rust
#[derive(Accounts)]
pub struct RawAccess<'info> {
    /// CHECK: Validated in instruction logic via key comparison.
    pub raw_account: AccountInfo<'info>,
}
```

---

## Init Constraints — Creating Accounts

### `init` — Create and Initialize a New Account

The `init` constraint tells Anchor to create a brand-new account. It always requires `payer` and `space`.

```rust
#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(
        init,                          // Create this account
        payer = authority,             // authority pays the rent
        space = 8 + 32 + 8,           // discriminator + Pubkey + u64
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]                    // payer must be mutable (lamports deducted)
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,  // required for account creation
}
```

Under the hood, `init` does:
1. Calls `system_program::create_account` with the specified space and lamports for rent exemption
2. Sets the new account's owner to the current program
3. Writes the 8-byte discriminator for type `T`
4. Serializes the default/initial data

**Solidity parallel:** Like deploying a new contract that acts as a storage slot.

### `payer = <account>` — Who Pays

Specifies which signer account pays the rent-exempt lamports. That account must be `mut` and a `Signer`.

### `space = <expr>` — Account Size in Bytes

The total number of bytes to allocate. This includes the 8-byte discriminator.

```rust
// Formula: 8 (discriminator) + sum of field sizes
space = 8 + 32 + 8  // discriminator + Pubkey + u64
```

### `init_if_needed` — Idempotent Creation

Creates the account only if it doesn't already exist. If it exists, skips creation and just deserializes.

```rust
#[derive(Accounts)]
pub struct InitIfNeeded<'info> {
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 8,
        seeds = [b"vault", user.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

> **Warning:** `init_if_needed` requires the `init-if-needed` Cargo feature flag. Use with caution — it can mask bugs where you expect an account to be fresh but it already exists with stale data.

Add to Cargo.toml:
```toml
[dependencies]
anchor-lang = { version = "0.30", features = ["init-if-needed"] }
```

---

## PDA Constraints — Program Derived Addresses

PDAs are Solana's answer to deterministic storage addresses. In Solidity, storage slots are implicit (the EVM maps `mapping(key => value)` to a slot). In Solana, you derive PDA addresses from seeds so anyone can compute the address off-chain.

### `seeds = [...]` — PDA Seed Values

```rust
#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Vault::INIT_SPACE,
        seeds = [b"vault", authority.key().as_ref()],  // deterministic address
        bump,                                           // auto-find bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

Seeds are byte slices combined to derive a unique address. Common seed patterns:
- Static string: `b"vault"` — namespace
- Pubkey: `user.key().as_ref()` — per-user uniqueness
- Integer: `&id.to_le_bytes()` — per-item uniqueness

### `bump` — PDA Bump Seed

When used with `init`, Anchor finds the canonical bump automatically. When used without `init`, you can either:

1. Let Anchor re-derive: `bump` (slightly more compute)
2. Supply a stored bump: `bump = vault.bump` (cheaper)

```rust
// On creation — Anchor finds and stores the bump
#[account(
    init,
    seeds = [b"vault", authority.key().as_ref()],
    bump,                        // Anchor finds canonical bump
    payer = authority,
    space = 8 + Vault::INIT_SPACE,
)]
pub vault: Account<'info, Vault>,

// On subsequent access — use stored bump for efficiency
#[account(
    seeds = [b"vault", authority.key().as_ref()],
    bump = vault.bump,           // Use stored bump (saves compute)
)]
pub vault: Account<'info, Vault>,
```

**Best practice:** Store the bump in your account struct and use it on subsequent accesses to save compute units.

### `seeds::program = <pubkey>` — Cross-Program PDA

Derive a PDA owned by a different program:

```rust
#[account(
    seeds = [b"metadata", token_mint.key().as_ref()],
    bump,
    seeds::program = metadata_program.key(),  // PDA of the metadata program
)]
pub metadata: AccountInfo<'info>,
```

---

## Relationship Constraints — Connecting Accounts

### `has_one = <field>` — Field Must Match Account Key

This is Anchor's most important security constraint. It verifies that a field inside the account's data matches another account's key in the struct.

```rust
#[account]
pub struct Vault {
    pub authority: Pubkey,   // stored authority
    pub balance: u64,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        has_one = authority,  // vault.authority == authority.key()
    )]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,  // name must match the field name!
}
```

**Critical detail:** The account field name in the struct must exactly match the field name in the data account. `has_one = authority` checks `vault.authority == authority.key()`.

**Solidity parallel:** Like `require(msg.sender == owner)` but enforced structurally.

### `constraint = <expr>` — Arbitrary Boolean Check

The most flexible constraint. Any boolean expression that references accounts in the struct.

```rust
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        has_one = authority,
        constraint = vault.balance >= amount @ VaultError::InsufficientFunds,
    )]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
}

// In your instruction:
pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> { ... }
```

The `@ ErrorVariant` syntax attaches a custom error. Without it, a generic `ConstraintViolated` error is returned.

### `address = <pubkey>` — Must Be Specific Address

Verifies the account has an exact public key.

```rust
use anchor_lang::solana_program::sysvar;

#[derive(Accounts)]
pub struct SpecificAccount<'info> {
    #[account(address = sysvar::clock::ID)]
    pub clock: AccountInfo<'info>,
}
```

---

## Mutation Constraints

### `mut` — Mark Account as Mutable

Any account whose data or lamports change must be marked `mut`. This corresponds to the `is_writable` flag in the transaction.

```rust
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]                // data will change (balance updated)
    pub vault: Account<'info, Vault>,
    #[account(mut)]                // lamports will change (SOL transferred)
    pub depositor: Signer<'info>,
}
```

**Solidity parallel:** Everything in Solidity that touches storage is implicitly mutable. Solana requires you to be explicit.

### `close = <account>` — Close Account, Reclaim Rent

Closes the account by:
1. Transferring all lamports to the specified account
2. Zeroing the account data
3. Setting the account's owner to the System Program

```rust
#[derive(Accounts)]
pub struct CloseVault<'info> {
    #[account(
        mut,
        has_one = authority,
        close = authority,          // send lamports to authority
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
}
```

**Solidity parallel:** Like `selfdestruct(payable(owner))` — destroys the account and refunds the rent.

### `realloc` — Resize Account Data

Dynamically resize an account's data region:

```rust
#[derive(Accounts)]
pub struct Resize<'info> {
    #[account(
        mut,
        realloc = 8 + 32 + 4 + new_len,  // new total size
        realloc::payer = authority,        // who pays if growing
        realloc::zero = true,             // zero-init new bytes
    )]
    pub data_account: Account<'info, DynamicData>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

---

## Token Constraints (Anchor SPL)

These constraints require the `anchor-spl` crate. They provide declarative validation for SPL Token accounts.

### Token Account Constraints

```rust
use anchor_spl::token::{Token, TokenAccount, Mint};

#[derive(Accounts)]
pub struct TokenTransfer<'info> {
    #[account(
        mut,
        token::mint = mint,              // token account's mint must match
        token::authority = authority,     // token account's authority must match
    )]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
```

### Mint Constraints

```rust
#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(
        init,
        payer = authority,
        mint::decimals = 6,              // 6 decimal places (like USDC)
        mint::authority = authority,      // who can mint
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
```

### Associated Token Account Constraints

```rust
use anchor_spl::associated_token::AssociatedToken;

#[derive(Accounts)]
pub struct CreateATA<'info> {
    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,        // ATA for this mint
        associated_token::authority = authority, // ATA owned by this authority
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
```

---

## Space Calculation Reference

Every account on Solana has a fixed size determined at creation. The first 8 bytes are always the Anchor discriminator (a hash of the account type name). You must calculate the remaining space based on your fields.

### Type Size Reference

| Type | Size (bytes) | Notes |
|------|-------------|-------|
| `bool` | 1 | |
| `u8` / `i8` | 1 | |
| `u16` / `i16` | 2 | |
| `u32` / `i32` | 4 | |
| `u64` / `i64` | 8 | |
| `u128` / `i128` | 16 | |
| `f32` | 4 | |
| `f64` | 8 | |
| `Pubkey` | 32 | |
| `String` | 4 + len | 4-byte length prefix + UTF-8 bytes |
| `Vec<T>` | 4 + len × size(T) | 4-byte length prefix + elements |
| `Option<T>` | 1 + size(T) | 1-byte tag (0=None, 1=Some) + T |
| `[T; N]` | N × size(T) | Fixed-size array, no length prefix |
| Enum | 1 + largest variant | 1-byte discriminant + data |

### Using `InitSpace` Derive Macro

Instead of manual calculation, use `#[derive(InitSpace)]`:

```rust
#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub authority: Pubkey,   // 32
    pub balance: u64,        // 8
    pub bump: u8,            // 1
    #[max_len(50)]
    pub name: String,        // 4 + 50 = 54
}

// In the accounts struct:
#[account(
    init,
    payer = authority,
    space = 8 + Vault::INIT_SPACE,  // 8 + 95 = 103
)]
pub vault: Account<'info, Vault>,
```

For `String` and `Vec`, you must annotate with `#[max_len(N)]` to specify the maximum length.

### Manual Calculation Example

```rust
#[account]
pub struct GameState {
    pub player: Pubkey,       // 32 bytes
    pub score: u64,           // 8 bytes
    pub level: u16,           // 2 bytes
    pub is_active: bool,      // 1 byte
    pub name: String,         // 4 + 32 = 36 bytes (max 32 chars)
    pub items: Vec<u64>,      // 4 + 10 * 8 = 84 bytes (max 10 items)
    pub badge: Option<Pubkey>,// 1 + 32 = 33 bytes
    pub bump: u8,             // 1 byte
}

// Total: 8 (discriminator) + 32 + 8 + 2 + 1 + 36 + 84 + 33 + 1 = 205 bytes
```

---

## Custom Errors

Define custom errors with the `#[error_code]` attribute:

```rust
#[error_code]
pub enum VaultError {
    #[msg("Insufficient funds in vault")]
    InsufficientFunds,
    #[msg("Deposit amount must be greater than zero")]
    ZeroDeposit,
    #[msg("Vault name too long (max 50 characters)")]
    NameTooLong,
    #[msg("Unauthorized: signer is not the vault authority")]
    Unauthorized,
}
```

Use errors with the `@` operator in constraints:

```rust
#[account(
    constraint = vault.balance >= amount @ VaultError::InsufficientFunds,
)]
pub vault: Account<'info, Vault>,
```

Or use `require!()` in instruction bodies for imperative checks:

```rust
pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require!(amount > 0, VaultError::ZeroDeposit);
    // ...
    Ok(())
}
```

---

## Best Practices

### 1. Prefer Declarative Constraints Over Manual Checks

```rust
// GOOD — checked before instruction runs
#[account(
    mut,
    has_one = authority @ VaultError::Unauthorized,
)]
pub vault: Account<'info, Vault>,

// BAD — easy to forget, checked during instruction
pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
    require!(ctx.accounts.vault.authority == ctx.accounts.authority.key(), VaultError::Unauthorized);
    // ...
}
```

### 2. Always Use `has_one` for Ownership Validation

Every account that has an "owner" or "authority" field should be validated with `has_one`. This is the number one source of security bugs when missing.

### 3. Always Add Custom Error Messages

```rust
// GOOD — clear error message for debugging
#[account(
    constraint = vault.balance > 0 @ VaultError::InsufficientFunds,
)]

// BAD — generic "ConstraintViolated" error, hard to debug
#[account(
    constraint = vault.balance > 0,
)]
```

### 4. Store and Reuse PDA Bumps

```rust
#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub bump: u8,  // store bump on init, reuse on subsequent accesses
}
```

### 5. Use `InitSpace` Derive for Space Calculation

Avoids manual calculation errors. Always prefer `#[derive(InitSpace)]` over hand-counting bytes.

### 6. Minimize `UncheckedAccount` Usage

Every `UncheckedAccount` is a potential security vulnerability. If you must use one, document exactly why and what manual checks you perform.

---

## Constraint Quick Reference

| Constraint | Purpose | Example |
|-----------|---------|---------|
| `init` | Create new account | `#[account(init, payer = user, space = 100)]` |
| `init_if_needed` | Create if missing | `#[account(init_if_needed, ...)]` |
| `mut` | Mark writable | `#[account(mut)]` |
| `has_one = x` | Field matches key | `#[account(has_one = authority)]` |
| `constraint = expr` | Boolean check | `#[account(constraint = x > 0)]` |
| `address = key` | Exact address | `#[account(address = MY_KEY)]` |
| `seeds = [..]` | PDA seeds | `#[account(seeds = [b"tag"], bump)]` |
| `bump` | PDA bump | `#[account(seeds = [...], bump)]` |
| `close = acct` | Close + refund | `#[account(close = authority)]` |
| `realloc = n` | Resize data | `#[account(realloc = 200, ...)]` |
| `token::mint` | Token mint check | `#[account(token::mint = mint)]` |
| `token::authority` | Token auth check | `#[account(token::authority = auth)]` |
| `@ Error` | Custom error | `#[account(constraint = x @ Err::Bad)]` |

---

## Summary

| Solidity Pattern | Anchor Equivalent |
|-----------------|-------------------|
| `require(msg.sender == owner)` | `has_one = authority` + `Signer` |
| `require(amount > 0)` | `constraint = amount > 0` |
| `require(addr == KNOWN)` | `address = KNOWN_PUBKEY` |
| Implicit storage mutability | Explicit `mut` constraint |
| `selfdestruct(owner)` | `close = authority` |
| `new Contract()` | `init` + `payer` + `space` |
| `mapping(key => val)` | `seeds = [key]` (PDA) |
| Custom `revert` messages | `#[error_code]` + `@ ErrorVariant` |

Anchor's constraint system turns Solana's account model from a footgun into a well-guarded, type-safe framework. Master these constraints and you'll write programs that are secure by construction.
