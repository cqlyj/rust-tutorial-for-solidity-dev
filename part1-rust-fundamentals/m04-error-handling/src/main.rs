// ============================================================
// Module 04: Error Handling in Rust
// ============================================================
// In Solidity, errors use require(), revert(), assert(), and try/catch.
// In Rust, there are NO exceptions. Errors are VALUES returned from
// functions using Result<T, E>. This is a fundamental paradigm shift.
// ============================================================

// Import the Error derive macro from thiserror crate
use thiserror::Error;

// Import Display and Formatter for manual Display implementation
use std::fmt;

// ============================================================
// PART 1: panic! — Unrecoverable Errors
// ============================================================
// panic! is like Solidity's assert(false) — it kills the program.
// We wrap it in a function to demonstrate without actually crashing.

// This function demonstrates what causes a panic
fn demonstrate_panic_sources() {
    // Print a header for this section
    println!("=== Part 1: panic! — Unrecoverable Errors ===\n");

    // Out-of-bounds access would panic — let's show it safely
    let numbers = vec![1, 2, 3]; // Create a vector with 3 elements
    // numbers[99] would panic with: "index out of bounds"
    // Instead, we use .get() which returns Option<&T> — safe!
    let safe_access = numbers.get(99); // Returns None instead of panicking
    println!("Safe access to index 99: {:?}", safe_access); // Prints: None

    // Demonstrating that we COULD panic but choose not to
    // panic!("This would kill the program!"); // <-- uncomment to see a panic
    println!("We avoided the panic by using safe access patterns.\n");
}

// ============================================================
// PART 2: Result<T, E> — Recoverable Errors
// ============================================================
// Result is Rust's core error handling type.
// Like if every Solidity function returned (bool success, T value).

// A function that can fail — returns Result instead of panicking
// Solidity equivalent: function divide(a, b) returns (uint) { require(b != 0); }
fn divide(a: f64, b: f64) -> Result<f64, String> {
    // Check for division by zero — like require(b != 0, "Division by zero")
    if b == 0.0 {
        // Return an error value — like revert("Division by zero")
        Err(String::from("Division by zero"))
    } else {
        // Return success with the computed value — like return a / b
        Ok(a / b)
    }
}

// Demonstrate basic Result usage
fn demonstrate_result() {
    // Print a header for this section
    println!("=== Part 2: Result<T, E> — Recoverable Errors ===\n");

    // Call divide with valid arguments — should succeed
    let good = divide(10.0, 3.0); // Returns Ok(3.333...)
    println!("10 / 3 = {:?}", good); // Prints: Ok(3.333...)

    // Call divide with zero — should fail
    let bad = divide(10.0, 0.0); // Returns Err("Division by zero")
    println!("10 / 0 = {:?}", bad); // Prints: Err("Division by zero")

    // Use match to handle both cases explicitly
    match divide(20.0, 4.0) {
        // If Ok, extract the value and print it
        Ok(value) => println!("20 / 4 = {}", value),
        // If Err, extract the error message and print it
        Err(e) => println!("Error: {}", e),
    }

    println!(); // Blank line for readability
}

// ============================================================
// PART 3: Option<T> — Values That Might Not Exist
// ============================================================
// Option replaces null/nil/address(0). Forces you to handle absence.

// Simulate looking up an account balance — might not exist
// Solidity: mapping(address => uint256) balances; — returns 0 if missing
// Rust: returns None if the account doesn't exist
fn get_balance(account: &str) -> Option<u64> {
    // Match on the account name to simulate a lookup
    match account {
        // Alice has 1000 tokens — return Some(1000)
        "alice" => Some(1000),
        // Bob has 500 tokens — return Some(500)
        "bob" => Some(500),
        // Anyone else — account not found, return None
        _ => None,
    }
}

