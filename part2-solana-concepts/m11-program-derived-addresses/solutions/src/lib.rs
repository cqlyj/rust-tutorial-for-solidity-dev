// ============================================================================
// Module 11 Solutions: Program Derived Addresses (PDAs)
// ============================================================================
// Complete solutions with every line commented.
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
    sysvar::Sysvar,
};

// ============================================================================
// Exercise 1 Solution: Derive a PDA from Seeds
// ============================================================================
// find_program_address is the primary function for PDA derivation.
// It tries bump values 255→0 and returns the first address that is NOT
// on the Ed25519 curve, along with that bump value.
pub fn exercise_1_derive_pda(
    // The user's public key — used as part of the seed to make the PDA unique per user.
    user_pubkey: &Pubkey,
    // The program ID — the PDA is scoped to this specific program.
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    // Call find_program_address with two seeds:
    //   1. b"user-profile" — a descriptive prefix to namespace this PDA type
    //   2. user_pubkey.as_ref() — converts Pubkey to &[u8] (32 bytes)
    // Returns (Pubkey, u8) — the derived address and the canonical bump.
    let (pda, bump) = Pubkey::find_program_address(
        // Seeds are passed as a slice of byte slices.
        &[b"user-profile", user_pubkey.as_ref()],
        // The program that "owns" this PDA — only this program can sign for it.
        program_id,
    );
    // Return both the address and the bump.
    (pda, bump)
}

// ============================================================================
// Exercise 2a Solution: Global Singleton PDA
// ============================================================================
// A singleton PDA uses only a fixed prefix — no dynamic seeds.
// This means exactly ONE such address exists per program.
pub fn exercise_2a_global_singleton(
    // The program ID that owns this PDA.
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    // With just one seed (the prefix), the PDA is a global singleton.
    // Changing the prefix would create a different singleton.
    let (pda, bump) = Pubkey::find_program_address(
        // Only one seed: the prefix. No user-specific data.
        &[b"global-config"],
        program_id,
    );
    (pda, bump)
}

// ============================================================================
// Exercise 2b Solution: User-Specific PDA
// ============================================================================
// Combines a prefix with a user's pubkey to create a unique address per user.
// Solidity equivalent: mapping(address => Data).
pub fn exercise_2b_user_account(
    // The user whose account we're deriving.
    user_pubkey: &Pubkey,
    // The program ID.
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    // Two seeds: prefix + user pubkey.
    // Each unique user_pubkey produces a different PDA.
    let (pda, bump) = Pubkey::find_program_address(
        &[b"user-data", user_pubkey.as_ref()],
        program_id,
    );
    (pda, bump)
}

// ============================================================================
// Exercise 2c Solution: Relationship PDA
// ============================================================================
// Three seeds model a relationship between two entities.
// Solidity equivalent: mapping(address => mapping(address => Data)).
pub fn exercise_2c_relationship_account(
    // The first key in the relationship (e.g., token mint).
    mint_pubkey: &Pubkey,
    // The second key (e.g., token owner).
    owner_pubkey: &Pubkey,
    // The program ID.
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    // Three seeds: prefix + mint + owner.
    // Order matters! [mint, owner] ≠ [owner, mint].
    let (pda, bump) = Pubkey::find_program_address(
        &[b"token-balance", mint_pubkey.as_ref(), owner_pubkey.as_ref()],
        program_id,
    );
    (pda, bump)
}

// ============================================================================
// Exercise 2d Solution: Sequential Item PDA
// ============================================================================
// Uses an integer ID as a seed by converting it to little-endian bytes.
// Solidity equivalent: mapping(uint64 => Data).
pub fn exercise_2d_sequential_item(
    // The item's numeric ID.
    item_id: u64,
    // The program ID.
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    // Convert the u64 to 8 bytes in little-endian format.
    // to_le_bytes() returns [u8; 8], and & gives us a &[u8] slice.
    let (pda, bump) = Pubkey::find_program_address(
        &[b"item", &item_id.to_le_bytes()],
        program_id,
    );
    (pda, bump)
}

