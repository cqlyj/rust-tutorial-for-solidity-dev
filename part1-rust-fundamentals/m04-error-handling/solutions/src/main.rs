// ============================================================
// Module 04 Solutions: Error Handling
// ============================================================
// Complete solutions for all exercises. Every line is commented.
// Run with: cargo run
// ============================================================

// Import thiserror's Error derive macro for exercises 7 and 8
use thiserror::Error;
// Import Display and Formatter for manual Display implementations
use std::fmt;

// ============================================================
// Exercise 1 Solution: Convert unwrap() to proper error handling
// ============================================================
// We changed the return type from f64 to Result<f64, String>.
// Now callers must handle the error case explicitly.

// Returns Result<f64, String> — either Ok(quotient) or Err(message)
fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    // Check for division by zero — like require(b != 0, "Division by zero")
    if b == 0.0 {
        // Return an error instead of panicking
        Err(String::from("Division by zero"))
    } else {
        // Return the result wrapped in Ok
        Ok(a / b)
    }
}

// ============================================================
// Exercise 2 Solution: Use the ? operator
// ============================================================
// Parse two strings as numbers and add them.
// .map_err() converts ParseIntError to String so both ?s return the same error type.

// Returns Result<i64, String> — either Ok(sum) or Err(parse error message)
fn add_strings(a: &str, b: &str) -> Result<i64, String> {
    // Parse first string as i64 — .map_err converts ParseIntError to String
    let num_a: i64 = a.parse::<i64>().map_err(|e| e.to_string())?; // ? returns Err early if parse fails

    // Parse second string as i64 — same pattern
    let num_b: i64 = b.parse::<i64>().map_err(|e| e.to_string())?; // ? returns Err early if parse fails

    // Return the sum wrapped in Ok
    Ok(num_a + num_b)
}

// ============================================================
// Exercise 3 Solution: Option handling
// ============================================================
// Use combinators to transform Option<&str> into String.
// No match statement — pure functional style.

// Returns the user's role as uppercase, or "UNKNOWN" if not found
fn get_user_role(user_id: u64) -> String {
    // Simulate a database lookup returning Option<&str>
    let role: Option<&str> = match user_id {
        1 => Some("admin"),      // User 1 is admin
        2 => Some("moderator"),  // User 2 is moderator
        3 => Some("user"),       // User 3 is regular user
        _ => None,               // Unknown user
    };

    // Chain: map to uppercase, then unwrap_or for default
    role.map(|r| r.to_uppercase())         // Some("admin") → Some("ADMIN")
        .unwrap_or(String::from("UNKNOWN")) // None → "UNKNOWN"
}

// ============================================================
// Exercise 4 Solution: Custom error type
// ============================================================
// Define StakingError with four variants and implement Display + Error manually.

// Custom error enum for staking operations — like Solidity custom errors
#[derive(Debug)] // Debug allows {:?} formatting
enum StakingError {
    // Caller is not a registered staker
    NotStaker,
    // Stake amount is below minimum — carries context data
    StakeTooLow { minimum: u64, provided: u64 },
    // Staking contract is paused
    StakingPaused,
    // Lockup period hasn't expired — carries the unlock timestamp
    LockupNotExpired { unlock_time: u64 },
}

// Implement Display to provide human-readable error messages
impl fmt::Display for StakingError {
    // Write the error message to the formatter
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Match each variant to produce the appropriate message
        match self {
            // NotStaker — simple message
            StakingError::NotStaker => write!(f, "Caller is not a registered staker"),
            // StakeTooLow — include the minimum and provided amounts
            StakingError::StakeTooLow { minimum, provided } => {
                write!(f, "Stake too low: minimum {}, provided {}", minimum, provided)
            }
            // StakingPaused — simple message
            StakingError::StakingPaused => write!(f, "Staking is currently paused"),
            // LockupNotExpired — include the unlock time
            StakingError::LockupNotExpired { unlock_time } => {
                write!(f, "Lockup not expired: unlock time is {}", unlock_time)
            }
        }
    }
}

// Implement Error trait — marks StakingError as a standard error type
impl std::error::Error for StakingError {}

// ============================================================
// Exercise 5 Solution: Using custom errors with ?
// ============================================================
// Staking function that validates inputs and returns Result.

// Minimum stake constant — like a Solidity immutable
const MINIMUM_STAKE: u64 = 100;
// Whether staking is active — like a Solidity state variable
const STAKING_ACTIVE: bool = true;

