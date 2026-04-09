# Module 11: Program Derived Addresses (PDAs)

## Overview

Program Derived Addresses (PDAs) are one of the most important concepts in Solana development. They give programs the ability to deterministically derive addresses and "sign" transactions for those addresses — without needing a private key. If you've worked with Solidity's `CREATE2`, you already have intuition for what PDAs do, but they're more flexible and more fundamental to Solana's architecture.

---

## Creating This Project From Scratch

```bash
cargo new m11-program-derived-addresses --lib
```

Then update your `Cargo.toml`:

```toml
[package]
name = "m11-program-derived-addresses"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "lib"]

[dependencies]
solana-program = "2"
borsh = "1"
```

The `cdylib` crate type produces a dynamically-linked library that the Solana runtime can load as an on-chain program. The `lib` target lets you also use it as a normal Rust library for testing.

---

## Solidity Comparison: CREATE2 vs PDAs

In Solidity, you can compute a deterministic address with `CREATE2`:

```solidity
// Solidity: deterministic address via CREATE2
address predicted = address(uint160(uint256(keccak256(
    abi.encodePacked(bytes1(0xff), deployer, salt, keccak256(bytecode))
))));
```

This gives you an address computed from a deployer, a salt, and contract bytecode. The resulting address is a normal Ethereum address — it could theoretically have a private key (though the chance is negligible).

In Solana, PDAs serve a similar purpose but with a critical difference:

```rust
// Solana: deterministic address via PDA
let (pda, bump) = Pubkey::find_program_address(
    &[b"user-data", user_pubkey.as_ref()],
    &program_id,
);
```

**Key differences from CREATE2:**

| Feature | Solidity CREATE2 | Solana PDA |
|---------|-----------------|------------|
| Derived from | deployer + salt + bytecode hash | seeds + program_id + bump |
| Has private key? | Technically possible (normal address) | **No** — guaranteed off the Ed25519 curve |
| Who can "sign"? | Whoever has the private key | **Only the owning program** via `invoke_signed` |
| Use case | Deterministic contract deployment | Deterministic account addresses + program authority |
| Reusable? | Address is burned after deployment | PDA can hold mutable data, reused freely |

The "no private key" guarantee is what makes PDAs special. Because the address is mathematically proven to not be on the Ed25519 curve, no external wallet can ever sign for it. Only the program that derived it can authorize operations on that account via Cross-Program Invocation (CPI) with `invoke_signed`.

---

## What Is a PDA?

A PDA is an address (a 32-byte public key) that:

1. **Is derived deterministically** from a set of seeds and a program ID
2. **Does NOT lie on the Ed25519 elliptic curve** — meaning no private key exists for it
3. **Can only be "signed for"** by the program that owns it, using `invoke_signed`

Think of it like this: a PDA is a locker that only your program has the key to. Anyone can look up the locker number (derive the address), but only your program can open it (sign transactions for it).

### Why PDAs Exist

In Solana's account model, programs need to:

- **Own accounts** — store data in accounts that the program controls
- **Sign for accounts** — authorize system-level operations (creating accounts, transferring SOL) on behalf of accounts the program manages
- **Create deterministic addresses** — so clients can find accounts without on-chain lookups

PDAs solve all three. They're THE way to create structured, deterministic data storage on Solana.

---

## How PDA Derivation Works

Under the hood, PDA derivation uses SHA-256 hashing:

```
hash_input = seeds[0] + seeds[1] + ... + seeds[n] + program_id + "ProgramDerivedAddress"
candidate  = SHA256(hash_input)
```

The result is a 32-byte value. We then check: **is this point on the Ed25519 curve?**

- If **yes** — this address could have a private key, so it's NOT a valid PDA. Try again with a different bump.
- If **no** — this address is guaranteed to have no private key. It's a valid PDA.

The **bump seed** is a single byte (0–255) appended to the seeds before hashing. It's the mechanism that nudges the hash output off-curve.

### The Canonical Bump

`find_program_address` tries bump values starting at **255** and counting down to **0**. It returns the **first** bump that produces a valid off-curve address. This is called the **canonical bump**.

```
Try bump = 255: hash(seeds + [255] + program_id) → on curve? → skip
Try bump = 254: hash(seeds + [254] + program_id) → off curve? → FOUND IT!
Return (address, 254)
```

The canonical bump is always the **highest** valid bump for a given set of seeds. This matters for security — always use the canonical bump to prevent attackers from using alternate bump values.

---

## Core Functions

### `Pubkey::find_program_address`

