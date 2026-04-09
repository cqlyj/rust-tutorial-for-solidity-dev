// =============================================================================
// Module 12: Cross-Program Invocation (CPI) — Demonstration Program
// =============================================================================
// This Solana program shows the core CPI patterns:
//   Instruction 0 — Create a new account via System Program (invoke)
//   Instruction 1 — Transfer SOL via System Program (invoke)
//   Instruction 2 — Transfer SOL from a PDA via System Program (invoke_signed)
//   Instruction 3 — Build a manual Instruction struct and invoke it
// =============================================================================

// Pull in everything we need from the Solana program SDK.
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    // Main account type passed into every instruction.
    account_info::{next_account_info, AccountInfo},
    // The standard entrypoint macro that wires our function to the runtime.
    entrypoint,
    // Convenience macro for logging on-chain (visible in transaction logs).
    msg,
    // Error types every Solana program uses.
    entrypoint::ProgramResult,
    program_error::ProgramError,
    // Core CPI functions.
    program::invoke,
    program::invoke_signed,
    // Types for building instructions manually.
    instruction::{AccountMeta, Instruction},
    // Helpers for common System Program instructions.
    system_instruction,
    // The System Program's well-known address.
    system_program,
    // Pubkey type — 32-byte public key.
    pubkey::Pubkey,
    // Rent sysvar for calculating rent-exempt minimum.
    rent::Rent,
    // Lets us read sysvar data from an AccountInfo.
    sysvar::Sysvar,
};

// ---------------------------------------------------------------------------
// Instruction enum — what the client can ask this program to do.
// ---------------------------------------------------------------------------
// We use Borsh serialization so the client and program agree on the wire
// format. The first byte selects the variant; subsequent bytes are the fields.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum CpiInstruction {
    /// Variant 0: Create a new account owned by a specified program.
    /// `space` — how many bytes the new account's data field should hold.
    CreateAccount { space: u64 },

    /// Variant 1: Transfer SOL from one account to another.
    /// `lamports` — amount of SOL (in lamports) to send.
    TransferSol { lamports: u64 },

    /// Variant 2: Transfer SOL **from a PDA** to a recipient.
    /// The PDA is derived from seeds; our program signs with invoke_signed.
    /// `lamports` — amount to transfer.
    /// `bump` — the PDA bump seed (client must supply it).
    PdaTransfer { lamports: u64, bump: u8 },

    /// Variant 3: Demonstrate building a raw Instruction struct by hand
    /// and invoking it, rather than using system_instruction helpers.
    /// `lamports` — amount for a manual System Program transfer.
    ManualInstruction { lamports: u64 },
}

// ---------------------------------------------------------------------------
// Entrypoint — the runtime calls this for every transaction instruction
// directed at our program_id.
// ---------------------------------------------------------------------------
entrypoint!(process_instruction);

// ---------------------------------------------------------------------------
// Top-level dispatch: deserialize the instruction and route to the handler.
// ---------------------------------------------------------------------------
pub fn process_instruction(
    // Our own program's public key — useful for PDA derivation.
    program_id: &Pubkey,
    // All accounts the client attached to this instruction.
    accounts: &[AccountInfo],
    // Raw instruction data (Borsh-encoded CpiInstruction).
    instruction_data: &[u8],
) -> ProgramResult {
    // Deserialize the instruction data into our enum.
    // `try_from_slice` reads the Borsh bytes and returns the variant.
    let instruction = CpiInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Route to the correct handler based on the variant.
    match instruction {
        CpiInstruction::CreateAccount { space } => {
            create_account(program_id, accounts, space)
        }
        CpiInstruction::TransferSol { lamports } => {
            transfer_sol(accounts, lamports)
        }
        CpiInstruction::PdaTransfer { lamports, bump } => {
            pda_transfer(program_id, accounts, lamports, bump)
        }
        CpiInstruction::ManualInstruction { lamports } => {
            manual_instruction(accounts, lamports)
        }
    }
}

