// =============================================================================
// Module 12 Solutions: Cross-Program Invocation (CPI)
// =============================================================================
// Every line is commented to explain the reasoning.
// =============================================================================

// Import everything we need from the Solana program SDK.
use solana_program::{
    // AccountInfo is the runtime representation of an on-chain account.
    // next_account_info is a helper to pull accounts from a slice iterator.
    account_info::{next_account_info, AccountInfo},
    // ProgramResult is an alias for Result<(), ProgramError>.
    entrypoint::ProgramResult,
    // AccountMeta describes an account's permissions in an Instruction.
    // Instruction is the struct we build for CPI targets.
    instruction::{AccountMeta, Instruction},
    // msg! logs to the transaction log (like console.log for Solana).
    msg,
    // invoke performs a CPI where all signers already signed the transaction.
    program::invoke,
    // invoke_signed performs a CPI where a PDA acts as signer.
    program::invoke_signed,
    // ProgramError is the standard error type for Solana programs.
    program_error::ProgramError,
    // Pubkey is a 32-byte public key (like an address in Solidity).
    pubkey::Pubkey,
    // system_instruction has helpers to build System Program instructions.
    system_instruction,
    // system_program::id() returns the System Program's well-known address.
    system_program,
};

// ===========================================================================
// Exercise 1 Solution: Build a System Program Transfer Instruction
// ===========================================================================
// We use the system_instruction::transfer helper to create a properly
// formatted Instruction for the System Program's transfer operation.
pub fn build_transfer_instruction(
    // The public key of the account sending SOL.
    from: &Pubkey,
    // The public key of the account receiving SOL.
    to: &Pubkey,
    // The amount of lamports (1 SOL = 1_000_000_000 lamports) to transfer.
    lamports: u64,
) -> Instruction {
    // system_instruction::transfer builds an Instruction with:
    //   program_id = system_program::id()
    //   accounts   = [AccountMeta(from, writable+signer), AccountMeta(to, writable)]
    //   data       = serialized transfer instruction (index 2 + lamports as u64)
    system_instruction::transfer(from, to, lamports)
}

// ===========================================================================
// Exercise 2 Solution: Transfer SOL via invoke()
// ===========================================================================
// This function performs a CPI to the System Program to transfer SOL.
pub fn exercise_transfer_sol(
    // The accounts passed to this instruction by the client.
    accounts: &[AccountInfo],
    // The amount of lamports to transfer.
    lamports: u64,
) -> ProgramResult {
    // Create a mutable iterator over the account slice.
    let account_iter = &mut accounts.iter();

    // Extract the three accounts in order.
    // Account 0: the sender — must be a signer and writable.
    let from = next_account_info(account_iter)?;
    // Account 1: the recipient — must be writable.
    let to = next_account_info(account_iter)?;
    // Account 2: the System Program — must be the real System Program.
    let system_program_account = next_account_info(account_iter)?;

    // SECURITY CHECK: Verify the System Program account is genuine.
    // Without this, a malicious client could substitute a fake program
    // that steals funds instead of transferring them.
    if system_program_account.key != &system_program::id() {
        // Log the error for debugging in transaction logs.
        msg!("Error: provided system program is not the real System Program");
        // Return a specific error so the client knows what went wrong.
        return Err(ProgramError::IncorrectProgramId);
    }

    // Build the transfer instruction using the System Program helper.
    // This creates a fully formed Instruction struct with the correct
    // program_id, account metas, and serialized instruction data.
    let transfer_ix = system_instruction::transfer(from.key, to.key, lamports);

    // Execute the CPI via invoke().
    // We pass references to AccountInfo for every account the callee needs.
    // The from account already has signer privilege from the original
    // transaction, and that privilege is forwarded through the CPI
    // (this is called privilege escalation).
    invoke(
        // The instruction describing what to execute.
        &transfer_ix,
        // The AccountInfo slice — the runtime matches these by pubkey
        // against the Instruction's AccountMeta entries.
        &[
            from.clone(),
            to.clone(),
            system_program_account.clone(),
        ],
    )?; // The ? propagates any CPI error back to our caller.

    // Log success for debugging.
    msg!("Transfer of {} lamports successful", lamports);

    // Return Ok to indicate the instruction completed successfully.
    Ok(())
}

