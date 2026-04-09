// =============================================================================
// Module 17 Solutions: Escrow Program Exercises — Fully Commented
// =============================================================================
//
// This file contains solutions for all four exercises.
// Every line is commented to explain what it does and WHY.
// =============================================================================

// Import the Anchor prelude — gives us all the macros, types, and traits
// needed for Solana program development.
use anchor_lang::prelude::*;

// Associated Token Account program — for creating deterministic token accounts.
use anchor_spl::associated_token::AssociatedToken;

// SPL Token interface types — for token transfers and account management via CPI.
use anchor_spl::token_interface::{
    close_account,       // CPI to close a token account and reclaim rent
    transfer_checked,    // CPI to transfer tokens with decimal verification
    CloseAccount,        // Accounts needed for close_account CPI
    Mint,                // Deserialized mint account type
    TokenAccount,        // Deserialized token account type
    TokenInterface,      // The SPL Token or Token-2022 program
    TransferChecked,     // Accounts needed for transfer_checked CPI
};

// Program ID — placeholder for this tutorial module.
declare_id!("Esc1111111111111111111111111111111111111111");

// The #[program] macro turns this module into a Solana program with
// auto-generated instruction routing.
#[program]
pub mod escrow_solutions {
    // Import everything from the parent scope into this module.
    use super::*;