// Demonstrate Option usage
fn demonstrate_option() {
    // Print a header for this section
    println!("=== Part 3: Option<T> — Values That Might Not Exist ===\n");

    // Look up Alice's balance — should be Some(1000)
    let alice_balance = get_balance("alice");
    println!("Alice's balance: {:?}", alice_balance); // Some(1000)

    // Look up Charlie's balance — should be None
    let charlie_balance = get_balance("charlie");
    println!("Charlie's balance: {:?}", charlie_balance); // None

    // Use match to safely extract the value
    match get_balance("bob") {
        // Bob exists — print the balance
        Some(bal) => println!("Bob has {} tokens", bal),
        // Bob doesn't exist — handle the absence
        None => println!("Bob's account not found"),
    }

    // Use if-let for when you only care about the Some case
    if let Some(bal) = get_balance("alice") {
        // Only runs if alice's balance exists
        println!("Alice confirmed: {} tokens", bal);
    }

    println!(); // Blank line for readability
}

// ============================================================
// PART 4: unwrap() and expect() — The Bad Way (for demos only)
// ============================================================

// Demonstrate unwrap and expect — DO NOT use in production!
fn demonstrate_unwrap_and_expect() {
    // Print a header for this section
    println!("=== Part 4: unwrap() and expect() — Quick & Dirty ===\n");

    // unwrap() on a successful Result — works fine
    let value = divide(10.0, 2.0).unwrap(); // Extracts 5.0 from Ok(5.0)
    println!("unwrap success: {}", value); // Prints: 5.0

    // unwrap() on a failed Result would panic:
    // divide(10.0, 0.0).unwrap(); // PANIC: "called unwrap() on an Err value"

    // expect() provides a custom panic message — slightly better for debugging
    let value = divide(10.0, 5.0).expect("Division should not fail here"); // Extracts 2.0
    println!("expect success: {}", value); // Prints: 2.0

    // unwrap_or() provides a fallback value instead of panicking
    let safe = divide(10.0, 0.0).unwrap_or(0.0); // Returns 0.0 on error
    println!("unwrap_or fallback: {}", safe); // Prints: 0.0

    // unwrap_or_else() computes the fallback lazily — useful for logging
    let safe = divide(10.0, 0.0).unwrap_or_else(|e| {
        // This closure runs only on error — good for logging
        println!("  (handled error: {})", e);
        -1.0 // Return -1.0 as fallback
    });
    println!("unwrap_or_else fallback: {}", safe); // Prints: -1.0

    println!(); // Blank line for readability
}

// ============================================================
// PART 5: The ? Operator — Elegant Error Propagation
// ============================================================
// The ? operator is like a require() that auto-returns Err.
// It can only be used in functions that return Result or Option.

// Validate a transfer — each step can fail, ? propagates errors up
// Solidity equivalent:
//   require(amount > 0, "Invalid amount");
//   require(balance >= amount, "Insufficient funds");
fn validate_and_transfer(from: &str, to: &str, amount: u64) -> Result<String, String> {
    // Step 1: Check that amount is valid — like require(amount > 0)
    if amount == 0 {
        // Return an error early — like revert("Amount must be > 0")
        return Err(String::from("Amount must be greater than zero"));
    }

    // Step 2: Look up sender's balance — convert Option to Result with ok_or
    // ok_or() turns None into Err — like require(balance != 0, "Account not found")
    let balance = get_balance(from)
        .ok_or(format!("Account '{}' not found", from))?; // ? returns Err if None

    // Step 3: Check sufficient funds — like require(balance >= amount)
    if balance < amount {
        // Return an error with context — like revert InsufficientBalance(amount, balance)
        return Err(format!(
            "Insufficient funds: requested {}, available {}",
            amount, balance
        ));
    }

    // All checks passed — return success with a description
    Ok(format!(
        "Transferred {} tokens from {} to {} (remaining: {})",
        amount, from, to, balance - amount
    ))
}

// A function that chains multiple fallible operations using ?
fn process_multiple_transfers() -> Result<(), String> {
    // Each transfer uses ? — if any fails, this function returns Err immediately
    let result1 = validate_and_transfer("alice", "bob", 200)?; // ? propagates Err
    println!("  Transfer 1: {}", result1); // Only runs if transfer succeeded

    let result2 = validate_and_transfer("bob", "alice", 100)?; // ? propagates Err
    println!("  Transfer 2: {}", result2); // Only runs if transfer succeeded

    // Return Ok(()) to indicate all transfers succeeded — like a void success
    Ok(())
}

