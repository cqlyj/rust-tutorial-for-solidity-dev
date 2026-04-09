# Part 2: Solana Core Concepts

This part teaches you how Solana works under the hood, before you use any framework. Understanding these concepts will make you a much better Anchor developer.

## The Big Mental Shift: EVM vs Solana

| | EVM (Solidity) | Solana (Rust) |
|---|---------------|---------------|
| **Code + State** | Combined in one contract | Separated: programs (code) + accounts (data) |
| **State Access** | Storage slots, implicit | Accounts passed explicitly to every instruction |
| **Caller Identity** | `msg.sender` built-in | Signer accounts passed and verified |
| **Execution** | Sequential | Parallel (accounts declare upfront) |
| **Fee Model** | Gas per opcode | Compute units + rent for storage |
| **Deployment** | Contract address from CREATE/CREATE2 | Program ID = keypair pubkey |
| **Upgrades** | Proxy pattern (hack) | Built-in upgrade authority |

## Prerequisites

- Completed Part 1 (Rust fundamentals)
- Solana CLI installed: `solana --version`
- Local keypair: `solana-keygen new`

## Module Order

1. **m09** - Solana vs EVM Architecture (conceptual, no code)
2. **m10** - Native Hello World (your first on-chain program)
3. **m11** - Program Derived Addresses (deterministic account addresses)
4. **m12** - Cross-Program Invocation (calling other programs)