// ============================================================================
// Exercise 3 Solution: Verify a PDA Matches Expected Address
// ============================================================================
// Uses create_program_address (the cheaper single-attempt method) to verify
// that a claimed PDA was actually derived from the expected seeds.
pub fn exercise_3_verify_pda(
    // The address someone claims is a valid PDA.
    claimed_pda: &Pubkey,
    // The user whose pubkey should be in the seeds.
    user_pubkey: &Pubkey,
    // The bump seed to use for derivation.
    bump: u8,
    // The program ID.
    program_id: &Pubkey,
) -> Result<bool, ProgramError> {
    // create_program_address takes seeds WITH the bump already included.
    // It does a single SHA-256 hash and checks if the result is off-curve.
    // Returns Err if the address lands on the curve (invalid bump for these seeds).
    let derived_pda = Pubkey::create_program_address(
        // Three seeds: prefix + user pubkey + bump byte.
        // The bump is the last seed, passed as a single-element slice.
        &[b"user-profile", user_pubkey.as_ref(), &[bump]],
        program_id,
    )
    // Map the PubkeyError to a ProgramError so the types align.
    .map_err(|_| ProgramError::InvalidArgument)?;

    // Compare the derived address with the claimed one.
    // If they match, the claimed PDA is valid for these seeds and bump.
    Ok(derived_pda == *claimed_pda)
}

// ============================================================================
// Exercise 4 Solution: Create an Account at a PDA Address
// ============================================================================
// The full flow: derive PDA → verify → create account via CPI → write data.

// Account data structure for the vault — stores owner, balance, and bump.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct VaultData {
    // The public key of the vault's owner.
    pub owner: Pubkey,
    // The SOL balance tracked by this vault (application-level, not lamports).
    pub balance: u64,
    // The canonical bump used to derive this PDA — stored to save future compute.
    pub bump: u8,
}

pub fn exercise_4_create_pda_account(
    // This program's public key.
    program_id: &Pubkey,
    // All accounts passed to this instruction.
    accounts: &[AccountInfo],
) -> ProgramResult {
    // Create an iterator to consume accounts in order.
    let accounts_iter = &mut accounts.iter();

    // Account 0: The payer — must be a signer (authorizes the SOL transfer).
    let payer = next_account_info(accounts_iter)?;
    // Verify payer actually signed this transaction.
    if !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Account 1: The vault PDA — will be created at this address.
    let vault_pda = next_account_info(accounts_iter)?;

    // Account 2: The System Program — required for the create_account CPI.
    let system_program_account = next_account_info(accounts_iter)?;

    // Step 1: Derive the PDA to get the expected address and canonical bump.
    // Seeds are [b"vault", payer's pubkey] — one vault per user.
    let (expected_pda, bump) = Pubkey::find_program_address(
        &[b"vault", payer.key.as_ref()],
        program_id,
    );

    // Step 2: Verify the passed account matches the derived PDA.
    // Without this check, an attacker could pass any account.
    if vault_pda.key != &expected_pda {
        msg!("Vault PDA mismatch");
        return Err(ProgramError::InvalidArgument);
    }

    // Step 3: Build the vault data.
    let vault_data = VaultData {
        // Owner is the payer who created this vault.
        owner: *payer.key,
        // Start with zero balance.
        balance: 0,
        // Store the canonical bump for cheap future re-derivation.
        bump,
    };

    // Step 4: Serialize the data to determine the required account size.
    let serialized = borsh::to_vec(&vault_data)
        .map_err(|_| ProgramError::InvalidAccountData)?;
    // The account's data field must be exactly this many bytes.
    let space = serialized.len();

    // Step 5: Calculate rent-exempt minimum — the SOL deposit needed so
    // the account is never garbage-collected by the runtime.
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(space);

    // Step 6: Create the account via CPI with invoke_signed.
    // The program "signs" for the PDA by providing the derivation seeds.
    // The Solana runtime verifies: SHA256(seeds + program_id) == vault_pda.key.
    invoke_signed(
        // Build the create_account instruction.
        &system_instruction::create_account(
            payer.key,       // Who pays the rent deposit.
            vault_pda.key,   // The new account's address (the PDA).
            lamports,        // SOL to deposit for rent exemption.
            space as u64,    // Data field size in bytes.
            program_id,      // The owner of the new account — this program.
        ),
        // AccountInfo references for all accounts involved in the CPI.
        &[
            payer.clone(),
            vault_pda.clone(),
            system_program_account.clone(),
        ],
        // Signer seeds — proves this program derived the PDA.
        // Must include the bump as the final seed element.
        &[&[b"vault", payer.key.as_ref(), &[bump]]],
    )?;

    // Step 7: Write the serialized data into the newly created account.
    // try_borrow_mut_data() gives mutable access to the account's data bytes.
    let mut account_data = vault_pda.try_borrow_mut_data()?;
    // Copy the serialized bytes into the account's data field.
    account_data[..serialized.len()].copy_from_slice(&serialized);

    // Log success for debugging.
    msg!("Created vault PDA: {}", vault_pda.key);
    msg!("Owner: {}, Bump: {}", payer.key, bump);

    Ok(())
}

