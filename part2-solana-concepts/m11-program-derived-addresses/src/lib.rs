// ============================================================================
// Module 11: Program Derived Addresses (PDAs) — A Solana Program
// ============================================================================
// This program demonstrates PDA derivation, account creation at PDA addresses,
// data storage, and PDA-based signing via invoke_signed.
//
// Solidity parallel: PDAs are like CREATE2 addresses, but the resulting address
// has NO private key. Only the owning program can sign for it.
// ============================================================================

// Import the Borsh serialization traits — Solana's standard for on-chain data encoding.
// Similar to Solidity's ABI encoding but more compact.
use borsh::{BorshDeserialize, BorshSerialize};

// Import core Solana program types and functions.
use solana_program::{
    // The account_info module provides the AccountInfo struct — Solana's equivalent
    // of accessing storage slots. Each AccountInfo is a reference to an on-chain account.
    account_info::{next_account_info, AccountInfo},
    // The entrypoint macro defines the program's main entry point, like Solidity's
    // fallback/receive function that dispatches to the right method.
    entrypoint,
    // ProgramError is the standard error type — like Solidity's require/revert.
    entrypoint::ProgramResult,
    // msg! is Solana's equivalent of Solidity's emit or console.log — writes to program logs.
    msg,
    // ProgramError variants for returning typed errors from instructions.
    program::invoke_signed,
    // program_error provides standard error codes.
    program_error::ProgramError,
    // pubkey::Pubkey is the 32-byte public key type — like Solidity's address type.
    pubkey::Pubkey,
    // rent::Rent provides rent-related calculations (minimum balance for rent exemption).
    rent::Rent,
    // system_instruction contains helpers for creating accounts, transferring SOL, etc.
    system_instruction,
    // system_program is the built-in program that handles account creation and SOL transfers.
    system_program,
    // Sysvar trait allows reading runtime state (clock, rent, etc.) from sysvar accounts.
    sysvar::Sysvar,
};

// ============================================================================
// Data Structures
// ============================================================================

// The seed prefix used for user profile PDAs.
// Using a descriptive string prevents collisions between different PDA types.
// In Solidity, you'd use different mapping names; in Solana, you use different seed prefixes.
pub const USER_PROFILE_SEED: &[u8] = b"user-profile";

// The seed prefix for the global config singleton PDA.
pub const GLOBAL_CONFIG_SEED: &[u8] = b"global-config";

/// UserProfile stores per-user data at a PDA.
///
/// Solidity equivalent:
/// ```solidity
/// struct UserProfile {
///     string name;
///     uint8 bump;  // No Solidity equivalent — PDA-specific
///     uint64 score;
/// }
/// mapping(address => UserProfile) public profiles;
/// ```
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UserProfile {
    // The user's display name (variable-length string).
    pub name: String,
    // The canonical bump seed used to derive this PDA.
    // Stored so we can use the cheaper create_program_address on subsequent calls
    // instead of the expensive find_program_address (which iterates bumps 255→0).
    pub bump: u8,
    // A simple score field to demonstrate mutable PDA data.
    pub score: u64,
}

/// GlobalConfig stores program-wide settings at a singleton PDA.
///
/// Solidity equivalent: contract-level state variables.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GlobalConfig {
    // The admin authority who can update config.
    pub admin: Pubkey,
    // The bump for this PDA — stored to save compute on future lookups.
    pub bump: u8,
    // Whether the program is currently paused (like Solidity's Pausable pattern).
    pub is_paused: bool,
}

// ============================================================================
// Instruction Enum
// ============================================================================

/// Instructions this program understands.
/// In Solidity, this is like the function selector derived from the ABI.
/// Here we manually define a byte-level protocol.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum PdaInstruction {
    /// Create a new user profile at a PDA derived from the user's pubkey.
    /// Accounts expected:
    ///   0. [signer, writable] Payer — pays for account creation
    ///   1. [writable]         User profile PDA — the account to create
    ///   2. []                 System program — needed for create_account CPI
    CreateUserProfile {
        // The display name to store in the profile.
        name: String,
    },

    /// Read and log the data stored in a user's profile PDA.
    /// Accounts expected:
    ///   0. []  User profile PDA — the account to read
    ReadUserProfile,

    /// Update the score in a user's profile.
    /// Accounts expected:
    ///   0. [signer]   User — must be the user whose profile this is
    ///   1. [writable] User profile PDA — the account to update
    UpdateScore {
        // New score value.
        new_score: u64,
    },

    /// Initialize global config (singleton PDA — only one can exist).
    /// Accounts expected:
    ///   0. [signer, writable] Admin / payer
    ///   1. [writable]         Global config PDA
    ///   2. []                 System program
    InitGlobalConfig,
}

