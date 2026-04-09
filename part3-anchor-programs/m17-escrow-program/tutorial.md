# Module 17: Escrow Program (Capstone)

> **The capstone project.** This module brings together everything from Modules 1–16: Rust ownership, structs, enums, error handling, traits, Anchor macros, account constraints, PDAs, and SPL token CPIs. You're building a real, production-style trustless escrow — the kind of program that handles real money on mainnet.

## What You're Building

A **trustless token escrow** — the Solana equivalent of an atomic swap. Two parties exchange different SPL tokens without trusting each other or any intermediary:

1. **Maker** creates an escrow: "I'll give 100 USDC for 2 SOL-wrapped tokens"
2. **Taker** fulfills it: sends 2 SOL-wrapped tokens to the maker, receives 100 USDC from the vault
3. Or the **Maker cancels**: reclaims their 100 USDC

No middleman. No trust. Just math and PDAs.

---

## Solidity Comparison

If you've built an escrow in Solidity, it probably looked something like this:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract Escrow {
    address public maker;
    address public taker;
    IERC20 public tokenA;      // Token the maker deposits
    IERC20 public tokenB;      // Token the maker wants
    uint256 public amountA;    // Amount of tokenA deposited
    uint256 public amountB;    // Amount of tokenB wanted

    bool public isActive;

    constructor(
        address _tokenA,
        address _tokenB,
        uint256 _amountA,
        uint256 _amountB
    ) {
        maker = msg.sender;
        tokenA = IERC20(_tokenA);
        tokenB = IERC20(_tokenB);
        amountA = _amountA;
        amountB = _amountB;
    }

    // Maker deposits tokenA into the contract
    function deposit() external {
        require(msg.sender == maker, "Only maker");
        tokenA.transferFrom(maker, address(this), amountA);
        isActive = true;
    }

    // Taker completes the exchange
    function exchange() external {
        require(isActive, "Not active");
        taker = msg.sender;

        // Taker sends tokenB to maker
        tokenB.transferFrom(taker, maker, amountB);

        // Contract sends tokenA to taker
        tokenA.transfer(taker, amountA);

        isActive = false;
    }

    // Maker cancels and reclaims tokens
    function cancel() external {
        require(msg.sender == maker, "Only maker");
        require(isActive, "Not active");

        tokenA.transfer(maker, amountA);
        isActive = false;
    }
}
```

### Key Differences on Solana

| Concept | Solidity/EVM | Solana/Anchor |
|---------|-------------|---------------|
| **Escrow storage** | Contract state variables | PDA account with serialized struct |
| **Token custody** | Contract holds ERC-20 balance (`address(this)`) | Vault token account owned by escrow PDA |
| **Identity** | `msg.sender` | `Signer<'info>` account |
| **Token transfer** | `token.transferFrom(from, to, amount)` | CPI to SPL Token program's `transfer_checked` |
| **Authorization** | Contract is implicitly authorized | PDA signs via `seeds` + `bump` |
| **Cancellation** | `token.transfer(maker, amount)` | CPI transfer from vault back to maker, then close vault |
| **Atomicity** | Transaction reverts on any `require` failure | Transaction reverts on any error (all-or-nothing) |
| **Reentrancy** | Must use `ReentrancyGuard` or checks-effects-interactions | Not possible — Solana runtime prevents it |
| **Account creation** | Implicit (contract storage slots) | Explicit (`init` constraint allocates + pays rent) |

The biggest mental shift: **on Solana, the escrow PDA doesn't "hold" tokens in a balance mapping — it _owns_ a separate token account (the vault) that actually holds the tokens.** The PDA's authority over the vault is what makes it trustless.

---

## How to Create This Project from Scratch

