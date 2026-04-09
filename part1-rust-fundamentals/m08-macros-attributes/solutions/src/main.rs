// =============================================================================
// Module 08: Macros and Attributes — SOLUTIONS
// =============================================================================
// Every exercise solved with every line commented.
// Run with: cargo run
// Test with: cargo test
// =============================================================================

// =============================================================================
// Exercise 1 Solution: Write a Simple Declarative Macro
// =============================================================================
// The `double` macro takes one expression and multiplies it by 2.
// macro_rules! defines a declarative (pattern-matching) macro.
macro_rules! double {
    // $val:expr captures any Rust expression (numbers, math, function calls, etc.)
    ($val:expr) => {
        // The expansion: multiply the captured expression by 2
        // Wrapping in parentheses ensures correct operator precedence
        ($val) * 2
    };
}

// =============================================================================
// Exercise 2 Solution: Macro with Repetition
// =============================================================================
// The `sum_all` macro takes one or more expressions and adds them together.
macro_rules! sum_all {
    // $( $x:expr ),+ means "one or more expressions separated by commas"
    // The + (not *) means at least one argument is required
    ( $( $x:expr ),+ ) => {
        // Start with 0 and add each captured expression
        // $( + $x )+ repeats "+ $x" for each captured value
        // So sum_all!(1, 2, 3) expands to: 0 + 1 + 2 + 3
        0 $( + $x )+
    };
}

// =============================================================================
// Exercise 3 Solution: Using Derive Macros
// =============================================================================
// #[derive(Debug)] — enables {:?} formatting for println!
// #[derive(Clone)] — enables .clone() to create deep copies
// #[derive(PartialEq)] — enables == and != comparison operators
#[derive(Debug, Clone, PartialEq)]
struct TokenAccount {
    mint: String,   // the token mint address (like an ERC-20 contract address)
    owner: String,  // who owns this token account (like msg.sender)
    amount: u64,    // token balance in smallest unit (like wei for ETH)
}

// Debug enables {:?} printing, PartialEq enables == comparison
#[derive(Debug, PartialEq)]
enum TransferStatus {
    Pending,          // transfer is queued but not executed
    Completed,        // transfer finished successfully
    Failed(String),   // transfer failed with an error message
}

// =============================================================================
// Exercise 4 Solution: Understanding What #[derive()] Does
// =============================================================================
// Price struct already has derive macros — we just use the traits they provide.
#[derive(Debug, Clone, PartialEq)]
struct Price {
    asset: String,   // the asset ticker symbol (e.g., "SOL", "BTC")
    usd_cents: u64,  // price in US cents (e.g., 15000 = $150.00)
}

// This function exercises all three derived traits: Debug, Clone, PartialEq
fn price_operations() -> bool {
    // Step 1: Create a Price for SOL at $150.00 (15000 cents)
    let sol_price = Price {
        asset: String::from("SOL"), // Solana's native token
        usd_cents: 15000,           // $150.00 expressed in cents
    };

    // Step 2: Clone creates a deep copy — both String and u64 are cloned
    let sol_copy = sol_price.clone(); // .clone() works because of #[derive(Clone)]

    // Step 3: PartialEq lets us compare with assert_eq!
    // This checks that all fields are equal (asset == asset AND usd_cents == usd_cents)
    assert_eq!(sol_price, sol_copy); // passes because clone creates an identical copy

    // Step 4: Debug lets us use {:?} format specifier
    // This prints something like: Price { asset: "SOL", usd_cents: 15000 }
    println!("  Price (Debug): {:?}", sol_price); // {:?} calls the Debug trait's fmt method

    // Step 5: Create a different price and verify inequality
    let btc_price = Price {
        asset: String::from("BTC"), // Bitcoin — different asset
        usd_cents: 6_000_000,       // $60,000.00 — different price
    };
    // assert_ne! uses PartialEq's != operator (the negation of ==)
    assert_ne!(sol_price, btc_price); // passes because the structs have different field values

    // All assertions passed — return true to signal success
    true
}

// =============================================================================
// Exercise 5 Solution: Conditional Compilation
// =============================================================================

// Version 1: compiled ONLY when running `cargo test`
// #[cfg(test)] means "include this item only in test builds"
#[cfg(test)]
fn get_environment() -> &'static str {
    "test" // returned when running `cargo test`
}

// Version 2: compiled when NOT running tests (i.e., `cargo run`)
// #[cfg(not(test))] means "include this item only in non-test builds"
#[cfg(not(test))]
fn get_environment() -> &'static str {
    "production" // returned when running `cargo run`
}

