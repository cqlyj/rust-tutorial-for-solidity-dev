// ============================================================================
// Module 11 Exercises: Program Derived Addresses (PDAs)
// ============================================================================
// Complete each exercise by replacing the todo!() macros with working code.
// Run `cargo check` to verify your solutions compile.
//
// These exercises build your understanding of PDA derivation, seed construction,
// address verification, and account creation via invoke_signed.
// ============================================================================

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    system_program,
    sysvar::Sysvar,
};

// ============================================================================
// Exercise 1: Derive a PDA from Seeds
// ============================================================================
// Given a user's pubkey and a program ID, derive the PDA for their profile.
//
// Solidity comparison: This is like computing a deterministic address with
// address(uint160(uint256(keccak256(abi.encodePacked(...)))))
// but the result is guaranteed to have no private key.
//
// Use Pubkey::find_program_address with seeds: [b"user-profile", user_pubkey]
// Return both the PDA pubkey and the canonical bump.
pub fn exercise_1_derive_pda(
    user_pubkey: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    // TODO: Use Pubkey::find_program_address to derive a PDA.
    // Seeds should be: [b"user-profile", user_pubkey.as_ref()]
    // Return the (pda, bump) tuple.
    todo!("Derive the PDA using find_program_address")
}

// ============================================================================
// Exercise 2: Create Seed Arrays for Different Patterns
// ============================================================================
// PDAs use different seed patterns depending on what they represent.
// Build the correct seeds for each scenario and return the derived PDA.

/// 2a: Global singleton — one account for the entire program.
/// Seeds: just a prefix string.
/// Solidity equivalent: a contract's state variables (single storage location).
pub fn exercise_2a_global_singleton(
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    // TODO: Derive a PDA with seeds [b"global-config"].
    // This creates a unique address that only exists once per program.
    todo!("Derive a global singleton PDA")
}

/// 2b: User-specific account — one per user.
/// Seeds: prefix + user's pubkey.
/// Solidity equivalent: mapping(address => Data).
pub fn exercise_2b_user_account(
    user_pubkey: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    // TODO: Derive a PDA with seeds [b"user-data", user_pubkey.as_ref()].
    todo!("Derive a user-specific PDA")
}

/// 2c: Relationship account — keyed by two pubkeys.
/// Seeds: prefix + mint pubkey + owner pubkey.
/// Solidity equivalent: mapping(address => mapping(address => Data)).
pub fn exercise_2c_relationship_account(
    mint_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    // TODO: Derive a PDA with seeds [b"token-balance", mint_pubkey.as_ref(), owner_pubkey.as_ref()].
    todo!("Derive a relationship PDA")
}

/// 2d: Sequential item — keyed by an integer ID.
/// Seeds: prefix + item_id as little-endian bytes.
/// Solidity equivalent: mapping(uint64 => Data).
pub fn exercise_2d_sequential_item(
    item_id: u64,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    // TODO: Derive a PDA with seeds [b"item", &item_id.to_le_bytes()].
    // Hint: to_le_bytes() converts u64 to [u8; 8] in little-endian format.
    todo!("Derive a sequential item PDA")
}

// ============================================================================
// Exercise 3: Verify a PDA Matches Expected Address
// ============================================================================
// Given a claimed PDA address and a bump, verify that it was correctly derived
// from the expected seeds and program ID.
//
// This is critical for security — without verification, an attacker could pass
// a fake account that isn't actually a valid PDA.
//
// Use Pubkey::create_program_address (the cheaper method that takes an explicit bump).
pub fn exercise_3_verify_pda(
    claimed_pda: &Pubkey,
    user_pubkey: &Pubkey,
    bump: u8,
    program_id: &Pubkey,
) -> Result<bool, ProgramError> {
    // TODO:
    // 1. Use Pubkey::create_program_address with seeds
    //    [b"user-profile", user_pubkey.as_ref(), &[bump]]
    //    and the program_id.
    // 2. Compare the result to claimed_pda.
    // 3. Return Ok(true) if they match, Ok(false) if they don't.
    //
    // Note: create_program_address returns Result<Pubkey, PubkeyError>.
    // If the bump produces an on-curve address, it returns an error —
    // map that to ProgramError::InvalidArgument.
    todo!("Verify the PDA using create_program_address")
}