```bash
# 1. Initialize a new Anchor project
anchor init escrow
cd escrow

# 2. The generated structure:
# escrow/
# ├── Anchor.toml
# ├── Cargo.toml
# ├── programs/
# │   └── escrow/
# │       ├── Cargo.toml      ← Add anchor-spl here
# │       └── src/
# │           └── lib.rs       ← Your escrow program
# ├── tests/
# │   └── escrow.ts            ← TypeScript integration tests
# └── migrations/
#     └── deploy.ts

# 3. Add anchor-spl dependency (for SPL Token CPIs)
cd programs/escrow
# Edit Cargo.toml to add:
#   anchor-spl = "0.30"

# 4. Build
anchor build

# 5. Get your program ID
anchor keys list
# Update declare_id!() in lib.rs and Anchor.toml with the output

# 6. Test
anchor test
```

> **Note**: In this tutorial module, we work with standalone `lib.rs` files and simplified `Cargo.toml` files to focus on the program logic. In a real Anchor project, you'd use the full project structure above.

---

## Architecture Deep Dive

### Account Layout

```
┌─────────────────────────────────────────────────────────────────┐
│                        ESCROW SYSTEM                            │
│                                                                 │
│  ┌──────────────┐         ┌──────────────────────────┐          │
│  │    Maker      │         │   Escrow PDA Account     │          │
│  │  (wallet)     │────────▶│  seeds: [b"escrow",      │          │
│  │              │         │         maker.key,        │          │
│  │              │         │         seed (u64)]       │          │
│  └──────┬───────┘         │                          │          │
│         │                 │  Fields:                  │          │
│         │                 │  - maker: Pubkey          │          │
│         │                 │  - mint_a: Pubkey         │          │
│         │                 │  - mint_b: Pubkey         │          │
│         │                 │  - amount_offered: u64    │          │
│         │                 │  - amount_wanted: u64     │          │
│         │                 │  - seed: u64              │          │
│         │                 │  - bump: u8               │          │
│         │                 └──────────┬───────────────┘          │
│         │                            │                           │
│         │                            │ authority (PDA signs)     │
│         │                            ▼                           │
│         │                 ┌──────────────────────────┐          │
│         │  deposit ──────▶│     Vault Token Account   │          │
│         │  (Token A)      │  (holds escrowed Token A) │          │
│         │                 │  owner = escrow PDA       │          │
│         │                 │  mint = mint_a            │          │
│         │                 └──────────────────────────┘          │
│                                                                 │
│  ┌──────────────┐                                               │
│  │    Taker      │  Sends Token B to maker's ATA               │
│  │  (wallet)     │  Receives Token A from vault                 │
│  └──────────────┘                                               │
└─────────────────────────────────────────────────────────────────┘
```

### Account Relationships

1. **Escrow State PDA**: Derived from `["escrow", maker_pubkey, seed]`. Stores the deal terms. The `seed` allows one maker to have multiple concurrent escrows.

2. **Vault Token Account**: A standard SPL Token account whose **authority** is the escrow PDA. This is the key insight — the vault is a normal token account, but only the PDA can sign transfers from it.

3. **Maker's Token Accounts**: The maker's ATAs (Associated Token Accounts) for both Token A and Token B.

4. **Taker's Token Accounts**: The taker's ATAs for both Token A and Token B.

### Why a Vault Instead of Storing in the PDA?

In Solidity, a contract can hold ERC-20 balances directly. On Solana, **programs can't hold token balances** — only token accounts can. So we create a dedicated token account (the vault) and set the escrow PDA as its authority. The PDA "controls" the vault the way a contract controls its `balanceOf`.

---

## Instruction Walkthrough

### 1. `make` — Create Escrow and Deposit

**What happens:**
1. Create the escrow PDA account (stores deal terms)
2. Create the vault token account (owned by escrow PDA)
3. Transfer Token A from maker's ATA → vault

**Accounts needed:**
- `maker` (signer, mutable) — pays for account creation
- `mint_a` — the token being offered
- `mint_b` — the token being requested
- `maker_ata_a` — maker's Token A account (source of deposit)
- `escrow` — the PDA account to create (stores state)
- `vault` — the token account to create (holds escrowed tokens)
- System program, Token program, Associated Token program

