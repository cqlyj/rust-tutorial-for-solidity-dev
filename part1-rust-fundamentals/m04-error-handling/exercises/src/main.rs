// ============================================================
// Module 04 Exercises: Error Handling
// ============================================================
// Fill in the TODOs to make each exercise compile and run.
// Run with: cargo run
// Check your answers in ../solutions/src/main.rs
// ============================================================

use thiserror::Error;

// ============================================================
// Exercise 1: Convert unwrap() to proper error handling
// ============================================================
// This function uses unwrap() which will panic on bad input.
// Rewrite it to return Result<f64, String> instead.
//
// Solidity analogy: You're replacing a function that just reverts
// with one that returns a proper error value.

fn safe_divide(a: f64, b: f64) -> f64 {
    // TODO: Change this function signature to return Result<f64, String>
    // TODO: Return Err("Division by zero") if b is 0.0
    // TODO: Return Ok(a / b) otherwise
    // Currently panics on division by zero — bad!
    if b == 0.0 {
        panic!("Division by zero!"); // Replace this with Err(...)
    }
    a / b // Replace this with Ok(...)
}

// ============================================================
// Exercise 2: Use the ? operator
// ============================================================
// Complete this function to parse two strings as numbers and add them.
// Use ? to propagate errors. The function should return Result.
//
// Hint: str.parse::<i64>() returns Result<i64, ParseIntError>
// You'll need to convert the error — use .map_err(|e| e.to_string())

fn add_strings(a: &str, b: &str) -> Result<i64, String> {
    // TODO: Parse `a` as i64 using .parse::<i64>()
    // Use .map_err(|e| e.to_string()) to convert the error, then ?
    let num_a: i64 = todo!("Parse a as i64");

    // TODO: Parse `b` as i64 using .parse::<i64>()
    let num_b: i64 = todo!("Parse b as i64");

    // TODO: Return Ok with the sum
    todo!("Return the sum")
}

// ============================================================
// Exercise 3: Option handling
// ============================================================
// Complete this function that looks up a user's role.
// Return the role as an uppercase string, or "UNKNOWN" if not found.
// Use Option combinators (map, unwrap_or).

fn get_user_role(user_id: u64) -> String {
    // This simulates a database lookup — returns Option<&str>
    let role: Option<&str> = match user_id {
        1 => Some("admin"),
        2 => Some("moderator"),
        3 => Some("user"),
        _ => None,
    };

    // TODO: Use .map() to convert the &str to uppercase (use .to_uppercase())
    // TODO: Then use .unwrap_or() or .unwrap_or_else() to default to "UNKNOWN"
    // Do it in one chain — no match statement needed!
    todo!("Transform role to uppercase with UNKNOWN default")
}

// ============================================================
// Exercise 4: Custom error type
// ============================================================
// Define a custom error enum for a staking program.
// Implement Display and Error for it manually (no thiserror).
//
// Solidity equivalent:
//   error NotStaker();
//   error StakeTooLow(uint256 minimum, uint256 provided);
//   error StakingPaused();
//   error LockupNotExpired(uint256 unlockTime);

#[derive(Debug)]
enum StakingError {
    // TODO: Add variant NotStaker
    // TODO: Add variant StakeTooLow { minimum: u64, provided: u64 }
    // TODO: Add variant StakingPaused
    // TODO: Add variant LockupNotExpired { unlock_time: u64 }
}

// TODO: Implement std::fmt::Display for StakingError
// Each variant should have a human-readable message.
// Example: StakeTooLow should print "Stake too low: minimum X, provided Y"

// TODO: Implement std::error::Error for StakingError (empty impl block)

// ============================================================
// Exercise 5: Using custom errors with ?
// ============================================================
// Complete the stake() function using your StakingError enum.
// This mirrors a Solana staking program's logic.

const MINIMUM_STAKE: u64 = 100;
const STAKING_ACTIVE: bool = true;

fn stake(staker: &str, amount: u64, current_stakers: &[&str]) -> Result<String, StakingError> {
    // TODO: Check if staking is active (use STAKING_ACTIVE constant)
    // If not, return Err(StakingError::StakingPaused)

    // TODO: Check if amount >= MINIMUM_STAKE
    // If not, return Err(StakingError::StakeTooLow { minimum: MINIMUM_STAKE, provided: amount })

    // TODO: Check if staker is already in current_stakers (use .contains())
    // If they are, we just add to their stake — print a message
    // If not, they're a new staker — print a different message

    // TODO: Return Ok with a success message
    todo!("Implement stake logic")
}

// ============================================================
// Exercise 6: From trait for error conversion
// ============================================================
// Create an AppError enum that wraps both std::num::ParseIntError
// and your StakingError. Implement From for both.
// Then write a function that uses ? to auto-convert errors.

#[derive(Debug)]
enum AppError {
    // TODO: Add variant Staking(StakingError)
    // TODO: Add variant Parse(std::num::ParseIntError)
}

// TODO: Implement Display for AppError

// TODO: Implement std::error::Error for AppError

// TODO: Implement From<StakingError> for AppError

// TODO: Implement From<std::num::ParseIntError> for AppError

