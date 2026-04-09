# Module 16: SPL Token Operations with Anchor

## Creating This Project

```bash
anchor init token-ops
cd token-ops
```

Then update `Cargo.toml` in the program crate (`programs/token-ops/Cargo.toml`):

```toml
[package]
name = "m16-token-operations"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "lib"]

[dependencies]
anchor-lang = "0.30"
anchor-spl = "0.30"
```

The `anchor-spl` crate provides Anchor-native wrappers around the SPL Token Program, SPL Associated Token Account Program, and Token-2022 — so you get compile-time account validation for token operations instead of building raw CPIs by hand.

---

## The Mental Model Shift: EVM Tokens vs Solana Tokens

This is the single biggest conceptual difference you will encounter as a Solidity developer learning Solana token development. Read this section carefully — everything else builds on it.

### EVM: Each Token IS a Contract

In Solidity, creating a new token means **deploying a new smart contract** that implements the ERC-20 interface:

```solidity
// Each token is its own deployed contract with its own storage
contract USDC is ERC20 {
    // All balances live INSIDE this contract's storage
    mapping(address => uint256) private _balances;
    // All allowances live INSIDE this contract's storage
    mapping(address => mapping(address => uint256)) private _allowances;
    uint256 private _totalSupply;
    string private _name;
    string private _symbol;
    uint8 private _decimals;
}

// A different token = a completely different contract deployment
contract WETH is ERC20 { ... }
```

When you call `usdc.transfer(bob, 100)`, the EVM:
1. Jumps into the USDC contract's code
2. Modifies the USDC contract's internal storage (`_balances` mapping)
3. Emits a `Transfer` event from that contract

The token's code and its data live **together** in one contract.

### Solana: ONE Program, Many Accounts

On Solana, there is **one single Token Program** (address `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`) that manages **every** SPL token in existence. USDC, SOL-wrapped tokens, NFTs — they all use the same program.

Instead of deploying a new contract per token, you create **accounts** that the shared Token Program operates on:

```
EVM:                              Solana:
┌─────────────────────┐           ┌──────────────────────────────────────┐
│ USDC Contract        │           │ Token Program (ONE shared program)   │
│ ├── name = "USDC"    │           │                                      │
│ ├── decimals = 6     │           │ ┌─────────────────────────────┐      │
│ ├── totalSupply=1B   │           │ │ USDC Mint Account            │      │
│ ├── balances:        │           │ │ ├── decimals = 6             │      │
│ │   alice => 100     │           │ │ ├── supply = 1,000,000,000   │      │
│ │   bob   => 50      │           │ │ ├── mint_authority = <key>   │      │
│ └── allowances:      │           │ │ └── freeze_authority = <key> │      │
│     alice=>bob => 25 │           │ └─────────────────────────────┘      │
└─────────────────────┘           │                                      │
                                  │ ┌─────────────────────────────┐      │
┌─────────────────────┐           │ │ Alice's USDC Token Account   │      │
│ WETH Contract        │           │ │ ├── mint = USDC Mint         │      │
│ ├── name = "WETH"    │           │ │ ├── owner = Alice's wallet   │      │
│ ├── decimals = 18    │           │ │ └── amount = 100             │      │
│ ├── balances: ...    │           │ └─────────────────────────────┘      │
│ └── ...              │           │                                      │
└─────────────────────┘           │ ┌─────────────────────────────┐      │
                                  │ │ Bob's USDC Token Account     │      │
                                  │ │ ├── mint = USDC Mint         │      │
                                  │ │ ├── owner = Bob's wallet     │      │
                                  │ │ └── amount = 50              │      │
                                  │ └─────────────────────────────┘      │
                                  └──────────────────────────────────────┘
```

### The Three Account Types

| Concept | EVM Equivalent | Solana SPL |
|---|---|---|
| **Mint Account** | Deploying an ERC-20 contract | An account holding token metadata: decimals, supply, authorities |
| **Token Account** | `balanceOf[address]` inside the contract | A separate account holding one user's balance for one mint |
| **Associated Token Account (ATA)** | No equivalent (addresses are implicit) | A deterministic token account address derived from (wallet, mint) |

#### Mint Account = Token Definition

A Mint Account is analogous to deploying an ERC-20 contract. It defines:
- **decimals**: how many decimal places (6 for USDC, 9 for most Solana tokens, 0 for NFTs)
- **supply**: total tokens minted so far (like `totalSupply()`)
- **mint_authority**: who can mint new tokens (like an `onlyOwner` modifier on `_mint`)
- **freeze_authority**: who can freeze token accounts (no direct EVM equivalent)

```
Solidity: deploy new ERC20("USDC", "USDC", 6)
Solana:   create a Mint Account with decimals=6, authority=your_wallet
```

