// Module 14: Anchor Account Constraints — Vault Program
//
// A SOL vault program demonstrating every major Anchor constraint type.
// The vault holds SOL on behalf of an authority who can deposit, withdraw, and close it.

use anchor_lang::prelude::*;
use anchor_lang::system_program;

// Program ID — replace with your actual deployed program ID.
declare_id!("11111111111111111111111111111111");

// ---------------------------------------------------------------------------
// Program entry point — each public function becomes an on-chain instruction.
// ---------------------------------------------------------------------------
#[program]
pub mod account_constraints {
    use super::*;

    /// Creates a new vault PDA owned by the signing authority.
    /// Demonstrates: init, payer, space, seeds, bump.
    pub fn create_vault(ctx: Context<CreateVault>, name: String) -> Result<()> {
        // Validate name length inside instruction body using require!().
        require!(name.len() <= 50, VaultError::NameTooLong);

        // Access the newly-created vault account.
        let vault = &mut ctx.accounts.vault;

        // Store the authority's public key so we can verify it on later calls.
        vault.authority = ctx.accounts.authority.key();

        // Initialize balance to zero — no SOL deposited yet.
        vault.balance = 0;

        // Store the PDA bump so we can reuse it and save compute on future accesses.
        vault.bump = ctx.bumps.vault;

        // Store the human-readable name.
        vault.name = name;

        Ok(())
    }

    /// Deposits SOL from the depositor into the vault PDA.
    /// Demonstrates: has_one, constraint (with custom error), mut.
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        // Imperative check — amount must be positive.
        require!(amount > 0, VaultError::ZeroDeposit);

        // Transfer SOL from depositor to vault via CPI to the System Program.
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.depositor.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                },
            ),
            amount,
        )?;

        // Update the vault's bookkeeping balance field.
        ctx.accounts.vault.balance = ctx
            .accounts
            .vault
            .balance
            .checked_add(amount)
            .ok_or(VaultError::Overflow)?;

        Ok(())
    }

    /// Withdraws SOL from the vault PDA back to the authority.
    /// Demonstrates: has_one (ownership), constraint (balance check), seeds/bump for PDA signing.
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        // Imperative check — amount must be positive.
        require!(amount > 0, VaultError::ZeroDeposit);

        let vault = &mut ctx.accounts.vault;

        // Update bookkeeping before transfer (checks-effects-interactions pattern).
        vault.balance = vault
            .balance
            .checked_sub(amount)
            .ok_or(VaultError::InsufficientFunds)?;

        // Transfer lamports directly from the vault PDA account.
        // Because the vault is a PDA owned by this program, we can modify its lamports.
        let vault_info = vault.to_account_info();
        let authority_info = ctx.accounts.authority.to_account_info();

        **vault_info.try_borrow_mut_lamports()? -= amount;
        **authority_info.try_borrow_mut_lamports()? += amount;

        Ok(())
    }

    /// Closes the vault, returning all remaining SOL to the authority.
    /// Demonstrates: close constraint, has_one.
    pub fn close_vault(_ctx: Context<CloseVault>) -> Result<()> {
        // The `close` constraint handles everything:
        // 1. Transfers all lamports to authority
        // 2. Zeros the account data
        // 3. Sets owner to System Program
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Account data structures — these define on-chain state.
// ---------------------------------------------------------------------------

/// The Vault account stores SOL on behalf of an authority.
/// Using InitSpace derive to auto-calculate field sizes.
#[account]
#[derive(InitSpace)]
pub struct Vault {
    /// The public key authorized to withdraw and close this vault.
    pub authority: Pubkey, // 32 bytes

    /// Bookkeeping: how much SOL has been deposited (in lamports).
    pub balance: u64, // 8 bytes

    /// The PDA bump seed — stored for efficient re-derivation.
    pub bump: u8, // 1 byte

    /// A human-readable name for the vault.
    #[max_len(50)]
    pub name: String, // 4 + 50 = 54 bytes
}
// Total with discriminator: 8 + 32 + 8 + 1 + 54 = 103 bytes

// ---------------------------------------------------------------------------
// Accounts structs — each defines the accounts an instruction expects,
// plus all the declarative constraints Anchor enforces automatically.
// ---------------------------------------------------------------------------

/// Accounts for `create_vault`.
///
/// Constraints demonstrated:
/// - `init`   — creates a brand-new account
/// - `payer`  — authority pays the rent
/// - `space`  — allocate exactly enough bytes
/// - `seeds`  — derive PDA from ["vault", authority pubkey]
/// - `bump`   — Anchor finds the canonical bump
#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(
        // Create a new on-chain account for the Vault.
        init,
        // The authority signer pays the SOL rent deposit.
        payer = authority,
        // Allocate space: 8-byte discriminator + auto-calculated field sizes.
        space = 8 + Vault::INIT_SPACE,
        // Derive PDA address from a static tag + the authority's pubkey.
        // This means each authority gets exactly one vault.
        seeds = [b"vault", authority.key().as_ref()],
        // Anchor finds and verifies the canonical bump seed.
        bump,
    )]
    pub vault: Account<'info, Vault>,

    // The authority must sign the transaction and is marked mut because
    // lamports are deducted from their account to pay rent.
    #[account(mut)]
    pub authority: Signer<'info>,

    // Required by `init` — the System Program creates the account.
    pub system_program: Program<'info, System>,
}

