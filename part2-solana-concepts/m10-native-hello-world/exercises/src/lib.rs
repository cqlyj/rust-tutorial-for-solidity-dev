// =============================================================================
// Module 10 Exercises: Native Solana Program
// =============================================================================
//
// Complete each exercise by replacing the TODO comments with working code.
// Run `cargo check` after each exercise to verify your solution compiles.
//
// Hint: You'll need these imports for all exercises.

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// =============================================================================
// Exercise 1: Set up the entrypoint
// =============================================================================
//
// The entrypoint! macro tells the Solana runtime which function to call.
// In Solidity, the EVM automatically routes to your contract's functions.
// In native Solana, you must explicitly register your entrypoint.
//
// TODO: Use the entrypoint! macro to register `process_instruction` as the entrypoint.
// Syntax: entrypoint!(function_name);

entrypoint!(process_instruction);

// The main instruction handler. All exercises are dispatched from here.
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // We use the first byte of instruction_data to pick which exercise to run.
    // This is similar to how Solidity uses the first 4 bytes of calldata as a function selector.
    let exercise = instruction_data.first().copied().unwrap_or(0);

    match exercise {
        1 => exercise_1_log_greeting(program_id)?,
        2 => exercise_2_log_accounts(accounts)?,
        3 => exercise_3_validate_signer(accounts)?,
        4 => exercise_4_read_instruction_data(instruction_data)?,
        5 => exercise_5_return_error(accounts)?,
        _ => msg!("Unknown exercise number. Pass 1-5 as the first byte of instruction_data."),
    }

    Ok(())
}

// =============================================================================
// Exercise 1: Log a greeting with the program ID
// =============================================================================
//
// Use the msg! macro to log:
//   1. A greeting message of your choice
//   2. The program_id (like logging address(this) in Solidity)
//
// msg! works like println! — use {} for format placeholders.
// Example: msg!("Value: {}", some_variable);

fn exercise_1_log_greeting(program_id: &Pubkey) -> ProgramResult {
    // TODO: Log a greeting message (any string you like)
    msg!("TODO: Replace this with your greeting");

    // TODO: Log the program_id
    // Hint: Pubkey implements Display, so you can use {} in msg!
    msg!("TODO: Log the program_id here: {}", program_id);

    Ok(())
}

// =============================================================================
// Exercise 2: Log detailed account information
// =============================================================================
//
// Iterate over the accounts slice and log each account's:
//   - Public key (account.key)
//   - Whether it's a signer (account.is_signer) — like checking msg.sender
//   - Whether it's writable (account.is_writable)
//   - Balance in lamports (account.lamports())
//   - Data length (account.data_len())
//
// Use a for loop with .iter().enumerate() to get the index and account.

fn exercise_2_log_accounts(accounts: &[AccountInfo]) -> ProgramResult {
    // TODO: Log the total number of accounts
    msg!("TODO: Log accounts.len()");

    // TODO: Loop through accounts with enumerate() and log each field listed above.
    // Example structure:
    //   for (i, account) in accounts.iter().enumerate() {
    //       msg!("Account {}: ...", i);
    //       // log is_signer, is_writable, lamports, data_len
    //   }
    for (i, account) in accounts.iter().enumerate() {
        msg!("Account {}: key = {}", i, account.key);
        msg!("TODO: Log is_signer, is_writable, lamports, data_len for account {}", i);
    }

    Ok(())
}

// =============================================================================
// Exercise 3: Validate that the first account is a signer
// =============================================================================
//
// In Solidity, you check `require(msg.sender == expectedAddress)`.
// In Solana, you check the `is_signer` flag on the relevant AccountInfo.
//
// Use `next_account_info` to get the first account from the iterator,
// then check if it's a signer. If not, return ProgramError::MissingRequiredSignature.
//
// Pattern:
//   let accounts_iter = &mut accounts.iter();
//   let first_account = next_account_info(accounts_iter)?;

fn exercise_3_validate_signer(accounts: &[AccountInfo]) -> ProgramResult {
    // Create an iterator over the accounts slice.
    let accounts_iter = &mut accounts.iter();

    // TODO: Get the first account using next_account_info(accounts_iter)?
    let first_account = next_account_info(accounts_iter)?;

    // TODO: Check if first_account.is_signer is false.
    // If it's not a signer, return Err(ProgramError::MissingRequiredSignature)
    // In Solidity, this is like: require(msg.sender == authorized, "Not authorized");
    if !first_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // TODO: Log a success message with the signer's public key
    msg!("TODO: Log that the signer was validated. Key: {}", first_account.key);

    Ok(())
}

// =============================================================================
// Exercise 4: Read and log instruction_data bytes
// =============================================================================
//
// In Solidity, calldata is ABI-encoded and the compiler handles parsing.
// In native Solana, instruction_data is raw bytes — you parse it yourself.
//
// Log:
//   1. The total length of instruction_data
//   2. Each byte (remember: byte 0 is the exercise selector, so real data starts at byte 1)
//
// Bonus: Try interpreting bytes 1..5 as a u32 using u32::from_le_bytes()

fn exercise_4_read_instruction_data(instruction_data: &[u8]) -> ProgramResult {
    // TODO: Log the total length of instruction_data
    msg!("TODO: Log instruction_data.len()");

    // TODO: Log the raw bytes starting from index 1 (index 0 is the exercise selector)
    // Hint: Use &instruction_data[1..] to get a sub-slice, and {:?} to debug-print it
    msg!("TODO: Log the instruction data bytes after the selector");

    // TODO (Bonus): If there are at least 5 bytes (1 selector + 4 data),
    // interpret bytes 1..5 as a little-endian u32.
    // Hint:
    //   if instruction_data.len() >= 5 {
    //       let bytes: [u8; 4] = instruction_data[1..5].try_into().unwrap();
    //       let value = u32::from_le_bytes(bytes);
    //       msg!("Decoded u32: {}", value);
    //   }
    if instruction_data.len() >= 5 {
        msg!("TODO: Decode bytes 1..5 as a u32 and log it");
    }

    Ok(())
}

// =============================================================================
// Exercise 5: Return a custom ProgramError when validation fails
// =============================================================================
//
// In Solidity: require(condition, "Error message") or revert CustomError()
// In Solana:   return Err(ProgramError::Custom(error_code))
//
// Write a function that:
//   1. Gets the first account
//   2. Checks if it is writable
//   3. If NOT writable, return Err(ProgramError::Custom(100))
//      (100 is our custom error code — like a custom error in Solidity)
//   4. If writable, log success and return Ok(())

fn exercise_5_return_error(accounts: &[AccountInfo]) -> ProgramResult {
    // TODO: Create an iterator and get the first account
    let accounts_iter = &mut accounts.iter();
    let target_account = next_account_info(accounts_iter)?;

    // TODO: Check if the account is writable.
    // If NOT writable, return Err(ProgramError::Custom(100))
    // This is like: revert NotWritable() in Solidity
    if !target_account.is_writable {
        return Err(ProgramError::Custom(100));
    }

    // TODO: Log a success message indicating the account is writable
    msg!("TODO: Log success — account {} is writable", target_account.key);

    Ok(())
}