// Demonstrate the ? operator
fn demonstrate_question_mark() {
    // Print a header for this section
    println!("=== Part 5: The ? Operator — Early Return on Error ===\n");

    // Run successful transfers
    match process_multiple_transfers() {
        // All transfers succeeded
        Ok(()) => println!("All transfers completed successfully!"),
        // At least one transfer failed — print the error
        Err(e) => println!("Transfer batch failed: {}", e),
    }

    println!(); // Blank line

    // Try a transfer that will fail — charlie doesn't exist
    match validate_and_transfer("charlie", "alice", 100) {
        // Should not reach here — charlie doesn't exist
        Ok(msg) => println!("{}", msg),
        // Should print the error about charlie's account
        Err(e) => println!("Expected error: {}", e),
    }

    // Try a transfer with insufficient funds — bob only has 500
    match validate_and_transfer("bob", "alice", 999) {
        // Should not reach here — bob doesn't have 999 tokens
        Ok(msg) => println!("{}", msg),
        // Should print the insufficient funds error
        Err(e) => println!("Expected error: {}", e),
    }

    println!(); // Blank line for readability
}

// ============================================================
// PART 6: Combinators — Functional Error Handling
// ============================================================

// Demonstrate map, and_then, unwrap_or, unwrap_or_else on Result/Option
fn demonstrate_combinators() {
    // Print a header for this section
    println!("=== Part 6: Combinators — Functional Error Handling ===\n");

    // map() transforms the Ok value without touching Err
    let doubled = divide(10.0, 2.0).map(|v| v * 2.0); // Ok(5.0) → Ok(10.0)
    println!("map (double result): {:?}", doubled); // Ok(10.0)

    // map() on an Err passes the error through unchanged
    let still_err = divide(10.0, 0.0).map(|v| v * 2.0); // Err stays Err
    println!("map on error: {:?}", still_err); // Err("Division by zero")

    // and_then() chains operations that can also fail
    let chained = divide(100.0, 5.0) // Ok(20.0)
        .and_then(|v| divide(v, 4.0)); // Ok(20.0) → divide(20.0, 4.0) → Ok(5.0)
    println!("and_then chain: {:?}", chained); // Ok(5.0)

    // and_then() stops at the first error
    let chained_err = divide(100.0, 0.0) // Err("Division by zero")
        .and_then(|v| divide(v, 4.0)); // Never runs — error propagated
    println!("and_then with error: {:?}", chained_err); // Err("Division by zero")

    // Option combinators
    let name = get_balance("alice") // Some(1000)
        .map(|bal| format!("Balance: {} tokens", bal)); // Some("Balance: 1000 tokens")
    println!("Option map: {:?}", name); // Some("Balance: 1000 tokens")

    // filter() keeps the Some only if the predicate is true
    let rich = get_balance("alice").filter(|&bal| bal > 500); // Some(1000) — keeps it
    let poor = get_balance("bob").filter(|&bal| bal > 500); // Some(500) — 500 is NOT > 500, None
    println!("alice > 500? {:?}", rich); // Some(1000)
    println!("bob > 500? {:?}", poor); // None

    // or_else() provides a fallback Option if None
    let found = get_balance("charlie") // None
        .or_else(|| get_balance("alice")); // Fallback to alice — Some(1000)
    println!("charlie or alice: {:?}", found); // Some(1000)

    println!(); // Blank line for readability
}

// ============================================================
// PART 7: Custom Error Types with Enums
// ============================================================
// In Solidity you define: error NotOwner(); error InsufficientFunds(uint256, uint256);
// In Rust, you define an enum with variants for each error case.

// Define a custom error enum — like Solidity custom errors
#[derive(Debug)] // Derive Debug so we can print it with {:?}
enum VaultError {
    // The caller is not the owner — like error NotOwner()
    NotOwner,
    // Not enough funds — carries context data like error InsufficientBalance(uint, uint)
    InsufficientBalance { requested: u64, available: u64 },
    // The vault is locked — like error VaultLocked()
    VaultLocked,
    // Amount is zero or invalid — like error InvalidAmount()
    InvalidAmount,
}