#### Token Account = Balance Holder

A Token Account holds one user's balance for one specific mint. Think of it as one entry in the `balanceOf` mapping:

```
Solidity: usdc.balanceOf(alice) → stored inside USDC contract
Solana:   Alice's USDC Token Account → a separate on-chain account
```

Key fields in a Token Account:
- **mint**: which token this account holds (pointer to the Mint Account)
- **owner**: the wallet that controls this balance (can transfer, burn)
- **amount**: the token balance
- **delegate**: an approved spender (like ERC-20 `approve`)
- **delegated_amount**: how much the delegate can spend (like ERC-20 `allowance`)

#### Associated Token Account (ATA) = Deterministic Address

In EVM, you don't think about "where" a balance is stored — `balanceOf[alice]` just works. On Solana, each balance needs its own account, which means someone has to create it and pay rent.

The Associated Token Account program solves this by deriving a **deterministic address** for each (wallet, mint) pair:

```
ATA address = PDA(wallet_address, TOKEN_PROGRAM_ID, mint_address)
```

This means:
- Given a wallet and a mint, there is exactly **one** canonical token account address
- Anyone can derive it off-chain without an RPC call
- The first transfer to a new user can create the ATA on the fly

```
Solidity: usdc.balanceOf(alice)   // alice's balance "just exists"
Solana:   getAssociatedTokenAddress(alice, usdc_mint)  // deterministic PDA
```

---

## SPL Token Program Operations

Let's map every common ERC-20 operation to its SPL Token equivalent:

### Operation Comparison Table

| ERC-20 (Solidity) | SPL Token (Solana) | Notes |
|---|---|---|
| `constructor()` — deploy contract | `InitializeMint` | Creates a Mint Account |
| `balanceOf(addr)` | Read Token Account's `amount` field | Token Account is a separate account |
| `transfer(to, amount)` | `Transfer` / `TransferChecked` | Must pass source, destination, and mint accounts |
| `approve(spender, amount)` | `Approve` | Sets delegate on the Token Account |
| `transferFrom(from, to, amount)` | `Transfer` (signed by delegate) | Delegate signs instead of owner |
| `_mint(to, amount)` | `MintTo` / `MintToChecked` | Mint authority must sign |
| `_burn(amount)` | `Burn` / `BurnChecked` | Token owner or delegate signs |
| `totalSupply()` | Read Mint Account's `supply` field | |
| `decimals()` | Read Mint Account's `decimals` field | |
| No equivalent | `FreezeAccount` / `ThawAccount` | Freeze authority can lock a token account |
| `renounceOwnership()` (for minting) | `SetAuthority` to `None` | Permanently disables minting |

### Checked vs Unchecked

SPL Token has both `Transfer` and `TransferChecked` (and `MintTo` / `MintToChecked`, etc.). The "checked" variants require you to pass the **expected decimals** and verify them against the mint — preventing you from accidentally sending 1,000,000 when you meant 1.000000 (6 decimals). **Always use the checked variants in production.** Anchor's helpers use the checked versions by default.

---

## Anchor SPL Helpers

The `anchor-spl` crate provides:

### Account Types

```rust
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
```

- `Mint` — deserializes and validates a Mint Account
- `TokenAccount` — deserializes and validates a Token Account
- `Token` — the Token Program itself (used as `Program<'info, Token>`)
- `AssociatedToken` — the ATA Program (used as `Program<'info, AssociatedToken>`)

### CPI Helpers

Instead of building raw `Instruction` structs, Anchor gives you typed CPI wrappers:

```rust
use anchor_spl::token;

// Mint tokens (like ERC-20 _mint)
token::mint_to(cpi_ctx, amount)?;

// Transfer tokens (like ERC-20 transfer)
token::transfer(cpi_ctx, amount)?;

// Burn tokens (like ERC-20 burn)
token::burn(cpi_ctx, amount)?;

// Approve delegate (like ERC-20 approve)
token::approve(cpi_ctx, amount)?;

// Revoke delegate (like ERC-20 approve(spender, 0))
token::revoke(cpi_ctx)?;
```

Each helper takes a `CpiContext` that bundles the accounts needed for that operation. This is Anchor's version of building the cross-program invocation — you get type safety and the framework handles the serialization.

### Account Constraints for Tokens

Anchor provides special constraints for token accounts in your `#[derive(Accounts)]` structs:

```rust
#[account(
    init,                          // Create this account
    payer = payer,                 // Who pays the rent
    mint::decimals = 6,            // Set decimals (like constructor arg)
    mint::authority = authority,    // Who can mint (like onlyOwner)
    mint::freeze_authority = authority,  // Who can freeze
)]
pub mint: Account<'info, Mint>,

#[account(
    init,
    payer = payer,
    associated_token::mint = mint,       // Which token this ATA holds
    associated_token::authority = owner,  // Whose ATA this is
)]
pub token_account: Account<'info, TokenAccount>,
```