    // =========================================================================
    // Make instruction — same as the main tutorial, provided for context.
    // =========================================================================
    // Creates an escrow and deposits Token A into the vault.
    pub fn make(
        ctx: Context<Make>,      // Validated account struct
        seed: u64,               // Unique seed for PDA derivation
        amount_offered: u64,     // Token A amount to deposit
        amount_wanted: u64,      // Token B amount wanted in return
    ) -> Result<()> {
        // Reject zero amounts — a zero escrow wastes rent and is meaningless.
        require!(amount_offered > 0, EscrowError::InvalidAmount);
        require!(amount_wanted > 0, EscrowError::InvalidAmount);

        // Store the escrow terms. set_inner replaces all fields at once.
        ctx.accounts.escrow.set_inner(Escrow {
            maker: ctx.accounts.maker.key(),       // Who created this escrow
            mint_a: ctx.accounts.mint_a.key(),      // Token being offered
            mint_b: ctx.accounts.mint_b.key(),      // Token wanted in return
            amount_offered,                         // How much Token A is in vault
            amount_wanted,                          // How much Token B is expected
            seed,                                   // Unique seed for PDA
            bump: ctx.bumps.escrow,                 // PDA bump, stored to save CUs
        });

        // CPI: transfer Token A from maker's wallet to the escrow vault.
        // The maker signs because the tokens come from their account.
        transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), // Token program to call
                TransferChecked {
                    from: ctx.accounts.maker_ata_a.to_account_info(),  // Source
                    mint: ctx.accounts.mint_a.to_account_info(),       // Decimal check
                    to: ctx.accounts.vault.to_account_info(),          // Destination
                    authority: ctx.accounts.maker.to_account_info(),   // Signer
                },
            ),
            amount_offered,                     // Amount to transfer
            ctx.accounts.mint_a.decimals,       // Expected decimals
        )?; // Propagate errors (insufficient balance, wrong mint, etc.)

        Ok(()) // Escrow created and funded
    }

    // =========================================================================
    // SOLUTION 1: Complete the `take` instruction
    // =========================================================================
    //
    // The taker completes the escrow swap atomically:
    //   1. Taker sends Token B → Maker
    //   2. Vault sends Token A → Taker (PDA-signed)
    //   3. Close the vault (rent → maker)
    //   4. Close the escrow PDA (handled by `close = maker` constraint)
    pub fn take(ctx: Context<Take>) -> Result<()> {
        // --- Step 1: Transfer Token B from taker to maker ---
        // The taker sends the requested amount of Token B to the maker.
        // This is a normal transfer — the taker is the authority/signer.
        // In Solidity: tokenB.transferFrom(taker, maker, amountB)
        transfer_checked(
            // CpiContext::new — no PDA signing needed, taker signs directly.
            CpiContext::new(
                // The token program to invoke.
                ctx.accounts.token_program.to_account_info(),
                // The accounts for the transfer.
                TransferChecked {
                    // Source: taker's Token B account.
                    from: ctx.accounts.taker_ata_b.to_account_info(),
                    // Mint B: for decimal verification safety.
                    mint: ctx.accounts.mint_b.to_account_info(),
                    // Destination: maker's Token B account (maker gets paid).
                    to: ctx.accounts.maker_ata_b.to_account_info(),
                    // Authority: the taker, who signed this transaction.
                    authority: ctx.accounts.taker.to_account_info(),
                },
            ),
            // Transfer the exact amount the maker requested when creating escrow.
            ctx.accounts.escrow.amount_wanted,
            // Decimal count from the mint — transfer_checked verifies this.
            ctx.accounts.mint_b.decimals,
        )?; // If taker lacks funds, this fails and the whole TX reverts.

        // --- Step 2: Transfer Token A from vault to taker ---
        // The vault is owned by the escrow PDA, so the PDA must sign.
        // We construct the signer seeds that prove our program controls this PDA.

        // Build PDA signer seeds: must match the seeds in the account constraint.
        // The runtime verifies: sha256("escrow" + maker_pubkey + seed + bump) == PDA
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"escrow",                                    // Static prefix seed
            ctx.accounts.escrow.maker.as_ref(),           // Maker's pubkey bytes
            &ctx.accounts.escrow.seed.to_le_bytes(),      // Seed as LE bytes
            &[ctx.accounts.escrow.bump],                  // Bump byte
        ]];

        // CPI with PDA signing — CpiContext::new_with_signer instead of ::new.
        // The signer_seeds tell the runtime "I (the program) control this PDA."
        transfer_checked(
            CpiContext::new_with_signer(
                // Token program to invoke.
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    // Source: the vault (holds escrowed Token A).
                    from: ctx.accounts.vault.to_account_info(),
                    // Mint A: decimal verification.
                    mint: ctx.accounts.mint_a.to_account_info(),
                    // Destination: taker's Token A account (taker gets tokens).
                    to: ctx.accounts.taker_ata_a.to_account_info(),
                    // Authority: the escrow PDA (owner of the vault).
                    authority: ctx.accounts.escrow.to_account_info(),
                },
                // PDA signer seeds — the "keyless signature."
                signer_seeds,
            ),
            // Transfer everything the maker deposited.
            ctx.accounts.escrow.amount_offered,
            // Verify decimals match the mint.
            ctx.accounts.mint_a.decimals,
        )?; // Fails if vault has insufficient funds (shouldn't happen).

        // --- Step 3: Close the vault token account ---
        // The vault is now empty. Close it to reclaim the rent-exempt SOL.
        // Rent goes to the maker (they originally paid for account creation).
        close_account(CpiContext::new_with_signer(
            // Token program handles the close.
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                // The account to close.
                account: ctx.accounts.vault.to_account_info(),
                // Rent SOL destination — return to the maker.
                destination: ctx.accounts.maker.to_account_info(),
                // Authority — escrow PDA owns the vault.
                authority: ctx.accounts.escrow.to_account_info(),
            },
            // Same PDA signer seeds.
            signer_seeds,
        ))?; // Propagate close errors.

        // Step 4: The escrow PDA itself is closed by Anchor's `close = maker`
        // constraint on the escrow account in the Take struct. Anchor handles
        // zeroing the data and transferring lamports to the maker.

        Ok(()) // Swap complete! Both parties received their tokens.
    }

    // Cancel — same as main tutorial, included for completeness.
    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        // Construct PDA signer seeds for vault operations.
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"escrow",
            ctx.accounts.escrow.maker.as_ref(),
            &ctx.accounts.escrow.seed.to_le_bytes(),
            &[ctx.accounts.escrow.bump],
        ]];

        // Return Token A from vault to maker.
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

        // Close the empty vault, returning rent to maker.
        close_account(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.vault.to_account_info(),
                destination: ctx.accounts.maker.to_account_info(),
                authority: ctx.accounts.escrow.to_account_info(),
            },
            signer_seeds,
        ))?;

        // Escrow PDA closed by `close = maker` constraint.
        Ok(())
    }

    // =========================================================================
    // SOLUTION 2: Escrow with deadline
    // =========================================================================
    //
    // The maker specifies a Unix timestamp after which the escrow can no longer
    // be taken. A deadline of 0 means the escrow never expires.
    //
    // This demonstrates:
    //   - Using Clock::get() to read on-chain time (M14 sysvar access)
    //   - Adding conditional validation logic beyond static constraints

    // Create an escrow with a deadline.
    pub fn make_with_deadline(
        ctx: Context<MakeWithDeadline>,  // Account struct with EscrowWithDeadline
        seed: u64,                        // PDA seed
        amount_offered: u64,              // Token A to deposit
        amount_wanted: u64,               // Token B wanted
        deadline: i64,                    // Unix timestamp (0 = no expiry)
    ) -> Result<()> {
        // Same zero-amount validation as regular make.
        require!(amount_offered > 0, EscrowError::InvalidAmount);
        require!(amount_wanted > 0, EscrowError::InvalidAmount);

        // If a deadline is set (non-zero), verify it's in the future.
        // Clock::get() returns the cluster's current timestamp.
        // This prevents creating already-expired escrows (a waste of rent).
        if deadline != 0 {
            // Get the current on-chain timestamp from the Clock sysvar.
            // This is Solana's equivalent of `block.timestamp` in Solidity.
            let clock = Clock::get()?;

            // Ensure the deadline hasn't already passed.
            require!(
                deadline > clock.unix_timestamp,
                EscrowError::InvalidDeadline
            );
        }

        // Initialize the escrow state with the deadline field.
        ctx.accounts.escrow.set_inner(EscrowWithDeadline {
            maker: ctx.accounts.maker.key(),       // Creator's pubkey
            mint_a: ctx.accounts.mint_a.key(),      // Token being offered
            mint_b: ctx.accounts.mint_b.key(),      // Token wanted in return
            amount_offered,                         // Amount deposited in vault
            amount_wanted,                          // Amount expected from taker
            seed,                                   // PDA derivation seed
            bump: ctx.bumps.escrow,                 // PDA bump, cached for efficiency
            deadline,                               // Expiry timestamp (0 = never)
        });

        // Transfer Token A from maker's wallet into the vault.
        // Identical CPI pattern to the regular make instruction.
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

        Ok(()) // Escrow created with deadline
    }

    // Take an escrow that has a deadline.
    // The only addition vs regular take: a time check at the start.
    pub fn take_with_deadline(ctx: Context<TakeWithDeadline>) -> Result<()> {
        // --- Deadline check ---
        // If the escrow has a deadline (non-zero), verify it hasn't expired.
        // This is the key addition over the regular take instruction.
        let escrow = &ctx.accounts.escrow;
        if escrow.deadline != 0 {
            // Read the current on-chain time.
            let clock = Clock::get()?;

            // If current time exceeds the deadline, reject the transaction.
            // The maker can still cancel — only taking is blocked after expiry.
            require!(
                clock.unix_timestamp <= escrow.deadline,
                EscrowError::DeadlinePassed
            );
        }

        // --- From here, identical to regular take ---

        // Step 1: Taker sends Token B to maker.
        transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.taker_ata_b.to_account_info(),
                    mint: ctx.accounts.mint_b.to_account_info(),
                    to: ctx.accounts.maker_ata_b.to_account_info(),
                    authority: ctx.accounts.taker.to_account_info(),
                },
            ),
            ctx.accounts.escrow.amount_wanted,
            ctx.accounts.mint_b.decimals,
        )?;

        // Step 2: Vault sends Token A to taker (PDA signs).
        // Note: seeds use "escrow_dl" to match the MakeWithDeadline PDA.
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"escrow_dl",                                 // Different prefix for deadline variant
            ctx.accounts.escrow.maker.as_ref(),           // Maker's pubkey
            &ctx.accounts.escrow.seed.to_le_bytes(),      // Seed bytes
            &[ctx.accounts.escrow.bump],                  // Bump byte
        ]];

        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.vault.to_account_info(),
                    mint: ctx.accounts.mint_a.to_account_info(),
                    to: ctx.accounts.taker_ata_a.to_account_info(),
                    authority: ctx.accounts.escrow.to_account_info(),
                },
                signer_seeds,
            ),
            ctx.accounts.escrow.amount_offered,
            ctx.accounts.mint_a.decimals,
        )?;

        // Step 3: Close vault, return rent to maker.
        close_account(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.vault.to_account_info(),
                destination: ctx.accounts.maker.to_account_info(),
                authority: ctx.accounts.escrow.to_account_info(),
            },
            signer_seeds,
        ))?;

        // Step 4: Escrow PDA closed by `close = maker` constraint.
        Ok(())
    }

    // =========================================================================
    // SOLUTION 3: Partial fills
    // =========================================================================
    //
    // Instead of all-or-nothing, takers can fill a portion of the escrow.
    // A 50% fill on a "100 USDC for 2 SOL" escrow sends 1 SOL to the maker
    // and releases 50 USDC to the taker.
    //
    // Key concepts:
    //   - Proportional math with overflow protection (u128 intermediate)
    //   - Mutable escrow state (updating remaining amounts)
    //   - Conditional close (only when fully filled)

    pub fn take_partial(ctx: Context<TakePartial>, fill_amount: u64) -> Result<()> {
        // --- Validation ---
        // fill_amount is how much Token B the taker is sending.
        // It must be positive and not exceed what the escrow still needs.
        require!(fill_amount > 0, EscrowError::InvalidAmount);
        require!(
            fill_amount <= ctx.accounts.escrow.amount_wanted_remaining,
            EscrowError::FillExceedsRemaining
        );

        // --- Proportional calculation ---
        // How much Token A does the taker receive for this partial fill?
        //
        // Formula: token_a_out = fill_amount * amount_offered / amount_wanted
        //
        // We use the ORIGINAL amounts (not remaining) for the ratio to avoid
        // rounding drift across multiple partial fills.
        //
        // CRITICAL: Use u128 for the intermediate multiplication to prevent
        // overflow. u64::MAX * u64::MAX would overflow u64 but fits in u128.
        // (Recall from M5: checked_mul, checked_div for safe arithmetic.)
        let token_a_out = (fill_amount as u128)
            // Multiply fill_amount by total Token A offered.
            .checked_mul(ctx.accounts.escrow.amount_offered as u128)
            // Divide by total Token B wanted to get the proportional amount.
            .and_then(|v| v.checked_div(ctx.accounts.escrow.amount_wanted as u128))
            // If any arithmetic fails (divide by zero, overflow), return error.
            .ok_or(EscrowError::Overflow)? as u64;

        // Sanity check: calculated output must be positive.
        require!(token_a_out > 0, EscrowError::InvalidAmount);

        // --- Extract PDA seed values into locals ---
        // We copy these values out of the escrow before any mutable borrow.
        // This avoids the classic Rust borrow-checker conflict: the signer
        // seeds reference escrow immutably, but later we need &mut escrow.
        // By copying the seed components to local variables, we own the data
        // and can build signer seeds independently of the escrow borrow.
        // (Recall from M1: ownership vs borrowing — this is it in action!)
        let maker_key = ctx.accounts.escrow.maker;
        let seed_bytes = ctx.accounts.escrow.seed.to_le_bytes();
        let bump = ctx.accounts.escrow.bump;

        // Build PDA signer seeds from our local copies.
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"escrow_partial",       // Prefix for partial-fill variant
            maker_key.as_ref(),      // Maker pubkey (from local copy)
            seed_bytes.as_ref(),     // Seed bytes (from local copy)
            &[bump],                 // Bump byte (from local copy)
        ]];

        // --- Step 1: Taker sends partial Token B to maker ---
        // Only sends fill_amount, not the full amount_wanted.
        transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.taker_ata_b.to_account_info(),
                    mint: ctx.accounts.mint_b.to_account_info(),
                    to: ctx.accounts.maker_ata_b.to_account_info(),
                    authority: ctx.accounts.taker.to_account_info(),
                },
            ),
            fill_amount,                         // Only the partial amount
            ctx.accounts.mint_b.decimals,
        )?;

        // --- Step 2: Vault sends proportional Token A to taker ---
        // PDA signs the transfer from the vault using our local signer seeds.
        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.vault.to_account_info(),
                    mint: ctx.accounts.mint_a.to_account_info(),
                    to: ctx.accounts.taker_ata_a.to_account_info(),
                    authority: ctx.accounts.escrow.to_account_info(),
                },
                signer_seeds,
            ),
            token_a_out,                         // Proportional Token A amount
            ctx.accounts.mint_a.decimals,
        )?;

        // --- Step 3: Update escrow state ---
        // Decrease the remaining amounts to reflect this partial fill.
        // The escrow stays open for future takers to fill the rest.
        // Now we can safely take a mutable reference — the signer_seeds
        // borrows are finished (no more CPIs that use them before the close).
        let new_wanted_remaining = ctx.accounts.escrow
            .amount_wanted_remaining
            .saturating_sub(fill_amount);
        let new_offered_remaining = ctx.accounts.escrow
            .amount_offered_remaining
            .saturating_sub(token_a_out);

        // Write the updated values.
        ctx.accounts.escrow.amount_wanted_remaining = new_wanted_remaining;
        ctx.accounts.escrow.amount_offered_remaining = new_offered_remaining;

        // --- Step 4: Close if fully filled ---
        // If no more Token B is needed, the escrow is complete.
        // Close the vault and the escrow PDA to reclaim rent.
        if new_wanted_remaining == 0 {
            // Close the vault — all tokens have been distributed.
            close_account(CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                CloseAccount {
                    account: ctx.accounts.vault.to_account_info(),
                    destination: ctx.accounts.maker.to_account_info(),
                    authority: ctx.accounts.escrow.to_account_info(),
                },
                signer_seeds,
            ))?;

            // Close the escrow PDA manually (no `close` constraint on this struct).
            // Transfer all lamports from escrow to maker, then zero the data.
            // This is what Anchor's `close` constraint does under the hood.
            let escrow_info = ctx.accounts.escrow.to_account_info();
            let maker_info = ctx.accounts.maker.to_account_info();

            // Move all lamports from escrow account to maker.
            **maker_info.try_borrow_mut_lamports()? += escrow_info.lamports();
            **escrow_info.try_borrow_mut_lamports()? = 0;

            // Zero out the escrow data so it can't be reused.
            // The runtime will garbage-collect zero-lamport accounts.
            escrow_info.data.borrow_mut().fill(0);
        }

        Ok(()) // Partial fill complete (or full fill with cleanup)
    }

    // =========================================================================
    // SOLUTION 4: Update instruction
    // =========================================================================
    //
    // Let the maker change the desired Token B amount without cancelling.
    // This is the simplest exercise — just validate and update a field.
    //
    // Why this is useful: if market conditions change, the maker can adjust
    // their ask price without paying to cancel + recreate the escrow.

    pub fn update(ctx: Context<Update>, new_amount_wanted: u64) -> Result<()> {
        // Validate: new amount must be positive.
        // Zero would mean the taker gets tokens for free — not intended.
        require!(new_amount_wanted > 0, EscrowError::InvalidAmount);

        // Update the escrow's amount_wanted field.
        // The has_one = maker constraint + Signer already verified that
        // only the original maker can call this instruction.
        ctx.accounts.escrow.amount_wanted = new_amount_wanted;

        // That's it! Anchor serializes the updated escrow back to the account
        // automatically when the instruction returns. (This is handled by the
        // Account type's Drop implementation — recall from M6 how traits work.)
        Ok(())
    }
}