**Key design decisions:**
- The vault is initialized with `authority = escrow PDA`, not the maker. This is what makes it trustless — the maker can't withdraw without going through the program.
- We use `associated_token::authority = escrow` so the vault is the escrow PDA's ATA for mint_a. This means the vault address is deterministic.
- The `seed` field lets a maker create multiple escrows. Without it, each maker could only have one active escrow.

### 2. `take` — Complete the Swap

**What happens (atomically):**
1. Transfer Token B from taker's ATA → maker's ATA (taker pays maker)
2. Transfer Token A from vault → taker's ATA (vault releases to taker)
3. Close the vault account (reclaim rent to maker)
4. Close the escrow PDA account (reclaim rent to maker)

**Why this order matters:**
- We transfer the taker's tokens first. If the taker doesn't have enough Token B, the transaction fails before any vault tokens move. (In practice Solana's atomicity handles this, but it's good defensive ordering.)
- Closing accounts last ensures all transfers complete before cleanup.

**The PDA signing pattern:**
```rust
// The escrow PDA must sign the transfer FROM the vault
let signer_seeds = &[
    b"escrow",
    maker_key.as_ref(),
    &seed.to_le_bytes(),
    &[bump],
];
// This is like the contract calling token.transfer() in Solidity —
// but instead of implicit authorization, the PDA explicitly signs.
```

### 3. `cancel` — Maker Reclaims Tokens

**What happens:**
1. Transfer Token A from vault → maker's ATA
2. Close the vault account (rent → maker)
3. Close the escrow PDA account (rent → maker)

**Security:**
- Only the maker can cancel (enforced by the `maker` signer constraint + `has_one = maker` on the escrow account)
- The escrow PDA signs the vault transfer using the same seed pattern

---

## Security Considerations