These constraints:
- Automatically call the Token Program to initialize the mint or token account
- Validate that the accounts are correctly configured on subsequent calls
- Replace manual CPI for account initialization

---

## Building the Token Operations Program

Let's walk through each instruction in our program. The full code is in `src/lib.rs`, but here's the logic explained:

### 1. Create Token (Initialize Mint)

**Solidity equivalent**: Deploying a new ERC-20 contract.

```rust
pub fn create_token(ctx: Context<CreateToken>, decimals: u8) -> Result<()>
```

This creates a new Mint Account. Anchor's `init` constraint handles the CPI to the Token Program's `InitializeMint` instruction. We specify:
- `decimals`: precision of the token (6 for stablecoins, 9 for most tokens, 0 for NFTs)
- `mint_authority`: who can mint new tokens (the `authority` signer)
- `freeze_authority`: who can freeze accounts (also set to `authority`)

After this instruction executes, a new token "type" exists on Solana — analogous to a freshly deployed ERC-20 contract.

### 2. Mint Tokens

**Solidity equivalent**: `_mint(to, amount)` — creating new tokens.

```rust
pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()>
```

This calls the Token Program's `MintTo` instruction via CPI:
- The mint authority must sign (like `onlyOwner` on a Solidity mint function)
- Tokens are minted directly into a Token Account (the recipient's ATA)
- The mint's `supply` field increases

### 3. Transfer Tokens

**Solidity equivalent**: `transfer(to, amount)`.

```rust
pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()>
```

This calls the Token Program's `Transfer` instruction via CPI:
- The token owner must sign (they authorize the transfer)
- Tokens move from source Token Account to destination Token Account
- Both accounts must be for the **same mint** (you can't accidentally mix tokens)

### 4. Burn Tokens

**Solidity equivalent**: `burn(amount)`.

```rust
pub fn burn_tokens(ctx: Context<BurnTokens>, amount: u64) -> Result<()>
```

This calls the Token Program's `Burn` instruction via CPI:
- The token owner must sign
- Tokens are destroyed from the specified Token Account
- The mint's `supply` field decreases

---

## CPI Context Building

Every token operation needs a `CpiContext`. This bundles the accounts required by the Token Program for a specific operation. Think of it as building the calldata for a Solidity low-level `.call()`:

```rust
// Solidity equivalent:
// usdc.transfer(to, amount)  ← the EVM auto-routes to USDC contract

// Solana equivalent: build a CPI context explicitly
let cpi_accounts = Transfer {
    from: ctx.accounts.from_ata.to_account_info(),
    to: ctx.accounts.to_ata.to_account_info(),
    authority: ctx.accounts.owner.to_account_info(),
};
let cpi_program = ctx.accounts.token_program.to_account_info();
let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
token::transfer(cpi_ctx, amount)?;
```

If your program is the authority (via a PDA), you use `CpiContext::new_with_signer` and pass the PDA seeds — just like `invoke_signed` from Module 12:

```rust
let seeds = &[b"vault", &[ctx.bumps.vault_authority]];
let signer_seeds = &[&seeds[..]];
let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
token::transfer(cpi_ctx, amount)?;
```

---

## Token Extensions (Token-2022)

The original SPL Token Program is frozen — it will never receive new features. Instead, Solana introduced **Token-2022** (also called Token Extensions), a new version of the token program at address `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb`.

Token-2022 is backwards-compatible with the original program but adds **extensions** that each mint can opt into:

| Extension | Description | EVM Analogy |
|---|---|---|
| **Transfer Fee** | Automatic fee on every transfer | Fee-on-transfer tokens (like SafeMoon) |
| **Transfer Hook** | Custom program called on every transfer | ERC-777 hooks / ERC-1363 callbacks |
| **Metadata** | On-chain token name, symbol, URI | Built into ERC-20 (`name()`, `symbol()`) |
| **Confidential Transfer** | Encrypted balances using zero-knowledge proofs | Tornado Cash-style privacy, but native |
| **Permanent Delegate** | An authority that can transfer/burn any holder's tokens | God-mode admin (centralized stablecoins) |
| **Non-Transferable** | Soulbound tokens | Soulbound NFTs (ERC-5192) |
| **Interest-Bearing** | Display balance increases over time | Rebasing tokens (like stETH) |
| **Default Account State** | New token accounts start frozen | Compliance-first tokens |
| **CPI Guard** | Prevents certain CPI-based attacks | No direct EVM equivalent |

In Anchor, you use `anchor_spl::token_2022` and `anchor_spl::token_interface` to work with Token-2022:

```rust
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
```

The `TokenInterface` type accepts **both** the original Token Program and Token-2022, making your program compatible with all SPL tokens.

---

## Metaplex Token Metadata (Brief Overview)

SPL Token Mints are minimal — they only store decimals, supply, and authorities. There is no built-in field for a token's name, symbol, or image URI.

The **Metaplex Token Metadata Program** fills this gap by creating a metadata PDA for each mint:

```
Metadata PDA = PDA("metadata", METAPLEX_PROGRAM_ID, mint_address)
```

This metadata account stores:
- `name`: "USD Coin"
- `symbol`: "USDC"
- `uri`: points to a JSON file with image, description, etc.
- `seller_fee_basis_points`: royalties (for NFTs)
- `creators`: list of creator addresses and shares

### How NFTs Work on Solana

An NFT is simply an SPL token with:
- `decimals = 0` (indivisible)
- `supply = 1` (unique)
- `mint_authority = None` (no more can ever be minted)
- A Metaplex Metadata account with name, image, attributes

This is fundamentally different from ERC-721, where each token ID exists within a single contract. On Solana, each NFT is its own Mint Account with its own metadata.

```
EVM (ERC-721):                    Solana (SPL + Metaplex):
┌─────────────────────┐           Each NFT is its own Mint:
│ BoredApe Contract    │           ├── Mint #1 (supply=1, decimals=0)
│ ├── ownerOf[1]=alice │           │   ├── Metadata PDA → {name, image, ...}
│ ├── ownerOf[2]=bob   │           │   └── Alice's Token Account (amount=1)
│ └── ownerOf[3]=carol │           ├── Mint #2 (supply=1, decimals=0)
└─────────────────────┘           │   ├── Metadata PDA → {name, image, ...}
                                  │   └── Bob's Token Account (amount=1)
                                  └── Mint #3 ...
```

### Collection NFTs

To group NFTs into a collection (like all Bored Apes), Metaplex uses a **Collection NFT** — a special mint whose metadata marks it as a collection. Individual NFTs reference this collection mint in their metadata. This replaces the implicit grouping of "all token IDs in one ERC-721 contract."

---

## Best Practices

### 1. Always Use ATAs

Don't create random token accounts. Use Associated Token Accounts so that:
- Addresses are deterministic and discoverable
- Wallets and block explorers can find a user's tokens automatically
- You avoid orphaned token accounts

### 2. Use Checked Transfers

Always use the "checked" variants (`TransferChecked`, `MintToChecked`, `BurnChecked`) which verify decimals. Anchor's CPI helpers do this by default.

### 3. Validate Mint and Authority

In your account constraints, always specify:
```rust
#[account(
    mut,
    token::mint = mint,              // Verify this token account is for the right mint
    token::authority = authority,     // Verify the signer is the owner
)]
pub token_account: Account<'info, TokenAccount>,
```

Without these, an attacker could pass in a token account for a different mint or one they don't own.

### 4. Use PDAs as Authorities for Program-Owned Vaults

If your program needs to hold tokens (like an escrow or vault), make a PDA the token account's authority. Then use `CpiContext::new_with_signer` to sign transfers out.

### 5. Handle Account Creation Gracefully

When transferring to a user for the first time, their ATA might not exist. Use `init_if_needed` (with caution — it requires enabling the `init-if-needed` feature in Anchor) or have the client create the ATA before calling your instruction.

### 6. Consider Token-2022 Compatibility

If you want your program to work with both classic SPL tokens and Token-2022 tokens, use `token_interface` types instead of `token` types:

```rust
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
```

---

## Key Takeaways

1. **One program, many accounts**: Solana's Token Program is shared. You create Mint Accounts (token types) and Token Accounts (balances), not new contracts.

2. **Everything is explicit**: Unlike Solidity where the EVM routes calls to the right contract and storage is implicit, Solana requires you to pass every account explicitly. This makes programs more composable but more verbose.

3. **ATAs are your friend**: Associated Token Accounts give you deterministic, discoverable addresses — the closest thing to Solidity's implicit `balanceOf` mapping.

4. **Anchor abstracts the CPIs**: Instead of manually building `Instruction` structs and calling `invoke()`, Anchor's `token::mint_to`, `token::transfer`, and `token::burn` helpers give you type-safe CPI wrappers.

5. **Token-2022 is the future**: New tokens should consider Token-2022 extensions for features like transfer fees, hooks, and on-chain metadata.

6. **NFTs are just tokens**: On Solana, an NFT is a Mint with supply=1, decimals=0, and Metaplex metadata. No separate standard like ERC-721 — just a convention on top of SPL Token.
