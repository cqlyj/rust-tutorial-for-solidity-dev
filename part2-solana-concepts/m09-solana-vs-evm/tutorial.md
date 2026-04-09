# Module 09: Solana vs EVM — The Mental Model Shift

This is the most important conceptual module in this course. Everything you know about
Solidity and the EVM is useful — but Solana's architecture is fundamentally different.
This module maps every EVM concept you rely on to its Solana equivalent, explains *why*
things are different, and prepares you to think in Solana's model natively.

No code to compile here. Just diagrams, comparisons, and the deepest architectural
walkthrough you'll find anywhere.

---

## 1. The Fundamental Difference: Programs vs Contracts

### What You Know (EVM)

In Solidity, a contract bundles code and storage into one deployable unit:

```solidity
contract Token {
    mapping(address => uint256) public balances;
    string public name;

    function transfer(address to, uint256 amount) external {
        balances[msg.sender] -= amount;
        balances[to] += amount;
    }
}
```

When you deploy this, the EVM creates an account at some address that holds:
- The compiled bytecode (the `transfer` logic)
- The storage trie (the `balances` mapping, the `name` string)

Code and data live together. The contract IS the state.

### What Solana Does Instead

Solana separates code from data completely.

- A **Program** holds only executable code. It is stateless. It stores no balances,
  no mappings, no user data. Think of it as a deployed binary.
- An **Account** holds only data. It is a raw byte buffer owned by a program.
  It stores balances, metadata, configuration — whatever the program needs.

When you "call" a Solana program, you pass it a list of accounts. The program reads
from them, writes to them, and returns. It never stores anything internally.

### The Object vs Function Analogy

```
EVM Model (Object-Oriented):
┌──────────────────────────────┐
│        Token Contract        │
│                              │
│  Storage:                    │
│    balances[alice] = 100     │
│    balances[bob]   = 50      │
│    name = "MyToken"          │
│                              │
│  Code:                       │
│    transfer(to, amount)      │
│    balanceOf(addr)           │
└──────────────────────────────┘

  You call: Token.transfer(bob, 10)
  The contract reads and writes its own storage.


Solana Model (Functional):
┌──────────────────┐     ┌──────────────────┐     ┌──────────────────┐
│  Token Program   │     │  Alice's Account │     │  Bob's Account   │
│  (code only)     │     │  (data only)     │     │  (data only)     │
│                  │     │  balance: 100    │     │  balance: 50     │
│  transfer()      │     │  owner: TokenProg│     │  owner: TokenProg│
│  balance_of()    │     └──────────────────┘     └──────────────────┘
└──────────────────┘
                                │                         │
                    ┌───────────┴─────────────────────────┘
                    ▼
  You call: TokenProgram.transfer(
      accounts: [alice_account, bob_account],
      data: { amount: 10 }
  )
  The program receives accounts, modifies them, returns.
```

Why does this matter? Three reasons:

1. **Parallel execution.** Because all state is in accounts and you declare every
   account upfront, the runtime knows which transactions touch which data. Two
   transactions that touch different accounts can run in parallel. This is how Solana
   achieves high throughput.

2. **Composability without re-deployment.** One program can serve millions of accounts.
   The SPL Token program manages every single token on Solana — thousands of tokens,
   millions of wallets — with one deployed binary.

3. **Explicit state.** There's no hidden state. Every piece of data a program touches
   must be passed in. This makes programs auditable and predictable.

### Quick Reference

| Concept | EVM | Solana |
|---------|-----|--------|
| Code + state | Combined in contract | Separated: Program + Accounts |
| Deploy creates | Contract account (code + storage) | Program account (code only) |
| State lives in | Contract's storage trie | Separate account data fields |
| Mental model | Object with methods | Pure function with parameters |

---

## 2. Account Model Deep Dive

### Everything Is an Account

On Solana, every piece of persistent data is stored in an **account**. Your wallet?
Account. A token balance? Account. A program's executable code? Also an account.
Configuration data for a DeFi protocol? Account.

There is no "storage trie" like EVM. There are no "storage slots." There are just
accounts — each one a flat structure with a handful of fields.

### Account Structure

Every Solana account has exactly these fields:

```
┌─────────────────────────────────────────────────────────────────┐
│                        Solana Account                           │
├─────────────────┬───────────────────────────────────────────────┤
│ lamports        │ u64 — balance in lamports (1 SOL = 1e9)      │
├─────────────────┼───────────────────────────────────────────────┤
│ data            │ Vec<u8> — arbitrary byte array (0 to 10 MB)   │
├─────────────────┼───────────────────────────────────────────────┤
│ owner           │ Pubkey — the program that controls this acct  │
├─────────────────┼───────────────────────────────────────────────┤
│ executable      │ bool — is this account a program?             │
├─────────────────┼───────────────────────────────────────────────┤
│ rent_epoch      │ u64 — epoch when rent was last collected      │
└─────────────────┴───────────────────────────────────────────────┘
```

Let's unpack each:

**lamports** — The SOL balance. Every account holds some SOL. Even data accounts need
SOL to exist (more on rent later). 1 SOL = 1,000,000,000 lamports.

**data** — A raw byte array. This is where your program stores its state. For a token
account, this might contain `{ mint: Pubkey, owner: Pubkey, amount: u64 }` serialized
as bytes. The runtime doesn't interpret this data at all — your program does.

**owner** — The most important field for security. Only the owning program can modify
the `data` field. If Alice's token account is owned by the Token Program, only the
Token Program can change her balance. Not Alice, not some other program, only the
owner. (Anyone can *credit* lamports to an account, but only the owner can *debit*
lamports or modify data.)

**executable** — If true, this account contains a BPF program that can be invoked.
Regular data accounts have this set to false.

**rent_epoch** — Internal bookkeeping for the rent system. You rarely interact with
this directly.

### The Owner Field: Solana's Permission System

