# Module 04: Error Handling

## The Big Picture: No Exceptions in Rust

If you're coming from Solidity, you're used to this pattern:

```solidity
function withdraw(uint256 amount) external {
    require(msg.sender == owner, "Not owner");           // revert with message
    require(balance >= amount, "Insufficient balance");   // revert with message
    assert(balance - amount <= balance);                  // panic on invariant violation
    
    balance -= amount;
    payable(msg.sender).transfer(amount);
}

// Or with custom errors (Solidity 0.8.4+):
error NotOwner();
error InsufficientBalance(uint256 requested, uint256 available);

function withdraw(uint256 amount) external {
    if (msg.sender != owner) revert NotOwner();
    if (balance < amount) revert InsufficientBalance(amount, balance);
    // ...
}
```

**In Rust, there are NO exceptions. No try/catch. No revert.** Instead, errors are **values** returned from functions using `Result<T, E>`. This is a fundamental paradigm shift:

| Solidity | Rust |
|----------|------|
| `require(condition, "msg")` | Return `Err(MyError::SomeVariant)` or use `?` |
| `revert CustomError()` | Return `Err(MyError::SomeVariant)` |
| `assert(condition)` | `panic!("msg")` (kills the program, not recoverable) |
| `try/catch` | `match` on `Result<T, E>` |
| Function succeeds silently | Function returns `Ok(value)` |
| `address(0)` / missing value | `Option<T>` → `None` |

This means **every function that can fail must declare that in its return type**. The compiler forces you to handle errors. You can't ignore them.

## Creating This Project

```bash
cargo new m04-error-handling
cd m04-error-handling
```

Then add `thiserror` to your `Cargo.toml` under `[dependencies]`:
```toml
[dependencies]
thiserror = "2"
```

---

## 1. `panic!` — Unrecoverable Errors

`panic!` is like Solidity's `assert` failure — it immediately kills the program. There's no catching it (in normal usage). Use it when something is so wrong that continuing would be dangerous.

```rust
fn main() {
    panic!("Something went catastrophically wrong!");
    // Program dies here. Nothing after this runs.
}
```

When does Rust panic automatically?
- Array out-of-bounds access: `let v = vec![1,2,3]; v[99];`
- Integer overflow in debug mode
- Calling `.unwrap()` on `None` or `Err`

**Solidity parallel**: `assert(false)` burns all remaining gas and reverts. `panic!` in Rust kills the process entirely.

**Rule**: Never use `panic!` for expected errors (like invalid user input). Use `Result` instead.

---

## 2. `Result<T, E>` — The Core of Rust Error Handling

`Result` is an enum with two variants:

```rust
enum Result<T, E> {
    Ok(T),   // Success — contains the value
    Err(E),  // Failure — contains the error
}
```

Every function that can fail returns `Result<SuccessType, ErrorType>`:

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))  // Like: revert("Division by zero")
    } else {
        Ok(a / b)  // Like: return a / b (but explicit success)
    }
}
```

**Solidity parallel**: Imagine if every Solidity function had to return `(bool success, bytes memory result)` and the compiler forced you to check `success` before using `result`. That's `Result<T, E>`.

---

## 3. `Option<T>` — Values That Might Not Exist

`Option` is for when a value might be absent. No null pointers, no `address(0)` hacks:

```rust
enum Option<T> {
    Some(T),  // Value exists
    None,     // No value
}
```

```rust
fn find_user(id: u64) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None  // User not found — no panic, no error, just "nothing"
    }
}
```

**Solidity parallel**: In Solidity, you might return `address(0)` to indicate "not found," and then hope the caller checks. In Rust, `Option` forces the caller to handle the `None` case.

---

## 4. `match` on Result and Option — Explicit Handling

`match` is how you extract values from `Result` and `Option`:

```rust
// Handling Result
let result = divide(10.0, 0.0);
match result {
    Ok(value) => println!("Result: {}", value),
    Err(e) => println!("Error: {}", e),
}

