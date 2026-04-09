// =============================================================================
// Module 10 Solutions: Native Solana Program
// =============================================================================
//
// Complete, commented solutions for all five exercises.

// Import all necessary types from the solana_program crate.
// This is the foundational crate that every native Solana program depends on.
use solana_program::{
    // next_account_info is a helper that advances an iterator over AccountInfo slices.
    // It returns the next AccountInfo or an error if there are no more accounts.
    account_info::{next_account_info, AccountInfo},
    // The entrypoint! macro generates the low-level C-compatible function
    // that the Solana runtime calls when an instruction targets this program.
    entrypoint,
    // ProgramResult is Result<(), ProgramError> — the return type of all instruction handlers.
    entrypoint::ProgramResult,
    // msg! logs messages to the transaction log (like console.log or emit in Solidity).
    msg,
    // ProgramError is the error enum for Solana programs.
    // It includes common variants like MissingRequiredSignature, InvalidInstructionData, etc.
    // Custom(u32) lets you define your own error codes (like custom errors in Solidity).
    program_error::ProgramError,
    // Pubkey is a 32-byte public key — the Solana equivalent of `address` in Solidity.
    pubkey::Pubkey,
};

// =============================================================================
// Exercise 1 Solution: Set up the entrypoint
// =============================================================================
//
// Register process_instruction as the program's entrypoint.
// The Solana runtime will call this function for every instruction targeting our program.
entrypoint!(process_instruction);

// The main dispatcher. Routes to the appropriate exercise based on instruction_data[0].
// This pattern (matching on the first byte) is how native Solana programs implement
// function dispatch — similar to how the EVM uses the first 4 bytes of calldata
// as a function selector.
fn process_instruction(
    // The public key of this deployed program (like address(this) in Solidity).
    program_id: &Pubkey,
    // All accounts the client passed to this instruction.
    // The client must declare every account upfront — unlike Solidity where storage is implicit.
    accounts: &[AccountInfo],
    // Raw instruction bytes. No ABI encoding — you parse it however you want.
    instruction_data: &[u8],
) -> ProgramResult {
    // Read the first byte as the exercise selector.
    // .first() returns Option<&u8>, .copied() converts to Option<u8>,
    // .unwrap_or(0) defaults to 0 if instruction_data is empty.
    let exercise = instruction_data.first().copied().unwrap_or(0);

    // Dispatch to the correct exercise handler.
    // This is our manual "function selector" — in Solidity, the compiler generates this for you.
    match exercise {
        // Each arm calls the exercise function and uses ? to propagate any errors.
        1 => exercise_1_log_greeting(program_id)?,
        2 => exercise_2_log_accounts(accounts)?,
        3 => exercise_3_validate_signer(accounts)?,
        4 => exercise_4_read_instruction_data(instruction_data)?,
        5 => exercise_5_return_error(accounts)?,
        // Default case: log a helpful message if the exercise number is unknown.
        _ => msg!("Unknown exercise number. Pass 1-5 as the first byte of instruction_data."),
    }

    // If we reach here, the instruction succeeded. Return Ok(()) to commit the transaction.
    Ok(())
}

// =============================================================================
// Exercise 1 Solution: Log a greeting with the program ID
// =============================================================================
fn exercise_1_log_greeting(program_id: &Pubkey) -> ProgramResult {
    // Log a greeting message to the transaction log.
    // msg! works like println! but writes to Solana's logging system.
    // Visible in `solana logs`, block explorers, and test output.
    msg!("Hello from a native Solana program!");

    // Log the program's own address (public key).
    // This is the equivalent of logging address(this) in Solidity.
    // Pubkey implements the Display trait, so {} formatting works.
    msg!("Program ID: {}", program_id);

    // Return success — the instruction completed without errors.
    Ok(())
}

// =============================================================================
// Exercise 2 Solution: Log detailed account information
// =============================================================================
fn exercise_2_log_accounts(accounts: &[AccountInfo]) -> ProgramResult {
    // Log the total number of accounts passed to this instruction.
    // In Solidity, you don't think about this — storage and msg.sender are implicit.
    // In Solana, every account the instruction touches must be explicitly listed.
    msg!("Total accounts passed: {}", accounts.len());

    // Iterate over each account with its index.
    // .iter() borrows each AccountInfo, .enumerate() adds the index.
    for (i, account) in accounts.iter().enumerate() {
        // Log the account's public key (on-chain address).
        msg!("--- Account {} ---", i);
        msg!("  Key: {}", account.key);

        // Log whether this account signed the transaction.
        // true means the account's private key was used to sign.
        // This is the primary mechanism for authorization — like checking msg.sender in Solidity.
        msg!("  Is signer: {}", account.is_signer);

        // Log whether the instruction is allowed to modify this account.
        // The client declares writability upfront and the runtime enforces it.
        // No direct Solidity equivalent — in Solidity, contracts can write to their own storage freely.
        msg!("  Is writable: {}", account.is_writable);

        // Log the account balance in lamports (1 SOL = 1,000,000,000 lamports).
        // The lamports() method returns the current balance.
        // Like address(account).balance in Solidity, but measured in lamports not wei.
        msg!("  Lamports: {}", account.lamports());

        // Log the size of the account's data buffer in bytes.
        // This is where the account stores its state — like a contract's storage in Solidity.
        // An empty account has data_len() == 0.
        msg!("  Data length: {} bytes", account.data_len());

        // Log which program owns this account.
        // Only the owner program can modify an account's data and debit its lamports.
        // In Solidity, ownership is implicit — a contract owns its own storage.
        msg!("  Owner: {}", account.owner);
    }

    // Return success.
    Ok(())
}