// ===========================================================================
// Exercise 3 Solution: Transfer SOL from a PDA using invoke_signed()
// ===========================================================================
// The PDA is derived from seeds [b"treasury", authority_pubkey, bump_byte].
// Only our program can sign for this PDA because only our program knows
// the derivation and can call invoke_signed with the correct seeds.
pub fn exercise_pda_transfer(
    // The accounts the client attached to this instruction.
    accounts: &[AccountInfo],
    // How many lamports to transfer from the PDA to the recipient.
    lamports: u64,
    // The PDA bump seed — the client computes this off-chain and passes it in.
    bump: u8,
) -> ProgramResult {
    // Create the account iterator.
    let account_iter = &mut accounts.iter();

    // Account 0: the PDA (vault) — holds SOL, must be writable.
    let pda_account = next_account_info(account_iter)?;
    // Account 1: the recipient — will receive SOL, must be writable.
    let recipient = next_account_info(account_iter)?;
    // Account 2: the authority — the user who controls this vault, must sign.
    let authority = next_account_info(account_iter)?;
    // Account 3: the System Program.
    let system_program_account = next_account_info(account_iter)?;

    // SECURITY CHECK: Verify the System Program is real.
    if system_program_account.key != &system_program::id() {
        msg!("Error: invalid System Program");
        return Err(ProgramError::IncorrectProgramId);
    }

    // SECURITY CHECK: The authority must have signed the transaction.
    // Without this, anyone could drain the vault.
    if !authority.is_signer {
        msg!("Error: authority must be a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Build the transfer instruction.
    // The "from" is the PDA — it has no private key, so we need invoke_signed.
    let transfer_ix = system_instruction::transfer(
        pda_account.key, // source: the PDA
        recipient.key,   // destination: the recipient
        lamports,        // amount
    );

    // Define the seeds used to derive the PDA.
    // These must exactly match the seeds used when the PDA was created.
    // The runtime will re-derive the address from these seeds + our program_id
    // and verify it matches the pda_account's key.
    let signer_seeds: &[&[u8]] = &[
        b"treasury",           // fixed prefix seed
        authority.key.as_ref(), // the authority's pubkey as bytes
        &[bump],               // the bump byte that makes it a valid PDA
    ];

    // Call invoke_signed to execute the CPI with PDA signing.
    // The third argument is a slice of seed-slices — one per PDA that needs
    // to sign in this CPI. We only have one PDA signer here.
    invoke_signed(
        &transfer_ix,
        &[
            pda_account.clone(),
            recipient.clone(),
            system_program_account.clone(),
        ],
        // Outer slice: one entry per PDA signer.
        // Inner slice: the seeds for that PDA.
        &[signer_seeds],
    )?;

    // Log success.
    msg!(
        "PDA transfer: {} lamports from {} to {}",
        lamports,
        pda_account.key,
        recipient.key
    );

    Ok(())
}

// ===========================================================================
// Exercise 4 Solution: Verify Program ID Before CPI
// ===========================================================================
// A simple but critical security function: compare the target program's key
// against the expected program ID before making any CPI call.
pub fn exercise_verify_program_id(
    // The AccountInfo for the program we're about to CPI into.
    target_program: &AccountInfo,
    // The Pubkey we expect that program to have.
    expected_program_id: &Pubkey,
) -> ProgramResult {
    // Compare the account's key (a &Pubkey) against our expected value.
    if target_program.key != expected_program_id {
        // Log a descriptive error message for debugging.
        msg!(
            "Program ID mismatch: expected {}, got {}",
            expected_program_id,
            target_program.key
        );
        // Return the standard error for wrong program IDs.
        return Err(ProgramError::IncorrectProgramId);
    }

    // The keys match — safe to proceed with CPI.
    Ok(())
}

// ===========================================================================
// Exercise 5 Solution: Handle CPI Errors Gracefully
// ===========================================================================
// Demonstrates catching a CPI error, logging it, and returning a custom
// error code instead of the raw CPI error.
pub fn exercise_handle_cpi_error(
    // Accounts for the transfer CPI.
    accounts: &[AccountInfo],
    // Amount of lamports to transfer.
    lamports: u64,
) -> ProgramResult {
    // Extract accounts.
    let account_iter = &mut accounts.iter();
    // Account 0: sender.
    let from = next_account_info(account_iter)?;
    // Account 1: recipient.
    let to = next_account_info(account_iter)?;
    // Account 2: System Program.
    let system_program_account = next_account_info(account_iter)?;

    // Build the transfer instruction.
    let transfer_ix = system_instruction::transfer(from.key, to.key, lamports);

    // Call invoke() but capture the Result instead of using ?.
    // This lets us inspect and transform the error before returning.
    let cpi_result = invoke(
        &transfer_ix,
        &[
            from.clone(),
            to.clone(),
            system_program_account.clone(),
        ],
    );

    // Match on the CPI result to handle success and failure separately.
    match cpi_result {
        // CPI succeeded — log and return success.
        Ok(()) => {
            msg!("CPI transfer succeeded: {} lamports", lamports);
            Ok(())
        }
        // CPI failed — log the error and return our custom error code.
        Err(err) => {
            // Log the original error so it appears in transaction logs.
            msg!("CPI transfer failed with error: {}", err);
            // Return a custom error code (100) instead of the raw CPI error.
            // This lets our program's clients handle the error specifically.
            Err(ProgramError::Custom(100))
        }
    }
}

// ===========================================================================
// Tests
// ===========================================================================
#[cfg(test)]
mod tests {
    // Import everything from the parent module.
    use super::*;

    // Test Exercise 1: Verify the transfer instruction is well-formed.
    #[test]
    fn test_build_transfer_instruction() {
        // Create two unique pubkeys for from and to.
        let from = Pubkey::new_unique();
        let to = Pubkey::new_unique();
        // Set a transfer amount.
        let lamports = 1_000_000u64;

        // Build the instruction using our solution.
        let ix = build_transfer_instruction(&from, &to, lamports);

        // Verify the instruction targets the System Program.
        assert_eq!(ix.program_id, system_program::id());
        // Verify there are exactly 2 account metas (from and to).
        assert_eq!(ix.accounts.len(), 2);
        // Verify the first account is the sender: writable + signer.
        assert_eq!(ix.accounts[0].pubkey, from);
        assert!(ix.accounts[0].is_writable);
        assert!(ix.accounts[0].is_signer);
        // Verify the second account is the recipient: writable, not signer.
        assert_eq!(ix.accounts[1].pubkey, to);
        assert!(ix.accounts[1].is_writable);
        assert!(!ix.accounts[1].is_signer);
    }

    // Test Exercise 4: Mismatched program ID should error.
    #[test]
    fn test_verify_program_id_mismatch() {
        // Create two different pubkeys.
        let correct_id = Pubkey::new_unique();
        let wrong_id = Pubkey::new_unique();
        // Build a minimal AccountInfo whose key is wrong_id.
        let lamports = &mut 0u64;
        let mut data = vec![];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &wrong_id,  // key — this is the "program" we're checking
            false,      // is_signer
            false,      // is_writable
            lamports,   // lamports balance
            &mut data,  // account data
            &owner,     // owner program
            false,      // executable
            0,          // rent_epoch
        );
        // Call our verification function — should fail.
        let result = exercise_verify_program_id(&account, &correct_id);
        // Verify it returns IncorrectProgramId.
        assert_eq!(result, Err(ProgramError::IncorrectProgramId));
    }

    // Test Exercise 4: Matching program ID should succeed.
    #[test]
    fn test_verify_program_id_match() {
        // Use the same pubkey for both the account and the expected ID.
        let id = Pubkey::new_unique();
        let lamports = &mut 0u64;
        let mut data = vec![];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &id,        // key matches expected_program_id
            false,
            false,
            lamports,
            &mut data,
            &owner,
            false,
            0,
        );
        // Call our verification — should pass.
        let result = exercise_verify_program_id(&account, &id);
        // Verify success.
        assert_eq!(result, Ok(()));
    }
}