// cfg!() is a macro (not an attribute) that evaluates conditions at compile time
// It returns a bool — true if the condition matches, false otherwise
fn is_debug_build() -> bool {
    // debug_assertions is true for `cargo build` / `cargo run` (debug mode)
    // debug_assertions is false for `cargo build --release` (release mode)
    cfg!(debug_assertions) // returns true or false at compile time
}

// =============================================================================
// Exercise 6 Solution: Reading Anchor-Like Macro Code
// =============================================================================

// This macro stores a program ID as a const — like Anchor's declare_id!
macro_rules! declare_program_id {
    // $id:expr captures the string literal passed to the macro
    ($id:expr) => {
        // Expands to a const declaration with the captured value
        const PROGRAM_ID: &str = $id;
    };
}

// Use the macro to set our program's ID
// In real Anchor, this would be a base58-encoded public key
declare_program_id!("MyProgram111111111111111111111111");

// Simulated msg! macro — logs with a program prefix like Solana does
macro_rules! sol_msg {
    // $($arg:tt)* captures ALL tokens (any number, any type)
    // This is the most flexible capture — it passes everything to format!
    ($($arg:tt)*) => {
        // Wrap the formatted message with Solana's log prefix
        println!("[Program log:] {}", format!($($arg)*));
    };
}

// This struct simulates an on-chain account (like an Anchor #[account] struct)
#[derive(Debug)]
struct VaultAccount {
    balance: u64,         // lamports (SOL's smallest unit) stored in the vault
    authority: String,    // the public key authorized to manage the vault (like Solidity's owner)
    is_initialized: bool, // tracks whether the account has been set up (Anchor does this automatically)
}

// This function simulates what an Anchor deposit instruction handler does
fn simulate_anchor_deposit(vault: &mut VaultAccount, amount: u64, signer: &str) -> Result<(), String> {
    // Check 1: Vault must be initialized before accepting deposits
    // In Anchor, #[account(constraint = vault.is_initialized)] does this automatically
    if !vault.is_initialized {
        return Err("Vault not initialized".to_string()); // early return with error
    }

    // Check 2: Only the vault authority can deposit
    // In Anchor, this is handled by a `has_one = authority` constraint
    if signer != vault.authority {
        return Err("Unauthorized".to_string()); // signer doesn't match the authority
    }

    // Check 3: Add amount with overflow protection
    // checked_add returns None if the addition would overflow u64::MAX
    // In Solidity, this is like using SafeMath (pre-0.8) or built-in overflow checks (0.8+)
    vault.balance = vault.balance
        .checked_add(amount)       // returns Option<u64> — Some(result) or None on overflow
        .ok_or("Overflow".to_string())?;  // convert None to Err("Overflow"), ? propagates it

    // Log the successful deposit using our sol_msg! macro
    // In real Anchor, msg!() writes to the transaction log (viewable in Solana Explorer)
    sol_msg!("Deposited {} lamports. New balance: {}", amount, vault.balance);

    // Return Ok(()) to indicate success — no data to return, just confirmation
    Ok(())
}

// =============================================================================
// Exercise 7 Solution: Build a Macro That Generates an Impl Block
// =============================================================================

// This macro generates a std::fmt::Display implementation for any struct.
// Display is what gets called when you use {} (not {:?}) in format strings.
// Note: we accept field names as $field:ident (not self.field expressions)
// because Rust macro hygiene prevents passing `self` through macro parameters.
macro_rules! impl_display {
    // $name:ident — the struct name (e.g., Validator)
    // $fmt:expr — the format string (e.g., "Validator {{ name: {}, stake: {} }}")
    // $( $field:ident ),* — field names to access on self
    ($name:ident, $fmt:expr, $( $field:ident ),*) => {
        // Generate the Display trait implementation
        impl std::fmt::Display for $name {
            // fmt is called by println!("{}", instance) and format!("{}", instance)
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // write! is like format! but writes to the formatter instead of creating a String
                // self.$field accesses each field — self is available here inside the method
                write!(f, $fmt, $( self.$field ),*)
            }
        }
    };
}

// A struct representing a Solana validator (uses Debug for {:?} formatting)
#[derive(Debug)]
struct Validator {
    name: String, // the validator's human-readable name
    stake: u64,   // the amount of SOL staked with this validator (in lamports)
}

// Use our macro to generate Display for Validator
// The {{ and }} in the format string produce literal { and } in the output
// We pass field names (not self.field) — the macro adds self. internally
impl_display!(Validator, "Validator {{ name: {}, stake: {} }}", name, stake);

// =============================================================================
// Main — runs all solved exercises
// =============================================================================