The `owner` field is the heart of Solana's security model. In Solidity terms, think
of it like this:

```
EVM:
  - A contract controls its own storage. Nobody else can write to it.
  - Permissions are enforced by code inside the contract (require(msg.sender == ...)).

Solana:
  - An account's owner PROGRAM is the only entity that can write to its data.
  - This is enforced by the RUNTIME, not by your code.
  - Your code still needs to verify the right people signed the transaction.
```

The flow:

```
  Transaction: "Transfer 10 tokens from Alice to Bob"
                    │
                    ▼
  ┌─────────────────────────────────┐
  │         Solana Runtime          │
  │                                 │
  │  Checks:                        │
  │   ✓ alice_account.owner ==      │
  │     token_program_id            │
  │   ✓ bob_account.owner ==        │
  │     token_program_id            │
  │   ✓ Alice signed the tx         │
  │                                 │
  │  Then hands control to:         │
  └────────────┬────────────────────┘
               │
               ▼
  ┌─────────────────────────────────┐
  │        Token Program            │
  │                                 │
  │  Checks:                        │
  │   ✓ alice_account has enough    │
  │   ✓ accounts are valid token    │
  │     accounts for the right mint │
  │                                 │
  │  Modifies:                      │
  │   alice_account.data.amount -= 10│
  │   bob_account.data.amount += 10 │
  └─────────────────────────────────┘
```

The runtime ensures the Token Program can only modify accounts it owns. The Token
Program's code ensures the transfer is valid (correct signer, sufficient balance, etc.).

### System Accounts vs Program-Owned Accounts

There are two categories of accounts you'll work with:

**System-owned accounts** — Owned by the System Program (`11111111111111111111111111111111`).
These are primarily wallet accounts. The System Program controls SOL transfers and
account creation. When you create a new Solana keypair, the resulting account (if it
has any SOL) is owned by the System Program.

**Program-owned accounts** — Owned by a specific program. When the Token Program
creates a token account for you, that account's owner field is set to the Token
Program's address. Only the Token Program can now modify its data.

```
Wallet (System-owned):                Token Account (Program-owned):
┌─────────────────────┐               ┌─────────────────────────┐
│ pubkey: Alice...     │               │ pubkey: 7xKp...         │
│ lamports: 5 SOL      │               │ lamports: 0.002 SOL     │
│ data: []             │               │ data: [mint, owner,     │
│ owner: SystemProgram │               │        amount, ...]     │
│ executable: false    │               │ owner: TokenProgram     │
└─────────────────────┘               │ executable: false       │
                                       └─────────────────────────┘
```

### The Solidity Analogy

Imagine if Solidity worked like this:

```solidity
// Instead of:
contract Token {
    mapping(address => uint256) balances;
    function transfer(address to, uint256 amount) external { ... }
}

// You had:
// A "Token Program" that is ONLY code:
contract TokenProgram {
    // No storage at all!
    function transfer(
        StorageSlot calldata fromAccount,  // passed in
        StorageSlot calldata toAccount,    // passed in
        uint256 amount
    ) external {
        fromAccount.write(fromAccount.read() - amount);
        toAccount.write(toAccount.read() + amount);
    }
}
// And every user's balance was a separate "StorageSlot" contract
// that you pass to the function as a parameter.
```

That's Solana. Every piece of state is external. Programs are pure logic.

---

## 3. How Transactions Work

### What You Know (EVM)

In the EVM, a transaction targets one contract and calls one function:

```
Transaction:
  to:   0xTokenContract
  data: transfer(bob, 10)   // ABI-encoded function call
  from: alice               // implicit msg.sender
```

The contract code runs, reads/writes its own storage, and optionally calls other
contracts. The caller is always available as `msg.sender`.

If you want to do multiple things (approve + swap), you either:
- Send multiple separate transactions, or
- Use a multicall/router contract that batches them

### What Solana Does

A Solana **transaction** contains one or more **instructions**. Each instruction
specifies:

```
Instruction:
  program_id:  TokenProgram        // which program to invoke
  accounts:    [                   // ALL accounts this instruction touches
    { pubkey: alice_token, is_signer: true,  is_writable: true  },
    { pubkey: bob_token,   is_signer: false, is_writable: true  },
    { pubkey: mint,        is_signer: false, is_writable: false },
  ]
  data:        [3, 10, 0, 0, ...]  // serialized instruction data
```

A transaction is a bundle of instructions that execute atomically:

```
Transaction:
┌──────────────────────────────────────────────────────┐
│                                                      │
│  Signers: [alice_keypair]                            │
│                                                      │
│  Instruction 1: Create associated token account      │
│    program: AssociatedTokenProgram                   │
│    accounts: [alice, bob_ata, mint, system, token]   │
│                                                      │
│  Instruction 2: Transfer tokens                      │
│    program: TokenProgram                             │
│    accounts: [alice_token, bob_ata, alice]           │
│    data: { amount: 10 }                              │
│                                                      │
│  Instruction 3: Log a memo                           │
│    program: MemoProgram                              │
│    accounts: [alice]                                 │
│    data: "payment for services"                      │
│                                                      │
└──────────────────────────────────────────────────────┘

All three execute atomically. If any fails, all revert.
```

This is like native multicall. No router contract needed.

### Why Accounts Must Be Declared Upfront

This is the key to Solana's performance. Every account a transaction will read or
write must be listed in the instruction *before* execution begins. The runtime uses
this to schedule transactions:

```
Transaction A touches accounts: [1, 2, 3]
Transaction B touches accounts: [4, 5, 6]
Transaction C touches accounts: [1, 7, 8]

                    ┌─────────────┐
  ┌─────────────────┤  Scheduler  ├─────────────────┐
  │                 └──────┬──────┘                  │
  │                        │                         │
  ▼                        ▼                         ▼
┌───────┐           ┌───────────┐             ┌───────────┐
│ Tx A  │           │   Tx B    │             │ Tx C must  │
│ runs  │  parallel │   runs    │   wait      │ wait for A │
│       │◄─────────►│           │             │ (shares    │
└───────┘           └───────────┘             │  acct 1)   │
                                              └───────────┘
```

