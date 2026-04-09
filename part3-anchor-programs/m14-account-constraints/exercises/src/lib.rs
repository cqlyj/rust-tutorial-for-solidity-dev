// Module 14 Exercises: Anchor Account Constraints
//
// Complete each exercise by filling in the missing constraints.
// Look for "TODO" comments — each marks a place you need to modify.
// The program should compile with `cargo check` when all TODOs are resolved.

use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod exercises {
    use super::*;

    // -----------------------------------------------------------------------
    // Exercise 1: Add correct constraints to create a Ledger PDA account.
    //
    // Requirements:
    //   - Create a new account (init)
    //   - The `owner` signer pays rent
    //   - Allocate space for the Ledger struct (use InitSpace)
    //   - Derive PDA from seeds: [b"ledger", owner pubkey]
    //   - Store the bump
    // -----------------------------------------------------------------------
    pub fn create_ledger(ctx: Context<CreateLedger>) -> Result<()> {
        let ledger = &mut ctx.accounts.ledger;
        ledger.owner = ctx.accounts.owner.key();
        ledger.entries = 0;
        ledger.total_amount = 0;
        // TODO (Exercise 1): Once you add seeds + bump to CreateLedger,
        // change this to: ledger.bump = ctx.bumps.ledger;
        ledger.bump = 0;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Exercise 2: Calculate the correct space for the Profile account.
    //
    // Profile fields:
    //   authority: Pubkey   → 32 bytes
    //   username: String    → max 20 chars → 4 + 20 = 24 bytes
    //   level: u16          → 2 bytes
    //   is_active: bool     → 1 byte
    //   scores: Vec<u64>    → max 5 items → 4 + 5*8 = 44 bytes
    //   badge: Option<Pubkey> → 1 + 32 = 33 bytes
    //   bump: u8            → 1 byte
    //
    // Total with discriminator: 8 + 32 + 24 + 2 + 1 + 44 + 33 + 1 = ???
    //
    // Replace the 0 in `space = 0` with the correct value.
    // -----------------------------------------------------------------------
    pub fn create_profile(ctx: Context<CreateProfile>) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        profile.authority = ctx.accounts.user.key();
        profile.username = String::new();
        profile.level = 1;
        profile.is_active = true;
        profile.scores = Vec::new();
        profile.badge = None;
        profile.bump = ctx.bumps.profile;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Exercise 3: Add a has_one constraint for ownership validation.
    //
    // The `add_entry` instruction should only be callable by the ledger's
    // owner. Add the appropriate has_one constraint with a custom error.
    // -----------------------------------------------------------------------
    pub fn add_entry(ctx: Context<AddEntry>, amount: u64) -> Result<()> {
        require!(amount > 0, LedgerError::ZeroAmount);
        let ledger = &mut ctx.accounts.ledger;
        ledger.entries += 1;
        ledger.total_amount = ledger
            .total_amount
            .checked_add(amount)
            .ok_or(LedgerError::Overflow)?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Exercise 4: Add seeds and bump constraints for PDA verification.
    //
    // The `view_ledger` instruction reads the ledger. Add seeds and bump
    // constraints to verify the PDA address is correct.
    // Hint: seeds should be [b"ledger", owner pubkey] and bump should
    // reference the stored bump.
    // -----------------------------------------------------------------------
    pub fn view_ledger(ctx: Context<ViewLedger>) -> Result<()> {
        let ledger = &ctx.accounts.ledger;
        msg!("Ledger owner: {}", ledger.owner);
        msg!("Entries: {}", ledger.entries);
        msg!("Total: {}", ledger.total_amount);
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Exercise 5: Add custom error messages to constraints.
    //
    // The `withdraw_entry` instruction has constraints but they use
    // generic error messages. Add custom error variants using `@`.
    // -----------------------------------------------------------------------
    pub fn withdraw_entry(ctx: Context<WithdrawEntry>, amount: u64) -> Result<()> {
        let ledger = &mut ctx.accounts.ledger;
        ledger.total_amount = ledger
            .total_amount
            .checked_sub(amount)
            .ok_or(LedgerError::InsufficientFunds)?;
        ledger.entries = ledger.entries.saturating_sub(1);
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Exercise 6: Fix the security vulnerabilities.
    //
    // The `close_ledger` instruction has several missing constraints that
    // make it insecure. Anyone could close anyone's ledger!
    //
    // Fix it by adding:
    //   - has_one constraint for ownership
    //   - close constraint to send lamports to owner
    //   - PDA verification with seeds/bump
    // -----------------------------------------------------------------------
    pub fn close_ledger(_ctx: Context<CloseLedger>) -> Result<()> {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Account data structures
// ---------------------------------------------------------------------------

#[account]
#[derive(InitSpace)]
pub struct Ledger {
    pub owner: Pubkey,      // 32
    pub entries: u64,       // 8
    pub total_amount: u64,  // 8
    pub bump: u8,           // 1
}

#[account]
pub struct Profile {
    pub authority: Pubkey,
    #[allow(dead_code)]
    pub username: String,
    pub level: u16,
    pub is_active: bool,
    pub scores: Vec<u64>,
    pub badge: Option<Pubkey>,
    pub bump: u8,
}

// ---------------------------------------------------------------------------
// Accounts structs — fill in the missing constraints!
// ---------------------------------------------------------------------------

// Exercise 1: Add the correct constraints inside #[account(...)].
#[derive(Accounts)]
pub struct CreateLedger<'info> {
    // TODO: Add init, payer, space, seeds, and bump constraints.
    // Hint: #[account(init, payer = ..., space = ..., seeds = [...], bump)]
    #[account(
        init,
        payer = owner,
        space = 8 + Ledger::INIT_SPACE,
        // TODO: Add seeds and bump constraints here.
        // seeds = [???],
        // bump,
    )]
    pub ledger: Account<'info, Ledger>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// Exercise 2: Replace `space = 0` with the correct calculated value.
#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(
        init,
        payer = user,
        // TODO: Calculate the correct space.
        // Fields: Pubkey(32) + String(4+20) + u16(2) + bool(1)
        //       + Vec<u64>(4+5*8) + Option<Pubkey>(1+32) + u8(1)
        // Don't forget the 8-byte discriminator!
        space = 0,
        seeds = [b"profile", user.key().as_ref()],
        bump,
    )]
    pub profile: Account<'info, Profile>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// Exercise 3: Add has_one constraint with custom error.
#[derive(Accounts)]
pub struct AddEntry<'info> {
    #[account(
        mut,
        // TODO: Add has_one = owner with a custom error message.
        // Hint: has_one = owner @ LedgerError::???
        seeds = [b"ledger", owner.key().as_ref()],
        bump = ledger.bump,
    )]
    pub ledger: Account<'info, Ledger>,

    pub owner: Signer<'info>,
}

