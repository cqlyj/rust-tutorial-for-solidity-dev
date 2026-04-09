# Part 3: Anchor Framework

Anchor is the standard framework for Solana development, used by the vast majority of production programs. It reduces boilerplate by ~60% compared to native Solana development.

## What Anchor Does for You

| Without Anchor (Native) | With Anchor |
|-------------------------|-------------|
| Manual account deserialization | `#[account]` auto-serializes |
| Manual signer checks | `Signer<'info>` type |
| Manual PDA validation | `seeds` and `bump` constraints |
| Manual rent exemption checks | `init` constraint handles it |
| Manual instruction routing | `#[program]` macro routes automatically |
| Hand-written IDL | Auto-generated from code |

Think of it like the difference between raw EVM opcodes and Solidity -- Anchor is your high-level language for Solana.

## Prerequisites

- Completed Parts 1 and 2
- Anchor CLI installed: `anchor --version`
- Node.js installed (for testing): `node --version`

## How to Create an Anchor Project from Scratch

```bash
# Create a new Anchor project
anchor init my-project
cd my-project

# What anchor created:
# my-project/
# ├── Anchor.toml           # Project config (like hardhat.config.js)
# ├── Cargo.toml             # Rust workspace
# ├── programs/              # Your on-chain programs
# │   └── my-project/
# │       ├── Cargo.toml
# │       └── src/
# │           └── lib.rs     # Your program code
# ├── tests/                 # TypeScript tests
# │   └── my-project.ts
# ├── app/                   # Frontend (optional)
# └── migrations/
#     └── deploy.ts

# Build the program
anchor build

# Run tests (starts local validator automatically)
anchor test

# Deploy to localnet
solana-test-validator &   # Start local validator
anchor deploy
```

## Module Order

1. **m13** - Counter Program (Hello Anchor)
2. **m14** - Account Constraints (validation deep dive)
3. **m15** - Todo App (full CRUD operations)
4. **m16** - Token Operations (SPL tokens, like ERC-20)
5. **m17** - Escrow Program (capstone project)