// Process a stake — returns Result with our custom StakingError
fn stake(staker: &str, amount: u64, current_stakers: &[&str]) -> Result<String, StakingError> {
    // Check if staking is active — like require(!paused, "Staking paused")
    if !STAKING_ACTIVE {
        // Return the StakingPaused error variant
        return Err(StakingError::StakingPaused);
    }

    // Check minimum stake — like require(amount >= MIN_STAKE)
    if amount < MINIMUM_STAKE {
        // Return StakeTooLow with the context data
        return Err(StakingError::StakeTooLow {
            minimum: MINIMUM_STAKE, // What the minimum is
            provided: amount,        // What was provided
        });
    }

    // Check if staker already exists
    if current_stakers.contains(&staker) {
        // Existing staker — add to their stake
        Ok(format!(
            "{} added {} to existing stake",
            staker, amount
        ))
    } else {
        // New staker — create their stake
        Ok(format!(
            "{} staked {} as a new staker",
            staker, amount
        ))
    }
}

// ============================================================
// Exercise 6 Solution: From trait for error conversion
// ============================================================
// AppError wraps both StakingError and ParseIntError.
// From implementations allow ? to auto-convert.

// Combined application error — wraps multiple error types
#[derive(Debug)] // Debug for {:?} formatting
enum AppError {
    // Wraps our StakingError
    Staking(StakingError),
    // Wraps the standard library's ParseIntError
    Parse(std::num::ParseIntError),
}

// Implement Display for AppError — delegates to inner errors
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Delegate to StakingError's Display
            AppError::Staking(e) => write!(f, "Staking error: {}", e),
            // Delegate to ParseIntError's Display
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
        }
    }
}

// Implement Error trait for AppError
impl std::error::Error for AppError {}

// From<StakingError> — allows ? to convert StakingError → AppError
impl From<StakingError> for AppError {
    fn from(e: StakingError) -> Self {
        AppError::Staking(e) // Wrap in the Staking variant
    }
}

// From<ParseIntError> — allows ? to convert ParseIntError → AppError
impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self {
        AppError::Parse(e) // Wrap in the Parse variant
    }
}

// Parse a string amount and stake — ? auto-converts errors via From
fn parse_and_stake(amount_str: &str, staker: &str) -> Result<String, AppError> {
    // Parse the string as u64 — ? converts ParseIntError to AppError automatically
    let amount: u64 = amount_str.parse::<u64>()?; // From<ParseIntError> kicks in

    // Define current stakers for this example
    let current_stakers = vec!["alice", "bob"]; // Simulated on-chain state

    // Call stake — ? converts StakingError to AppError automatically
    let result = stake(staker, amount, &current_stakers)?; // From<StakingError> kicks in

    // Return the success message
    Ok(result)
}

// ============================================================
// Exercise 7 Solution: thiserror crate
// ============================================================
// Same as StakingError but using thiserror derive macros.
// Much less boilerplate — no manual Display or From needed.

// Derive Debug and thiserror's Error — replaces manual impls
#[derive(Debug, Error)]
enum StakingErrorV2 {
    // #[error("...")] auto-generates the Display message
    #[error("Caller is not a registered staker")]
    NotStaker,

    // Named fields are interpolated in the error message
    #[error("Stake too low: minimum {minimum}, provided {provided}")]
    StakeTooLow { minimum: u64, provided: u64 },

    // Simple variant with static message
    #[error("Staking is currently paused")]
    StakingPaused,

    // Named field interpolated in message
    #[error("Lockup not expired: unlock time is {unlock_time}")]
    LockupNotExpired { unlock_time: u64 },