### 1. Mint Validation
The escrow stores `mint_a` and `mint_b`. When a taker calls `take`, we verify:
- The vault's mint matches `mint_a` (enforced by `associated_token::mint = mint_a`)
- The taker is sending the correct `mint_b` (enforced by `token::mint = mint_b` on the taker's source ATA)
- The maker's receiving ATA matches `mint_b`

Without these checks, a taker could send worthless tokens.

### 2. Amount Verification
The `take` instruction transfers exactly `amount_wanted` from taker to maker and `amount_offered` from vault to taker. These amounts are stored in the escrow state at creation time and can't be modified.

### 3. Authority Checks
- `has_one = maker` ensures only the real maker can cancel
- The vault authority is the escrow PDA — nobody else can drain it
- Signer constraints ensure only wallets that actually sign can be maker/taker

### 4. Reentrancy
In Solidity, reentrancy is a major concern with token transfers. On Solana, the runtime prevents reentrancy: a program can't be called recursively within the same transaction. This is enforced at the VM level — one less thing to worry about.

### 5. Edge Cases
- **Zero amounts**: Validate `amount_offered > 0` and `amount_wanted > 0` in the `make` instruction
- **Self-trading**: A maker could technically be their own taker. This is harmless (they just get their own tokens back minus fees) but you could add a check if desired
- **Duplicate escrows**: The `seed` parameter prevents collisions. Two escrows with the same maker and seed would have the same PDA, and the second `init` would fail

---

## The PDA Authority Pattern (Recap from Module 14)

This is the most important pattern in Solana token programs. Let's make sure it's crystal clear:

```
In Solidity:
  contract.transfer(token, to, amount)
  // The contract IS the caller, so it's authorized implicitly

In Solana:
  // 1. Create a PDA (deterministic address from seeds)
  // 2. Set PDA as authority of a token account
  // 3. When you need to transfer, provide PDA seeds as "signer seeds"
  // 4. The runtime verifies: sha256(seeds) == PDA address
  // 5. Transfer proceeds as if the PDA "signed" the transaction
```

The PDA never has a private key. It can never sign a real transaction. But within a CPI (Cross-Program Invocation), the program can assert "I am this PDA" by providing the seeds that derive it. The runtime checks the math and authorizes the transfer.

This is how the escrow vault works:
- Vault authority = escrow PDA
- Only the escrow program knows the seeds
- Only the escrow program can authorize transfers from the vault

---

## Testing Strategy

Here's how you'd test this escrow in an Anchor test suite (TypeScript):

### Setup Phase
```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  createMint, mintTo, getAccount, getAssociatedTokenAddress
} from "@solana/spl-token";

// 1. Create two mints (Token A and Token B)
const mintA = await createMint(connection, payer, payer.publicKey, null, 6);
const mintB = await createMint(connection, payer, payer.publicKey, null, 6);

// 2. Create maker and taker wallets
const maker = anchor.web3.Keypair.generate();
const taker = anchor.web3.Keypair.generate();

// 3. Fund both with SOL (for transaction fees)
await airdrop(maker.publicKey, 2 * LAMPORTS_PER_SOL);
await airdrop(taker.publicKey, 2 * LAMPORTS_PER_SOL);

// 4. Create ATAs and mint tokens
//    Maker gets 1000 Token A
//    Taker gets 1000 Token B
const makerAtaA = await createATA(connection, maker, mintA);
await mintTo(connection, payer, mintA, makerAtaA, payer, 1000_000000);

const takerAtaB = await createATA(connection, taker, mintB);
await mintTo(connection, payer, mintB, takerAtaB, payer, 1000_000000);
```

### Test: Make Escrow
```typescript
it("Maker creates escrow", async () => {
  const seed = new anchor.BN(1);
  const amountOffered = new anchor.BN(100_000000);  // 100 Token A
  const amountWanted = new anchor.BN(50_000000);     // 50 Token B

  await program.methods
    .make(seed, amountOffered, amountWanted)
    .accounts({
      maker: maker.publicKey,
      mintA: mintA,
      mintB: mintB,
      makerAtaA: makerAtaA,
      escrow: escrowPda,
      vault: vaultAddress,
    })
    .signers([maker])
    .rpc();

  // Verify escrow state
  const escrowAccount = await program.account.escrow.fetch(escrowPda);
  assert.equal(escrowAccount.maker.toBase58(), maker.publicKey.toBase58());
  assert.equal(escrowAccount.amountOffered.toNumber(), 100_000000);
  assert.equal(escrowAccount.amountWanted.toNumber(), 50_000000);

  // Verify vault received tokens
  const vaultAccount = await getAccount(connection, vaultAddress);
  assert.equal(Number(vaultAccount.amount), 100_000000);
});
```

### Test: Take Escrow
```typescript
it("Taker completes the swap", async () => {
  // Taker needs an ATA for Token A (to receive)
  const takerAtaA = await createATA(connection, taker, mintA);
  // Maker needs an ATA for Token B (to receive)
  const makerAtaB = await createATA(connection, maker, mintB);

  await program.methods
    .take()
    .accounts({
      taker: taker.publicKey,
      maker: maker.publicKey,
      mintA: mintA,
      mintB: mintB,
      takerAtaA: takerAtaA,
      takerAtaB: takerAtaB,
      makerAtaB: makerAtaB,
      escrow: escrowPda,
      vault: vaultAddress,
    })
    .signers([taker])
    .rpc();

  // Verify: taker got Token A
  const takerA = await getAccount(connection, takerAtaA);
  assert.equal(Number(takerA.amount), 100_000000);

  // Verify: maker got Token B
  const makerB = await getAccount(connection, makerAtaB);
  assert.equal(Number(makerB.amount), 50_000000);

  // Verify: escrow account closed
  try {
    await program.account.escrow.fetch(escrowPda);
    assert.fail("Escrow should be closed");
  } catch (e) {
    // Expected: account doesn't exist
  }
});
```

### Test: Cancel Escrow
```typescript
it("Maker cancels escrow and reclaims tokens", async () => {
  // First, create another escrow (seed = 2)
  // ... (make instruction) ...

  const makerBalanceBefore = await getAccount(connection, makerAtaA);

  await program.methods
    .cancel()
    .accounts({
      maker: maker.publicKey,
      mintA: mintA,
      makerAtaA: makerAtaA,
      escrow: escrowPda2,
      vault: vaultAddress2,
    })
    .signers([maker])
    .rpc();

  // Verify: maker got tokens back
  const makerBalanceAfter = await getAccount(connection, makerAtaA);
  assert.equal(
    Number(makerBalanceAfter.amount) - Number(makerBalanceBefore.amount),
    100_000000
  );
});
```

### Edge Case Tests
```typescript
it("Non-maker cannot cancel", async () => {
  // Should fail with constraint error
  try {
    await program.methods.cancel()
      .accounts({ maker: taker.publicKey, /* ... */ })
      .signers([taker])
      .rpc();
    assert.fail("Should have failed");
  } catch (e) {
    assert.include(e.message, "ConstraintHasOne");
  }
});

it("Cannot create escrow with zero amount", async () => {
  try {
    await program.methods.make(seed, new BN(0), new BN(50))
      .accounts({ /* ... */ })
      .signers([maker])
      .rpc();
    assert.fail("Should have failed");
  } catch (e) {
    assert.include(e.message, "InvalidAmount");
  }
});
```

---

## Best Practices Summary

1. **Use PDA authorities for vaults** — Never use a regular keypair as vault authority. PDAs are deterministic and controlled by your program.

2. **Validate all mint addresses** — Use `token::mint = expected_mint` and `associated_token::mint` constraints. Never trust the client to send the right mint.

3. **Store the bump** — Save the PDA bump in the escrow account to avoid recomputing it on every instruction. This saves compute units.

4. **Use `close` constraints** — When an escrow is taken or cancelled, close the accounts to reclaim rent. This is good hygiene and returns SOL to the maker.

5. **Handle edge cases** — Validate non-zero amounts, check for overflows on multiplication, and consider what happens if mints have different decimal places.

6. **Use Associated Token Accounts** — ATAs are deterministic (derived from owner + mint), so you don't need to pass random token accounts. This prevents "wrong account" attacks.

7. **Emit events** — In production, emit Anchor events for indexers to track escrow creation, completion, and cancellation.

---

## What You've Learned (Across All 17 Modules)

This capstone combined:

| Module | Concept Used Here |
|--------|-------------------|
| **M1-M4** (Rust basics) | Ownership, structs, enums, pattern matching |
| **M5** (Error handling) | Custom error enum with `#[error_code]` |
| **M6** (Traits) | `#[account]` derives Anchor traits automatically |
| **M7** (Collections) | Vectors aren't used here, but the account model is Solana's "collection" |
| **M8** (Modules) | Program organized into logical instruction modules |
| **M9** (Generics) | Anchor's `Account<'info, T>` is a generic type |
| **M10-M12** (Solana basics) | Accounts, transactions, PDAs |
| **M13** (Anchor intro) | `#[program]`, `#[derive(Accounts)]`, `declare_id!` |
| **M14** (Constraints) | `init`, `seeds`, `bump`, `has_one`, `close`, `constraint` |
| **M15** (CRUD) | Create, Read, Update, Delete pattern for escrow lifecycle |
| **M16** (Token ops) | `transfer_checked` CPI, mint validation, token accounts |

**You now have the tools to build real Solana programs.** The escrow pattern is the foundation for DEXes, lending protocols, NFT marketplaces, and more. Every DeFi protocol on Solana uses some variation of this PDA-vault-CPI pattern.

---

## Exercises

After studying the code in `src/lib.rs`, try the exercises in the `exercises/` directory:

1. **Complete the `take` instruction** — Given the account struct, implement the instruction logic
2. **Add a deadline** — Escrows expire after a Unix timestamp
3. **Add partial fills** — Taker can fill a portion of the escrow
4. **Add an `update` instruction** — Maker can change the desired amount

Solutions are in the `solutions/` directory. Every line is commented.