// Exercise 4: Add seeds and bump constraints.
#[derive(Accounts)]
pub struct ViewLedger<'info> {
    #[account(
        // TODO: Add seeds and bump to verify PDA.
        // Hint: seeds = [b"ledger", owner.key().as_ref()],
        //       bump = ledger.bump,
    )]
    pub ledger: Account<'info, Ledger>,

    /// CHECK: Used only for PDA derivation.
    pub owner: UncheckedAccount<'info>,
}

// Exercise 5: Add custom errors to existing constraints using `@`.
#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct WithdrawEntry<'info> {
    #[account(
        mut,
        has_one = owner,
        // TODO: Add @ LedgerError::Unauthorized to the has_one above.
        constraint = ledger.total_amount >= amount,
        // TODO: Add @ LedgerError::InsufficientFunds to the constraint above.
        seeds = [b"ledger", owner.key().as_ref()],
        bump = ledger.bump,
    )]
    pub ledger: Account<'info, Ledger>,

    pub owner: Signer<'info>,
}

// Exercise 6: Fix security vulnerabilities — add missing constraints.
#[derive(Accounts)]
pub struct CloseLedger<'info> {
    // BUG: No ownership check! Anyone can close any ledger.
    // BUG: No close constraint! Lamports are not reclaimed.
    // BUG: No PDA verification! Wrong account could be passed.
    //
    // TODO: Add has_one, close, seeds, and bump constraints.
    #[account(mut)]
    pub ledger: Account<'info, Ledger>,

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