// ============================================================================
// Exercise 4: Create an Account at a PDA Address Using invoke_signed
// ============================================================================
// This is the core PDA operation: creating an on-chain account at a
// deterministically derived address, with the program signing for the PDA.
//
// Solidity comparison: In Solidity, you just declare a mapping and write to it.
// In Solana, you must explicitly create the account, fund it with rent, and
// have the program sign for the PDA via invoke_signed.

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct VaultData {
    pub owner: Pubkey,
    pub balance: u64,
    pub bump: u8,
}

pub fn exercise_4_create_pda_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // Account 0: Payer (signer, writable) — funds the new account.
    let payer = next_account_info(accounts_iter)?;
    if !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Account 1: The PDA account to create (writable).
    let vault_pda = next_account_info(accounts_iter)?;

    // Account 2: System program.
    let system_program_account = next_account_info(accounts_iter)?;

    // TODO: Complete the following steps:
    //
    // Step 1: Derive the PDA using find_program_address.
    //   Seeds: [b"vault", payer.key.as_ref()]
    //   Store the returned (expected_pda, bump).
    //
    // Step 2: Verify vault_pda.key matches expected_pda.
    //   Return ProgramError::InvalidArgument if they don't match.
    //
    // Step 3: Create the VaultData struct with:
    //   - owner: *payer.key
    //   - balance: 0
    //   - bump: the bump from step 1
    //
    // Step 4: Serialize the VaultData to get the byte length (space).
    //   Use borsh::to_vec(&vault_data).
    //
    // Step 5: Calculate the rent-exempt minimum lamports.
    //   Use Rent::get()?.minimum_balance(space).
    //
    // Step 6: Call invoke_signed with:
    //   - system_instruction::create_account(payer, vault_pda, lamports, space, program_id)
    //   - account infos: [payer, vault_pda, system_program]
    //   - signer seeds: [b"vault", payer.key.as_ref(), &[bump]]
    //
    // Step 7: Write the serialized data to vault_pda.
    //   Use vault_pda.try_borrow_mut_data() and copy_from_slice.

    todo!("Create the PDA account using invoke_signed")
}

// ============================================================================
// Exercise 5: Read and Deserialize Data from a PDA Account
// ============================================================================
// Read the VaultData stored in a PDA, verify the PDA derivation,
// and return the deserialized data.

pub fn exercise_5_read_pda_data(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> Result<(Pubkey, u64, u8), ProgramError> {
    let accounts_iter = &mut accounts.iter();

    // Account 0: The vault PDA to read.
    let vault_pda = next_account_info(accounts_iter)?;

    // Account 1: The owner whose vault this should be.
    let owner = next_account_info(accounts_iter)?;

    // TODO: Complete the following steps:
    //
    // Step 1: Verify vault_pda.owner == program_id.
    //   Return ProgramError::IncorrectProgramId if not.
    //
    // Step 2: Deserialize the account data into VaultData.
    //   Use vault_pda.try_borrow_data() and VaultData::try_from_slice().
    //
    // Step 3: Verify the PDA by re-deriving it.
    //   Use Pubkey::create_program_address with seeds
    //   [b"vault", owner.key.as_ref(), &[vault_data.bump]]
    //   and confirm it matches vault_pda.key.
    //
    // Step 4: Return Ok((vault_data.owner, vault_data.balance, vault_data.bump)).

    todo!("Read and verify data from a PDA account")
}

// ============================================================================
// Exercise 6: Build Signer Seeds for invoke_signed
// ============================================================================
// Given an action that requires the program to sign for a PDA,
// construct the correct signer seeds array.
//
// This tests your understanding of how signer seeds map to PDA derivation.
// The signer seeds must exactly reproduce the PDA address when hashed
// with the program ID.

/// Construct signer seeds for a user-profile PDA.
/// Returns a Vec of Vec<u8> representing the seed slices.
///
/// The caller will convert these to &[&[u8]] for invoke_signed.
pub fn exercise_6_build_signer_seeds(
    user_pubkey: &Pubkey,
    bump: u8,
) -> Vec<Vec<u8>> {
    // TODO: Return a Vec containing three elements:
    //   1. b"user-profile".to_vec()       — the seed prefix
    //   2. user_pubkey.to_bytes().to_vec() — the user's pubkey as bytes
    //   3. vec![bump]                      — the bump as a single-byte vec
    //
    // These seeds, when passed to invoke_signed as &[&[u8]], allow the
    // program to sign for the PDA derived from these same seeds.
    todo!("Build the signer seeds vector")
}