// Handling Option
let user = find_user(42);
match user {
    Some(name) => println!("Found: {}", name),
    None => println!("User not found"),
}
```

The compiler ensures you handle **both** cases. You can't forget to check for errors.

---

## 5. `unwrap()` and `expect()` — Quick and Dirty

`unwrap()` extracts the value from `Ok`/`Some`, but **panics** if it's `Err`/`None`:

```rust
let value = divide(10.0, 2.0).unwrap();  // 5.0 — fine
let crash = divide(10.0, 0.0).unwrap();  // PANIC! Program dies.
```

`expect()` is the same but with a custom panic message:

```rust
let crash = divide(10.0, 0.0).expect("Division failed");
// PANIC: "Division failed: Division by zero"
```

**Solidity parallel**: `unwrap()` is like accessing a mapping value without checking if it exists, then being surprised when your transaction reverts. Except in Rust, it kills your whole program.

**Rule**: NEVER use `unwrap()` in production code. It's fine for quick prototypes and tests. In Solana programs, it will abort the entire transaction.

---

## 6. The `?` Operator — Early Return on Error

The `?` operator is Rust's elegant version of `require()`. It says: "If this is `Err`, return the error immediately. If it's `Ok`, unwrap the value and continue."

```rust
fn process_transfer(from: &str, to: &str, amount: u64) -> Result<String, String> {
    let balance = get_balance(from)?;        // Returns Err early if this fails
    validate_amount(amount)?;                 // Returns Err early if this fails
    check_sufficient_funds(balance, amount)?; // Returns Err early if this fails
    
    Ok(format!("Transferred {} from {} to {}", amount, from, to))
}
```

Without `?`, you'd need nested `match` statements everywhere. The `?` operator makes error propagation clean and linear — just like a series of `require()` statements in Solidity:

```solidity
// Solidity equivalent of the above:
function processTransfer(address from, address to, uint256 amount) external {
    uint256 balance = getBalance(from);           // reverts internally if fails
    require(amount > 0, "Invalid amount");        // like validate_amount()?
    require(balance >= amount, "Insufficient");   // like check_sufficient_funds()?
    // ... transfer logic
}
```

**Important**: You can only use `?` inside functions that return `Result` (or `Option`).

---

## 7. Combinators — Functional Error Handling

Rust provides methods on `Result` and `Option` for concise transformations:

```rust
// map — transform the Ok/Some value
let doubled: Result<f64, String> = divide(10.0, 2.0).map(|v| v * 2.0);

// and_then — chain operations that can also fail
let result = divide(10.0, 2.0).and_then(|v| divide(v, 3.0));

// unwrap_or — provide a default on error
let safe: f64 = divide(10.0, 0.0).unwrap_or(0.0);

// unwrap_or_else — compute default lazily
let safe: f64 = divide(10.0, 0.0).unwrap_or_else(|e| {
    println!("Error occurred: {}", e);
    0.0
});