fn main() {
    println!("=== Module 08 Solutions: Macros and Attributes ===\n");

    // Exercise 1: double! macro
    println!("Exercise 1: double! macro");
    println!("  double!(5) = {}", double!(5));           // expands to (5) * 2 = 10
    println!("  double!(3+1) = {}", double!(3 + 1));     // expands to (3 + 1) * 2 = 8
    assert_eq!(double!(5), 10);                           // verify the result
    assert_eq!(double!(3 + 1), 8);                        // verify with compound expression
    println!("  ✓ Exercise 1 passed!\n");

    // Exercise 2: sum_all! macro
    println!("Exercise 2: sum_all! macro");
    println!("  sum_all!(1, 2, 3) = {}", sum_all!(1, 2, 3));   // 0 + 1 + 2 + 3 = 6
    println!("  sum_all!(10, 20) = {}", sum_all!(10, 20));      // 0 + 10 + 20 = 30
    println!("  sum_all!(5) = {}", sum_all!(5));                 // 0 + 5 = 5
    assert_eq!(sum_all!(1, 2, 3), 6);                            // verify three args
    assert_eq!(sum_all!(10, 20), 30);                            // verify two args
    assert_eq!(sum_all!(5), 5);                                  // verify single arg
    println!("  ✓ Exercise 2 passed!\n");

    // Exercise 3: derive macros
    println!("Exercise 3: Derive Macros");
    let account = TokenAccount {
        mint: String::from("So11111111111111111111111111111112"),  // wrapped SOL mint
        owner: String::from("7xKXxyz..."),                        // fake owner address
        amount: 1_000_000_000,                                    // 1 billion smallest units
    };
    let account_copy = account.clone();                  // Clone creates a deep copy
    println!("  Account: {:?}", account);                // Debug enables {:?} formatting
    assert_eq!(account, account_copy);                   // PartialEq enables == comparison
    let status = TransferStatus::Completed;              // create an enum variant
    println!("  Status: {:?}", status);                  // Debug works on enums too
    assert_eq!(status, TransferStatus::Completed);       // PartialEq works on enums too
    println!("  ✓ Exercise 3 passed!\n");

    // Exercise 4: using derived traits
    println!("Exercise 4: Using Derived Traits");
    assert!(price_operations());                         // runs all the trait demonstrations
    println!("  ✓ Exercise 4 passed!\n");

    // Exercise 5: conditional compilation
    println!("Exercise 5: Conditional Compilation");
    println!("  Environment: {}", get_environment());    // "production" in cargo run, "test" in cargo test
    println!("  Debug build: {}", is_debug_build());     // true for cargo run, false for cargo run --release
    println!("  ✓ Exercise 5 passed!\n");

    // Exercise 6: Anchor-like patterns
    println!("Exercise 6: Anchor-Like Patterns");
    println!("  Program ID: {}", PROGRAM_ID);            // set by declare_program_id! macro

    // Test valid deposit
    let mut vault = VaultAccount {
        balance: 1_000_000,                              // initial balance: 1 million lamports
        authority: String::from("admin"),                 // only "admin" can deposit
        is_initialized: true,                            // vault is ready to use
    };
    assert!(simulate_anchor_deposit(&mut vault, 500_000, "admin").is_ok()); // valid deposit
    assert_eq!(vault.balance, 1_500_000);                // balance increased by 500k

    // Test unauthorized deposit (should fail)
    assert!(simulate_anchor_deposit(&mut vault, 100, "hacker").is_err()); // wrong signer

    // Test uninitialized vault (should fail)
    let mut uninit_vault = VaultAccount {
        balance: 0,                                      // empty balance
        authority: String::from("admin"),                 // correct authority
        is_initialized: false,                           // NOT initialized — will fail
    };
    assert!(simulate_anchor_deposit(&mut uninit_vault, 100, "admin").is_err()); // vault not ready
    println!("  ✓ Exercise 6 passed!\n");

    // Exercise 7: impl_display macro
    println!("Exercise 7: impl_display Macro");
    let validator = Validator {
        name: String::from("SuperValidator"),             // validator name
        stake: 10_000_000,                                // 10 million lamports staked
    };
    println!("  {}", validator);                          // uses Display trait (not Debug)
    // Display output: Validator { name: SuperValidator, stake: 10000000 }
    println!("  ✓ Exercise 7 passed!\n");

    println!("=== All solutions verified! ===");
}

// =============================================================================
// Tests — verify all solutions
// =============================================================================

#[cfg(test)]
mod tests {
    // Import everything from the parent module
    use super::*;