// =============================================================================
// Account State Structs
// =============================================================================

// Base escrow state — used by make, take, cancel, and update.
#[account]
pub struct Escrow {
    pub maker: Pubkey,           // 32 bytes — who created this escrow
    pub mint_a: Pubkey,          // 32 bytes — token being offered
    pub mint_b: Pubkey,          // 32 bytes — token wanted in return
    pub amount_offered: u64,     // 8 bytes  — how much Token A in vault
    pub amount_wanted: u64,      // 8 bytes  — how much Token B expected
    pub seed: u64,               // 8 bytes  — unique PDA seed
    pub bump: u8,                // 1 byte   — PDA bump (cached)
}

impl Escrow {
    // Total space: 8 (discriminator) + 32*3 + 8*3 + 1 = 129 bytes
    pub const INIT_SPACE: usize = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 1;
}

// Escrow with deadline (Exercise 2)
#[account]
pub struct EscrowWithDeadline {
    pub maker: Pubkey,           // 32 bytes — escrow creator
    pub mint_a: Pubkey,          // 32 bytes — offered token mint
    pub mint_b: Pubkey,          // 32 bytes — wanted token mint
    pub amount_offered: u64,     // 8 bytes  — vault balance
    pub amount_wanted: u64,      // 8 bytes  — expected payment
    pub seed: u64,               // 8 bytes  — PDA seed
    pub bump: u8,                // 1 byte   — PDA bump
    pub deadline: i64,           // 8 bytes  — expiry timestamp (0 = never)
}

