// =============================================================================
// Module 17 Exercises: Escrow Program Challenges
// =============================================================================
//
// These exercises build on the escrow program from the tutorial.
// Each exercise adds a feature or asks you to implement a missing piece.
//
// Instructions:
//   1. Read each exercise's description
//   2. Replace the TODO!() markers with your implementation
//   3. Run `cargo check` (or `cargo build`) to verify
//   4. Check solutions/ if you get stuck
//
// The exercises are ordered by difficulty:
//   Exercise 1: Complete the `take` instruction (medium)
//   Exercise 2: Add a deadline feature (medium)
//   Exercise 3: Add partial fills (hard)
//   Exercise 4: Add an `update` instruction (medium)
// =============================================================================

use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
    TransferChecked,
};

declare_id!("Esc1111111111111111111111111111111111111111");

#[program]
pub mod escrow_exercises {
    use super::*;

    // =========================================================================
    // Exercise 1: Complete the `take` instruction
    // =========================================================================
    //
    // The `make` and `cancel` instructions are provided. Your job is to
    // implement `take` — the instruction where a taker completes the swap.
    //
    // Steps:
    //   1. Transfer `amount_wanted` of Token B from taker → maker
    //   2. Transfer `amount_offered` of Token A from vault → taker
    //      (Remember: the escrow PDA must sign this transfer!)
    //   3. Close the vault account (rent → maker)
    //
    // Hints:
    //   - Use `transfer_checked` for both transfers
    //   - For the vault transfer, use `CpiContext::new_with_signer` with PDA seeds
    //   - For closing the vault, use `close_account` with PDA seeds
    //   - The escrow PDA's signer seeds are:
    //     [b"escrow", maker_pubkey, seed_bytes, [bump]]
    //   - The `close = maker` on the escrow account handles closing the escrow PDA