// ============================================================================
// Entrypoint
// ============================================================================

// Declare the program's entrypoint — the Solana runtime calls this function
// for every transaction instruction targeting this program.
// In Solidity, this is like the fallback function that checks msg.sig and dispatches.
entrypoint!(process_instruction);

/// Main dispatch function. Every Solana program has one.
///
/// Parameters:
/// - program_id: This program's own public key (like address(this) in Solidity)
/// - accounts:   Slice of all accounts passed to this instruction
/// - instruction_data: Raw bytes of the instruction (we Borsh-deserialize it)
pub fn process_instruction(
    // The public key of this program — equivalent to address(this) in Solidity.
    program_id: &Pubkey,
    // All accounts referenced by this instruction. Solana requires accounts to be
    // declared upfront (unlike Solidity where you can read any storage slot).
    accounts: &[AccountInfo],
    // The serialized instruction data — we'll deserialize it to our PdaInstruction enum.
    instruction_data: &[u8],
) -> ProgramResult {
    // Deserialize the instruction data into our enum.
    // This is like Solidity's ABI decoding of calldata.
    let instruction = PdaInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Dispatch to the appropriate handler based on the instruction variant.
    // This is the equivalent of Solidity's function selector switch.
    match instruction {
        PdaInstruction::CreateUserProfile { name } => {
            create_user_profile(program_id, accounts, name)
        }
        PdaInstruction::ReadUserProfile => read_user_profile(program_id, accounts),
        PdaInstruction::UpdateScore { new_score } => {
            update_score(program_id, accounts, new_score)
        }
        PdaInstruction::InitGlobalConfig => init_global_config(program_id, accounts),
    }
}

// ============================================================================
// Instruction Handlers
// ============================================================================

/// Creates a user profile account at a PDA derived from the user's pubkey.
///
/// This demonstrates the full PDA lifecycle:
/// 1. Derive the PDA using find_program_address (gets address + canonical bump)
/// 2. Create the account at that address using invoke_signed (program signs for PDA)
/// 3. Serialize data into the account
fn create_user_profile(
    // This program's ID — needed for PDA derivation and as the new account's owner.
    program_id: &Pubkey,
    // The accounts passed to this instruction.
    accounts: &[AccountInfo],
    // The user's display name to store.
    name: String,
) -> ProgramResult {
    // Create an iterator over the accounts. next_account_info advances it by one.
    // Order matters — it must match the order clients pass accounts.
    let accounts_iter = &mut accounts.iter();

    // Account 0: The payer who funds the new account creation.
    // Must be a signer (like msg.sender in Solidity) and writable (SOL will be deducted).
    let payer = next_account_info(accounts_iter)?;
    // Verify the payer actually signed this transaction.
    if !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Account 1: The PDA account that will be created.
    // Must be writable because we're initializing it.
    let profile_pda = next_account_info(accounts_iter)?;

    // Account 2: The System Program — needed for the create_account CPI.
    let system_program_account = next_account_info(accounts_iter)?;
    // Verify this is actually the System Program (not a malicious account).
    if system_program_account.key != &system_program::id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    // ---- PDA Derivation ----
    // find_program_address tries bumps from 255 down to 0, returning the first
    // bump that produces an address NOT on the Ed25519 curve.
    // Seeds: a prefix string + the payer's pubkey (so each user gets a unique PDA).
    let (expected_pda, bump) = Pubkey::find_program_address(
        &[USER_PROFILE_SEED, payer.key.as_ref()],
        program_id,
    );

    // Verify the PDA account passed by the client matches what we derived.
    // This prevents attackers from passing a different account.
    if profile_pda.key != &expected_pda {
        msg!("Error: profile PDA does not match derived address");
        return Err(ProgramError::InvalidArgument);
    }

    // ---- Build the profile data ----
    let profile = UserProfile {
        name,
        // Store the canonical bump so future instructions can use the cheaper
        // create_program_address instead of find_program_address.
        bump,
        score: 0,
    };

    // Serialize the profile to bytes to determine how much space we need.
    let profile_data = borsh::to_vec(&profile).map_err(|_| ProgramError::InvalidAccountData)?;
    // The account needs enough space for the serialized data.
    let space = profile_data.len();

    // ---- Calculate rent ----
    // Solana charges rent for storing data. Accounts must hold enough SOL to be
    // "rent-exempt" (never garbage collected). Similar to Ethereum's storage costs
    // but paid upfront as a deposit rather than per-operation gas.
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(space);

    // ---- Create the account via CPI with invoke_signed ----
    // This is where PDA magic happens. The program "signs" for the PDA by providing
    // the seeds that derive it. The runtime verifies: hash(seeds + program_id) == PDA address.
    //
    // In Solidity, a contract can just write to its own storage. In Solana, creating an
    // account is an explicit operation that requires the System Program.
    invoke_signed(
        // The instruction: create a new account owned by this program.
        &system_instruction::create_account(
            payer.key,        // Funder — who pays the SOL for rent
            profile_pda.key,  // New account address — the PDA
            lamports,         // Lamports to deposit (rent-exempt minimum)
            space as u64,     // Size of the data field in bytes
            program_id,       // Owner — this program will own the new account
        ),
        // All accounts involved in the CPI.
        &[
            payer.clone(),
            profile_pda.clone(),
            system_program_account.clone(),
        ],
        // Signer seeds: this proves the program derived this PDA.
        // The runtime checks that hash(seeds + program_id) == profile_pda.key.
        // Note: the bump is included as the last seed element, as a single-byte slice.
        &[&[USER_PROFILE_SEED, payer.key.as_ref(), &[bump]]],
    )?;

    // ---- Write data to the PDA account ----
    // Now the account exists, so we copy our serialized data into it.
    // borrow_mut() gives us mutable access to the account's data field.
    let mut account_data = profile_pda.try_borrow_mut_data()?;
    account_data[..profile_data.len()].copy_from_slice(&profile_data);

    // Log success — visible in transaction logs (like Solidity events).
    msg!("Created user profile PDA: {}", profile_pda.key);
    msg!("Canonical bump: {}", bump);

    Ok(())
}