// =============================================================================
// Exercise 3 Solution: Validate that the first account is a signer
// =============================================================================
fn exercise_3_validate_signer(accounts: &[AccountInfo]) -> ProgramResult {
    // Create a mutable iterator over the accounts slice.
    // next_account_info() advances this iterator each time it's called.
    let accounts_iter = &mut accounts.iter();

    // Get the first account from the iterator.
    // The ? operator returns early with an error if there are no accounts.
    // This would return ProgramError::NotEnoughAccountKeys.
    let first_account = next_account_info(accounts_iter)?;

    // Check if this account signed the transaction.
    // This is the Solana equivalent of require(msg.sender == authorized) in Solidity.
    // The is_signer flag is set by the runtime and CANNOT be forged.
    if !first_account.is_signer {
        // Return an error if the account didn't sign.
        // MissingRequiredSignature is the standard error for this case.
        // The transaction will be rolled back — like revert in Solidity.
        msg!("Error: Account {} did not sign the transaction!", first_account.key);
        return Err(ProgramError::MissingRequiredSignature);
    }

    // If we reach here, the signer check passed. Log success.
    msg!(
        "Signer validated! Account {} is an authorized signer.",
        first_account.key
    );

    // Return success.
    Ok(())
}

// =============================================================================
// Exercise 4 Solution: Read and log instruction_data bytes
// =============================================================================
fn exercise_4_read_instruction_data(instruction_data: &[u8]) -> ProgramResult {
    // Log the total length of the instruction data.
    // This includes the exercise selector byte at index 0.
    msg!("Instruction data length: {} bytes", instruction_data.len());

    // Log the raw data bytes after the selector (index 0).
    // {:?} uses the Debug trait to print the slice as [byte1, byte2, ...].
    // In Solidity, the compiler handles ABI decoding for you.
    // In native Solana, you get raw bytes and must parse them yourself.
    msg!("Data after selector: {:?}", &instruction_data[1..]);

    // Bonus: If there are enough bytes, decode a u32 from bytes 1..5.
    // Solana (and most modern systems) use little-endian byte order.
    // This is like abi.decode(data, (uint32)) in Solidity.
    if instruction_data.len() >= 5 {
        // Convert the 4-byte slice into a fixed-size array.
        // try_into() converts &[u8] to [u8; 4], which from_le_bytes requires.
        // unwrap() is safe here because we already checked the length.
        let bytes: [u8; 4] = instruction_data[1..5].try_into().unwrap();

        // Interpret the 4 bytes as a little-endian u32.
        let value = u32::from_le_bytes(bytes);

        // Log the decoded value.
        msg!("Decoded u32 from bytes 1..5: {}", value);
    } else {
        // Not enough data to decode a u32. Log what we have.
        msg!(
            "Not enough data to decode a u32 (need 5 bytes, got {})",
            instruction_data.len()
        );
    }

    // Return success.
    Ok(())
}

// =============================================================================
// Exercise 5 Solution: Return a custom ProgramError when validation fails
// =============================================================================
fn exercise_5_return_error(accounts: &[AccountInfo]) -> ProgramResult {
    // Create an iterator and get the first account.
    let accounts_iter = &mut accounts.iter();

    // Get the target account. If no accounts were passed, this returns an error automatically.
    let target_account = next_account_info(accounts_iter)?;

    // Check if the account is writable.
    // In a real program, you'd check this before writing to the account's data.
    // The runtime also enforces this, but checking in your program is defense-in-depth.
    if !target_account.is_writable {
        // Log the error for debugging (logs are visible even on failed transactions).
        msg!(
            "Error: Account {} is not writable!",
            target_account.key
        );

        // Return a custom error with code 100.
        // ProgramError::Custom(u32) lets you define application-specific error codes.
        // This is like revert NotWritable() with a custom error in Solidity.
        // Clients can match on this error code to provide user-friendly messages.
        return Err(ProgramError::Custom(100));
    }

    // Validation passed — the account is writable.
    msg!(
        "Success! Account {} is writable. Ready for data operations.",
        target_account.key
    );

    // Return success.
    Ok(())
}