impl EscrowWithDeadline {
    // 129 + 8 (deadline) = 137 bytes
    pub const INIT_SPACE: usize = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 1 + 8;
}

// Escrow with partial fill tracking (Exercise 3)
#[account]
pub struct EscrowPartial {
    pub maker: Pubkey,                    // 32 bytes — escrow creator
    pub mint_a: Pubkey,                   // 32 bytes — offered token mint
    pub mint_b: Pubkey,                   // 32 bytes — wanted token mint
    pub amount_offered: u64,              // 8 bytes  — original total offered
    pub amount_wanted: u64,               // 8 bytes  — original total wanted
    pub amount_offered_remaining: u64,    // 8 bytes  — Token A still in vault
    pub amount_wanted_remaining: u64,     // 8 bytes  — Token B still needed
    pub seed: u64,                        // 8 bytes  — PDA seed
    pub bump: u8,                         // 1 byte   — PDA bump
}

impl EscrowPartial {
    // 129 + 8 + 8 = 145 bytes (two extra u64 fields)
    pub const INIT_SPACE: usize = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 1;
}

// =============================================================================
// Account Validation Structs
// =============================================================================

// Make — creates a new escrow and vault, deposits Token A.
#[derive(Accounts)]
#[instruction(seed: u64)]  // Access seed parameter for PDA derivation.
pub struct Make<'info> {
    // Maker's wallet — signs and pays for account creation.
    #[account(mut)]
    pub maker: Signer<'info>,

    // Mint for Token A (what maker deposits).
    pub mint_a: InterfaceAccount<'info, Mint>,

    // Mint for Token B (what maker wants).
    pub mint_b: InterfaceAccount<'info, Mint>,

    // Maker's Token A account — source of the deposit.
    #[account(
        mut,                                   // Balance will decrease
        token::mint = mint_a,                  // Must hold Token A
        token::authority = maker,              // Must belong to maker
        token::token_program = token_program,  // Correct token program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    // Escrow PDA — stores the deal terms.
    #[account(
        init,                                  // Create this account
        payer = maker,                         // Maker pays rent
        space = Escrow::INIT_SPACE,            // Allocate exact bytes needed
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,                                  // Anchor finds the bump
    )]
    pub escrow: Account<'info, Escrow>,

    // Vault — token account owned by the escrow PDA.
    #[account(
        init,                                          // Create this token account
        payer = maker,                                 // Maker pays rent
        associated_token::mint = mint_a,               // Holds Token A
        associated_token::authority = escrow,           // Owned by escrow PDA
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // Required programs.
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

