// =============================================================================
// Module 13 Solutions: Anchor Counter
// =============================================================================
// Complete solutions for all 5 exercises. Every line is commented.
// =============================================================================

use anchor_lang::prelude::*;

// Program address placeholder — gets replaced on `anchor build`.
declare_id!("11111111111111111111111111111111");

#[program]
pub mod counter_solutions {
    use super::*;

    // Initialize: creates the counter account with count=0 and records the authority.
    // The authority is whoever signs the initialize transaction — they become the "owner."
    // Solidity equivalent:
    //   constructor() { owner = msg.sender; count = 0; }
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Get mutable reference to the freshly-created counter account.
        // Anchor already allocated space, paid rent, and set the discriminator.
        let counter = &mut ctx.accounts.counter;

        // Set initial count to zero.
        counter.count = 0;

        // Store the user's public key as the authority.
        // .key() returns the Pubkey of the account.
        // In Solidity: owner = msg.sender;
        counter.authority = ctx.accounts.user.key();

        msg!("Counter initialized by authority: {}", counter.authority);
        Ok(())
    }

    // Basic increment — anyone can call this (no authority check).
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!("Incremented to: {}", counter.count);
        Ok(())
    }

    // =========================================================================
    // Solution 1: The Initialize accounts struct is complete below.
    // =========================================================================
    // See the Initialize struct — all constraints are properly applied:
    //   - init: creates the account
    //   - payer = user: user pays the rent
    //   - space = COUNTER_SIZE: allocates exact bytes needed
    //   - mut on user: required because SOL is deducted

    // =========================================================================
    // Solution 2: Reset instruction — sets count back to 0
    // =========================================================================
    // This resets the counter without destroying the account.
    // The account persists with the same authority.
    // Solidity equivalent:
    //   function reset() public { count = 0; }
    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        // Get mutable reference to the existing counter account.
        let counter = &mut ctx.accounts.counter;

        // Set count back to zero.
        counter.count = 0;

        msg!("Counter reset to 0");
        Ok(())
    }

    // =========================================================================
    // Solution 3: Authority-validated increment
    // =========================================================================
    // Only the authority (set during initialize) can call this.
    // The `has_one` constraint on IncrementAuth validates the signer.
    // Solidity equivalent:
    //   function increment() public onlyOwner { count += 1; }
    pub fn increment_auth(ctx: Context<IncrementAuth>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!(
            "Authority {} incremented counter to: {}",
            ctx.accounts.authority.key(),
            counter.count
        );
        Ok(())
    }

    // =========================================================================
    // Solution 4: Space calculation (see GAME_STATE_SIZE constant below)
    // =========================================================================
    // No instruction needed — this exercise is about calculating the constant.
    // The answer: 8 + 32 + 8 + 2 + 1 + (4 + 32) = 87 bytes
    // See GAME_STATE_SIZE below.

    // =========================================================================
    // Solution 5: Safe decrement with custom error
    // =========================================================================
    // Checks for underflow before decrementing.
    // Solidity equivalent:
    //   function decrement() public {
    //       require(count > 0, "Underflow");
    //       count -= 1;
    //   }
    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        // require! checks the condition and returns the error if false.
        // This prevents unsigned integer underflow.
        // The error code (ErrorCode::Underflow) gets returned to the client
        // with its numeric code and message.
        require!(counter.count > 0, ErrorCode::Underflow);

        // Safe to subtract — we know count > 0.
        counter.count -= 1;

        msg!("Decremented to: {}", counter.count);
        Ok(())
    }
}

// =============================================================================
// Constants
// =============================================================================

// Counter account size:
//   8 bytes — Anchor discriminator (SHA-256 hash of "account:Counter", truncated)
//   8 bytes — count (u64, little-endian)
//  32 bytes — authority (Pubkey, 32-byte Ed25519 public key)
// Total: 48 bytes
pub const COUNTER_SIZE: usize = 8 + 8 + 32;

