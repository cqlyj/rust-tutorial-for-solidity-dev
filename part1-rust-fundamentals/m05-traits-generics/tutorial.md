# Module 05: Traits and Generics

## Create This Project

```bash
cargo new m05-traits-generics
cd m05-traits-generics
```

---

## Solidity Developer Mental Model

If you come from Solidity, here's the quick mapping:

| Solidity | Rust | Key Difference |
|----------|------|----------------|
| `interface IERC20 { ... }` | `trait Token { ... }` | Rust traits can have default method implementations |
| `contract MyToken is IERC20 { ... }` | `impl Token for MyToken { ... }` | No inheritance hierarchy — just flat implementation |
| No generics | `fn transfer<T: Token>(t: T)` | Static dispatch — the compiler generates specialized code per type, zero runtime cost |
| Dynamic dispatch via interface | `dyn Trait` (boxed trait objects) | Rust *can* do dynamic dispatch, but it's opt-in and explicit |
| `abi.encode(...)` | `BorshSerialize` / `BorshDeserialize` | Solana uses Borsh for deterministic binary serialization |

**Why this matters for Solana**: Anchor programs rely heavily on traits like `AnchorSerialize`, `AnchorDeserialize`, `Accounts`, and custom validation traits. Understanding how traits and generics work is essential before you touch Anchor.

---

## 1. Defining Traits

A trait defines a set of methods that a type must implement. Think of it as a Solidity `interface`, but more powerful.

```rust
// Solidity equivalent:
// interface IValidator {
//     function validate() external view returns (bool);
//     function error_message() external view returns (string memory);
// }

trait Validate {
    fn validate(&self) -> bool;
    fn error_message(&self) -> String;
}
```

- `&self` means the method borrows the implementing type immutably (like `view` in Solidity).
- Every type that `impl Validate` must provide both methods.

---

## 2. Implementing Traits for Types

```rust
struct Account {
    balance: u64,
    owner: String,
}

impl Validate for Account {
    fn validate(&self) -> bool {
        self.balance > 0 && !self.owner.is_empty()
    }

    fn error_message(&self) -> String {
        String::from("Account must have positive balance and a non-empty owner")
    }
}
```

This is like writing `contract Account is IValidator { ... }` in Solidity. The difference is that in Rust you can implement a trait for *any* type, even ones you didn't define — as long as either the trait or the type is local to your crate (the "orphan rule").

---

## 3. Default Method Implementations

Solidity interfaces can't have default implementations (though abstract contracts can). In Rust, traits can:

```rust
trait Validate {
    fn validate(&self) -> bool;

    // Default implementation — implementors can override or keep it
    fn error_message(&self) -> String {
        String::from("Validation failed")
    }

    // Default method that calls other trait methods
    fn validate_or_panic(&self) {
        if !self.validate() {
            panic!("{}", self.error_message());
        }
    }
}
```

Now any type implementing `Validate` only *needs* to implement `validate()`. The other two methods come for free (but can be overridden).

---

## 4. Traits as Function Parameters

### The `impl Trait` Syntax

```rust
fn print_validation(item: &impl Validate) {
    if item.validate() {
        println!("Valid!");
    } else {
        println!("Invalid: {}", item.error_message());
    }
}
```

`&impl Validate` means "any reference to a type that implements `Validate`." The compiler monomorphizes this — it generates a separate version of the function for each concrete type you call it with. **Zero runtime cost**, unlike Solidity's dynamic dispatch through interfaces.

### The Generic Syntax (Equivalent)

```rust
fn print_validation<T: Validate>(item: &T) {
    if item.validate() {
        println!("Valid!");
    } else {
        println!("Invalid: {}", item.error_message());
    }
}
```

These two forms are equivalent. The generic syntax is more flexible when you need the same type in multiple positions:

```rust
// Both items must be the SAME concrete type
fn compare<T: Validate>(a: &T, b: &T) -> bool {
    a.validate() == b.validate()
}
```

---

## 5. Trait Bounds with Generics

### Single Bound

```rust
fn process<T: Validate>(item: T) { ... }
```

### Multiple Bounds with `+`