The EVM can't do this because a contract can call any other contract at runtime,
and you don't know what storage it'll touch until it runs.

### Signers vs Read-Only Accounts

Every account in an instruction has two flags:

- **is_signer** — This account's private key signed the transaction. The runtime
  verifies this cryptographically. In your program, you check this to authorize
  actions ("did Alice actually approve this transfer?").

- **is_writable** — The program is allowed to modify this account. If an account
  is passed as read-only (is_writable = false), the runtime will reject any attempt
  to change it. This is another optimization: read-only accounts don't cause
  transaction conflicts for scheduling.

```
EVM equivalent:                         Solana:
msg.sender is always the signer    →    Signer is explicit per-account
Storage writes are implicit        →    Write permission is declared
```

### Transaction Size Limit

A Solana transaction is capped at **1232 bytes**. This includes all instructions,
account addresses, signatures, and metadata. This is small! In practice, it means:

- You can fit roughly 20-30 account keys in a transaction
- Complex DeFi transactions (like Jupiter swaps across many pools) sometimes hit
  this limit
- Address Lookup Tables (ALTs) compress account keys to mitigate this
- If you need more space, you use multiple transactions (but lose atomicity)

For comparison, EVM calldata is limited by the block gas limit, which in practice
allows much larger payloads.

---

## 4. State & Storage

### EVM Storage Model

Solidity gives you automatic storage management:

```solidity
contract Vault {
    uint256 public totalDeposits;        // slot 0
    mapping(address => uint256) balances; // slot 1 (hash-based)
    address public owner;                // slot 2

    function deposit() external payable {
        balances[msg.sender] += msg.value;  // SSTORE
        totalDeposits += msg.value;         // SSTORE
    }
}
```

Under the hood, the EVM uses a key-value store with 256-bit keys and 256-bit values.
The compiler handles layout. `SSTORE` writes a 32-byte word for ~20,000 gas.
`SLOAD` reads one for ~2,100 gas.

You never think about serialization. The compiler handles converting your `uint256`,
`address`, and `mapping` types to storage slot reads/writes.

### Solana Storage Model

On Solana, each account's `data` field is a flat byte array. There are no storage
slots, no automatic layout, no key-value trie. Your program is responsible for
interpreting those bytes.

Here's what the same Vault might look like conceptually:

```
Account: vault_state (data account)
┌──────────────────────────────────────────────────┐
│ Byte offset  │ Field            │ Type           │
├──────────────┼──────────────────┼────────────────┤
│ 0..8         │ total_deposits   │ u64 (8 bytes)  │
│ 8..40        │ owner            │ Pubkey (32 b)  │
│ 40..48       │ initialized      │ bool + padding │
└──────────────┴──────────────────┴────────────────┘

Account: user_deposit_alice (separate account per user!)
┌──────────────────────────────────────────────────┐
│ 0..32        │ vault             │ Pubkey         │
│ 32..64       │ depositor         │ Pubkey         │
│ 64..72       │ amount            │ u64            │
└──────────────┴───────────────────┴────────────────┘
```

Notice two critical differences:

1. **No mappings.** Solana has no built-in mapping type. Instead of
   `mapping(address => uint256)`, you create a separate account per user. The
   account's address is derived deterministically (via PDA — covered in section 6).

2. **Explicit serialization.** You must convert your Rust struct to/from bytes.

### Borsh Serialization

Solana programs typically use **Borsh** (Binary Object Representation Serializer for
Hashing) to serialize and deserialize data. It's deterministic, efficient, and has a
known schema:

```rust
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct VaultState {
    pub total_deposits: u64,     // 8 bytes
    pub owner: Pubkey,           // 32 bytes
    pub initialized: bool,       // 1 byte
}

// Writing to an account:
let state = VaultState { total_deposits: 0, owner, initialized: true };
state.serialize(&mut &mut account.data.borrow_mut()[..])?;

// Reading from an account:
let state = VaultState::try_from_slice(&account.data.borrow())?;
```

Anchor (the dominant Solana framework) handles most of this boilerplate for you,
but understanding what happens underneath is essential.

### Comparison Table

| Aspect | EVM | Solana |
|--------|-----|--------|
| Storage model | Key-value trie (256-bit slots) | Flat byte array per account |
| Mappings | `mapping(key => value)` native | Separate account per entry (via PDA) |
| Serialization | Automatic (compiler) | Manual (Borsh/custom) |
| Max storage | Unlimited (but expensive) | 10 MB per account |
| Cost | Gas per SSTORE/SLOAD | Rent for allocated space |
| Layout | Compiler-determined slots | Developer-determined byte offsets |

### The "No Global State" Rule

In EVM, any contract can read any other contract's public state. In Solana, a program
can only access accounts that are explicitly passed to it in the instruction. You can't
just "look up" an account at an arbitrary address during execution.

This seems limiting, but it's what enables parallel execution and makes state access
predictable. In practice, the client (your frontend or script) is responsible for
computing the right accounts and passing them in.

---

## 5. Identity & Signatures

### EVM Identity

The EVM has a simple (maybe too simple) identity model:

- **EOA (Externally Owned Account)** — A user. Has a private key. Can sign
  transactions. Is the `msg.sender` / `tx.origin` when calling a contract.
- **Contract Account** — No private key. Can't initiate transactions. Has code.
- **ECDSA signatures** on the secp256k1 curve.
- **msg.sender** — Implicit. Always available. Tells your contract who called it.

```solidity
function withdraw() external {
    require(msg.sender == owner, "not owner");  // implicit auth
    payable(owner).transfer(address(this).balance);
}
```

### Solana Identity

Solana uses a different curve and a different auth model:

- **Ed25519 keypairs** — Every account is identified by a 32-byte public key.
  The corresponding private key signs transactions.
- **No EOA vs contract distinction** — An account is an account. Whether it's a
  "wallet" or "data" depends on its owner and executable fields, not its type.
- **No msg.sender** — There is no implicit caller identity. Instead, the program
  checks which accounts in the instruction are marked as **signers**.

```rust
// Solana equivalent of the Solidity `withdraw` above:
pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
    // `ctx.accounts.authority` is explicitly passed and marked as a signer
    // Anchor validates the signature automatically via account constraints
    require_keys_eq!(
        ctx.accounts.authority.key(),
        ctx.accounts.vault.owner,
        ErrorCode::Unauthorized
    );
    // transfer SOL...
    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, VaultState>,
    pub authority: Signer<'info>,  // <-- MUST be a signer, enforced by runtime
}
```

The crucial difference:

```
EVM:   msg.sender is magic. The runtime gives it to you.
       You never explicitly pass "who is calling."

Solana: The signer is an account you explicitly pass and the runtime
        verifies the signature. Your program checks: "is this the
        right signer for this operation?"
```

### Why This Matters

In Solidity, `msg.sender` propagation through call chains creates reentrancy risks
and complex auth patterns. On Solana, authentication is explicit and unambiguous:
either an account signed the transaction or it didn't. The runtime guarantees this
before your program code even runs.

```
EVM call chain:                         Solana:
Alice → ContractA → ContractB           Alice signs tx
  msg.sender in B = ContractA           → Program sees Alice as signer
  tx.origin = Alice                       No confusion about "who called"
  (reentrancy risk here)                  (no reentrancy possible)
```

---

## 6. Program Derived Addresses (PDAs)

This concept has no direct EVM parallel, but it's one of the most important ideas
on Solana. PDAs are how you create deterministic, program-controlled accounts.

### The Problem PDAs Solve

On Solana, programs are stateless. But programs often need to "own" accounts or
create predictable addresses. How do you:

1. Create a token vault that a program controls (no human has the private key)?
2. Create a user-specific account at a deterministic address (like a mapping)?
3. Let a program sign transactions on behalf of an account it controls?

Answer: PDAs.

### How PDAs Work

A PDA is derived from:
- One or more **seeds** (arbitrary byte arrays)
- A **program_id**

```
PDA = hash(seed1, seed2, ..., program_id, "ProgramDerivedAddress")
```

The critical property: **a PDA has no corresponding private key.** It falls off the
Ed25519 curve. No human or program can produce a signature for it — except the
program that derived it, using a special mechanism called `invoke_signed`.

### The Bump Seed

The hash above doesn't always produce a point off the curve. When it lands on the
curve (meaning a private key could theoretically exist), we add a "bump" byte and
try again:

```
Try: hash(seeds..., program_id, 255) → on curve? try again
Try: hash(seeds..., program_id, 254) → off curve! ✓ This is our PDA
                                        bump = 254
```

The runtime tries bump values from 255 down to 0. The first one that produces an
off-curve point is the **canonical bump**. You store this bump and use it in future
operations.

### PDA as Deterministic Mapping

This is where it clicks for Solidity developers. Remember how EVM has mappings?

```solidity
mapping(address => uint256) balances;
// balances[alice] is at: keccak256(alice, slot_number)
```

On Solana, you use PDAs:

```
User deposit account for Alice in our Vault:

PDA = findProgramAddress(
    seeds: ["deposit", alice_pubkey],
    program_id: vault_program
)

User deposit account for Bob:
PDA = findProgramAddress(
    seeds: ["deposit", bob_pubkey],
    program_id: vault_program
)
```

Each user gets a unique, deterministic account address. You derive it from known
inputs — just like `keccak256(key, slot)` derives a storage location. But instead
of a storage slot in a contract, it's a full account on the blockchain.

### PDA as Program Authority

Because PDAs have no private key, only the deriving program can "sign" for them
(via `invoke_signed`). This makes PDAs perfect for authority roles:

```
┌─────────────────────────────────────────────┐
│              Vault Program                  │
│                                             │
│  PDA: vault_authority                       │
│  seeds: ["vault", vault_id]                 │
│                                             │
│  This PDA "owns" the vault's token account  │
│  Only the Vault Program can sign for it     │
│  No human has the private key               │
│                                             │
│  To transfer tokens out:                    │
│    invoke_signed(                            │
│      transfer_ix,                           │
│      seeds: ["vault", vault_id, &[bump]]    │
│    )                                        │
└─────────────────────────────────────────────┘
```

### EVM Comparison: CREATE2

The closest EVM analog is `CREATE2`, which gives deterministic contract addresses:

```
EVM CREATE2:
  address = keccak256(0xff, deployer, salt, bytecodeHash)
  → Predictable address, but it's a full contract

Solana PDA:
  address = findProgramAddress(seeds, program_id)
  → Predictable address, no private key, program can sign for it
```

But PDAs are much more versatile. They're not just for deployment — they're used
everywhere for account addressing, authority delegation, and data organization.

### PDA Pattern Summary

| Pattern | Seeds | Purpose |
|---------|-------|---------|
| Global config | `["config"]` | One config account per program |
| Per-user data | `["user", user_pubkey]` | Like `mapping(address => data)` |
| Per-pair data | `["pair", mint_a, mint_b]` | Like `mapping(a => mapping(b => data))` |
| Vault authority | `["vault", vault_id]` | Program-controlled signer |
| Mint authority | `["mint_auth"]` | Program controls token minting |

---

## 7. Fees & Rent

### EVM Gas Model

You know this well:

- Every opcode has a gas cost (`ADD` = 3, `SSTORE` = 20,000, etc.)
- You set `gasPrice` (or `maxFeePerGas` post-EIP-1559)
- Total cost = gas used × gas price
- Unused gas is refunded
- Block gas limit caps computation per block

### Solana Compute Units

Solana uses **compute units** (CUs) instead of gas:

- Each instruction gets a default budget of **200,000 CUs**
- A transaction can request up to **1,400,000 CUs** total
- Operations cost CUs (roughly proportional to CPU cycles)
- If you exceed the budget, the transaction fails (like out-of-gas)

Unlike EVM gas, compute unit costs are not directly tied to transaction fees.
The base transaction fee on Solana is a flat **5,000 lamports** (0.000005 SOL)
per signature, regardless of computation.

**Priority fees** are optional. You can add them to get your transaction processed
faster during congestion (similar to EIP-1559 tips):

```
Total fee = base fee (5,000 lamports/sig) + priority fee (CU price × CUs requested)
```

### Rent: Paying for Storage

This is unique to Solana. On EVM, once you `SSTORE`, that data exists forever (you
paid gas once). On Solana, storing data has an ongoing cost — **rent**.

Every account must maintain a minimum SOL balance proportional to its data size.
If the balance drops below this threshold, the account gets garbage-collected
(deleted) by the runtime.

**Rent exemption:** If you deposit enough SOL to cover roughly 2 years of rent,
the account becomes **rent-exempt** and will never be collected. In practice,
virtually every account on Solana is rent-exempt.

The formula (approximate):

```
Rent-exempt minimum ≈ 0.00089088 SOL per byte of data per epoch
                    ≈ (data_size + 128) × 6.96e-6 SOL

Examples:
  Empty account (0 bytes data):     ~0.00089 SOL
  Token account (165 bytes):        ~0.00204 SOL
  Small data account (500 bytes):   ~0.00437 SOL
  1 KB data account:                ~0.00786 SOL
```

(The 128 is overhead for the account metadata itself.)

### Rent vs Gas: A Comparison

```
EVM:
  Creating storage:    Pay gas once for SSTORE     → data lives forever
  Storage is "free"    after initial write          → no ongoing cost
  But:                 storage bloat is an issue    → state grows forever

Solana:
  Creating an account: Pay rent deposit upfront     → data lives while funded
  Rent-exempt:         ~2 years rent = permanent    → practically same as EVM
  Closing accounts:    You get the SOL back!        → incentive to clean up
  Result:              less state bloat
```

The rent system creates an economic incentive to close accounts you no longer need.
When you close an account, its lamports are returned to you. This is unlike EVM
where storage refunds are minimal and nobody bothers cleaning up.

---

## 8. Token Standards

This section might be the biggest "aha moment" for Solidity developers.

### EVM: Every Token Is Its Own Contract

In the EVM world, every token deploys its own contract:

```
USDC:  ERC-20 contract at 0xA0b8...   (USDC's code + USDC's balances)
WETH:  ERC-20 contract at 0xC02a...   (WETH's code + WETH's balances)
DAI:   ERC-20 contract at 0x6B17...   (DAI's code  + DAI's balances)

Each has its own:
  - balanceOf() implementation
  - transfer() implementation
  - approve() implementation
  - Its own storage for all balances
```

Every ERC-20 token reinvents the wheel. They all implement the same interface but
each is a separate contract with separate code. Some add transfer fees, some have
hooks, some have bugs.

### Solana: One Program to Rule Them All

On Solana, the **SPL Token Program** is a single deployed program that handles ALL
fungible tokens:

```
SPL Token Program (one binary, one address: TokenkegQf...):
  ├── manages USDC
  ├── manages Wrapped SOL
  ├── manages Raydium
  ├── manages Jupiter token
  ├── manages Bonk
  └── manages ... every token on Solana
```

How? Because code and data are separate. The SPL Token Program is the code. Every
token's data (supply, decimals, authority) and every user's balance live in separate
accounts.

### Mint Accounts and Token Accounts

Two account types make this work:

**Mint Account** — Defines a token. One per token type. Contains:
```
┌──────────────────────────────────────────────────┐
│                  Mint Account                    │
│              (one per token type)                │
├──────────────────┬───────────────────────────────┤
│ mint_authority   │ who can mint new tokens       │
│ supply           │ total tokens in circulation   │
│ decimals         │ 6 for USDC, 9 for SOL, etc   │
│ is_initialized   │ true                          │
│ freeze_authority │ who can freeze accounts       │
└──────────────────┴───────────────────────────────┘
```

**Token Account** — Holds a balance. One per user per token. Contains:
```
┌──────────────────────────────────────────────────┐
│                Token Account                     │
│          (one per user per token)                │
├──────────────────┬───────────────────────────────┤
│ mint             │ which token this holds        │
│ owner            │ the wallet that controls it   │
│ amount           │ token balance                 │
│ delegate         │ approved spender (like ERC-20)│
│ state            │ active / frozen               │
│ delegated_amount │ allowance for delegate        │
└──────────────────┴───────────────────────────────┘
```

The relationship:

```
  Alice's Wallet                      USDC Mint
  (System-owned)                      ┌──────────────┐
  ┌──────────────┐                    │ supply: 10B  │
  │ pubkey: Ali..│                    │ decimals: 6  │
  │ SOL: 2.5     │                    └──────┬───────┘
  └──────┬───────┘                           │
         │ owns (via owner field)            │ (mint field)
         ▼                                   ▼
  ┌───────────────────┐        ┌───────────────────┐
  │ Alice's USDC      │        │ Alice's BONK      │
  │ Token Account     │        │ Token Account     │
  │ mint: USDC        │        │ mint: BONK        │
  │ owner: Alice      │        │ owner: Alice      │
  │ amount: 1000      │        │ amount: 5000000   │
  └───────────────────┘        └───────────────────┘

  Both token accounts are owned by the SPL Token Program
  (the program can modify their data).
  The "owner" field INSIDE the data refers to Alice's wallet
  (the human who controls the tokens).
```

Note the confusing double meaning of "owner": the account's `owner` (program that
controls the data) vs the token account's internal `owner` field (the wallet that
controls the tokens). The former is always the Token Program. The latter is the user.