// Take — taker completes the swap (Solution 1).
#[derive(Accounts)]
pub struct Take<'info> {
    // Taker signs and may pay for ATA creation.
    #[account(mut)]
    pub taker: Signer<'info>,

    // Maker doesn't sign but receives tokens and rent.
    /// CHECK: Validated by has_one = maker on escrow account.
    #[account(mut)]
    pub maker: UncheckedAccount<'info>,

    // Token A mint — for vault verification.
    pub mint_a: InterfaceAccount<'info, Mint>,

    // Token B mint — for taker's source verification.
    pub mint_b: InterfaceAccount<'info, Mint>,

    // Taker's Token A account — receives escrowed tokens.
    #[account(
        init_if_needed,                                // Create if doesn't exist
        payer = taker,                                 // Taker pays for their ATA
        associated_token::mint = mint_a,               // Must hold Token A
        associated_token::authority = taker,            // Belongs to taker
        associated_token::token_program = token_program,
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,

    // Taker's Token B account — source of payment to maker.
    #[account(
        mut,                                   // Balance decreases
        token::mint = mint_b,                  // Must hold Token B
        token::authority = taker,              // Belongs to taker
        token::token_program = token_program,
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    // Maker's Token B account — receives payment from taker.
    #[account(
        init_if_needed,                                // Create if maker never held Token B
        payer = taker,                                 // Taker pays for maker's ATA
        associated_token::mint = mint_b,               // Must hold Token B
        associated_token::authority = maker,            // Belongs to maker
        associated_token::token_program = token_program,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,

    // Escrow PDA — read terms, then close.
    #[account(
        mut,                       // Will be closed
        close = maker,             // Rent → maker when closed
        has_one = maker,           // escrow.maker == maker account
        has_one = mint_a,          // escrow.mint_a == mint_a account
        has_one = mint_b,          // escrow.mint_b == mint_b account
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,        // Use stored bump (saves CU)
    )]
    pub escrow: Account<'info, Escrow>,

    // Vault — transfer out, then close.
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // Required programs.
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