    pub fn make(
        ctx: Context<Make>,
        seed: u64,
        amount_offered: u64,
        amount_wanted: u64,
    ) -> Result<()> {
        require!(amount_offered > 0, EscrowError::InvalidAmount);
        require!(amount_wanted > 0, EscrowError::InvalidAmount);

        ctx.accounts.escrow.set_inner(Escrow {
            maker: ctx.accounts.maker.key(),
            mint_a: ctx.accounts.mint_a.key(),
            mint_b: ctx.accounts.mint_b.key(),
            amount_offered,
            amount_wanted,
            seed,
            bump: ctx.bumps.escrow,
        });

        transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.maker_ata_a.to_account_info(),
                    mint: ctx.accounts.mint_a.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                    authority: ctx.accounts.maker.to_account_info(),
                },
            ),
            amount_offered,
            ctx.accounts.mint_a.decimals,
        )?;

        Ok(())
    }

    // YOUR TURN: Implement the `take` instruction.
    //
    // The Take account struct is already provided below.
    // You need to write the function body.
    pub fn take(ctx: Context<Take>) -> Result<()> {
        // TODO: Step 1 — Transfer Token B from taker to maker
        // Hint: transfer_checked with CpiContext::new (taker is the authority)

        // TODO: Step 2 — Transfer Token A from vault to taker
        // Hint: Build signer_seeds for the escrow PDA, use CpiContext::new_with_signer

        // TODO: Step 3 — Close the vault account
        // Hint: close_account with CpiContext::new_with_signer

        todo!("Implement the take instruction — see hints above")
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"escrow",
            ctx.accounts.escrow.maker.as_ref(),
            &ctx.accounts.escrow.seed.to_le_bytes(),
            &[ctx.accounts.escrow.bump],
        ]];

        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.vault.to_account_info(),
                    mint: ctx.accounts.mint_a.to_account_info(),
                    to: ctx.accounts.maker_ata_a.to_account_info(),
                    authority: ctx.accounts.escrow.to_account_info(),
                },
                signer_seeds,
            ),
            ctx.accounts.escrow.amount_offered,
            ctx.accounts.mint_a.decimals,
        )?;

        close_account(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.vault.to_account_info(),
                destination: ctx.accounts.maker.to_account_info(),
                authority: ctx.accounts.escrow.to_account_info(),
            },
            signer_seeds,
        ))?;

        Ok(())
    }

    // =========================================================================
    // Exercise 2: Add a deadline feature
    // =========================================================================
    //
    // Escrows should be able to expire. Add a `deadline` field (Unix timestamp)
    // to the escrow state. The `make` instruction should accept a deadline
    // parameter. The `take` instruction should reject if the deadline has passed.
    //
    // Steps:
    //   1. Add a `deadline: i64` field to the EscrowWithDeadline struct (done)
    //   2. Update INIT_SPACE to account for the new field (done)
    //   3. Implement `make_with_deadline` to store the deadline
    //   4. Implement `take_with_deadline` that checks the deadline
    //
    // Hints:
    //   - Use `Clock::get()?.unix_timestamp` to get current time
    //   - A deadline of 0 means no expiration
    //   - Compare: current_time <= deadline (or deadline == 0)

    pub fn make_with_deadline(
        ctx: Context<MakeWithDeadline>,
        seed: u64,
        amount_offered: u64,
        amount_wanted: u64,
        deadline: i64,
    ) -> Result<()> {
        // TODO: Validate amounts (same as regular make)

        // TODO: If deadline is non-zero, validate it's in the future
        // Hint: let clock = Clock::get()?;
        //       require!(deadline > clock.unix_timestamp, EscrowError::DeadlinePassed);

        // TODO: Initialize the EscrowWithDeadline state (like regular make, plus deadline)

        // TODO: Transfer tokens to vault (same CPI as regular make)

        todo!("Implement make_with_deadline")
    }

    pub fn take_with_deadline(ctx: Context<TakeWithDeadline>) -> Result<()> {
        // TODO: Check if the escrow has expired
        // Hint: if deadline != 0, check Clock::get()?.unix_timestamp <= deadline

        // TODO: Rest is the same as regular take — transfer B to maker,
        //       transfer A to taker, close vault

        todo!("Implement take_with_deadline")
    }

    // =========================================================================
    // Exercise 3: Add partial fills
    // =========================================================================
    //
    // Instead of all-or-nothing, let a taker fill a portion of the escrow.
    // If the escrow offers 100 Token A for 50 Token B, a taker could fill
    // 50% by sending 25 Token B and receiving 50 Token A.
    //
    // Steps:
    //   1. Add a `amount_filled` field to track how much has been filled
    //   2. Implement `take_partial` that accepts a `fill_amount` parameter
    //   3. Calculate proportional Token A to release
    //   4. Update the filled amount (don't close if partially filled)
    //   5. Close only when fully filled
    //
    // Hints:
    //   - Proportional calculation: token_a_amount = fill_amount * amount_offered / amount_wanted
    //   - Watch for integer overflow! Use u128 for intermediate calculations
    //   - Don't close accounts if there are remaining tokens

    pub fn take_partial(ctx: Context<TakePartial>, fill_amount: u64) -> Result<()> {
        // TODO: Validate fill_amount > 0 and doesn't exceed remaining

        // TODO: Calculate proportional Token A to release
        // Hint: let token_a_out = (fill_amount as u128)
        //           .checked_mul(ctx.accounts.escrow.amount_offered as u128)
        //           .and_then(|v| v.checked_div(ctx.accounts.escrow.amount_wanted as u128))
        //           .ok_or(EscrowError::Overflow)? as u64;

        // TODO: Transfer fill_amount of Token B from taker → maker

        // TODO: Transfer token_a_out of Token A from vault → taker

        // TODO: Update escrow state (amount_filled, remaining amounts)

        // TODO: If fully filled, close vault and escrow

        todo!("Implement take_partial")
    }

    // =========================================================================
    // Exercise 4: Add an `update` instruction
    // =========================================================================
    //
    // Let the maker update the desired Token B amount (amount_wanted) without
    // cancelling and recreating the escrow. This saves transaction fees.
    //
    // Steps:
    //   1. Validate new_amount_wanted > 0
    //   2. Update escrow.amount_wanted
    //
    // Constraints (already in the Update struct):
    //   - Only the maker can update (has_one = maker, maker is Signer)
    //   - PDA seeds are verified

    pub fn update(ctx: Context<Update>, new_amount_wanted: u64) -> Result<()> {
        // TODO: Validate new amount is greater than zero

        // TODO: Update the escrow's amount_wanted field

        todo!("Implement update — this one is short!")
    }
}