### Associated Token Accounts (ATAs)

How does Alice's USDC token account get its address? It's a PDA:

```
ATA address = findProgramAddress(
    seeds: [alice_wallet, token_program_id, usdc_mint],
    program_id: associated_token_program
)
```

This is deterministic. Given a wallet and a mint, you can always compute the ATA
address. No lookup needed. This is the standard — every wallet integration uses ATAs.

In EVM terms, it's like if `balanceOf(alice)` wasn't a storage read, but instead
you computed a deterministic address and looked up that account.

### Why This Design Is Powerful

1. **One audit.** The SPL Token Program is audited once. Every token benefits.
   No more ERC-20 contracts with custom backdoors.

2. **Universal tooling.** Wallets, explorers, and DEXes don't need custom logic
   per token. Every token works the same way because it's the same program.

3. **Composability.** Cross-program invocation (CPI) to the Token Program is
   standardized. Every DeFi program interacts with tokens identically.

4. **NFTs too.** On Solana, NFTs are just tokens with supply = 1 and decimals = 0.
   Same program. (Metaplex adds metadata on top, but the base is SPL Token.)

---

## 9. Upgradeability

### EVM: The Proxy Pattern Hack

Solidity contracts are immutable by default. To make them upgradeable, the
community invented proxy patterns:

```
┌──────────────┐     delegatecall     ┌──────────────────┐
│  Proxy       │ ──────────────────► │  Implementation  │
│  (storage)   │                      │  v1 (code only)  │
│              │                      └──────────────────┘
│  admin: 0x.. │
│  impl: 0x.. ─┼─── can be changed ──► Implementation v2
└──────────────┘
```

This works but it's a hack:
- `delegatecall` was designed for libraries, not upgradeability
- Storage layout must be perfectly compatible between versions
- Proxy patterns are a common source of critical bugs
- There's no standard (UUPS, Transparent, Diamond, Beacon...)

### Solana: Upgradeable by Default

On Solana, programs are upgradeable out of the box. The architecture:

```
┌──────────────────────┐
│  Program Account     │
│  pubkey: Prog111...  │
│  executable: true    │
│  data: [pointer to   │◄──── This is what gets invoked
│         programdata] │
└──────────┬───────────┘
           │ points to
           ▼
┌──────────────────────┐
│  ProgramData Account │
│  upgrade_authority:  │◄──── Who can upgrade (or None = frozen)
│    Dev's pubkey      │
│  data: [actual BPF   │
│         bytecode]    │
└──────────────────────┘
```

Upgrading is a first-class operation:

```bash
# Deploy v1
solana program deploy target/deploy/my_program.so

# Upgrade to v2 (same program ID, new code)
solana program deploy target/deploy/my_program.so --program-id <PROGRAM_ID>
```

The program's address stays the same. The bytecode changes. State accounts are
unaffected because they're separate from the code (remember: code ≠ state).

### Freezing a Program

To make a program immutable (like a non-upgradeable Solidity contract), you set the
upgrade authority to `None`:

```bash
solana program set-upgrade-authority <PROGRAM_ID> --final
```

After this, nobody can ever change the code. This is useful for protocols that want
to signal trust and immutability.

### Comparison

| Aspect | EVM | Solana |
|--------|-----|--------|
| Default | Immutable | Upgradeable |
| Upgrade mechanism | Proxy pattern (delegatecall) | Native (BPF loader) |
| Storage compatibility | Must preserve slot layout | N/A (state is separate) |
| Make immutable | Don't use proxy (or renounce admin) | Set authority to None |
| Complexity | High (proxy bugs are common) | Low (built-in) |
| State migration | Tricky (storage slots) | Easier (accounts are independent) |

---

## 10. Developer Tooling Comparison

### Side-by-Side

| Category | EVM Ecosystem | Solana Ecosystem |
|----------|---------------|------------------|
| **Language** | Solidity (or Vyper) | Rust (via Anchor framework) |
| **Framework** | Hardhat / Foundry | Anchor |
| **Testing** | Mocha + ethers.js / Forge tests | Bankrun / Anchor test (Mocha + solana-web3.js) |
| **Local chain** | Hardhat node / Anvil | solana-test-validator / Bankrun |
| **Deploy** | `forge create` / `hardhat deploy` | `anchor deploy` / `solana program deploy` |
| **Explorer** | Etherscan / Blockscout | Solscan / Solana Explorer |
| **Wallet** | MetaMask / Rainbow | Phantom / Solflare / Backpack |
| **RPC providers** | Infura / Alchemy / QuickNode | Helius / QuickNode / Triton |
| **Client SDK** | ethers.js / viem / web3.js | @solana/web3.js / @solana/kit |
| **IDL / ABI** | ABI JSON (from compilation) | IDL JSON (from Anchor build) |
| **Package mgr** | npm (for JS) / soldeer | Cargo (Rust crates) |
| **Linting** | Slither / Solhint | Clippy (Rust linter) |
| **Fuzzing** | Echidna / Forge fuzz | Trident / custom |

### The Anchor Framework

Anchor is to Solana what Hardhat + OpenZeppelin is to EVM development. It provides:

- **Account validation macros** — `#[derive(Accounts)]` auto-checks account ownership,
  signer status, and constraints
- **Automatic (de)serialization** — structs are serialized to account data via Borsh
- **Error handling** — typed errors with readable messages
- **IDL generation** — like ABI, but for Solana programs
- **Client generation** — TypeScript client from IDL
- **Testing framework** — local validator integration

Here's a rough workflow comparison:

```
EVM Development:                        Solana Development:
1. Write Solidity (.sol)                1. Write Rust with Anchor macros (.rs)
2. Compile: solc / forge build          2. Compile: anchor build
3. Get ABI JSON                         3. Get IDL JSON
4. Deploy: forge create                 4. Deploy: anchor deploy
5. Test: forge test / hardhat test      5. Test: anchor test
6. Interact: ethers.js + ABI            6. Interact: @solana/web3.js + IDL
7. Verify: Etherscan                    7. Verify: Anchor verify / Solscan
```