// Cancel — maker reclaims tokens.
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

// MakeWithDeadline — creates escrow with expiry (Solution 2).
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
    // Different seed prefix ("escrow_dl") to avoid collisions with regular escrows.
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

// TakeWithDeadline — take an escrow with deadline check (Solution 2).
#[derive(Accounts)]
pub struct TakeWithDeadline<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    /// CHECK: Validated by has_one constraint.
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
    // Uses "escrow_dl" seeds to match the MakeWithDeadline PDA.
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

// TakePartial — partial fill of an escrow (Solution 3).
#[derive(Accounts)]
pub struct TakePartial<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    /// CHECK: Validated by has_one constraint.
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
    // No `close` constraint — partial fills keep the account open.
    // We close it manually in the instruction when fully filled.
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

// Update — maker changes the desired Token B amount (Solution 4).
#[derive(Accounts)]
pub struct Update<'info> {
    // Maker must sign to authorize the change.
    #[account(mut)]
    pub maker: Signer<'info>,

    // Escrow to update — has_one = maker ensures only the creator can modify.
    #[account(
        mut,                       // Will be modified
        has_one = maker,           // Only the creator can update
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,        // Verify correct PDA
    )]
    pub escrow: Account<'info, Escrow>,
}

// =============================================================================
// Custom Errors — descriptive error messages for each failure case
// =============================================================================
#[error_code]
pub enum EscrowError {
    // Amount parameters must be positive.
    #[msg("Amount must be greater than zero")]
    InvalidAmount,

    // Taker tried to take an expired escrow.
    #[msg("Escrow deadline has passed")]
    DeadlinePassed,

    // Maker tried to set a deadline in the past.
    #[msg("Deadline must be in the future")]
    InvalidDeadline,

    // Partial fill amount exceeds what the escrow still needs.
    #[msg("Fill amount exceeds remaining")]
    FillExceedsRemaining,

    // Arithmetic overflow in proportional calculations.
    #[msg("Arithmetic overflow")]
    Overflow,
}