// =============================================================================
// Account State Structs
// =============================================================================

#[account]
pub struct Escrow {
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub amount_offered: u64,
    pub amount_wanted: u64,
    pub seed: u64,
    pub bump: u8,
}

impl Escrow {
    pub const INIT_SPACE: usize = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 1;
}

// For Exercise 2: Escrow with deadline
#[account]
pub struct EscrowWithDeadline {
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub amount_offered: u64,
    pub amount_wanted: u64,
    pub seed: u64,
    pub bump: u8,
    pub deadline: i64,       // Unix timestamp; 0 means no expiration
}

impl EscrowWithDeadline {
    // Same as Escrow + 8 bytes for i64 deadline
    pub const INIT_SPACE: usize = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 1 + 8;
}

// For Exercise 3: Escrow with partial fill tracking
#[account]
pub struct EscrowPartial {
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub amount_offered: u64,
    pub amount_wanted: u64,
    pub amount_offered_remaining: u64,  // Decreases as takers fill
    pub amount_wanted_remaining: u64,   // Decreases as takers fill
    pub seed: u64,
    pub bump: u8,
}

impl EscrowPartial {
    // Same as Escrow + 8 bytes for each remaining field
    pub const INIT_SPACE: usize = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 1;
}

// =============================================================================
// Account Validation Structs
// =============================================================================

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        token::mint = mint_a,
        token::authority = maker,
        token::token_program = token_program,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        space = Escrow::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

// Exercise 1: The Take accounts are provided — you just implement the logic.
#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    /// CHECK: Validated by escrow's has_one constraint.
    #[account(mut)]
    pub maker: UncheckedAccount<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = mint_b,
        token::authority = taker,
        token::token_program = token_program,
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = mint_a,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

// Exercise 2: Make/Take with deadline
#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct MakeWithDeadline<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        token::mint = mint_a,
        token::authority = maker,
        token::token_program = token_program,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        space = EscrowWithDeadline::INIT_SPACE,
        seeds = [b"escrow_dl", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, EscrowWithDeadline>,
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TakeWithDeadline<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    /// CHECK: Validated by escrow's has_one constraint.
    #[account(mut)]
    pub maker: UncheckedAccount<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = mint_b,
        token::authority = taker,
        token::token_program = token_program,
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [b"escrow_dl", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, EscrowWithDeadline>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

// Exercise 3: Partial fill accounts
#[derive(Accounts)]
pub struct TakePartial<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    /// CHECK: Validated by escrow's has_one constraint.
    #[account(mut)]
    pub maker: UncheckedAccount<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = mint_b,
        token::authority = taker,
        token::token_program = token_program,
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,
    // NOTE: No `close` constraint here because partial fills may not close the escrow.
    // We handle closing manually in the instruction logic when fully filled.
    #[account(
        mut,
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [b"escrow_partial", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, EscrowPartial>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

// Exercise 4: Update accounts
#[derive(Accounts)]
pub struct Update<'info> {
    // Maker must sign — only they can update their own escrow.
    #[account(mut)]
    pub maker: Signer<'info>,

    // The escrow to update. has_one = maker ensures only the creator can modify.
    // Seeds verify it's a real escrow PDA.
    #[account(
        mut,
        has_one = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
}

// =============================================================================
// Errors
// =============================================================================
#[error_code]
pub enum EscrowError {
    #[msg("Amount must be greater than zero")]
    InvalidAmount,

    #[msg("Escrow deadline has passed")]
    DeadlinePassed,

    #[msg("Deadline must be in the future")]
    InvalidDeadline,

    #[msg("Fill amount exceeds remaining")]
    FillExceedsRemaining,

    #[msg("Arithmetic overflow")]
    Overflow,
}