/// Accounts for `deposit`.
///
/// Constraints demonstrated:
/// - `mut`       — vault data changes (balance field updated)
/// - `has_one`   — vault.authority must equal authority.key()
/// - `seeds/bump` — re-derive PDA to verify the vault address
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        // Vault data is modified (balance increases).
        mut,
        // Verify the vault's stored authority matches the signer.
        has_one = authority @ VaultError::Unauthorized,
        // Re-derive PDA to confirm the vault address is correct.
        seeds = [b"vault", authority.key().as_ref()],
        // Use stored bump for cheaper verification.
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    // The depositor sends SOL — must be a signer and mutable.
    #[account(mut)]
    pub depositor: Signer<'info>,

    // The authority doesn't need to sign the deposit, but must be provided
    // so has_one can verify against vault.authority.
    /// CHECK: Only used for PDA derivation and has_one check; not written to.
    pub authority: UncheckedAccount<'info>,

    // Required for the SOL transfer CPI.
    pub system_program: Program<'info, System>,
}

/// Accounts for `withdraw`.
///
/// Constraints demonstrated:
/// - `mut`        — vault data and lamports change
/// - `has_one`    — ownership check
/// - `constraint` — custom boolean check with error message
/// - `Signer`     — authority must sign
#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Withdraw<'info> {
    #[account(
        // Vault is modified: balance decreases, lamports transferred out.
        mut,
        // Verify the signer is the vault's authority.
        has_one = authority @ VaultError::Unauthorized,
        // Declarative balance check — runs before instruction body.
        constraint = vault.balance >= amount @ VaultError::InsufficientFunds,
        // Verify PDA address.
        seeds = [b"vault", authority.key().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    // Authority must sign the withdrawal AND receives the SOL.
    #[account(mut)]
    pub authority: Signer<'info>,
}

/// Accounts for `close_vault`.
///
/// Constraints demonstrated:
/// - `close`    — closes the account and sends lamports to authority
/// - `has_one`  — only the authority can close
/// - `mut`      — account data and lamports change
#[derive(Accounts)]
pub struct CloseVault<'info> {
    #[account(
        // Account must be writable to zero its data and transfer lamports.
        mut,
        // Only the vault's authority can close it.
        has_one = authority @ VaultError::Unauthorized,
        // Close the vault: zero data, transfer all lamports to authority,
        // reassign owner to System Program.
        close = authority,
        // Verify PDA.
        seeds = [b"vault", authority.key().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    // Receives the reclaimed lamports. Must be mutable and must sign.
    #[account(mut)]
    pub authority: Signer<'info>,
}

// ---------------------------------------------------------------------------
// Custom errors — used with `@` in constraints and `require!()` in logic.
// ---------------------------------------------------------------------------
#[error_code]
pub enum VaultError {
    #[msg("Insufficient funds in vault for this withdrawal")]
    InsufficientFunds,

    #[msg("Deposit amount must be greater than zero")]
    ZeroDeposit,

    #[msg("Vault name exceeds maximum length of 50 characters")]
    NameTooLong,

    #[msg("Unauthorized: signer is not the vault authority")]
    Unauthorized,

    #[msg("Arithmetic overflow")]
    Overflow,
}
