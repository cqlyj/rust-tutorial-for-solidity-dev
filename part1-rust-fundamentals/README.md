# Part 1: Rust Fundamentals for Solana Developers

This part teaches you the Rust you need for Solana development. No blockchain tools required -- just `rustc` and `cargo`.

## Why Rust for Solana?

Solana programs (smart contracts) are written in Rust. Unlike Solidity which was purpose-built for EVM, Rust is a general-purpose systems language. This means:

- **More power**: Full control over memory, no garbage collector
- **More safety**: The compiler catches bugs at compile time that Solidity misses
- **More complexity**: Ownership and borrowing have no Solidity equivalent
- **Same idea**: You're still writing deterministic programs that process state

## How Rust Compares to Solidity

| Concept | Solidity | Rust |
|---------|----------|------|
| Variables | `uint256 x = 5;` | `let x: u64 = 5;` |
| Mutability | Mutable by default | Immutable by default (`let mut`) |
| Strings | `string` | `String` (heap) or `&str` (reference) |
| Arrays | `uint256[]` | `Vec<u64>` |
| Mappings | `mapping(address => uint)` | `HashMap<Pubkey, u64>` |
| Structs | `struct Foo { ... }` | `struct Foo { ... }` (similar!) |
| Enums | `enum State { A, B }` | `enum State { A, B(data) }` (can carry data!) |
| Interfaces | `interface IERC20` | `trait ERC20` |
| Errors | `require(x > 0, "msg")` | `Result<T, E>` and `?` operator |
| Visibility | `public/private/internal` | `pub` or private (default) |
| Imports | `import "./Foo.sol"` | `use crate::foo` or `use some_crate` |

## Module Order

Work through these in order -- each builds on the previous:

1. **m01** - Variables, Types & Functions (the basics)
2. **m02** - Ownership & Borrowing (the hard part -- take your time)
3. **m03** - Structs & Enums (data modeling)
4. **m04** - Error Handling (no try-catch in Rust)
5. **m05** - Traits & Generics (polymorphism)
6. **m06** - Collections & Iterators (data structures)
7. **m07** - Modules & Crates (project organization)
8. **m08** - Macros & Attributes (understanding Anchor's magic)

## Running Each Module

```bash
cd m01-variables-types-functions
cargo run          # See the tutorial code in action
cd exercises
cargo run          # Try the exercises (fill in TODOs first)
cd ../solutions
cargo run          # Check your answers
```