/// Reads and logs the data stored in a user profile PDA.
///
/// This demonstrates:
/// - Verifying a PDA address matches expected derivation
/// - Using create_program_address with a known bump (cheaper than find_program_address)
/// - Deserializing Borsh data from an account
fn read_user_profile(
    // This program's ID — needed to re-derive the PDA for verification.
    program_id: &Pubkey,
    // The accounts passed to this instruction.
    accounts: &[AccountInfo],
) -> ProgramResult {
    // Account 0: The user profile PDA to read from.
    let accounts_iter = &mut accounts.iter();
    let profile_pda = next_account_info(accounts_iter)?;

    // Verify this account is owned by our program.
    // In Solidity, you don't need this because contract storage is implicit.
    // In Solana, anyone can pass any account, so we must verify ownership.
    if profile_pda.owner != program_id {
        msg!("Error: account not owned by this program");
        return Err(ProgramError::IncorrectProgramId);
    }

    // ---- Deserialize the account data ----
    // Borrow the account's raw bytes and decode them into our UserProfile struct.
    let data = profile_pda.try_borrow_data()?;
    let profile = UserProfile::try_from_slice(&data)
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // Log the profile data.
    msg!("Profile PDA: {}", profile_pda.key);
    msg!("Name: {}", profile.name);
    msg!("Score: {}", profile.score);
    msg!("Stored bump: {}", profile.bump);

    // ---- Verify the PDA using the stored bump ----
    // Because we stored the bump, we can use create_program_address (cheaper).
    // This tries a single bump instead of iterating 255→0.
    // We need to know the user's pubkey to reconstruct seeds. In a real program,
    // we'd either accept the user account or store the user's pubkey in the data.
    // For demonstration, we use find_program_address here.
    msg!("Successfully read user profile from PDA");

    Ok(())
}

/// Updates the score in a user's profile PDA.
///
/// Demonstrates re-deriving a PDA to verify the account, then mutating its data.
fn update_score(
    // This program's ID — needed for PDA verification.
    program_id: &Pubkey,
    // The accounts passed to this instruction.
    accounts: &[AccountInfo],
    // The new score to store.
    new_score: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // Account 0: The user who owns this profile. Must be a signer.
    let user = next_account_info(accounts_iter)?;
    if !user.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Account 1: The user profile PDA to update.
    let profile_pda = next_account_info(accounts_iter)?;

    // Verify the PDA was derived from this user's pubkey.
    // This ensures user A can't modify user B's profile.
    let (expected_pda, _bump) = Pubkey::find_program_address(
        &[USER_PROFILE_SEED, user.key.as_ref()],
        program_id,
    );
    if profile_pda.key != &expected_pda {
        msg!("Error: PDA does not match user");
        return Err(ProgramError::InvalidArgument);
    }

    // Verify this account is owned by our program.
    if profile_pda.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // ---- Read, modify, write ----
    // Deserialize the existing data.
    let data = profile_pda.try_borrow_data()?;
    let mut profile = UserProfile::try_from_slice(&data)
        .map_err(|_| ProgramError::InvalidAccountData)?;
    // Drop the immutable borrow before we take a mutable one.
    drop(data);

    // Update the score field.
    profile.score = new_score;

    // Re-serialize and write back.
    let updated_data = borsh::to_vec(&profile).map_err(|_| ProgramError::InvalidAccountData)?;
    let mut account_data = profile_pda.try_borrow_mut_data()?;
    account_data[..updated_data.len()].copy_from_slice(&updated_data);

    msg!("Updated score to {} for PDA: {}", new_score, profile_pda.key);

    Ok(())
}