```rust
fn process<T: Validate + std::fmt::Display>(item: T) {
    println!("Processing: {}", item); // Display
    if !item.validate() {             // Validate
        println!("Warning: invalid item");
    }
}
```

### `where` Clauses (For Complex Bounds)

When bounds get long, use `where`:

```rust
fn complex_process<T, U>(t: T, u: U) -> bool
where
    T: Validate + Clone + std::fmt::Debug,
    U: Validate + Default,
{
    let t_clone = t.clone();
    t_clone.validate() && u.validate()
}
```

This is purely a readability choice — it's identical to putting bounds after the generic parameter.

---

## 6. Returning Types That Implement Traits

```rust
fn create_default_account() -> impl Validate {
    Account {
        balance: 100,
        owner: String::from("system"),
    }
}
```

The caller knows the return type implements `Validate`, but doesn't know (or care) it's an `Account`. This is useful for hiding implementation details.

**Limitation**: You can only return a *single* concrete type from an `-> impl Trait` function. You can't conditionally return different types.

---

## 7. Common Standard Library Traits

These are the traits you'll use every day in Rust:

### `Display` — Human-Readable Output

```rust
use std::fmt;

struct Token {
    symbol: String,
    amount: u64,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.symbol)
    }
}
// Now you can do: println!("{}", my_token);
```

### `Debug` — Developer-Facing Output

Usually derived:

```rust
#[derive(Debug)]
struct Token {
    symbol: String,
    amount: u64,
}
// Now you can do: println!("{:?}", my_token);
```

### `Clone` and `Copy`

- `Clone`: Explicit deep copy via `.clone()`. Any type can implement this.
- `Copy`: Implicit bitwise copy. Only for simple stack-only types (integers, bools, etc.). `Copy` requires `Clone`.

```rust
#[derive(Clone, Copy)]  // Only works because both fields are Copy
struct Point {
    x: f64,
    y: f64,
}
```

Types with heap data (like `String`, `Vec`) cannot be `Copy`.

### `Default`

```rust
#[derive(Default)]
struct Config {
    max_retries: u32,    // defaults to 0
    timeout_ms: u64,     // defaults to 0
    verbose: bool,       // defaults to false
}

let config = Config::default();
```

### `PartialEq`, `Eq`

```rust
#[derive(PartialEq, Eq)]
struct AccountId(u64);

// Now you can: if account_a == account_b { ... }
```

- `PartialEq`: Allows `==` and `!=`. Can be partial (like `f64` — `NaN != NaN`).
- `Eq`: Marker trait saying equality is total (reflexive). Required for `HashMap` keys.

### `Hash`

```rust
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct AccountId(u64);

let mut balances: HashMap<AccountId, u64> = HashMap::new();
```

---

## 8. Derive Macros

`#[derive(...)]` auto-generates trait implementations. This is the Rust equivalent of "the compiler writes the boilerplate for you."

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Transaction {
    from: String,
    to: String,
    amount: u64,
    memo: Option<String>,
}
```

**Rule of thumb**: Derive everything you reasonably can. It's free, it's correct, and it saves you from writing boring code.

Common derivable traits: `Debug`, `Clone`, `Copy`, `Default`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`.

---

## 9. Generics on Structs and Enums

### Generic Structs

```rust
struct Wallet<T> {
    owner: String,
    balance: T,  // Could be u64, f64, a BigInt type, etc.
}

impl<T> Wallet<T> {
    fn new(owner: String, balance: T) -> Self {
        Wallet { owner, balance }
    }
}

// Constrained impl — only for types that can be displayed
impl<T: std::fmt::Display> Wallet<T> {
    fn print_balance(&self) {
        println!("{}'s balance: {}", self.owner, self.balance);
    }
}
```

### Generic Enums