Finds the PDA and its canonical bump. This is what you'll use most of the time.

```rust
use solana_program::pubkey::Pubkey;

// Derive a PDA for a user's profile
let (pda, bump) = Pubkey::find_program_address(
    &[b"user-profile", user_pubkey.as_ref()],  // seeds
    &program_id,                                 // the program that "owns" this PDA
);
// pda  = the derived address (Pubkey)
// bump = the canonical bump seed (u8)
```

**Cost**: This function iterates through bump values, calling `create_program_address` internally. On-chain, this costs compute units. That's why we store the bump after first derivation.

### `Pubkey::create_program_address`

Creates a PDA from seeds that **already include the bump**. Cheaper than `find_program_address` because it doesn't search — it tries exactly one bump.

```rust
// If you already know the bump is 254:
let pda = Pubkey::create_program_address(
    &[b"user-profile", user_pubkey.as_ref(), &[254]],  // seeds WITH bump
    &program_id,
)?;
```

This returns `Result<Pubkey, PubkeyError>` — it will error if the resulting address IS on the curve (meaning 254 wasn't a valid bump for these seeds).

### When to Use Which

| Function | Use When | Cost |
|----------|----------|------|
| `find_program_address` | First time deriving, or when you don't know the bump | Higher (iterates bumps) |
| `create_program_address` | You have the bump stored (e.g., in account data) | Lower (single hash) |

---

## Seeds: The Building Blocks

Seeds are arbitrary byte slices that determine the PDA address. You can combine different types:

```rust
// String literal seeds (most common for prefixes)
let seeds: &[&[u8]] = &[b"my-prefix"];

// Pubkey as seed (for user-specific accounts)
let seeds: &[&[u8]] = &[b"user-data", user_pubkey.as_ref()];

// Multiple pubkeys (for relationship accounts)
let seeds: &[&[u8]] = &[b"balance", mint.as_ref(), owner.as_ref()];

// Integer seeds (convert to bytes)
let id: u64 = 42;
let seeds: &[&[u8]] = &[b"item", &id.to_le_bytes()];

// Combining everything
let seeds: &[&[u8]] = &[
    b"escrow",
    seller.as_ref(),
    buyer.as_ref(),
    &price.to_le_bytes(),
];
```

### Seed Constraints

- Each individual seed: max **32 bytes**
- Total seeds (including bump): max **16 seeds**
- Seeds are order-dependent: `[b"a", b"b"]` ≠ `[b"b", b"a"]`

---

## Common PDA Patterns

### Pattern 1: Global Singleton State

One account for the entire program (like a Solidity contract's storage):

```rust
// Only one of these can exist per program
let (config_pda, bump) = Pubkey::find_program_address(
    &[b"global-config"],
    &program_id,
);
```

Solidity equivalent: contract state variables that live at a single address.

### Pattern 2: User-Specific Accounts

One account per user (like `mapping(address => UserData)` in Solidity):

```rust
// Each user gets their own PDA
let (user_pda, bump) = Pubkey::find_program_address(
    &[b"user-profile", user_pubkey.as_ref()],
    &program_id,
);
```

Solidity equivalent:
```solidity
mapping(address => UserProfile) public profiles;
```

### Pattern 3: Relationship / Mapping Accounts

Accounts keyed by multiple values (like nested mappings):

```rust
// Balance of a specific token for a specific user
let (balance_pda, bump) = Pubkey::find_program_address(
    &[b"balance", mint_pubkey.as_ref(), owner_pubkey.as_ref()],
    &program_id,
);
```

Solidity equivalent:
```solidity
mapping(address => mapping(address => uint256)) public balances;
// balances[mint][owner]
```

### Pattern 4: Sequential Items

Accounts with an incrementing ID:

```rust
let item_id: u64 = 7;
let (item_pda, bump) = Pubkey::find_program_address(
    &[b"item", &item_id.to_le_bytes()],
    &program_id,
);
```

Solidity equivalent:
```solidity
mapping(uint256 => Item) public items;
// items[7]
```

### Pattern 5: PDA as Authority

A PDA that acts as the signer/authority for other resources (like token vaults):

```rust
// The program's vault authority — controls a token account
let (vault_authority, bump) = Pubkey::find_program_address(
    &[b"vault-authority"],
    &program_id,
);
```

---

## PDAs as Signers: `invoke_signed`

The most powerful feature of PDAs is that programs can sign for them in CPIs. This is how programs create accounts, transfer tokens, and perform other privileged operations.

```rust
use solana_program::program::invoke_signed;

// Create an account at the PDA address
invoke_signed(
    &system_instruction::create_account(
        payer.key,          // who pays for the account
        pda_account.key,    // the PDA address to create
        lamports,           // rent-exempt minimum
        space as u64,       // data size
        program_id,         // owner of the new account
    ),
    &[
        payer.clone(),
        pda_account.clone(),
        system_program.clone(),
    ],
    // The "signer seeds" — this is how the program proves it derived this PDA
    &[&[b"user-profile", user_pubkey.as_ref(), &[bump]]],
)?;
```

The signer seeds in `invoke_signed` must **exactly match** the seeds used to derive the PDA (including the bump). The Solana runtime verifies: "if I hash these seeds with this program ID, do I get the account address being signed for?" If yes, the signature is valid.

### Solidity Comparison

In Solidity, contracts can call other contracts freely because `msg.sender` is always the calling contract. In Solana, programs need explicit authorization via `invoke_signed` because there's no implicit `msg.sender` for program-owned accounts.

```solidity
// Solidity: contract just calls, msg.sender is implicit
token.transfer(recipient, amount);

// Solana: program must provide signer seeds to prove PDA ownership
invoke_signed(&transfer_ix, &accounts, &[&signer_seeds])?;
```

---

## PDA vs Keypair Accounts

| Aspect | PDA Account | Keypair Account |
|--------|-------------|-----------------|
| Address source | Derived from seeds + program_id | Random keypair generation |
| Private key? | **No** | Yes |
| Who can sign? | Only the owning program | Whoever holds the private key |
| Deterministic? | **Yes** — same seeds = same address | No — random each time |
| Client lookup | Derive from known seeds | Must store/share the pubkey |
| Use case | Program-controlled state & authority | User wallets, temporary accounts |

**Rule of thumb**: If a program needs to control it or clients need to find it by convention, use a PDA. If a user needs to directly sign for it, use a keypair.

---

## Storing the Bump: Saving Compute Units

`find_program_address` iterates through bump values, which costs compute units. On-chain, you should compute the bump once and store it in the account data:

```rust
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct UserProfile {
    pub name: String,
    pub bump: u8,  // Store the canonical bump!
}
```

Then on subsequent calls, use `create_program_address` with the stored bump:

```rust
// First time: expensive — searches for canonical bump
let (pda, bump) = Pubkey::find_program_address(&seeds, &program_id);

// Subsequent times: cheap — uses stored bump directly
let pda = Pubkey::create_program_address(
    &[b"user-profile", user.as_ref(), &[stored_bump]],
    &program_id,
)?;
```

---

## Best Practices

1. **Always use the canonical bump** — `find_program_address` returns it. Never accept a user-provided bump without verifying it matches the canonical one.

2. **Use descriptive seed prefixes** — `b"user-profile"` is better than `b"u"`. This prevents seed collisions between different PDA types in the same program.

3. **Store bumps in account data** — Saves compute units on every subsequent access. The bump never changes for a given set of seeds.

4. **Validate PDA addresses on-chain** — When an account is passed to your program, re-derive the PDA and confirm the passed account matches. This prevents attackers from passing fake accounts.

5. **Keep seeds stable** — Changing the seed structure is a breaking change. Clients that derive PDAs will get wrong addresses. Version your seed prefixes if you need to migrate: `b"user-profile-v2"`.

6. **Mind the seed limits** — Max 16 seeds, max 32 bytes each. Plan your seed structure accordingly.

7. **Seeds are public** — Anyone can derive a PDA if they know the seeds. Don't rely on seed secrecy for access control. Use program logic for authorization.

---

## Summary

| Concept | Key Point |
|---------|-----------|
| PDA | Address derived from seeds + program_id, guaranteed off-curve (no private key) |
| `find_program_address` | Finds PDA + canonical bump (tries 255→0) |
| `create_program_address` | Creates PDA with explicit bump (cheaper) |
| Canonical bump | Highest valid bump — always use this one |
| Seeds | Byte slices: strings, pubkeys, integers — max 16, max 32 bytes each |
| `invoke_signed` | How programs sign for PDAs in CPIs |
| Store the bump | Save it in account data to avoid recomputation |
| Solidity parallel | Like CREATE2 addresses but with guaranteed no-private-key property |

PDAs are the backbone of Solana program architecture. Every meaningful program uses them for deterministic account addressing and program-controlled authority. Master PDAs and you've mastered a core building block of Solana development.