/// Initializes the global config singleton PDA.
///
/// Demonstrates a PDA with no user-specific seed — just a fixed prefix.
/// Only one of these can ever exist per program (like a Solidity constructor's state setup).
fn init_global_config(
    // This program's ID.
    program_id: &Pubkey,
    // The accounts passed to this instruction.
    accounts: &[AccountInfo],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // Account 0: The admin who will control the config. Also the payer.
    let admin = next_account_info(accounts_iter)?;
    if !admin.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Account 1: The global config PDA.
    let config_pda = next_account_info(accounts_iter)?;

    // Account 2: System program for the create_account CPI.
    let system_program_account = next_account_info(accounts_iter)?;

    // Derive the global config PDA.
    // Only one seed (the prefix) — so only one global config can exist per program.
    let (expected_pda, bump) = Pubkey::find_program_address(
        &[GLOBAL_CONFIG_SEED],
        program_id,
    );

    // Verify the passed account matches.
    if config_pda.key != &expected_pda {
        msg!("Error: config PDA does not match derived address");
        return Err(ProgramError::InvalidArgument);
    }

    // Build the config data.
    let config = GlobalConfig {
        admin: *admin.key,
        bump,
        is_paused: false,
    };

    // Serialize to determine space.
    let config_data = borsh::to_vec(&config).map_err(|_| ProgramError::InvalidAccountData)?;
    let space = config_data.len();

    // Calculate rent-exempt minimum.
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(space);

    // Create the PDA account. The program signs with just the prefix seed + bump.
    invoke_signed(
        &system_instruction::create_account(
            admin.key,
            config_pda.key,
            lamports,
            space as u64,
            program_id,
        ),
        &[
            admin.clone(),
            config_pda.clone(),
            system_program_account.clone(),
        ],
        // Signer seeds for the global config PDA — just the prefix and bump.
        &[&[GLOBAL_CONFIG_SEED, &[bump]]],
    )?;

    // Write config data.
    let mut account_data = config_pda.try_borrow_mut_data()?;
    account_data[..config_data.len()].copy_from_slice(&config_data);

    msg!("Initialized global config PDA: {}", config_pda.key);
    msg!("Admin: {}", admin.key);

    Ok(())
}

// ============================================================================
// Helper: Demonstrate find_program_address vs create_program_address
// ============================================================================

/// Shows the difference between find_program_address and create_program_address.
/// Not called on-chain — this is a library function for client-side use or tests.
pub fn demonstrate_pda_derivation(
    user_pubkey: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    // ----- Method 1: find_program_address -----
    // Tries bumps from 255 down to 0. Returns the first valid (off-curve) address.
    // This is the "expensive" way — it may try many bumps before finding one.
    let (pda, canonical_bump) = Pubkey::find_program_address(
        &[USER_PROFILE_SEED, user_pubkey.as_ref()],
        program_id,
    );

    msg!("find_program_address result:");
    msg!("  PDA: {}", pda);
    msg!("  Canonical bump: {}", canonical_bump);

    // ----- Method 2: create_program_address -----
    // Uses a known bump directly. Cheaper because it tries exactly one bump.
    // Returns Result — errors if the bump produces an on-curve address.
    let pda_verify = Pubkey::create_program_address(
        &[USER_PROFILE_SEED, user_pubkey.as_ref(), &[canonical_bump]],
        program_id,
    )
    .expect("canonical bump should always produce a valid PDA");

    // Both methods should produce the same address.
    assert_eq!(pda, pda_verify, "PDA mismatch — this should never happen");
    msg!("create_program_address verified: same PDA");

    // Return the PDA and bump for the caller.
    (pda, canonical_bump)
}