// ===========================================================================
// Handler 0: Create a new account via CPI to the System Program.
// ===========================================================================
// Expected accounts:
//   [0] payer       — signer, writable — pays for the new account
//   [1] new_account — signer, writable — the account to create
//   [2] system_program — executable, read-only
//   [3] rent_sysvar — read-only (we read Rent from it)
fn create_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    space: u64,
) -> ProgramResult {
    // Create an iterator to safely pull accounts in order.
    let account_iter = &mut accounts.iter();

    // Pull each account by position. `next_account_info` advances the iterator
    // and returns a reference, or errors if the client didn't supply enough.
    let payer = next_account_info(account_iter)?;
    let new_account = next_account_info(account_iter)?;
    let system_program_account = next_account_info(account_iter)?;

    // --- Security: verify the System Program is actually the System Program ---
    // A malicious client could pass a fake program. Always check.
    if system_program_account.key != &system_program::id() {
        msg!("Error: invalid System Program account");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Calculate the lamports needed for rent exemption at this data size.
    // We read the Rent sysvar directly (no sysvar account needed with Sysvar::get).
    let rent = Rent::get()?;
    let required_lamports = rent.minimum_balance(space as usize);

    msg!(
        "Creating account with {} bytes, {} lamports for rent",
        space,
        required_lamports
    );

    // Build the System Program create_account instruction.
    // This helper returns a fully formed Instruction struct.
    let create_ix = system_instruction::create_account(
        payer.key,        // who pays
        new_account.key,  // the new account's pubkey
        required_lamports, // lamports to fund
        space,            // data size in bytes
        program_id,       // the program that will own this account
    );

    // Execute the CPI. Both payer and new_account must be signers
    // (the client already signed the transaction with both keypairs).
    invoke(
        &create_ix,
        // Pass all AccountInfos the callee needs. Order does not matter here;
        // the runtime matches them by pubkey against the Instruction's AccountMetas.
        &[
            payer.clone(),
            new_account.clone(),
            system_program_account.clone(),
        ],
    )?;

    msg!("Account created successfully");
    Ok(())
}

// ===========================================================================
// Handler 1: Transfer SOL via CPI to the System Program.
// ===========================================================================
// Expected accounts:
//   [0] from           — signer, writable
//   [1] to             — writable
//   [2] system_program — executable, read-only
fn transfer_sol(accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    // Pull accounts in the expected order.
    let account_iter = &mut accounts.iter();
    let from = next_account_info(account_iter)?;
    let to = next_account_info(account_iter)?;
    let system_program_account = next_account_info(account_iter)?;

    // --- Security: verify the system program ---
    if system_program_account.key != &system_program::id() {
        msg!("Error: invalid System Program account");
        return Err(ProgramError::IncorrectProgramId);
    }

    // --- Security: verify the sender actually signed ---
    // Without this check, someone could pass an unsigned account and
    // the CPI would fail at the System Program level, but catching
    // it here gives a clearer error message.
    if !from.is_signer {
        msg!("Error: sender must be a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    msg!("Transferring {} lamports from {} to {}", lamports, from.key, to.key);

    // Build the transfer instruction using the System Program helper.
    let transfer_ix = system_instruction::transfer(from.key, to.key, lamports);

    // Invoke the System Program. `from` has signer privilege because the
    // original transaction was signed by that keypair — privilege escalation
    // propagates it through the CPI.
    invoke(
        &transfer_ix,
        &[
            from.clone(),
            to.clone(),
            system_program_account.clone(),
        ],
    )?;

    msg!("Transfer complete");
    Ok(())
}

// ===========================================================================
// Handler 2: Transfer SOL FROM a PDA using invoke_signed.
// ===========================================================================
// The PDA is derived as: PDA = find_program_address([b"vault", authority], program_id)
// Only our program can sign for this PDA because only we know the derivation.
//
// Expected accounts:
//   [0] pda_account    — writable (the vault PDA that holds SOL)
//   [1] recipient      — writable
//   [2] authority      — signer, read-only (the user who controls this vault)
//   [3] system_program — executable, read-only
fn pda_transfer(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    lamports: u64,
    bump: u8,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let pda_account = next_account_info(account_iter)?;
    let recipient = next_account_info(account_iter)?;
    let authority = next_account_info(account_iter)?;
    let system_program_account = next_account_info(account_iter)?;

    // --- Security: verify system program ---
    if system_program_account.key != &system_program::id() {
        msg!("Error: invalid System Program account");
        return Err(ProgramError::IncorrectProgramId);
    }

    // --- Security: verify the authority signed the transaction ---
    if !authority.is_signer {
        msg!("Error: authority must sign");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // --- Security: re-derive the PDA and verify it matches ---
    // This prevents a client from passing an arbitrary account as the PDA.
    let expected_pda = Pubkey::create_program_address(
        &[b"vault", authority.key.as_ref(), &[bump]],
        program_id,
    )
    .map_err(|_| ProgramError::InvalidSeeds)?;

    if pda_account.key != &expected_pda {
        msg!("Error: PDA account does not match derived address");
        return Err(ProgramError::InvalidSeeds);
    }

    msg!(
        "PDA transfer: {} lamports from {} to {}",
        lamports,
        pda_account.key,
        recipient.key
    );

    // Build the transfer instruction. The "from" is the PDA — which has no
    // private key. We will use invoke_signed to let the runtime verify our
    // seeds and grant signer privilege to the PDA.
    let transfer_ix = system_instruction::transfer(pda_account.key, recipient.key, lamports);

    // The signer_seeds slice tells the runtime: "derive a PDA from these seeds
    // and this program_id. If it matches an account marked is_signer in the
    // instruction, grant that account signer privilege."
    //
    // The outer slice can contain multiple PDA seed sets if you need to sign
    // for more than one PDA in a single CPI. Here we only need one.
    let signer_seeds: &[&[u8]] = &[b"vault", authority.key.as_ref(), &[bump]];

    invoke_signed(
        &transfer_ix,
        &[
            pda_account.clone(),
            recipient.clone(),
            system_program_account.clone(),
        ],
        // &[signer_seeds] — an array of seed-arrays, one per PDA signer.
        &[signer_seeds],
    )?;

    msg!("PDA transfer complete");
    Ok(())
}

// ===========================================================================
// Handler 3: Build an Instruction struct manually and invoke it.
// ===========================================================================
// This demonstrates the low-level approach — constructing the Instruction
// yourself instead of using a helper like system_instruction::transfer.
// Useful when the target program has no published Rust crate.
//
// Expected accounts:
//   [0] from           — signer, writable
//   [1] to             — writable
//   [2] system_program — executable, read-only
fn manual_instruction(accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let from = next_account_info(account_iter)?;
    let to = next_account_info(account_iter)?;
    let system_program_account = next_account_info(account_iter)?;

    // --- Security: verify system program ---
    if system_program_account.key != &system_program::id() {
        msg!("Error: invalid System Program account");
        return Err(ProgramError::IncorrectProgramId);
    }

    // --- Security: verify signer ---
    if !from.is_signer {
        msg!("Error: sender must be a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    msg!("Building manual instruction for {} lamports", lamports);

    // The System Program's transfer instruction is index 2.
    // Its data layout (little-endian):
    //   bytes 0..4  — instruction index (u32) = 2
    //   bytes 4..12 — lamports (u64)
    let mut data = Vec::with_capacity(12);
    // Instruction index 2 = Transfer, encoded as a 4-byte little-endian u32.
    data.extend_from_slice(&2u32.to_le_bytes());
    // The transfer amount as an 8-byte little-endian u64.
    data.extend_from_slice(&lamports.to_le_bytes());

    // Construct the Instruction by hand.
    let instruction = Instruction {
        // Target program: the System Program.
        program_id: system_program::id(),
        // Account list: from (writable + signer), to (writable, not signer).
        accounts: vec![
            AccountMeta::new(*from.key, true),  // writable, signer
            AccountMeta::new(*to.key, false),    // writable, not signer
        ],
        // Our hand-built serialized data.
        data,
    };

    // Invoke exactly like before — the runtime does not care whether we used
    // a helper or built the instruction manually.
    invoke(
        &instruction,
        &[
            from.clone(),
            to.clone(),
            system_program_account.clone(),
        ],
    )?;

    msg!("Manual instruction transfer complete");
    Ok(())
}

// ===========================================================================
// Unit tests — run with `cargo test`
// ===========================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // Verify that our instruction enum serializes and deserializes correctly.
    #[test]
    fn test_instruction_roundtrip() {
        // Create a TransferSol variant with a known value.
        let original = CpiInstruction::TransferSol { lamports: 1_000_000 };
        // Serialize to bytes.
        let encoded = borsh::to_vec(&original).unwrap();
        // Deserialize back.
        let decoded = CpiInstruction::try_from_slice(&encoded).unwrap();
        // Verify the roundtrip preserved the data.
        match decoded {
            CpiInstruction::TransferSol { lamports } => {
                assert_eq!(lamports, 1_000_000);
            }
            _ => panic!("Wrong variant after deserialization"),
        }
    }

    // Verify PdaTransfer serialization includes both fields.
    #[test]
    fn test_pda_transfer_serialization() {
        let original = CpiInstruction::PdaTransfer {
            lamports: 500_000,
            bump: 254,
        };
        let encoded = borsh::to_vec(&original).unwrap();
        let decoded = CpiInstruction::try_from_slice(&encoded).unwrap();
        match decoded {
            CpiInstruction::PdaTransfer { lamports, bump } => {
                assert_eq!(lamports, 500_000);
                assert_eq!(bump, 254);
            }
            _ => panic!("Wrong variant"),
        }
    }

    // Verify CreateAccount serialization.
    #[test]
    fn test_create_account_serialization() {
        let original = CpiInstruction::CreateAccount { space: 128 };
        let encoded = borsh::to_vec(&original).unwrap();
        let decoded = CpiInstruction::try_from_slice(&encoded).unwrap();
        match decoded {
            CpiInstruction::CreateAccount { space } => {
                assert_eq!(space, 128);
            }
            _ => panic!("Wrong variant"),
        }
    }

    // Verify that invalid instruction data produces an error.
    #[test]
    fn test_invalid_instruction_data() {
        // Empty data should fail deserialization.
        let result = CpiInstruction::try_from_slice(&[]);
        assert!(result.is_err());
    }

    // Verify the manual instruction data layout matches what System Program expects.
    #[test]
    fn test_manual_transfer_data_layout() {
        let lamports: u64 = 42_000;
        // Build the data buffer the same way our handler does.
        let mut data = Vec::with_capacity(12);
        data.extend_from_slice(&2u32.to_le_bytes()); // instruction index
        data.extend_from_slice(&lamports.to_le_bytes()); // amount
        // Total should be 12 bytes: 4 (index) + 8 (lamports).
        assert_eq!(data.len(), 12);
        // First 4 bytes should be [2, 0, 0, 0] (little-endian u32 = 2).
        assert_eq!(&data[0..4], &[2, 0, 0, 0]);
        // Next 8 bytes should be 42_000 in little-endian.
        assert_eq!(&data[4..12], &42_000u64.to_le_bytes());
    }
}