// Option combinators
let name: String = find_user(42).unwrap_or(String::from("Anonymous"));
let upper: Option<String> = find_user(1).map(|n| n.to_uppercase());
```

**Solidity parallel**: These are like chaining modifiers, but for return values. There's no direct Solidity equivalent — this is one of Rust's superpowers.

---

## 8. Custom Error Types with Enums

In production Rust (and especially Solana programs), you define custom error enums:

```rust
#[derive(Debug)]
enum VaultError {
    NotOwner,
    InsufficientBalance { requested: u64, available: u64 },
    VaultLocked,
    InvalidAmount,
}
```

**Solidity parallel**: This is exactly like custom errors in Solidity 0.8.4+:

```solidity
error NotOwner();
error InsufficientBalance(uint256 requested, uint256 available);
error VaultLocked();
error InvalidAmount();
```

To use your custom error as a proper Rust error, you implement two traits:

### `Display` — How the error looks when printed

```rust
impl std::fmt::Display for VaultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VaultError::NotOwner => write!(f, "Caller is not the owner"),
            VaultError::InsufficientBalance { requested, available } => {
                write!(f, "Insufficient balance: requested {}, available {}", requested, available)
            }
            VaultError::VaultLocked => write!(f, "Vault is currently locked"),
            VaultError::InvalidAmount => write!(f, "Invalid amount"),
        }
    }
}
```

### `Error` — Marks it as a standard error type

```rust
impl std::error::Error for VaultError {}
```

---

## 9. The `From` Trait — Automatic Error Conversion

When your function can produce different error types, `From` lets the `?` operator auto-convert them:

```rust
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Custom(String),
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self {
        AppError::Parse(e)
    }
}
```

Now `?` automatically converts these error types:

```rust
fn read_number_from_file(path: &str) -> Result<i64, AppError> {
    let content = std::fs::read_to_string(path)?;  // io::Error → AppError via From
    let number = content.trim().parse::<i64>()?;    // ParseIntError → AppError via From
    Ok(number)
}
```

---

## 10. `thiserror` Crate — Derive All the Boilerplate

Writing `Display` and `From` by hand is tedious. The `thiserror` crate does it for you:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum VaultError {
    #[error("Caller is not the owner")]
    NotOwner,

    #[error("Insufficient balance: requested {requested}, available {available}")]
    InsufficientBalance { requested: u64, available: u64 },

    #[error("Vault is currently locked")]
    VaultLocked,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),  // Auto-implements From<std::io::Error>

    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),  // Auto-implements From<ParseIntError>
}
```

That single `#[derive(Error)]` + `#[error("...")]` replaces dozens of lines of manual trait implementations. `thiserror` is used extensively in Solana ecosystem crates.

---

## 11. How Solana Programs Return Errors

In Solana, every instruction handler returns `ProgramResult`:

```rust
// From the solana_program crate:
pub type ProgramResult = Result<(), ProgramError>;
```

This means: "Either succeed with nothing (`Ok(())`), or fail with a `ProgramError`." The `ProgramError` enum includes variants like:

```rust
pub enum ProgramError {
    Custom(u32),              // Your custom error codes
    InvalidArgument,
    InvalidInstructionData,
    InsufficientFunds,
    AccountAlreadyInitialized,
    // ... many more
}
```

A raw Solana instruction handler looks like:

```rust
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Validate the first account is the signer
    let account = &accounts[0];
    if !account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Everything passed — succeed
    Ok(())
}
```

---

## 12. Anchor's `#[error_code]` — Preview

Anchor (the most popular Solana framework) simplifies custom errors:

```rust
// In an Anchor program:
#[error_code]
pub enum MyProgramError {
    #[msg("You are not authorized to perform this action")]
    Unauthorized,

    #[msg("Insufficient funds for this transfer")]
    InsufficientFunds,

    #[msg("The account has already been initialized")]
    AlreadyInitialized,
}

// Usage in an instruction:
pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require!(ctx.accounts.vault.balance >= amount, MyProgramError::InsufficientFunds);
    // ...
    Ok(())
}
```

Notice how Anchor's `require!` macro brings back Solidity-like `require()` syntax, but under the hood it's all `Result<T, E>`. This is the pattern you'll use daily when writing Solana programs.

---

## Best Practices

1. **Never use `unwrap()` in production** — always handle errors with `?`, `match`, or combinators
2. **Define custom error enums** — don't use `String` as your error type in real projects
3. **Use `?` everywhere** — it's the idiomatic way to propagate errors
4. **Use `thiserror`** — don't hand-write `Display` and `From` implementations
5. **Make errors informative** — include context (like Solidity's error parameters)
6. **Use `expect()` only when you can prove it won't fail** — document why with the message
7. **For Solana**: every instruction returns `Result<(), ProgramError>` or Anchor's `Result<()>`

---

## Running the Code

```bash
# Main tutorial code
cargo run

# Try the exercises
cd exercises
cargo run    # Fill in the TODOs first!

# Check solutions
cd ../solutions
cargo run
```
