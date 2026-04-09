// Module 14 Solutions: Anchor Account Constraints
//
// Every exercise solved with detailed comments explaining each constraint.

use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod solutions {
    use super::*;

    // -----------------------------------------------------------------------
    // Exercise 1 Solution: All init constraints for the Ledger PDA.
    // -----------------------------------------------------------------------
    pub fn create_ledger(ctx: Context<CreateLedger>) -> Result<()> {
        // Access the freshly-created ledger account.
        let ledger = &mut ctx.accounts.ledger;

        // Store the owner's pubkey — this is what has_one checks against later.
        ledger.owner = ctx.accounts.owner.key();

        // Initialize counters to zero.
        ledger.entries = 0;
        ledger.total_amount = 0;

        // Store the PDA bump for cheaper verification on future instructions.
        // ctx.bumps.ledger is populated by Anchor when `bump` is in the constraint.
        ledger.bump = ctx.bumps.ledger;

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Exercise 2 Solution: Correct space = 145.
    //
    // Breakdown:
    //   8   — Anchor discriminator (always first 8 bytes)
    //   32  — authority: Pubkey
    //   24  — username: String (4 byte length prefix + 20 max chars)
    //   2   — level: u16
    //   1   — is_active: bool
    //   44  — scores: Vec<u64> (4 byte length prefix + 5 items × 8 bytes)
    //   33  — badge: Option<Pubkey> (1 byte tag + 32 byte Pubkey)
    //   1   — bump: u8
    //   ----
    //   145 total bytes
    // -----------------------------------------------------------------------
    pub fn create_profile(ctx: Context<CreateProfile>) -> Result<()> {
        let profile = &mut ctx.accounts.profile;

        // Store the creating user as the authority.
        profile.authority = ctx.accounts.user.key();

        // Initialize with empty/default values.
        profile.username = String::new();
        profile.level = 1;
        profile.is_active = true;
        profile.scores = Vec::new();
        profile.badge = None;

        // Store bump for future PDA verification.
        profile.bump = ctx.bumps.profile;

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Exercise 3 Solution: has_one = owner with custom error.
    // -----------------------------------------------------------------------
    pub fn add_entry(ctx: Context<AddEntry>, amount: u64) -> Result<()> {
        // Imperative check: amount must be positive.
        require!(amount > 0, LedgerError::ZeroAmount);

        let ledger = &mut ctx.accounts.ledger;

        // Increment the entry count.
        ledger.entries += 1;

        // Add to total with overflow protection.
        ledger.total_amount = ledger
            .total_amount
            .checked_add(amount)
            .ok_or(LedgerError::Overflow)?;

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Exercise 4 Solution: seeds and bump for PDA verification.
    // -----------------------------------------------------------------------
    pub fn view_ledger(ctx: Context<ViewLedger>) -> Result<()> {
        // Read-only access — log the ledger's state.
        let ledger = &ctx.accounts.ledger;
        msg!("Ledger owner: {}", ledger.owner);
        msg!("Entries: {}", ledger.entries);
        msg!("Total: {}", ledger.total_amount);
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Exercise 5 Solution: Custom error messages on constraints.
    // -----------------------------------------------------------------------
    pub fn withdraw_entry(ctx: Context<WithdrawEntry>, amount: u64) -> Result<()> {
        let ledger = &mut ctx.accounts.ledger;

        // Subtract from total with underflow protection.
        ledger.total_amount = ledger
            .total_amount
            .checked_sub(amount)
            .ok_or(LedgerError::InsufficientFunds)?;

        // Decrement entry count (saturating to avoid underflow).
        ledger.entries = ledger.entries.saturating_sub(1);

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Exercise 6 Solution: All security constraints added.
    // -----------------------------------------------------------------------
    pub fn close_ledger(_ctx: Context<CloseLedger>) -> Result<()> {
        // The `close` constraint on the accounts struct handles everything:
        //   1. Transfers all lamports from ledger to owner
        //   2. Zeros the account data (prevents reuse attacks)
        //   3. Sets the account owner to the System Program
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Account data structures
// ---------------------------------------------------------------------------

/// The Ledger account — tracks entries and a running total.
/// InitSpace derives INIT_SPACE constant automatically from field types.
#[account]
#[derive(InitSpace)]
pub struct Ledger {
    pub owner: Pubkey,      // 32 bytes — who controls this ledger
    pub entries: u64,       // 8 bytes  — number of entries added
    pub total_amount: u64,  // 8 bytes  — running sum of all entries
    pub bump: u8,           // 1 byte   — PDA bump for re-derivation
}
// INIT_SPACE = 32 + 8 + 8 + 1 = 49
// Total space with discriminator = 8 + 49 = 57

/// The Profile account — a more complex struct for space calculation practice.
#[account]
pub struct Profile {
    pub authority: Pubkey,      // 32 bytes
    #[allow(dead_code)]
    pub username: String,       // 4 + 20 = 24 bytes (max 20 chars)
    pub level: u16,             // 2 bytes
    pub is_active: bool,        // 1 byte
    pub scores: Vec<u64>,       // 4 + 5*8 = 44 bytes (max 5 items)
    pub badge: Option<Pubkey>,  // 1 + 32 = 33 bytes
    pub bump: u8,               // 1 byte
}
// Total = 32 + 24 + 2 + 1 + 44 + 33 + 1 = 137
// With discriminator = 8 + 137 = 145

// ---------------------------------------------------------------------------
// Accounts structs — all constraints filled in correctly.
// ---------------------------------------------------------------------------

/// Exercise 1 Solution: Complete init constraints.
///
/// Key constraints:
/// - `init`  → tells Anchor to create a brand-new account
/// - `payer` → the owner signer pays rent (must be mut + Signer)
/// - `space` → 8 (discriminator) + Ledger::INIT_SPACE (49) = 57 bytes
/// - `seeds` → [b"ledger", owner_pubkey] makes this a unique PDA per owner
/// - `bump`  → Anchor finds the canonical bump during creation
#[derive(Accounts)]
pub struct CreateLedger<'info> {
    #[account(
        // SOLUTION: Create a new account owned by this program.
        init,
        // SOLUTION: The owner signer pays the rent deposit.
        payer = owner,
        // SOLUTION: 8-byte discriminator + auto-calculated struct size.
        space = 8 + Ledger::INIT_SPACE,
        // SOLUTION: Derive PDA from tag + owner pubkey → one ledger per user.
        seeds = [b"ledger", owner.key().as_ref()],
        // SOLUTION: Anchor finds the canonical bump seed.
        bump,
    )]
    pub ledger: Account<'info, Ledger>,

    // Payer must be mut (lamports deducted) and a Signer (authorization).
    #[account(mut)]
    pub owner: Signer<'info>,

    // System Program is required for `init` — it creates the account.
    pub system_program: Program<'info, System>,
}

/// Exercise 2 Solution: Correct space calculation = 145.
///
/// 8 (discriminator)
/// + 32 (Pubkey)
/// + 24 (String: 4 + 20)
/// + 2  (u16)
/// + 1  (bool)
/// + 44 (Vec<u64>: 4 + 5*8)
/// + 33 (Option<Pubkey>: 1 + 32)
/// + 1  (u8)
/// = 145
#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(
        init,
        payer = user,
        // SOLUTION: 8 + 32 + 24 + 2 + 1 + 44 + 33 + 1 = 145
        space = 145,
        seeds = [b"profile", user.key().as_ref()],
        bump,
    )]
    pub profile: Account<'info, Profile>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Exercise 3 Solution: has_one with custom error.
///
/// `has_one = owner` checks that `ledger.owner == owner.key()`.
/// The `@ LedgerError::Unauthorized` provides a clear error if it fails.
#[derive(Accounts)]
pub struct AddEntry<'info> {
    #[account(
        mut,
        // SOLUTION: Verify the signer matches the stored owner field.
        // Without this, anyone could add entries to any ledger!
        has_one = owner @ LedgerError::Unauthorized,
        // PDA verification ensures the correct ledger account is used.
        seeds = [b"ledger", owner.key().as_ref()],
        bump = ledger.bump,
    )]
    pub ledger: Account<'info, Ledger>,

    // The owner must sign — Signer type enforces this automatically.
    pub owner: Signer<'info>,
}