// ============================================================================
// Exercise 5 Solution: Read and Deserialize Data from a PDA Account
// ============================================================================
// Reads a vault PDA, verifies its derivation, and returns the data.
pub fn exercise_5_read_pda_data(
    // This program's public key.
    program_id: &Pubkey,
    // All accounts passed to this instruction.
    accounts: &[AccountInfo],
) -> Result<(Pubkey, u64, u8), ProgramError> {
    // Consume accounts in order.
    let accounts_iter = &mut accounts.iter();

    // Account 0: The vault PDA to read from.
    let vault_pda = next_account_info(accounts_iter)?;

    // Account 1: The expected owner of this vault.
    let owner = next_account_info(accounts_iter)?;

    // Step 1: Verify this account is actually owned by our program.
    // If someone passes an account owned by a different program,
    // we could read garbage data or be tricked into incorrect behavior.
    if vault_pda.owner != program_id {
        msg!("Account not owned by this program");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Step 2: Deserialize the account data into our VaultData struct.
    // try_borrow_data() gives us a read-only reference to the raw bytes.
    let data = vault_pda.try_borrow_data()?;
    // Borsh deserialization converts raw bytes back into the struct.
    let vault_data = VaultData::try_from_slice(&data)
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // Step 3: Re-derive the PDA using the stored bump to verify the address.
    // create_program_address is cheaper than find_program_address because
    // it tries exactly one bump instead of iterating 255→0.
    let derived_pda = Pubkey::create_program_address(
        // Use the same seeds as creation, with the stored bump.
        &[b"vault", owner.key.as_ref(), &[vault_data.bump]],
        program_id,
    )
    .map_err(|_| ProgramError::InvalidArgument)?;

    // Verify the derived address matches the account that was passed in.
    if derived_pda != *vault_pda.key {
        msg!("PDA verification failed — account doesn't match derivation");
        return Err(ProgramError::InvalidArgument);
    }

    // Log the data for debugging.
    msg!("Vault owner: {}", vault_data.owner);
    msg!("Vault balance: {}", vault_data.balance);
    msg!("Vault bump: {}", vault_data.bump);

    // Step 4: Return the deserialized fields as a tuple.
    Ok((vault_data.owner, vault_data.balance, vault_data.bump))
}

// ============================================================================
// Exercise 6 Solution: Build Signer Seeds for invoke_signed
// ============================================================================
// Constructs the seed components as owned Vec<u8> values.
// The caller converts these to &[&[u8]] for use with invoke_signed.
pub fn exercise_6_build_signer_seeds(
    // The user's public key — part of the PDA derivation.
    user_pubkey: &Pubkey,
    // The canonical bump — must match what find_program_address returned.
    bump: u8,
) -> Vec<Vec<u8>> {
    // Build a vector of three seed components:
    vec![
        // Seed 1: The prefix string as bytes.
        // b"user-profile" is a &[u8; 12] — .to_vec() converts to owned Vec<u8>.
        b"user-profile".to_vec(),
        // Seed 2: The user's pubkey as 32 bytes.
        // .to_bytes() returns [u8; 32], .to_vec() converts to Vec<u8>.
        user_pubkey.to_bytes().to_vec(),
        // Seed 3: The bump as a single byte.
        // vec![bump] creates a Vec<u8> with one element.
        // In invoke_signed, this becomes &[bump] — a single-byte slice.
        vec![bump],
    ]
    // When using these with invoke_signed, the caller converts:
    //   let seed_slices: Vec<&[u8]> = seeds.iter().map(|s| s.as_slice()).collect();
    //   invoke_signed(&ix, &accounts, &[&seed_slices])?;
}