### Key Differences in Dev Experience

**Compilation time.** Rust compiles slower than Solidity. An Anchor build can take
30-60 seconds on first build. Incremental builds are faster. This is the price you
pay for Rust's type safety and optimization.

**Testing.** Solana tests typically use a local test validator (a real Solana node
running locally) or Bankrun (an in-process lightweight runtime). This is similar to
Hardhat's in-memory network but with real BPF execution.

**Debugging.** Solana programs emit logs via `msg!()` macro. You read these from
transaction logs. There's no `console.log` equivalent that appears in your terminal
during testing — you inspect transaction results after execution. Tools like Anchor's
error parsing help translate numeric error codes to readable messages.

---

## 11. Common Gotchas for Solidity Developers

This section catalogues the traps that catch EVM developers when they start
building on Solana.

### Gotcha #1: No `msg.sender`

**EVM habit:**
```solidity
function withdraw() external {
    require(msg.sender == owner);
}
```

**Solana reality:** There is no `msg.sender`. You must explicitly pass a signer
account and verify it.

```rust
// Anchor handles this with account constraints:
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(has_one = authority)]  // checks vault.authority == authority.key()
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,    // runtime verifies this signed the tx
}
```

If you forget to check signers, anyone can call your program pretending to be anyone.

### Gotcha #2: Accounts Must Be Passed In

**EVM habit:** Read any contract's state at will.
```solidity
uint256 price = IPriceOracle(oracleAddress).getPrice();
```

**Solana reality:** You can only access accounts passed in the instruction. The client
must provide every account the program needs.

This means your client code must know the full account dependency graph before
submitting a transaction. For complex DeFi operations, this account resolution can
be the hardest part of the integration.

### Gotcha #3: No Reentrancy

**EVM habit:** Guard against reentrancy with checks-effects-interactions or
reentrancy guards.

**Solana reality:** Reentrancy is impossible. The runtime prevents a program from
being invoked again while it's already executing (recursive CPI to the same program
is blocked). You can delete your reentrancy-guard mental overhead entirely.

However, Solana has its own version of this: **cross-program invocation (CPI)**
ordering matters, and you should update state before making CPI calls to avoid
related issues.

### Gotcha #4: No Try-Catch for CPI

**EVM habit:**
```solidity
try externalContract.call() {
    // success
} catch {
    // handle failure
}
```

**Solana reality:** If a CPI call fails, the entire transaction reverts. There is
no way to catch and handle a failed CPI. Design your programs assuming CPI calls
either succeed or everything rolls back.

### Gotcha #5: Transaction Size Limit (1232 bytes)

**EVM habit:** Pack as much calldata as you want (limited by block gas limit).

**Solana reality:** The entire transaction — signatures, account keys, instructions,
and data — must fit in 1232 bytes. This is an IPv6 MTU constraint (Solana uses UDP).

Practical impact:
- ~30 account keys max per transaction
- Complex multi-hop swaps can hit this limit
- Address Lookup Tables (ALTs) help by compressing account keys
- Very large operations may need to span multiple transactions

### Gotcha #6: Account Size Limits

**EVM habit:** Dynamic arrays and mappings grow indefinitely.

**Solana reality:** An account's data size is fixed at creation (unless you use
`realloc`). Maximum size is 10 MB. You must think about your data layout upfront.

If you need "infinite" storage (like a growing list), you typically create multiple
accounts — one per entry — rather than one large account.

### Gotcha #7: Numeric Types

**EVM habit:** `uint256` everywhere. 256-bit arithmetic is native.

**Solana reality:** Rust has `u8`, `u16`, `u32`, `u64`, `u128`. There is no native
`u256`. For token amounts, `u64` is standard (max ~18.4 quintillion). For operations
needing larger numbers, you use the `u128` type or dedicated big-number crates.

In practice, this rarely matters because Solana's token amounts fit in `u64` — but
if you're porting DeFi math that relies on `uint256`, you'll need to adapt.

### Gotcha #8: Clock and Block Info

**EVM habit:** `block.timestamp`, `block.number`, `blockhash()` are globally available.

**Solana reality:** You access the clock via a **sysvar account** (`Clock::get()?`),
which gives you:
- `slot` (roughly equivalent to block number)
- `unix_timestamp` (wall clock time, but can drift)
- `epoch` (Solana-specific concept for staking periods)

There's no `blockhash()` equivalent for randomness — and just like EVM, on-chain
randomness is problematic. Solana uses VRFs (verifiable random functions) or oracle
solutions.

### Gotcha #9: Error Handling

**EVM habit:** `require(condition, "message")` with string error messages.

**Solana reality:** Errors are numeric codes. Anchor gives you typed errors:

```rust
#[error_code]
pub enum MyError {
    #[msg("Insufficient funds for withdrawal")]
    InsufficientFunds,
    #[msg("Not authorized")]
    Unauthorized,
}

// Usage:
require!(amount <= vault.balance, MyError::InsufficientFunds);
```

The `#[msg]` string is for client-side display and logging — it's not stored on-chain
like Solidity revert strings.

### Gotcha #10: No Native ETH Equivalent in Programs

**EVM habit:** `msg.value` sends ETH with a function call. `payable` functions.

**Solana reality:** SOL transfers are explicit instructions to the System Program.
There's no concept of "sending SOL with your instruction." You either:
- Include a separate SOL transfer instruction in the transaction, or
- Have your program CPI to the System Program to transfer lamports

---

## 12. Architecture at a Glance

Here's the full picture, side by side:

```
╔══════════════════════════════════════════════════════════════════════════╗
║                         EVM ARCHITECTURE                               ║
╠══════════════════════════════════════════════════════════════════════════╣
║                                                                        ║
║   EOA (Alice)                                                          ║
║      │                                                                 ║
║      │ signs tx: Token.transfer(Bob, 100)                              ║
║      ▼                                                                 ║
║   ┌──────────────────────────────────────────┐                         ║
║   │          Token Contract (0xABC...)       │                         ║
║   │  ┌────────────────────────────────────┐  │                         ║
║   │  │ Code:                              │  │                         ║
║   │  │   transfer(to, amount)             │  │                         ║
║   │  │   approve(spender, amount)         │  │                         ║
║   │  │   balanceOf(addr)                  │  │                         ║
║   │  ├────────────────────────────────────┤  │                         ║
║   │  │ Storage:                           │  │                         ║
║   │  │   balances[alice] = 1000           │  │                         ║
║   │  │   balances[bob]   = 500            │  │                         ║
║   │  │   totalSupply     = 10000          │  │                         ║
║   │  └────────────────────────────────────┘  │                         ║
║   └──────────────────────────────────────────┘                         ║
║                                                                        ║
║   Code + Storage = ONE unit. msg.sender is implicit.                   ║
║                                                                        ║
╚══════════════════════════════════════════════════════════════════════════╝

╔══════════════════════════════════════════════════════════════════════════╗
║                       SOLANA ARCHITECTURE                              ║
╠══════════════════════════════════════════════════════════════════════════╣
║                                                                        ║
║   Wallet (Alice)                 Token Program                         ║
║   ┌──────────────┐               ┌───────────────────┐                 ║
║   │ SOL: 5.0     │               │ Code only:        │                 ║
║   │ owner: System│               │  transfer()       │                 ║
║   └──────┬───────┘               │  mint_to()        │                 ║
║          │ signs tx               │  burn()           │                 ║
║          │                       └─────────┬─────────┘                 ║
║          │                                 │                           ║
║          │    Transaction:                 │ invoked with              ║
║          │    ┌────────────────────────┐   │ accounts:                 ║
║          └───►│ program: TokenProgram  │───┘                           ║
║               │ accounts:              │                               ║
║               │   alice_ata (signer,w) │──► ┌─────────────────────┐    ║
║               │   bob_ata   (w)        │    │ Alice's Token Acct  │    ║
║               │   alice     (signer)   │    │ mint: USDC          │    ║
║               │ data: {amount: 100}    │    │ owner: Alice        │    ║
║               └────────────────────────┘    │ amount: 1000        │    ║
║                                             │ acct_owner: TokenPrg│    ║
║                            ┌────────────┐   └─────────────────────┘    ║
║                            │ Bob's ATA  │                              ║
║                            │ mint: USDC │                              ║
║                            │ owner: Bob │                              ║
║                            │ amount: 500│                              ║
║                            └────────────┘                              ║
║                                                                        ║
║   Code and State are SEPARATE. Signer is explicit. Accounts passed in. ║
║                                                                        ║
╚══════════════════════════════════════════════════════════════════════════╝
```

---

## 13. Concept Mapping Cheat Sheet

Use this table when you catch yourself thinking in EVM terms:

| EVM Concept | Solana Equivalent | Key Difference |
|-------------|-------------------|----------------|
| Contract | Program | Stateless; no storage |
| Contract storage | Account data | Separate from code; byte array |
| `msg.sender` | `Signer<'info>` | Explicit, not implicit |
| `mapping(k => v)` | PDA-derived accounts | One account per key |
| `SSTORE` / `SLOAD` | Borsh serialize/deserialize | You manage the bytes |
| `address` (20 bytes) | `Pubkey` (32 bytes) | Ed25519, not secp256k1 |
| Gas | Compute units | Plus rent for storage |
| ERC-20 | SPL Token Program | One program for all tokens |
| `approve` / `transferFrom` | Delegate on token account | Similar concept, different mechanics |
| Proxy pattern | Native upgradeability | Built-in, no delegatecall |
| `CREATE2` | PDA (Program Derived Address) | No private key, program signs |
| `block.timestamp` | `Clock` sysvar | Must explicitly access |
| `block.number` | Slot number | Solana has slots, not blocks (roughly) |
| Constructor | `initialize` instruction | Programs have no constructor |
| `selfdestruct` | Close account | Returns rent to receiver |
| `view` / `pure` functions | Simulated transactions | No free reads by default |
| Events / `emit` | Program logs (`msg!`) | Indexed via transaction logs |
| `payable` | Explicit SOL transfer via System Program | No implicit value transfer |
| EOA vs Contract | No distinction | All accounts are accounts |
| `tx.origin` | First signer | But rarely needed |
| Inheritance | Rust traits + CPI | Composition over inheritance |
| Interface | CPI to known program ID | Call by program ID + expected accounts |
| `abi.encode` | Borsh serialization | Binary, not ABI-packed |
| Multicall | Multiple instructions in one tx | Native, not a pattern |
| Reentrancy guard | Not needed | Runtime prevents it |
| `receive()` / `fallback()` | No equivalent | All calls are explicit instructions |

---

## Summary

The shift from EVM to Solana boils down to three mental model changes:

1. **Separate code from state.** Programs are stateless binaries. Accounts are
   data buckets. You pass state *to* programs rather than programs owning state.

2. **Everything is explicit.** No `msg.sender`. No implicit storage access. No
   hidden state. Every account, every signer, every piece of data is declared
   upfront in the transaction.

3. **Accounts are the primitive.** On EVM, the contract is the unit of abstraction.
   On Solana, the account is. Your design patterns, data modeling, and mental
   architecture all revolve around accounts — how to create them, address them
   (PDAs), organize them, and pass them to programs.

Once these three ideas click, everything else follows naturally. The SPL Token design
makes sense (one program, many accounts). PDAs make sense (deterministic account
addressing). The performance makes sense (declare accounts upfront, enable parallelism).

Next module, we start writing actual Solana programs. The concepts from this module
will be the foundation for everything that follows.