// Solution 4: GameState account size:
//   8 bytes — Anchor discriminator
//  32 bytes — authority (Pubkey)
//   8 bytes — score (u64)
//   2 bytes — level (u16)
//   1 byte  — is_active (bool)
//   4 bytes — player_name length prefix (Borsh encodes String as 4-byte len + bytes)
//  32 bytes — player_name max content (assuming max 32 ASCII characters)
// Total: 87 bytes
pub const GAME_STATE_SIZE: usize = 8 + 32 + 8 + 2 + 1 + (4 + 32);

// =============================================================================
// Account Data Structs
// =============================================================================

// Counter account — stores the count and who controls it.
// The #[account] macro generates:
//   - BorshSerialize/BorshDeserialize for binary encoding
//   - An 8-byte discriminator for type safety
//   - Owner check (must be owned by this program)
#[account]
pub struct Counter {
    // The current count value. u64 gives us 0 to 2^64 - 1.
    pub count: u64,
    // The public key of the authority who can perform privileged operations.
    // Equivalent to Solidity's `address public owner`.
    pub authority: Pubkey,
}

// GameState for Exercise 4 — demonstrates complex space calculation.
#[account]
pub struct GameState {
    pub authority: Pubkey,
    pub score: u64,
    pub level: u16,
    pub is_active: bool,
    pub player_name: String,
}

// =============================================================================
// Account Validation Structs
// =============================================================================

// Solution 1: Complete Initialize struct with all constraints.
#[derive(Accounts)]
pub struct Initialize<'info> {
    // `init` — tells Anchor to create this account via System Program CPI.
    // `payer = user` — the `user` account pays the SOL rent deposit.
    // `space = COUNTER_SIZE` — allocate exactly 48 bytes (8 disc + 8 u64 + 32 Pubkey).
    // After creation, Anchor writes the discriminator and sets the program as owner.
    #[account(init, payer = user, space = COUNTER_SIZE)]
    pub counter: Account<'info, Counter>,

    // `mut` — user's SOL balance decreases (pays rent), so it must be mutable.
    // `Signer` — verifies this account signed the transaction.
    // Combined: this is a mutable, verified signer — like msg.sender who pays gas.
    #[account(mut)]
    pub user: Signer<'info>,

    // Required by the `init` constraint — Anchor calls create_account on this program.
    // `Program<'info, System>` verifies the account is actually the System Program.
    pub system_program: Program<'info, System>,
}

// Basic Increment — no authority check, anyone can increment.
#[derive(Accounts)]
pub struct Increment<'info> {
    // `mut` — we're modifying the count field, so the account must be writable.
    // `Account<'info, Counter>` checks: owned by this program + valid discriminator.
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

// Solution 2: Reset accounts struct.
// Same shape as Increment — we just need mutable access to overwrite count.
#[derive(Accounts)]
pub struct Reset<'info> {
    // `mut` — we're writing count = 0.
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

// Solution 3: Authority-validated increment.
// Uses `has_one` constraint for access control.
#[derive(Accounts)]
pub struct IncrementAuth<'info> {
    // `has_one = authority` — Anchor checks that counter.authority == authority.key().
    // If they don't match, the transaction fails with a ConstraintHasOne error.
    // This is Anchor's equivalent of Solidity's:
    //   modifier onlyOwner() { require(msg.sender == owner); _; }
    // `mut` — we're modifying count.
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,

    // The authority signer. Anchor verifies:
    //   1. This account signed the transaction (Signer check).
    //   2. This account's key matches counter.authority (has_one check).
    // Together, only the original initializer can call increment_auth.
    pub authority: Signer<'info>,
}

// Solution 5: Decrement accounts struct.
#[derive(Accounts)]
pub struct Decrement<'info> {
    // `mut` — we're modifying count.
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

// =============================================================================
// Custom Errors
// =============================================================================
// Solution 5: Error enum with Underflow variant.
//
// #[error_code] generates:
//   - Error codes starting at 6000 (Anchor convention)
//   - The #[msg] string is attached to each variant
//   - Clients receive the code + message for display
//
// Solidity equivalent:
//   error Underflow();
//   // or: require(count > 0, "Cannot decrement: counter is already at zero");
#[error_code]
pub enum ErrorCode {
    // Error code 6000: returned when decrement is called with count == 0.
    #[msg("Cannot decrement: counter is already at zero")]
    Underflow,
}