// Implement Display to define human-readable error messages
impl fmt::Display for VaultError {
    // The fmt function writes the error message to the formatter
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Match on self to produce different messages per variant
        match self {
            // NotOwner gets a simple message
            VaultError::NotOwner => write!(f, "Caller is not the vault owner"),
            // InsufficientBalance includes the numeric context
            VaultError::InsufficientBalance {
                requested,
                available,
            } => write!(
                f,
                "Insufficient balance: requested {}, available {}",
                requested, available
            ),
            // VaultLocked gets a simple message
            VaultError::VaultLocked => write!(f, "Vault is currently locked"),
            // InvalidAmount gets a simple message
            VaultError::InvalidAmount => write!(f, "Invalid amount: must be greater than zero"),
        }
    }
}

// Implement the Error trait — marks VaultError as a standard error type
impl std::error::Error for VaultError {}

// A simulated vault with owner and balance
struct Vault {
    owner: String,    // The vault owner's name
    balance: u64,     // The vault's token balance
    is_locked: bool,  // Whether the vault is locked
}

// Implement methods on Vault — like Solidity contract functions
impl Vault {
    // Create a new vault — like a constructor
    fn new(owner: &str, balance: u64) -> Self {
        Vault {
            owner: String::from(owner), // Set the owner
            balance,                     // Set initial balance
            is_locked: false,            // Vault starts unlocked
        }
    }

    // Withdraw from the vault — returns Result with our custom error
    // Solidity equivalent:
    //   function withdraw(uint amount) external {
    //       require(msg.sender == owner, "Not owner");
    //       require(!isLocked, "Vault locked");
    //       require(amount > 0, "Invalid amount");
    //       require(balance >= amount, "Insufficient balance");
    //       balance -= amount;
    //   }
    fn withdraw(&mut self, caller: &str, amount: u64) -> Result<u64, VaultError> {
        // Check caller is owner — like require(msg.sender == owner)
        if caller != self.owner {
            return Err(VaultError::NotOwner); // Return the NotOwner error
        }

        // Check vault isn't locked — like require(!isLocked)
        if self.is_locked {
            return Err(VaultError::VaultLocked); // Return the VaultLocked error
        }

        // Check amount is valid — like require(amount > 0)
        if amount == 0 {
            return Err(VaultError::InvalidAmount); // Return the InvalidAmount error
        }

        // Check sufficient funds — like require(balance >= amount)
        if self.balance < amount {
            return Err(VaultError::InsufficientBalance {
                requested: amount,       // How much was requested
                available: self.balance, // How much is available
            });
        }

        // All checks passed — perform the withdrawal
        self.balance -= amount; // Deduct the amount
        Ok(self.balance) // Return the new balance as Ok
    }
}

// Demonstrate custom error types
fn demonstrate_custom_errors() {
    // Print a header for this section
    println!("=== Part 7: Custom Error Types ===\n");

    // Create a vault owned by alice with 1000 tokens
    let mut vault = Vault::new("alice", 1000);

    // Successful withdrawal by the owner
    match vault.withdraw("alice", 300) {
        // Should succeed — alice is the owner and has enough
        Ok(remaining) => println!("Withdrew 300. Remaining: {}", remaining),
        // Should not reach here
        Err(e) => println!("Error: {}", e),
    }

    // Failed: wrong caller — bob is not the owner
    match vault.withdraw("bob", 100) {
        // Should not reach here
        Ok(_) => println!("This shouldn't print"),
        // Should print: "Caller is not the vault owner"
        Err(e) => println!("Expected error: {}", e),
    }

    // Failed: insufficient balance — only 700 left after first withdrawal
    match vault.withdraw("alice", 9999) {
        // Should not reach here
        Ok(_) => println!("This shouldn't print"),
        // Should print the InsufficientBalance message with amounts
        Err(e) => println!("Expected error: {}", e),
    }

    // Lock the vault and try again
    vault.is_locked = true; // Lock it
    match vault.withdraw("alice", 100) {
        // Should not reach here
        Ok(_) => println!("This shouldn't print"),
        // Should print: "Vault is currently locked"
        Err(e) => println!("Expected error: {}", e),
    }

    println!(); // Blank line for readability
}

// ============================================================
// PART 8: From Trait — Automatic Error Conversion
// ============================================================
// When a function can produce different error types, From lets ? convert them.

// A combined error type that wraps multiple underlying errors
#[derive(Debug)] // Debug for {:?} printing
enum DataError {
    // Wraps a standard IO error (file not found, permission denied, etc.)
    Io(std::io::Error),
    // Wraps a parse error (invalid number string)
    Parse(std::num::ParseIntError),
    // A custom message for our own validation logic
    Validation(String),
}