You already know `Option<T>` and `Result<T, E>` — these are generic enums:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Option<T> {
    Some(T),
    None,
}
```

You can define your own:

```rust
enum TransactionResult<T> {
    Success(T),
    InsufficientFunds(u64),  // how much was needed
    InvalidAccount,
}
```

---

## 10. Trait Objects and Dynamic Dispatch

### Static Dispatch (Generics) — The Default

```rust
fn validate_item<T: Validate>(item: &T) {
    // Compiler generates one version per concrete type.
    // Fast — no vtable lookup at runtime.
}
```

### Dynamic Dispatch (`dyn Trait`) — When You Need Heterogeneous Collections

```rust
fn validate_item(item: &dyn Validate) {
    // Uses a vtable pointer at runtime to find the right method.
    // Slower, but allows mixed types.
}
```

The real use case — storing different types in one collection:

```rust
let items: Vec<Box<dyn Validate>> = vec![
    Box::new(account),
    Box::new(transaction),
    Box::new(program_id),
];

for item in &items {
    if !item.validate() {
        println!("Error: {}", item.error_message());
    }
}
```

In Solidity, *all* interface calls are dynamically dispatched. Rust makes you choose — and generics (static dispatch) should be your default.

---

## 11. The `Sized` Trait and Why `dyn Trait` Needs `Box`

Every Rust type is `Sized` by default — the compiler knows its size at compile time. But `dyn Validate` is *not* `Sized`: it could be any type that implements `Validate`, and those types might have different sizes.

You can't put an unsized type on the stack:

```rust
// Won't compile — compiler doesn't know the size
let item: dyn Validate = ???;
```

You need a pointer — and `Box` is the most common one:

```rust
let item: Box<dyn Validate> = Box::new(some_account);
```

Other options: `&dyn Validate` (borrowed reference) or `Arc<dyn Validate>` (shared ownership).

**Solidity parallel**: In Solidity, everything is behind a pointer anyway (contract addresses). Rust makes the indirection explicit.

---

## 12. Solana-Relevant Traits (Preview)

When you start writing Solana programs with Anchor, you'll encounter these traits constantly:

### `BorshSerialize` / `BorshDeserialize`

Borsh (Binary Object Representation Serializer for Hashing) is the serialization format Solana uses. These traits convert your structs to/from bytes for on-chain storage.

```rust
// You'll write this in Anchor programs:
#[derive(BorshSerialize, BorshDeserialize)]
pub struct GameState {
    pub player: Pubkey,
    pub score: u64,
    pub is_active: bool,
}
```

This is like `abi.encode()` / `abi.decode()` in Solidity, but deterministic and more efficient.

### `AnchorSerialize` / `AnchorDeserialize`

Anchor's wrapper around Borsh with additional features for the framework:

```rust
#[account]
pub struct UserProfile {
    pub authority: Pubkey,
    pub username: String,
    pub created_at: i64,
}
// The #[account] macro derives AnchorSerialize, AnchorDeserialize, and more
```

### `Accounts` Trait

Defines how to deserialize and validate a set of accounts passed to an instruction:

```rust
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

You don't write these traits by hand — Anchor's macros do the heavy lifting. But understanding that they're *traits* helps you debug when things go wrong.

---

## Best Practices

1. **Prefer generics (static dispatch) over trait objects (dynamic dispatch).**
   Generics are faster and let the compiler catch more errors. Use `dyn Trait` only when you genuinely need a heterogeneous collection.

2. **Derive everything you can.**
   `#[derive(Debug, Clone, PartialEq)]` is free. Do it. You'll thank yourself when debugging.

3. **Use trait bounds to constrain types.**
   Don't accept `T` when you mean `T: Display + Validate`. The compiler errors are better, and your intent is clearer.

4. **Use `where` clauses for readability.**
   If your function signature is getting long, move bounds to a `where` clause.

5. **Implement `Display` for your types.**
   It makes `println!("{}", my_thing)` work and is far more useful than raw `Debug` output.

6. **Know the orphan rule.**
   You can only implement a trait for a type if either the trait or the type is defined in your crate. When you need to work around this, use the newtype pattern (wrap the foreign type in a single-field struct).

---

## Key Takeaways

- Traits are Rust's version of interfaces — but with default implementations, static dispatch, and no inheritance hierarchy.
- Generics give you zero-cost abstraction. The compiler generates specialized code for each type.
- `dyn Trait` is opt-in dynamic dispatch for when you need heterogeneous collections.
- `#[derive(...)]` is your best friend — use it liberally.
- Solana/Anchor uses traits *everywhere*: serialization, deserialization, account validation. This module prepares you for all of it.