    // Exercise 1 tests
    #[test]
    fn test_double_macro() {
        assert_eq!(double!(5), 10);         // basic integer
        assert_eq!(double!(0), 0);          // zero case
        assert_eq!(double!(3 + 1), 8);      // expression (not just literal)
        assert_eq!(double!(100), 200);      // larger number
    }

    // Exercise 2 tests
    #[test]
    fn test_sum_all_macro() {
        assert_eq!(sum_all!(1, 2, 3), 6);         // three args: 0+1+2+3
        assert_eq!(sum_all!(10, 20), 30);          // two args: 0+10+20
        assert_eq!(sum_all!(5), 5);                // single arg: 0+5
        assert_eq!(sum_all!(1, 1, 1, 1, 1), 5);   // five args: 0+1+1+1+1+1
    }

    // Exercise 3 tests
    #[test]
    fn test_token_account_derive() {
        let a = TokenAccount {
            mint: String::from("mint1"),      // test mint address
            owner: String::from("owner1"),    // test owner address
            amount: 100,                       // test balance
        };
        let b = a.clone();                    // Clone creates a deep copy
        assert_eq!(a, b);                     // PartialEq verifies equality
        println!("{:?}", a);                  // Debug enables {:?} formatting
    }

    // Exercise 3 tests for enum
    #[test]
    fn test_transfer_status_derive() {
        assert_eq!(TransferStatus::Pending, TransferStatus::Pending);              // same variant
        assert_ne!(TransferStatus::Pending, TransferStatus::Completed);            // different variants
        assert_eq!(
            TransferStatus::Failed("err".to_string()),
            TransferStatus::Failed("err".to_string())
        ); // same variant with same data
    }

    // Exercise 4 tests
    #[test]
    fn test_price_operations() {
        assert!(price_operations());  // all internal assertions pass
    }

    // Exercise 5 tests
    #[test]
    fn test_conditional_compilation() {
        // When running cargo test, #[cfg(test)] version is compiled
        assert_eq!(get_environment(), "test"); // the test version returns "test"
    }

    #[test]
    fn test_is_debug_build() {
        // cargo test runs in debug mode by default
        assert!(is_debug_build()); // debug_assertions is true in debug builds
    }

    // Exercise 6 tests
    #[test]
    fn test_anchor_deposit_valid() {
        let mut vault = VaultAccount {
            balance: 1_000_000,                  // starting balance
            authority: String::from("admin"),     // authorized signer
            is_initialized: true,                // vault is ready
        };
        // Deposit with correct authority should succeed
        assert!(simulate_anchor_deposit(&mut vault, 500_000, "admin").is_ok());
        assert_eq!(vault.balance, 1_500_000);    // balance should increase
    }

    #[test]
    fn test_anchor_deposit_unauthorized() {
        let mut vault = VaultAccount {
            balance: 1_000_000,                  // starting balance
            authority: String::from("admin"),     // authorized signer is "admin"
            is_initialized: true,                // vault is ready
        };
        // Deposit with wrong signer should fail
        assert!(simulate_anchor_deposit(&mut vault, 100, "hacker").is_err());
        assert_eq!(vault.balance, 1_000_000);    // balance should NOT change
    }

    #[test]
    fn test_anchor_deposit_uninitialized() {
        let mut vault = VaultAccount {
            balance: 0,                           // empty vault
            authority: String::from("admin"),      // correct authority
            is_initialized: false,                // NOT initialized
        };
        // Deposit to uninitialized vault should fail
        assert!(simulate_anchor_deposit(&mut vault, 100, "admin").is_err());
    }

    #[test]
    fn test_anchor_deposit_overflow() {
        let mut vault = VaultAccount {
            balance: u64::MAX,                    // already at maximum
            authority: String::from("admin"),     // correct authority
            is_initialized: true,                // vault is ready
        };
        // Adding anything should trigger overflow protection
        assert!(simulate_anchor_deposit(&mut vault, 1, "admin").is_err());
    }

    #[test]
    fn test_program_id() {
        // Verify declare_program_id! set the correct value
        assert_eq!(PROGRAM_ID, "MyProgram111111111111111111111111");
    }

    // Exercise 7 tests
    #[test]
    fn test_validator_display() {
        let v = Validator {
            name: String::from("TestValidator"),   // test name
            stake: 42,                              // test stake amount
        };
        // format!("{}", v) uses the Display implementation generated by our macro
        let display = format!("{}", v);
        // Verify the output matches our format string
        assert!(display.contains("TestValidator")); // name is in the output
        assert!(display.contains("42"));            // stake is in the output
    }
}