/// Exercise 4 Solution: Seeds and bump for PDA verification.
///
/// Even though this is a read-only instruction, we verify the PDA to
/// ensure the caller passed the correct ledger for the given owner.
#[derive(Accounts)]
pub struct ViewLedger<'info> {
    #[account(
        // SOLUTION: Verify PDA address matches [b"ledger", owner_pubkey].
        seeds = [b"ledger", owner.key().as_ref()],
        // SOLUTION: Use stored bump — cheaper than re-deriving.
        bump = ledger.bump,
    )]
    pub ledger: Account<'info, Ledger>,

    /// CHECK: Used only for PDA seed derivation, not written to.
    pub owner: UncheckedAccount<'info>,
}

/// Exercise 5 Solution: Custom error messages with `@`.
///
/// The `@` operator attaches a specific error variant to a constraint.
/// Without it, failures return generic ConstraintViolated / ConstraintHasOne.
#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct WithdrawEntry<'info> {
    #[account(
        mut,
        // SOLUTION: has_one with clear error — tells the caller exactly what's wrong.
        has_one = owner @ LedgerError::Unauthorized,
        // SOLUTION: constraint with clear error — specifies which check failed.
        constraint = ledger.total_amount >= amount @ LedgerError::InsufficientFunds,
        seeds = [b"ledger", owner.key().as_ref()],
        bump = ledger.bump,
    )]
    pub ledger: Account<'info, Ledger>,

    pub owner: Signer<'info>,
}

/// Exercise 6 Solution: Fully secured close instruction.
///
/// Three critical constraints were missing:
/// 1. `has_one = owner` — prevents anyone from closing someone else's ledger
/// 2. `close = owner` — properly reclaims lamports and zeros account data
/// 3. `seeds/bump` — verifies the PDA so a spoofed account can't be passed
#[derive(Accounts)]
pub struct CloseLedger<'info> {
    #[account(
        // Account must be writable for close to zero data and move lamports.
        mut,
        // SOLUTION: Only the ledger's owner can close it.
        has_one = owner @ LedgerError::Unauthorized,
        // SOLUTION: Close the account — sends all lamports to owner,
        // zeros data, reassigns ownership to System Program.
        close = owner,
        // SOLUTION: Verify PDA to prevent passing a fake ledger account.
        seeds = [b"ledger", owner.key().as_ref()],
        bump = ledger.bump,
    )]
    pub ledger: Account<'info, Ledger>,

    // Receives reclaimed lamports. Must be mut and a Signer.
    #[account(mut)]
    pub owner: Signer<'info>,
}

// ---------------------------------------------------------------------------
// Custom errors
// ---------------------------------------------------------------------------
#[error_code]
pub enum LedgerError {
    #[msg("Only the ledger owner can perform this action")]
    Unauthorized,

    #[msg("Amount must be greater than zero")]
    ZeroAmount,

    #[msg("Insufficient funds in ledger")]
    InsufficientFunds,

    #[msg("Arithmetic overflow")]
    Overflow,
}