fn parse_and_stake(amount_str: &str, staker: &str) -> Result<String, AppError> {
    // TODO: Parse amount_str as u64 using .parse::<u64>()
    // The ? operator will auto-convert ParseIntError to AppError
    let amount: u64 = todo!("Parse amount");

    // TODO: Call stake() with the parsed amount
    // The ? operator will auto-convert StakingError to AppError
    let result = todo!("Call stake");

    // Return the result
    Ok(result)
}

// ============================================================
// Exercise 7: thiserror crate
// ============================================================
// Rewrite the StakingError using thiserror derive macros.
// This should be MUCH less code than Exercise 4.

#[derive(Debug, Error)]
enum StakingErrorV2 {
    // TODO: Add #[error("...")] attribute and variant for NotStaker
    // TODO: Add #[error("...")] attribute and variant for StakeTooLow
    // TODO: Add #[error("...")] attribute and variant for StakingPaused
    // TODO: Add #[error("...")] attribute and variant for LockupNotExpired
    // TODO: Add a variant that wraps ParseIntError using #[from]
}

// ============================================================
// Exercise 8: Simulating Solana ProgramResult
// ============================================================
// Complete this Solana-style instruction handler.
// Use the provided ProgramError and ProgramResult types.

#[derive(Debug, Error)]
enum ProgramError {
    #[error("Missing required signature")]
    MissingRequiredSignature,
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Invalid instruction data")]
    InvalidInstructionData,
    #[error("Account already initialized")]
    AccountAlreadyInitialized,
    #[error("Custom error: {0}")]
    Custom(u32),
}

type ProgramResult = Result<(), ProgramError>;

struct AccountInfo {
    key: String,
    is_signer: bool,
    lamports: u64,
    is_initialized: bool,
}

fn process_deposit(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    // TODO: Get the depositor account (index 0) using .get()
    // Convert None to ProgramError::InvalidInstructionData using .ok_or()?

    // TODO: Get the vault account (index 1) using .get()
    // Convert None to ProgramError::InvalidInstructionData using .ok_or()?

    // TODO: Check that the depositor is a signer
    // Return Err(ProgramError::MissingRequiredSignature) if not

    // TODO: Check that the vault is initialized
    // Return Err(ProgramError::AccountAlreadyInitialized) if not initialized
    // (In a real program you'd want it initialized, but this tests the error)

    // TODO: Check that the depositor has enough lamports (>= amount)
    // Return Err(ProgramError::InsufficientFunds) if not

    // TODO: Print a success message showing the deposit amount

    // TODO: Return Ok(())
    todo!("Implement deposit handler")
}

// ============================================================
// MAIN — Run exercises (uncomment as you complete them)
// ============================================================

fn main() {
    println!("=== Module 04 Exercises: Error Handling ===\n");

    // Uncomment each section as you complete the exercises:

    // --- Exercise 1 ---
    // println!("Exercise 1: safe_divide");
    // println!("  10 / 3 = {:?}", safe_divide(10.0, 3.0));
    // println!("  10 / 0 = {:?}", safe_divide(10.0, 0.0));
    // println!();

    // --- Exercise 2 ---
    // println!("Exercise 2: add_strings");
    // println!("  '42' + '8' = {:?}", add_strings("42", "8"));
    // println!("  'abc' + '8' = {:?}", add_strings("abc", "8"));
    // println!();

    // --- Exercise 3 ---
    // println!("Exercise 3: get_user_role");
    // println!("  User 1: {}", get_user_role(1));
    // println!("  User 2: {}", get_user_role(2));
    // println!("  User 99: {}", get_user_role(99));
    // println!();

    // --- Exercise 4 & 5 ---
    // println!("Exercise 4 & 5: Custom StakingError");
    // let stakers = vec!["alice", "bob"];
    // println!("  Stake 200: {:?}", stake("charlie", 200, &stakers));
    // println!("  Stake 50: {:?}", stake("dave", 50, &stakers));
    // println!("  Stake 200 (existing): {:?}", stake("alice", 200, &stakers));
    // println!();

    // --- Exercise 6 ---
    // println!("Exercise 6: AppError with From");
    // println!("  Parse '500': {:?}", parse_and_stake("500", "charlie"));
    // println!("  Parse 'abc': {:?}", parse_and_stake("abc", "charlie"));
    // println!();

    // --- Exercise 7 ---
    // println!("Exercise 7: thiserror StakingErrorV2");
    // let err = StakingErrorV2::StakeTooLow { minimum: 100, provided: 50 };
    // println!("  Error display: {}", err);
    // println!();

    // --- Exercise 8 ---
    // println!("Exercise 8: Solana-style ProgramResult");
    // let depositor = AccountInfo {
    //     key: String::from("Alice"),
    //     is_signer: true,
    //     lamports: 1_000_000,
    //     is_initialized: true,
    // };
    // let vault = AccountInfo {
    //     key: String::from("Vault"),
    //     is_signer: false,
    //     lamports: 5_000_000,
    //     is_initialized: true,
    // };
    // println!("  Deposit 500_000: {:?}", process_deposit(&[depositor, vault], 500_000));
    // println!();

    println!("Uncomment exercises in main() as you complete them!");
    println!("Check solutions in ../solutions/src/main.rs");
}