// Implement Display for DataError — required for error types
impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Delegate to the inner IO error's Display
            DataError::Io(e) => write!(f, "IO error: {}", e),
            // Delegate to the inner parse error's Display
            DataError::Parse(e) => write!(f, "Parse error: {}", e),
            // Print our custom validation message
            DataError::Validation(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

// Implement Error trait for DataError
impl std::error::Error for DataError {}

// Implement From<io::Error> so ? can auto-convert IO errors to DataError
impl From<std::io::Error> for DataError {
    fn from(e: std::io::Error) -> Self {
        DataError::Io(e) // Wrap the IO error in our enum
    }
}

// Implement From<ParseIntError> so ? can auto-convert parse errors to DataError
impl From<std::num::ParseIntError> for DataError {
    fn from(e: std::num::ParseIntError) -> Self {
        DataError::Parse(e) // Wrap the parse error in our enum
    }
}

// A function that does IO + parsing — both can fail with different error types
// The ? operator auto-converts each error via the From implementations above
fn read_config_value(content: &str) -> Result<i64, DataError> {
    // Simulate reading config — in real code this would be fs::read_to_string(path)?
    // For this demo, we parse the content directly

    // Trim whitespace from the content string
    let trimmed = content.trim();

    // Check for empty input — our own validation
    if trimmed.is_empty() {
        // Return our custom Validation error
        return Err(DataError::Validation(String::from("Config value is empty")));
    }

    // Parse the string as i64 — ? auto-converts ParseIntError to DataError
    let value: i64 = trimmed.parse()?; // Uses From<ParseIntError> for DataError

    // Validate the parsed value is positive
    if value < 0 {
        // Return our custom Validation error for negative values
        return Err(DataError::Validation(format!(
            "Config value must be positive, got {}",
            value
        )));
    }

    // Return the successfully parsed and validated value
    Ok(value)
}

// Demonstrate From trait and automatic error conversion
fn demonstrate_from_trait() {
    // Print a header for this section
    println!("=== Part 8: From Trait — Error Conversion ===\n");

    // Test with valid input — should succeed
    match read_config_value("  42  ") {
        Ok(v) => println!("Parsed config value: {}", v),   // Prints: 42
        Err(e) => println!("Error: {}", e),                  // Should not reach
    }

    // Test with invalid number — ParseIntError converted via From
    match read_config_value("not_a_number") {
        Ok(v) => println!("Value: {}", v),                   // Should not reach
        Err(e) => println!("Expected parse error: {}", e),   // Parse error
    }

    // Test with empty input — our custom Validation error
    match read_config_value("   ") {
        Ok(v) => println!("Value: {}", v),                   // Should not reach
        Err(e) => println!("Expected validation error: {}", e), // Validation error
    }

    // Test with negative number — our custom Validation error
    match read_config_value("-5") {
        Ok(v) => println!("Value: {}", v),                   // Should not reach
        Err(e) => println!("Expected validation error: {}", e), // Negative value
    }

    println!(); // Blank line for readability
}

// ============================================================
// PART 9: thiserror Crate — Derive All the Boilerplate
// ============================================================
// thiserror auto-generates Display and From implementations.
// This is the standard approach in the Solana ecosystem.

// Define error enum using thiserror — replaces manual Display + From impls
#[derive(Debug, Error)] // Derive both Debug and thiserror's Error
#[allow(dead_code)] // Some variants shown for completeness but not exercised
enum TokenError {
    // #[error("...")] generates the Display impl for this variant
    #[error("Insufficient token balance: need {needed}, have {available}")]
    InsufficientBalance {
        needed: u64,    // How many tokens are needed
        available: u64, // How many tokens are available
    },

    // Simple variant with a static error message
    #[error("Token account is frozen")]
    AccountFrozen,

    // Simple variant with a static error message
    #[error("Unauthorized: signer does not match owner")]
    Unauthorized,

    // #[from] auto-generates From<ParseIntError> for TokenError
    #[error("Failed to parse token amount: {0}")]
    ParseError(#[from] std::num::ParseIntError),

    // Variant with a custom string message
    #[error("Token error: {0}")]
    Custom(String),
}

// A simulated token account
struct TokenAccount {
    owner: String,   // Who owns this account
    balance: u64,    // Token balance
    frozen: bool,    // Whether the account is frozen
}

// Implement methods on TokenAccount
impl TokenAccount {
    // Transfer tokens out — returns Result with TokenError
    fn transfer(&mut self, signer: &str, amount: u64) -> Result<(), TokenError> {
        // Check authorization — like require(msg.sender == owner)
        if signer != self.owner {
            return Err(TokenError::Unauthorized); // Auto-generated Display
        }

        // Check if account is frozen — like require(!frozen)
        if self.frozen {
            return Err(TokenError::AccountFrozen); // Auto-generated Display
        }

        // Check sufficient balance — like require(balance >= amount)
        if self.balance < amount {
            return Err(TokenError::InsufficientBalance {
                needed: amount,          // How much we need
                available: self.balance, // How much we have
            });
        }

        // Perform the transfer
        self.balance -= amount; // Deduct from balance
        Ok(()) // Return success with unit type — like void return in Solidity
    }
}

// Parse a string amount and transfer — demonstrates ? with From
fn parse_and_transfer(account: &mut TokenAccount, signer: &str, amount_str: &str) -> Result<(), TokenError> {
    // Parse the amount string — ? converts ParseIntError to TokenError via #[from]
    let amount: u64 = amount_str.parse()?; // Auto-conversion via From

    // Perform the transfer — ? propagates any TokenError
    account.transfer(signer, amount)?; // Propagate errors up

    // Print success message
    println!(
        "  Transferred {} tokens. New balance: {}",
        amount, account.balance
    );

    // Return success
    Ok(())
}

// Demonstrate thiserror
fn demonstrate_thiserror() {
    // Print a header for this section
    println!("=== Part 9: thiserror — Derive Error Boilerplate ===\n");

    // Create a token account owned by alice with 500 tokens
    let mut account = TokenAccount {
        owner: String::from("alice"), // Alice owns it
        balance: 500,                  // Start with 500 tokens
        frozen: false,                 // Not frozen
    };

    // Successful transfer
    match parse_and_transfer(&mut account, "alice", "200") {
        Ok(()) => println!("  Transfer succeeded!"),                // Should print
        Err(e) => println!("  Error: {}", e),                       // Should not reach
    }

    // Parse error — "abc" is not a number
    match parse_and_transfer(&mut account, "alice", "abc") {
        Ok(()) => println!("  This shouldn't print"),               // Should not reach
        Err(e) => println!("  Expected parse error: {}", e),       // ParseError
    }

    // Unauthorized — bob is not the owner
    match parse_and_transfer(&mut account, "bob", "100") {
        Ok(()) => println!("  This shouldn't print"),               // Should not reach
        Err(e) => println!("  Expected auth error: {}", e),        // Unauthorized
    }

    // Insufficient balance — only 300 left after first transfer
    match parse_and_transfer(&mut account, "alice", "9999") {
        Ok(()) => println!("  This shouldn't print"),               // Should not reach
        Err(e) => println!("  Expected balance error: {}", e),     // InsufficientBalance
    }

    // Freeze the account and try again
    account.frozen = true; // Freeze it
    match parse_and_transfer(&mut account, "alice", "10") {
        Ok(()) => println!("  This shouldn't print"),               // Should not reach
        Err(e) => println!("  Expected frozen error: {}", e),      // AccountFrozen
    }

    println!(); // Blank line for readability
}

// ============================================================
// PART 10: Simulating Solana-Style Error Handling
// ============================================================
// In Solana programs, every instruction returns ProgramResult.
// ProgramResult is just: Result<(), ProgramError>
// Let's simulate this pattern:

// Simulate Solana's ProgramError enum — a simplified version
#[derive(Debug, Error)] // Use thiserror for convenience
#[allow(dead_code)] // Some variants shown for completeness but not exercised
enum ProgramError {
    // A custom error code — like ProgramError::Custom(u32) in Solana
    #[error("Custom error: {0}")]
    Custom(u32),

    // Missing required signature — account didn't sign the transaction
    #[error("Missing required signature")]
    MissingRequiredSignature,

    // Insufficient funds for the operation
    #[error("Insufficient funds")]
    InsufficientFunds,

    // Account data is invalid or corrupted
    #[error("Invalid account data")]
    InvalidAccountData,

    // The account has already been initialized
    #[error("Account already initialized")]
    AccountAlreadyInitialized,
}

// Define ProgramResult — the return type for all Solana instruction handlers
// This is EXACTLY how it's defined in the real solana_program crate
type ProgramResult = Result<(), ProgramError>;

// Simulate AccountInfo — a simplified version of Solana's AccountInfo
struct AccountInfo {
    key: String,       // The account's public key (simplified as String)
    is_signer: bool,   // Whether this account signed the transaction
    lamports: u64,     // The account's SOL balance in lamports
    data: Vec<u8>,     // The account's data (raw bytes)
}

// Simulate a Solana instruction: initialize a vault account
fn process_initialize(accounts: &[AccountInfo]) -> ProgramResult {
    // Get the first account — the vault to initialize
    let vault_account = accounts.get(0) // Safe access returns Option
        .ok_or(ProgramError::InvalidAccountData)?; // Convert None to Err

    // Verify the account signed the transaction — like a Solidity modifier
    if !vault_account.is_signer {
        // Return error — this account must sign to initialize
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check the account isn't already initialized (non-empty data means initialized)
    if !vault_account.data.is_empty() {
        // Return error — don't allow re-initialization
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // In a real program, we'd write initial data here
    // vault_account.data = serialize(&VaultState::new());
    println!("  Vault account {} initialized successfully!", vault_account.key);

    // Return success — Ok with unit type, like a void return
    Ok(())
}

// Simulate a Solana instruction: transfer SOL from vault
fn process_transfer(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    // Get the source account (index 0)
    let source = accounts.get(0) // Safe access returns Option
        .ok_or(ProgramError::InvalidAccountData)?; // Convert None to Err

    // Get the destination account (index 1)
    let _destination = accounts.get(1) // Safe access returns Option
        .ok_or(ProgramError::InvalidAccountData)?; // Convert None to Err

    // Verify the source signed the transaction
    if !source.is_signer {
        // Must sign to authorize the transfer
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check sufficient funds
    if source.lamports < amount {
        // Not enough SOL to transfer
        return Err(ProgramError::InsufficientFunds);
    }

    // In a real program, we'd modify lamports here:
    // **source.lamports.borrow_mut() -= amount;
    // **destination.lamports.borrow_mut() += amount;
    println!(
        "  Transferred {} lamports from {} (remaining: {})",
        amount,
        source.key,
        source.lamports - amount
    );

    // Return success
    Ok(())
}

// Demonstrate Solana-style error handling
fn demonstrate_solana_style() {
    // Print a header for this section
    println!("=== Part 10: Solana-Style ProgramResult ===\n");

    // Create simulated accounts
    let signer_account = AccountInfo {
        key: String::from("7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU"), // Fake pubkey
        is_signer: true,         // This account signed the tx
        lamports: 1_000_000_000, // 1 SOL in lamports
        data: vec![],            // Empty — not yet initialized
    };

    let non_signer = AccountInfo {
        key: String::from("SysvarRent111111111111111111111111111111111"), // Fake pubkey
        is_signer: false,        // This account did NOT sign
        lamports: 500_000_000,   // 0.5 SOL
        data: vec![],            // Empty
    };

    let already_init = AccountInfo {
        key: String::from("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM"), // Fake pubkey
        is_signer: true,         // Signed
        lamports: 100_000_000,   // 0.1 SOL
        data: vec![1, 2, 3, 4],  // Non-empty — already initialized
    };

    let destination = AccountInfo {
        key: String::from("Bnk5HCDPz6FPKn3pZ46GRmCjNwBU5B3diNXAhWxcRQbi"), // Fake pubkey
        is_signer: false,        // Destination doesn't need to sign
        lamports: 0,             // Empty account
        data: vec![],            // No data
    };

    // Test 1: Successful initialization
    println!("Test 1: Initialize vault (should succeed)");
    match process_initialize(&[signer_account]) {
        Ok(()) => println!("  Result: Success!\n"),          // Should print
        Err(e) => println!("  Result: Error — {}\n", e),    // Should not reach
    }

    // Recreate signer_account since we moved it
    let signer_account = AccountInfo {
        key: String::from("7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU"),
        is_signer: true,
        lamports: 1_000_000_000,
        data: vec![],
    };

    // Test 2: Missing signature
    println!("Test 2: Initialize without signing (should fail)");
    match process_initialize(&[non_signer]) {
        Ok(()) => println!("  Result: Success!\n"),          // Should not reach
        Err(e) => println!("  Result: Error — {}\n", e),    // MissingRequiredSignature
    }

    // Recreate non_signer since we moved it
    let non_signer = AccountInfo {
        key: String::from("SysvarRent111111111111111111111111111111111"),
        is_signer: false,
        lamports: 500_000_000,
        data: vec![],
    };

    // Test 3: Already initialized
    println!("Test 3: Initialize already-initialized account (should fail)");
    match process_initialize(&[already_init]) {
        Ok(()) => println!("  Result: Success!\n"),          // Should not reach
        Err(e) => println!("  Result: Error — {}\n", e),    // AccountAlreadyInitialized
    }

    // Test 4: Successful transfer
    println!("Test 4: Transfer 500_000 lamports (should succeed)");
    match process_transfer(&[signer_account, destination], 500_000) {
        Ok(()) => println!("  Result: Success!\n"),          // Should print
        Err(e) => println!("  Result: Error — {}\n", e),    // Should not reach
    }

    // Recreate accounts for test 5
    let broke_signer = AccountInfo {
        key: String::from("7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU"),
        is_signer: true,
        lamports: 100,           // Very little SOL
        data: vec![],
    };
    let destination2 = AccountInfo {
        key: String::from("Bnk5HCDPz6FPKn3pZ46GRmCjNwBU5B3diNXAhWxcRQbi"),
        is_signer: false,
        lamports: 0,
        data: vec![],
    };

    // Test 5: Insufficient funds
    println!("Test 5: Transfer more than available (should fail)");
    match process_transfer(&[broke_signer, destination2], 1_000_000) {
        Ok(()) => println!("  Result: Success!\n"),          // Should not reach
        Err(e) => println!("  Result: Error — {}\n", e),    // InsufficientFunds
    }

    // Test 6: Unsigned transfer
    println!("Test 6: Transfer without signing (should fail)");
    match process_transfer(&[non_signer], 100) {
        Ok(()) => println!("  Result: Success!\n"),          // Should not reach
        Err(e) => println!("  Result: Error — {}\n", e),    // MissingRequiredSignature
    }
}

// ============================================================
// MAIN — Run all demonstrations in order
// ============================================================

fn main() {
    // Print module title
    println!("╔══════════════════════════════════════════════╗");
    println!("║  Module 04: Error Handling in Rust           ║");
    println!("║  From Solidity's require() to Rust's Result  ║");
    println!("╚══════════════════════════════════════════════╝\n");

    // Part 1: panic! — unrecoverable errors (don't use for expected failures)
    demonstrate_panic_sources();

    // Part 2: Result<T, E> — the core error handling mechanism
    demonstrate_result();

    // Part 3: Option<T> — for values that might not exist
    demonstrate_option();

    // Part 4: unwrap/expect — the quick and dirty way (avoid in production)
    demonstrate_unwrap_and_expect();

    // Part 5: The ? operator — elegant error propagation
    demonstrate_question_mark();

    // Part 6: Combinators — functional-style error handling
    demonstrate_combinators();

    // Part 7: Custom error types with manual Display/Error impls
    demonstrate_custom_errors();

    // Part 8: From trait for automatic error conversion
    demonstrate_from_trait();

    // Part 9: thiserror crate — derive macros eliminate boilerplate
    demonstrate_thiserror();

    // Part 10: Simulating Solana's ProgramResult pattern
    demonstrate_solana_style();

    // Print summary
    println!("╔══════════════════════════════════════════════╗");
    println!("║  Key Takeaways:                              ║");
    println!("║  • Never use unwrap() in production          ║");
    println!("║  • Use Result<T, E> for all fallible ops     ║");
    println!("║  • Use ? for clean error propagation         ║");
    println!("║  • Define custom error enums                 ║");
    println!("║  • Use thiserror to eliminate boilerplate     ║");
    println!("║  • Solana: ProgramResult = Result<(), E>     ║");
    println!("╚══════════════════════════════════════════════╝");
}
