# Module 15: Full CRUD Todo App in Anchor

## Overview

This module builds a **complete CRUD (Create, Read, Update, Delete) Todo application** using the Anchor framework on Solana. If you've built a todo app in Solidity before, this module will show you exactly how the same concepts map to Solana's account model — and where they diverge dramatically.

By the end you will know how to:
- Design multi-account data models with PDAs
- Perform all four CRUD operations on-chain
- Handle variable-length strings in account data
- Close accounts and reclaim rent
- Query program accounts from the client side

---

## Table of Contents

1. [Solidity vs. Solana: The Todo App Compared](#1-solidity-vs-solana-the-todo-app-compared)
2. [Project Setup](#2-project-setup)
3. [Designing the Data Model](#3-designing-the-data-model)
4. [CRUD Operations Deep Dive](#4-crud-operations-deep-dive)
5. [PDA Patterns for User-Specific Data](#5-pda-patterns-for-user-specific-data)
6. [String Handling and Space Calculation](#6-string-handling-and-space-calculation)
7. [Closing Accounts and Reclaiming SOL](#7-closing-accounts-and-reclaiming-sol)
8. [Error Handling](#8-error-handling)
9. [Client-Side Queries](#9-client-side-queries)
10. [Testing Strategy](#10-testing-strategy)
11. [Best Practices](#11-best-practices)
12. [What's Next](#12-whats-next)

---

## 1. Solidity vs. Solana: The Todo App Compared

### The Solidity Version

Here's a typical Solidity Todo contract. You've probably written something like this:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract TodoApp {
    struct TodoItem {
        uint256 id;
        string content;
        bool completed;
    }

    // All state lives inside this single contract
    mapping(address => TodoItem[]) private userTodos;

    function addTodo(string calldata _content) external {
        uint256 nextId = userTodos[msg.sender].length;
        userTodos[msg.sender].push(TodoItem(nextId, _content, false));
    }

    function toggleTodo(uint256 _id) external {
        TodoItem storage item = userTodos[msg.sender][_id];
        item.completed = !item.completed;
    }

    function updateTodo(uint256 _id, string calldata _content) external {
        userTodos[msg.sender][_id].content = _content;
    }

    function removeTodo(uint256 _id) external {
        // Swap with last and pop
        TodoItem[] storage todos = userTodos[msg.sender];
        todos[_id] = todos[todos.length - 1];
        todos.pop();
    }

    function getTodos() external view returns (TodoItem[] memory) {
        return userTodos[msg.sender];
    }
}
```

**Key characteristics:**
- All state lives in a single contract's storage
- `mapping(address => TodoItem[])` — one mapping, all users
- Reading is free (view function, no gas)
- Deleting requires swap-and-pop since Solidity arrays can't have gaps
- State grows inside the contract — no separate "accounts"

### The Anchor/Solana Version (This Module)

On Solana, the architecture is fundamentally different:

| Concept | Solidity | Solana/Anchor |
|---------|----------|---------------|
| State location | Inside the contract | Separate accounts (PDAs) |
| Per-user data | `mapping(address => ...)` | PDA seeded with user pubkey |
| Each todo item | Entry in a dynamic array | Its own account on-chain |
| Reading data | `view` function (free) | Off-chain RPC call (`getProgramAccounts`) |
| Deleting | Swap-and-pop in array | Close the account, reclaim rent SOL |
| Cost model | Gas per operation | Rent for account storage + tx fee |

**The fundamental shift:** In Solidity you have one contract with a big mapping. On Solana, each piece of data is its own account. A user with 10 todos has 11 accounts: 1 `TodoList` metadata account + 10 `TodoItem` accounts.

This sounds heavier, but it enables:
- **Parallel execution**: Different users' todos are in different accounts, so transactions don't conflict
- **Rent reclamation**: When you delete a todo, you get SOL back
- **Efficient queries**: You can filter accounts by size, owner, or data prefix without loading everything

---

## 2. Project Setup

### Creating the Project from Scratch

If you were starting from a clean Anchor workspace:

```bash
# Install Anchor CLI if you haven't already
# See https://www.anchor-lang.com/docs/installation

# Create a new Anchor project
anchor init todo-app
cd todo-app
```

This generates:
```
todo-app/
├── Anchor.toml          # Project configuration (cluster, program ID, etc.)
├── Cargo.toml           # Rust workspace
├── app/                 # (optional) Frontend code
├── migrations/          # Deploy scripts
├── programs/
│   └── todo-app/
│       ├── Cargo.toml   # Program's Rust dependencies
│       └── src/
│           └── lib.rs   # Your program code goes here
└── tests/
    └── todo-app.ts      # TypeScript tests (Mocha + Chai)
```

### Key Files

**`Anchor.toml`** — tells the Anchor CLI which cluster to use and maps program names to IDs:
```toml
[features]
seeds = false
skip-lint = false

[programs.localnet]
todo_app = "YourProgramIdHere11111111111111111111111111"

[registry]
url = "https://api.apr.dev"

[provider]
cluster = "Localnet"
wallet = "~/.config/solana/id.json"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
```

**`programs/todo-app/Cargo.toml`** — the program's dependencies:
```toml
[package]
name = "todo-app"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "lib"]

[dependencies]
anchor-lang = "0.30"
```

### Building and Testing

```bash
# Build the program (compiles to BPF bytecode)
anchor build

# Run the TypeScript test suite against a local validator
anchor test

# Get the deployed program's public key
anchor keys list
```

For this tutorial module, we've simplified the structure to focus on the Rust program logic in `src/lib.rs`.

---

## 3. Designing the Data Model

### What Accounts Do We Need?

Think about what state exists and how to partition it:

#### TodoList Account (one per user)

This is the metadata account — it tracks who owns the list and how many items have been created (so we can derive unique PDAs for each item).

```
┌─────────────────────────────────────────┐
│ TodoList (PDA: ["todo-list", user])      │
├─────────────────────────────────────────┤
│ authority: Pubkey (32 bytes)            │  ← who owns this list
│ last_idx:  u64    (8 bytes)             │  ← next item index
├─────────────────────────────────────────┤
│ + 8 bytes discriminator                 │
│ = 48 bytes total                        │
└─────────────────────────────────────────┘
```

**Why `last_idx` and not a count?** Because we never decrement it. When you delete todo #3, the index isn't reused. This guarantees PDA uniqueness — if we reused indices, a deleted account's PDA could collide with a new one.

#### TodoItem Account (one per todo)

Each todo item is its own on-chain account:

```
┌──────────────────────────────────────────────────────┐
│ TodoItem (PDA: ["todo-item", user, item_idx])        │
├──────────────────────────────────────────────────────┤
│ authority: Pubkey  (32 bytes)                        │
│ idx:       u64     (8 bytes)                         │
│ content:   String  (4 + up to 256 bytes)             │
│ completed: bool    (1 byte)                          │
├──────────────────────────────────────────────────────┤
│ + 8 bytes discriminator                              │
│ = 309 bytes total (with 256-byte content max)        │
└──────────────────────────────────────────────────────┘
```

### The Account Relationship

```
 User Wallet (signer)
      │
      ▼
 TodoList PDA ──────── seeds: ["todo-list", user_pubkey]
      │
      ├── TodoItem PDA ── seeds: ["todo-item", user_pubkey, 0]
      ├── TodoItem PDA ── seeds: ["todo-item", user_pubkey, 1]
      ├── TodoItem PDA ── seeds: ["todo-item", user_pubkey, 2]
      └── ...
```

**Solidity mental model:** Think of `TodoList` as the mapping key's metadata, and each `TodoItem` as a slot in the mapping — except each slot is a full account with its own address and rent deposit.

---

## 4. CRUD Operations Deep Dive

### Create: `initialize_list`

Before a user can add todos, they need a `TodoList` account:

```rust
pub fn initialize_list(ctx: Context<InitializeList>) -> Result<()> {
    let todo_list = &mut ctx.accounts.todo_list;
    todo_list.authority = ctx.accounts.authority.key();
    todo_list.last_idx = 0;
    Ok(())
}
```

The account creation happens in the `#[derive(Accounts)]` struct via the `init` constraint:

```rust
#[account(
    init,                                          // Create the account
    payer = authority,                             // Who pays rent
    space = TODO_LIST_SIZE,                        // How many bytes
    seeds = [b"todo-list", authority.key().as_ref()], // PDA seeds
    bump                                           // Canonical bump
)]
pub todo_list: Account<'info, TodoList>,
```

**Solidity comparison:** This is like the constructor, but it's explicit — the user must call it, and they pay rent for the storage.

### Create: `add_todo`

This creates a `TodoItem` account with a PDA derived from the user and the current `last_idx`:

```rust
pub fn add_todo(ctx: Context<AddTodo>, content: String) -> Result<()> {
    let todo_list = &mut ctx.accounts.todo_list;
    let todo_item = &mut ctx.accounts.todo_item;

    todo_item.authority = ctx.accounts.authority.key();
    todo_item.idx = todo_list.last_idx;
    todo_item.content = content;
    todo_item.completed = false;

    todo_list.last_idx = todo_list.last_idx
        .checked_add(1)
        .ok_or(TodoError::MaxTodosReached)?;

    Ok(())
}
```

Notice the PDA seed includes `todo_list.last_idx`:

```rust
seeds = [
    b"todo-item",
    authority.key().as_ref(),
    &todo_list.last_idx.to_le_bytes()   // Current index BEFORE increment
]
```

**Solidity comparison:** Like `userTodos[msg.sender].push(...)`, but instead of growing an array, we're creating a brand-new account.

### Read: Off-chain Only

There is no "read" instruction. On Solana, reading is done client-side via RPC calls. This is fundamentally different from Solidity's `view` functions.

```typescript
// Fetch a specific TodoItem by deriving its PDA
const [todoPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("todo-item"), user.toBuffer(), new BN(0).toArrayLike(Buffer, "le", 8)],
  programId
);
const todoItem = await program.account.todoItem.fetch(todoPda);

// Fetch ALL TodoItems for a user using getProgramAccounts with a filter
const allTodos = await program.account.todoItem.all([
  { memcmp: { offset: 8, bytes: user.toBase58() } }  // Filter by authority
]);
```

We'll cover this in detail in [Section 9](#9-client-side-queries).

### Update: `toggle_todo` and `update_todo`

Toggle is the simplest operation — just flip a boolean:

```rust
pub fn toggle_todo(ctx: Context<ToggleTodo>) -> Result<()> {
    let todo_item = &mut ctx.accounts.todo_item;
    todo_item.completed = !todo_item.completed;
    Ok(())
}
```

Update replaces the content string:

```rust
pub fn update_todo(ctx: Context<UpdateTodo>, new_content: String) -> Result<()> {
    require!(new_content.len() <= 256, TodoError::ContentTooLong);
    require!(!new_content.is_empty(), TodoError::ContentEmpty);
    let todo_item = &mut ctx.accounts.todo_item;
    todo_item.content = new_content;
    Ok(())
}
```

**Why check length again?** The account was allocated for max 256 bytes of content. If we wrote more, Borsh serialization would exceed the account size and the transaction would fail. Explicit validation gives a better error message.

### Delete: `remove_todo`

This is where Solana really differs from Solidity:

```rust
pub fn remove_todo(ctx: Context<RemoveTodo>) -> Result<()> {
    msg!("Todo #{} removed", ctx.accounts.todo_item.idx);
    // Closing happens via the `close` constraint — no code needed here!
    Ok(())
}
```

The magic is in the accounts struct:

```rust
#[account(
    mut,
    has_one = authority,
    close = authority      // ← This is the key part
)]
pub todo_item: Account<'info, TodoItem>,
```

`close = authority` does three things:
1. Transfers all lamports from `todo_item` to `authority`
2. Zeroes the account data
3. Sets the account owner to the System Program (effectively deleting it)

**Solidity comparison:** In Solidity, `delete` just zeroes storage but you don't get gas back (except for the storage refund, which is partial and capped). On Solana, you get **all** the rent SOL back.

---

## 5. PDA Patterns for User-Specific Data

### What's a PDA?

A Program Derived Address (PDA) is a deterministic address generated from seeds and a program ID. It doesn't have a corresponding private key, so only the program can sign for it.

```
PDA = hash(seeds, program_id, bump)
```

### Our PDA Scheme

```
TodoList PDA:
  seeds = ["todo-list", user_pubkey]
  → One per user, deterministic

TodoItem PDA:
  seeds = ["todo-item", user_pubkey, item_index_as_le_bytes]
  → One per item, deterministic from user + index
```

### Why This Pattern Works

1. **Uniqueness**: Each combination of seeds produces a unique address
2. **Determinism**: Anyone can derive the address without storing it
3. **Authorization**: The `has_one = authority` constraint ensures only the owner can modify their items
4. **No mapping needed**: Unlike Solidity where you need `mapping(address => ...)`, the seeds ARE the lookup key

### Deriving PDAs Client-Side

```typescript
// Derive the TodoList PDA for a user
const [todoListPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("todo-list"), userPubkey.toBuffer()],
  programId
);

// Derive a specific TodoItem PDA
const [todoItemPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("todo-item"),
    userPubkey.toBuffer(),
    new BN(itemIndex).toArrayLike(Buffer, "le", 8)  // u64 as little-endian bytes
  ],
  programId
);
```

---

## 6. String Handling and Space Calculation

### How Borsh Encodes Strings

Anchor uses Borsh serialization. Strings are encoded as:

```
[4 bytes: length as u32 little-endian] [N bytes: UTF-8 content]
```

So the string `"Buy milk"` (8 bytes) is stored as:
```
[08 00 00 00] [42 75 79 20 6d 69 6c 6b]
  ↑ length       ↑ "Buy milk" in UTF-8
```

### Space Calculation

When you create an account with `init`, you must specify `space` — the total byte size. This is calculated at compile time and cannot change.

```
TodoItem space:
    8                    Anchor discriminator (8-byte hash of the account name)
  + 32                   authority: Pubkey
  +  8                   idx: u64
  +  4                   content: String length prefix (u32)
  + 256                  content: max bytes of actual string data
  +  1                   completed: bool
  ─────
  = 309 bytes
```

**Common field sizes for reference:**

| Type | Size (bytes) |
|------|-------------|
| `bool` | 1 |
| `u8` / `i8` | 1 |
| `u16` / `i16` | 2 |
| `u32` / `i32` | 4 |
| `u64` / `i64` | 8 |
| `u128` / `i128` | 16 |
| `Pubkey` | 32 |
| `String` | 4 + content length |
| `Vec<T>` | 4 + (item_size × count) |
| `Option<T>` | 1 + T size |

### Why We Limit String Length

If a user passes a 10,000-byte string, Borsh would try to serialize it into a 309-byte account and the transaction would fail with a generic error. By validating up front:

```rust
require!(content.len() <= 256, TodoError::ContentTooLong);
```

We give a clear, actionable error message.

**Solidity comparison:** In Solidity, dynamic strings just cost more gas — the storage grows as needed. On Solana, the account size is fixed at creation. You must plan for the maximum.

---

## 7. Closing Accounts and Reclaiming SOL

### The Rent Model

Every account on Solana must hold enough lamports to be "rent-exempt." For a 309-byte account, that's roughly **0.00254 SOL** (varies slightly with rent parameters).

When a user creates 50 todos, they've locked up ~0.127 SOL in rent deposits. Closing accounts gives that SOL back.

### How `close` Works

The `close` constraint in Anchor:

```rust
#[account(
    mut,
    has_one = authority,
    close = authority    // ← recipient of the lamports
)]
pub todo_item: Account<'info, TodoItem>,
```

Under the hood, Anchor generates code that:
1. Transfers all lamports from the account to the `authority`
2. Fills the account data with zeroes
3. Sets the account's owner to the System Program
4. After the transaction, the runtime garbage-collects the zero-lamport account

### Security: The Revival Attack

If you close an account but **don't zero the data**, someone could observe the transaction and "revive" the account by sending lamports to it before the runtime garbage-collects it. The account would still have valid data and could be used maliciously.

Anchor's `close` constraint handles this by zeroing the discriminator, which makes the account unrecognizable to the program even if it's revived.

---

## 8. Error Handling

### Custom Error Codes with `#[error_code]`

```rust
#[error_code]
pub enum TodoError {
    #[msg("Todo content exceeds the 256-byte maximum.")]
    ContentTooLong,        // 6000

    #[msg("Todo content must not be empty.")]
    ContentEmpty,          // 6001

    #[msg("Maximum number of todos reached.")]
    MaxTodosReached,       // 6002
}
```

Anchor assigns error codes starting at 6000. Built-in constraint errors (like `has_one` failing) use codes below 6000.

### Using Custom Errors

There are two common patterns:

```rust
// Pattern 1: require! macro (preferred for simple checks)
require!(content.len() <= 256, TodoError::ContentTooLong);

// Pattern 2: Returning Err (for complex logic)
if content.len() > 256 {
    return Err(TodoError::ContentTooLong.into());
}
```

### Client-Side Error Handling

In TypeScript tests, Anchor automatically maps error codes to names:

```typescript
try {
    await program.methods.addTodo("").rpc();
} catch (err) {
    // err.error.errorCode.code === "ContentEmpty"
    // err.error.errorCode.number === 6001
    // err.error.errorMessage === "Todo content must not be empty."
}
```

---

## 9. Client-Side Queries

### Fetching a Single Account

If you know the PDA seeds, you can derive the address and fetch directly:

```typescript
const [todoItemPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("todo-item"), user.toBuffer(), new BN(0).toArrayLike(Buffer, "le", 8)],
  programId
);

// Fetch and deserialize automatically
const todoItem = await program.account.todoItem.fetch(todoItemPda);
console.log(todoItem.content);     // "Buy milk"
console.log(todoItem.completed);   // false
```

### Fetching All Accounts of a Type

```typescript
// Fetch ALL TodoItem accounts from the program
const allTodoItems = await program.account.todoItem.all();
```

### Filtering with `memcmp`

`getProgramAccounts` is a powerful RPC method that lets you filter accounts by their data. Anchor wraps this:

```typescript
// Fetch all TodoItems belonging to a specific user
const userTodos = await program.account.todoItem.all([
  {
    memcmp: {
      offset: 8,           // Skip the 8-byte discriminator
      bytes: user.toBase58() // Match the authority field (first 32 bytes of data)
    }
  }
]);
```

**How `memcmp` works:**
- `offset`: byte position in the account data to start comparing
- `bytes`: the base58-encoded value to match

Since our `TodoItem` layout is `[8 discriminator][32 authority][8 idx][4+N content][1 completed]`, the authority starts at byte 8.

### Sorting and Pagination

Since `getProgramAccounts` returns all matching accounts, sort client-side:

```typescript
const userTodos = await program.account.todoItem.all([
  { memcmp: { offset: 8, bytes: user.toBase58() } }
]);

// Sort by index
userTodos.sort((a, b) => a.account.idx.toNumber() - b.account.idx.toNumber());

// Pagination (client-side)
const page = userTodos.slice(offset, offset + pageSize);
```

---

## 10. Testing Strategy

### TypeScript Test Structure

Anchor generates a test file using Mocha + Chai. Here's the strategy:

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TodoApp } from "../target/types/todo_app";
import { expect } from "chai";

describe("todo-app", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.TodoApp as Program<TodoApp>;
  const user = provider.wallet;

  it("initializes a todo list", async () => {
    const [todoListPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("todo-list"), user.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeList()
      .accounts({ authority: user.publicKey })
      .rpc();

    const todoList = await program.account.todoList.fetch(todoListPda);
    expect(todoList.authority.equals(user.publicKey)).to.be.true;
    expect(todoList.lastIdx.toNumber()).to.equal(0);
  });

  it("adds a todo item", async () => {
    await program.methods
      .addTodo("Buy groceries")
      .accounts({ authority: user.publicKey })
      .rpc();

    // Fetch and verify...
  });

  it("prevents adding empty content", async () => {
    try {
      await program.methods.addTodo("").accounts({ authority: user.publicKey }).rpc();
      expect.fail("Should have thrown");
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("ContentEmpty");
    }
  });

  it("toggles a todo item", async () => { /* ... */ });
  it("updates a todo item", async () => { /* ... */ });
  it("removes a todo and reclaims rent", async () => { /* ... */ });
  it("prevents unauthorized access", async () => { /* ... */ });
});
```

### What to Test

1. **Happy paths**: Each CRUD operation succeeds with valid inputs
2. **Authorization**: Another user cannot toggle/update/remove your todos
3. **Validation**: Empty content, too-long content, double-init
4. **Account closure**: After remove, the account no longer exists and lamports returned
5. **Edge cases**: Maximum-length content, special characters (UTF-8)

---

## 11. Best Practices

### 1. Use PDAs for User-Scoped Data

Always derive account addresses from the user's pubkey. This gives you:
- Deterministic addressing (no need to store addresses)
- Natural access control (seeds prove ownership)
- Efficient client-side filtering

### 2. Close Accounts When No Longer Needed

Unlike Ethereum where `selfdestruct` is being deprecated, Solana actively encourages closing unused accounts. Users get their rent back, and the network stays lean.

### 3. Limit String and Vector Lengths

Always enforce maximum lengths for variable-size fields. Account space is fixed at creation — if serialization exceeds the space, the transaction fails with an opaque error.

```rust
require!(content.len() <= 256, TodoError::ContentTooLong);
```

### 4. Validate All Inputs

Don't rely on serialization to catch bad data. Explicit validation with custom errors is clearer:

```rust
require!(!content.is_empty(), TodoError::ContentEmpty);
```

### 5. Use `checked_add` / `checked_sub`

Prevent overflow/underflow panics, which would abort the transaction:

```rust
todo_list.last_idx = todo_list.last_idx
    .checked_add(1)
    .ok_or(TodoError::MaxTodosReached)?;
```

### 6. Never Reuse PDA Indices

When you delete a TodoItem, don't decrement `last_idx`. The index is permanently consumed. This prevents PDA collisions and replay attacks.

### 7. Log Meaningful Messages

Use `msg!()` for debugging and for indexers that parse transaction logs:

```rust
msg!("Todo #{} added for {}", todo_item.idx, todo_item.authority);
```

---

## 12. What's Next

In the next modules we'll build on these patterns:
- **Token integration**: Reward users with SPL tokens for completing todos
- **Cross-program invocation (CPI)**: Call the Token Program from your program
- **Advanced PDA patterns**: Multi-level hierarchies (projects → lists → items)

For now, make sure you can:
- [ ] Explain why each todo is its own account
- [ ] Calculate the space for an account with mixed field types
- [ ] Derive a PDA client-side and on-chain
- [ ] Use `close` to reclaim rent
- [ ] Query accounts with `memcmp` filters

---

## Glossary

| Term | Definition |
|------|-----------|
| **PDA** | Program Derived Address — deterministic address with no private key |
| **Discriminator** | 8-byte hash Anchor prepends to every account to identify its type |
| **Rent** | Lamports deposited to keep an account alive (rent-exempt = permanent) |
| **Borsh** | Binary Object Representation Serializer for Hashing — Solana's serialization format |
| **`close`** | Anchor constraint that zeroes an account and transfers lamports to a recipient |
| **`has_one`** | Anchor constraint that validates an account field matches another account's key |
| **`memcmp`** | Memory-compare filter for `getProgramAccounts` — matches bytes at a specific offset |