    // #[from] auto-generates From<ParseIntError> — no manual impl needed
    #[error("Failed to parse amount: {0}")]
    ParseError(#[from] std::num::ParseIntError),
}

// ============================================================
// Exercise 8 Solution: Simulating Solana ProgramResult
// ============================================================
// A complete Solana-style instruction handler.

// Simulated ProgramError — like solana_program::program_error::ProgramError
#[derive(Debug, Error)]
enum ProgramError {
    // Missing required signature — account didn't sign the tx
    #[error("Missing required signature")]
    MissingRequiredSignature,
    // Not enough SOL/tokens
    #[error("Insufficient funds")]
    InsufficientFunds,
    // Instruction data was malformed
    #[error("Invalid instruction data")]
    InvalidInstructionData,
    // Trying to initialize an already-initialized account
    #[error("Account already initialized")]
    AccountAlreadyInitialized,
    // Custom error with a numeric code
    #[error("Custom error: {0}")]
    Custom(u32),
}

// ProgramResult is the return type for all Solana instruction handlers
type ProgramResult = Result<(), ProgramError>;

// Simulated AccountInfo — simplified version of Solana's
struct AccountInfo {
    key: String,          // Account's public key (simplified)
    is_signer: bool,      // Whether this account signed the transaction
    lamports: u64,        // SOL balance in lamports
    is_initialized: bool, // Whether the account has been initialized
}

// Process a deposit instruction — Solana-style handler
fn process_deposit(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    // Get the depositor account (index 0) — .get() returns Option, .ok_or() converts to Result
    let depositor = accounts.get(0)
        .ok_or(ProgramError::InvalidInstructionData)?; // None → Err, then ? returns it

    // Get the vault account (index 1) — same pattern
    let vault = accounts.get(1)
        .ok_or(ProgramError::InvalidInstructionData)?; // None → Err, then ? returns it

    // Verify the depositor signed the transaction — like a Solidity modifier
    if !depositor.is_signer {
        // Return error — deposits must be signed
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Verify the vault is initialized — can't deposit into uninitialized account
    if !vault.is_initialized {
        // Return error — vault must be initialized first
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Check the depositor has enough lamports
    if depositor.lamports < amount {
        // Return error — not enough SOL
        return Err(ProgramError::InsufficientFunds);
    }

    // All checks passed — process the deposit
    println!(
        "  Deposited {} lamports from {} into vault {}",
        amount, depositor.key, vault.key
    );

    // Return success — Ok(()) is like a void return in Solidity
    Ok(())
}

// ============================================================
// MAIN — Run all solutions
// ============================================================

fn main() {
    // Print module title
    println!("╔══════════════════════════════════════════════╗");
    println!("║  Module 04 Solutions: Error Handling          ║");
    println!("╚══════════════════════════════════════════════╝\n");

    // --- Exercise 1: safe_divide ---
    println!("Exercise 1: safe_divide");
    // Test with valid division — should return Ok(3.333...)
    println!("  10 / 3 = {:?}", safe_divide(10.0, 3.0));
    // Test with division by zero — should return Err("Division by zero")
    println!("  10 / 0 = {:?}", safe_divide(10.0, 0.0));
    println!(); // Blank line

    // --- Exercise 2: add_strings ---
    println!("Exercise 2: add_strings");
    // Test with valid numbers — should return Ok(50)
    println!("  '42' + '8' = {:?}", add_strings("42", "8"));
    // Test with invalid input — should return Err with parse error
    println!("  'abc' + '8' = {:?}", add_strings("abc", "8"));
    // Test with another valid case
    println!("  '-10' + '30' = {:?}", add_strings("-10", "30"));
    println!(); // Blank line

    // --- Exercise 3: get_user_role ---
    println!("Exercise 3: get_user_role");
    // User 1 is admin — should return "ADMIN"
    println!("  User 1: {}", get_user_role(1));
    // User 2 is moderator — should return "MODERATOR"
    println!("  User 2: {}", get_user_role(2));
    // User 3 is user — should return "USER"
    println!("  User 3: {}", get_user_role(3));
    // Unknown user — should return "UNKNOWN"
    println!("  User 99: {}", get_user_role(99));
    println!(); // Blank line

    // --- Exercise 4 & 5: Custom StakingError ---
    println!("Exercise 4 & 5: Custom StakingError");
    // Define current stakers
    let stakers = vec!["alice", "bob"]; // Simulated staker registry
    // New staker with valid amount — should succeed
    println!("  Stake 200 (new): {:?}", stake("charlie", 200, &stakers));
    // Stake below minimum — should return StakeTooLow error
    println!("  Stake 50 (too low): {:?}", stake("dave", 50, &stakers));
    // Existing staker — should succeed with "added to existing" message
    println!("  Stake 200 (existing): {:?}", stake("alice", 200, &stakers));
    println!(); // Blank line

    // --- Exercise 6: AppError with From ---
    println!("Exercise 6: AppError with From");
    // Valid stake — should succeed
    println!("  Parse '500': {:?}", parse_and_stake("500", "charlie"));
    // Invalid number — should return Parse error
    println!("  Parse 'abc': {:?}", parse_and_stake("abc", "charlie"));
    // Below minimum — should return Staking error
    println!("  Parse '50': {:?}", parse_and_stake("50", "charlie"));
    println!(); // Blank line

    // --- Exercise 7: thiserror ---
    println!("Exercise 7: thiserror StakingErrorV2");
    // Create each error variant and print its Display output
    let err1 = StakingErrorV2::NotStaker; // Create NotStaker variant
    println!("  {}", err1); // Prints: "Caller is not a registered staker"
    let err2 = StakingErrorV2::StakeTooLow {
        minimum: 100, // Minimum amount
        provided: 50, // Amount that was provided
    };
    println!("  {}", err2); // Prints: "Stake too low: minimum 100, provided 50"
    let err3 = StakingErrorV2::LockupNotExpired {
        unlock_time: 1700000000, // Unix timestamp
    };
    println!("  {}", err3); // Prints: "Lockup not expired: unlock time is 1700000000"
    println!(); // Blank line

    // --- Exercise 8: Solana-style ProgramResult ---
    println!("Exercise 8: Solana-style ProgramResult");

    // Create test accounts for the deposit instruction
    let depositor = AccountInfo {
        key: String::from("Alice"),  // Depositor's key
        is_signer: true,              // Alice signed the transaction
        lamports: 1_000_000,          // 0.001 SOL
        is_initialized: true,         // Account is initialized
    };
    let vault = AccountInfo {
        key: String::from("Vault"),  // Vault's key
        is_signer: false,             // Vault doesn't need to sign
        lamports: 5_000_000,          // 0.005 SOL
        is_initialized: true,         // Vault is initialized
    };
    // Successful deposit
    println!("  Deposit 500_000: {:?}", process_deposit(&[depositor, vault], 500_000));

    // Test: unsigned deposit
    let unsigned_depositor = AccountInfo {
        key: String::from("Bob"),    // Bob's key
        is_signer: false,             // Bob did NOT sign
        lamports: 1_000_000,          // Has enough lamports
        is_initialized: true,         // Initialized
    };
    let vault2 = AccountInfo {
        key: String::from("Vault"),  // Vault key
        is_signer: false,             // Vault doesn't sign
        lamports: 5_000_000,          // Vault balance
        is_initialized: true,         // Initialized
    };
    // Should fail with MissingRequiredSignature
    println!(
        "  Unsigned deposit: {:?}",
        process_deposit(&[unsigned_depositor, vault2], 100)
    );

    // Test: insufficient funds
    let broke_depositor = AccountInfo {
        key: String::from("Charlie"), // Charlie's key
        is_signer: true,               // Charlie signed
        lamports: 100,                  // Very little SOL
        is_initialized: true,           // Initialized
    };
    let vault3 = AccountInfo {
        key: String::from("Vault"),   // Vault key
        is_signer: false,              // Vault doesn't sign
        lamports: 5_000_000,           // Vault balance
        is_initialized: true,          // Initialized
    };
    // Should fail with InsufficientFunds
    println!(
        "  Broke deposit: {:?}",
        process_deposit(&[broke_depositor, vault3], 1_000_000)
    );

    // Test: uninitialized vault
    let depositor2 = AccountInfo {
        key: String::from("Alice"),  // Alice's key
        is_signer: true,              // Alice signed
        lamports: 1_000_000,          // Has enough
        is_initialized: true,         // Initialized
    };
    let uninit_vault = AccountInfo {
        key: String::from("NewVault"), // New vault
        is_signer: false,               // Doesn't sign
        lamports: 0,                     // Empty
        is_initialized: false,           // NOT initialized
    };
    // Should fail with AccountAlreadyInitialized (we reuse this error for demo)
    println!(
        "  Uninit vault: {:?}",
        process_deposit(&[depositor2, uninit_vault], 500)
    );

    // Print summary
    println!();
    println!("All exercises completed successfully!");
    println!("Key patterns demonstrated:");
    println!("  1. Result<T, E> instead of panic!/unwrap()");
    println!("  2. The ? operator for clean error propagation");
    println!("  3. Option combinators (map, unwrap_or)");
    println!("  4. Custom error enums with Display + Error");
    println!("  5. From trait for automatic error conversion");
    println!("  6. thiserror for derive-based error types");
    println!("  7. Solana-style ProgramResult pattern");
}
