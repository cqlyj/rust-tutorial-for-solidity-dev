// =============================================================================
// Module 12 Exercises: Cross-Program Invocation (CPI)
// =============================================================================
// Complete each exercise by replacing the `todo!()` macros with working code.
// Run `cargo test` to verify your solutions.
//
// Reminder — key imports you'll need:
//   solana_program::program::invoke
//   solana_program::program::invoke_signed
//   solana_program::system_instruction
//   solana_program::instruction::{AccountMeta, Instruction}
//   solana_program::system_program
//   solana_program::pubkey::Pubkey
// =============================================================================

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    system_program,
};

// ===========================================================================
// Exercise 1: Build a System Program Transfer Instruction
// ===========================================================================
// Use `system_instruction::transfer` to create an Instruction that transfers
// `lamports` from `from` to `to`.
//
// Hint: system_instruction::transfer(from_pubkey, to_pubkey, lamports)
//       returns an Instruction.
pub fn build_transfer_instruction(
    from: &Pubkey,
    to: &Pubkey,
    lamports: u64,
) -> Instruction {
    // TODO: Return a System Program transfer instruction.
    todo!("Exercise 1: build a transfer instruction using system_instruction::transfer")
}

// ===========================================================================
// Exercise 2: Transfer SOL via invoke()
// ===========================================================================
// Given a list of accounts, transfer `lamports` from accounts[0] to
// accounts[1] via CPI to the System Program (accounts[2]).
//
// Steps:
//   1. Extract accounts: from (index 0), to (index 1), system_program (index 2)
//   2. Verify the system program account key matches system_program::id()
//   3. Build a transfer instruction with system_instruction::transfer
//   4. Call invoke() with the instruction and relevant AccountInfos
//
// Hint: invoke(&instruction, &[from.clone(), to.clone(), sys.clone()])
pub fn exercise_transfer_sol(
    accounts: &[AccountInfo],
    lamports: u64,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let from = next_account_info(account_iter)?;
    let to = next_account_info(account_iter)?;
    let system_program_account = next_account_info(account_iter)?;

    // TODO: Verify the system program account is correct.
    // TODO: Build the transfer instruction.
    // TODO: Call invoke() with the instruction and accounts.
    todo!("Exercise 2: invoke a System Program transfer CPI")
}

// ===========================================================================
// Exercise 3: Transfer SOL from a PDA using invoke_signed()
// ===========================================================================
// The PDA is derived from seeds: [b"treasury", authority.key, &[bump]].
// Transfer `lamports` from the PDA to the recipient.
//
// Accounts:
//   [0] pda_account    — writable (the PDA holding SOL)
//   [1] recipient      — writable
//   [2] authority      — signer
//   [3] system_program — the System Program
//
// Steps:
//   1. Extract all four accounts
//   2. Verify the system program
//   3. Verify the authority is a signer
//   4. Build a transfer instruction from pda_account to recipient
//   5. Call invoke_signed() with signer_seeds = &[b"treasury", authority.key.as_ref(), &[bump]]
pub fn exercise_pda_transfer(
    accounts: &[AccountInfo],
    lamports: u64,
    bump: u8,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let pda_account = next_account_info(account_iter)?;
    let recipient = next_account_info(account_iter)?;
    let authority = next_account_info(account_iter)?;
    let system_program_account = next_account_info(account_iter)?;

    // TODO: Verify the system program account key.
    // TODO: Verify authority is a signer.
    // TODO: Build the transfer instruction (from PDA to recipient).
    // TODO: Define signer_seeds and call invoke_signed().
    todo!("Exercise 3: invoke_signed with PDA seeds")
}

// ===========================================================================
// Exercise 4: Verify Program ID Before CPI
// ===========================================================================
// This function receives a `target_program` AccountInfo and a known
// `expected_program_id`. Verify they match before proceeding.
//
// Return Ok(()) if they match, or Err(ProgramError::IncorrectProgramId) if not.
//
// This is a critical security check — never CPI to an unverified program.
pub fn exercise_verify_program_id(
    target_program: &AccountInfo,
    expected_program_id: &Pubkey,
) -> ProgramResult {
    // TODO: Compare target_program.key with expected_program_id.
    // TODO: Return IncorrectProgramId error if they don't match.
    // TODO: Return Ok(()) if they match.
    todo!("Exercise 4: verify program ID before CPI")
}

// ===========================================================================
// Exercise 5: Handle CPI Errors Gracefully
// ===========================================================================
// Attempt a SOL transfer via CPI. If the CPI fails, log the error and
// return a custom error (ProgramError::Custom(100)) instead of the
// original error.
//
// Accounts:
//   [0] from           — signer, writable
//   [1] to             — writable
//   [2] system_program
//
// Steps:
//   1. Extract accounts
//   2. Build a transfer instruction
//   3. Call invoke() but DO NOT use `?` — instead, match on the Result
//   4. On Ok(()) — log success and return Ok(())
//   5. On Err(e) — log the error with msg!() and return Err(ProgramError::Custom(100))
pub fn exercise_handle_cpi_error(
    accounts: &[AccountInfo],
    lamports: u64,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let from = next_account_info(account_iter)?;
    let to = next_account_info(account_iter)?;
    let system_program_account = next_account_info(account_iter)?;

    // TODO: Build the transfer instruction.
    // TODO: Call invoke() and capture the Result (don't use ?).
    // TODO: Match on the result — log success or log+remap the error.
    todo!("Exercise 5: handle CPI errors gracefully")
}

// ===========================================================================
// Tests — run with `cargo test`
// ===========================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // Test Exercise 1: verify the instruction has correct fields.
    #[test]
    fn test_build_transfer_instruction() {
        let from = Pubkey::new_unique();
        let to = Pubkey::new_unique();
        let lamports = 1_000_000u64;

        let ix = build_transfer_instruction(&from, &to, lamports);

        // The instruction should target the System Program.
        assert_eq!(ix.program_id, system_program::id());
        // It should have exactly 2 account metas.
        assert_eq!(ix.accounts.len(), 2);
        // First account = from, writable + signer.
        assert_eq!(ix.accounts[0].pubkey, from);
        assert!(ix.accounts[0].is_writable);
        assert!(ix.accounts[0].is_signer);
        // Second account = to, writable, not signer.
        assert_eq!(ix.accounts[1].pubkey, to);
        assert!(ix.accounts[1].is_writable);
        assert!(!ix.accounts[1].is_signer);
    }

    // Test Exercise 4: program ID verification.
    #[test]
    fn test_verify_program_id_mismatch() {
        let correct_id = Pubkey::new_unique();
        let wrong_id = Pubkey::new_unique();
        // Create a minimal AccountInfo with key = wrong_id.
        let lamports = &mut 0u64;
        let mut data = vec![];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &wrong_id,   // key
            false,       // is_signer
            false,       // is_writable
            lamports,    // lamports
            &mut data,   // data
            &owner,      // owner
            false,       // executable
            0,           // rent_epoch
        );
        // Should return IncorrectProgramId because wrong_id != correct_id.
        let result = exercise_verify_program_id(&account, &correct_id);
        assert_eq!(result, Err(ProgramError::IncorrectProgramId));
    }

    // Test Exercise 4: matching IDs should succeed.
    #[test]
    fn test_verify_program_id_match() {
        let id = Pubkey::new_unique();
        let lamports = &mut 0u64;
        let mut data = vec![];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &id,
            false,
            false,
            lamports,
            &mut data,
            &owner,
            false,
            0,
        );
        let result = exercise_verify_program_id(&account, &id);
        assert_eq!(result, Ok(()));
    }
}
