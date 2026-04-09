// =============================================================================
// Module 13 Exercises: Anchor Counter
// =============================================================================
// Complete each exercise by filling in the TODO sections.
// Run `cargo check` to verify your solutions compile.
// =============================================================================

use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod counter_exercises {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.authority = ctx.accounts.user.key();
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        Ok(())
    }

    // =========================================================================
    // Exercise 1: Complete the Initialize accounts struct
    // =========================================================================
    // The Initialize struct below is missing its account constraints.
    // Add the correct constraints to each field:
    //   - `counter` needs: init, payer, and space (use COUNTER_SIZE constant)
    //   - `user` needs: mut (because they pay rent)
    //   - `system_program` is fine as-is
    //
    // Hint: space should be 8 (discriminator) + 8 (u64 count) + 32 (Pubkey authority)
    // Hint: The COUNTER_SIZE constant is already defined below for you.

    // =========================================================================
    // Exercise 2: Add a `reset` instruction
    // =========================================================================
    // Create a new instruction called `reset` that sets the counter back to 0.
    // Requirements:
    //   - Takes Context<Reset>
    //   - Sets counter.count = 0
    //   - Returns Result<()>
    //   - Log a message with msg!
    //
    // TODO: Write the reset function here
    // pub fn reset(...) -> Result<()> { ... }

    // =========================================================================
    // Exercise 3: Add authority validation to increment
    // =========================================================================
    // The current `increment` function allows anyone to increment.
    // Create a new version called `increment_auth` that only allows the
    // authority (stored in Counter.authority) to increment.
    //
    // Requirements:
    //   - Takes Context<IncrementAuth>
    //   - The IncrementAuth struct should validate that the signer matches
    //     the counter's authority field (use `has_one = authority` constraint)
    //   - Increments counter.count by 1
    //
    // TODO: Write the increment_auth function here
    // pub fn increment_auth(...) -> Result<()> { ... }

    // =========================================================================
    // Exercise 4: Calculate space for a complex account
    // =========================================================================
    // See the GameState struct defined below. Calculate its correct `space`
    // value and assign it to the GAME_STATE_SIZE constant.
    //
    // Fields:
    //   authority: Pubkey    → 32 bytes
    //   score: u64           → 8 bytes
    //   level: u16           → 2 bytes
    //   is_active: bool      → 1 byte
    //   player_name: String  → 4 + max_len (assume max 32 characters)
    //   + 8 bytes for discriminator
    //
    // TODO: Replace 0 with the correct size
    // pub const GAME_STATE_SIZE: usize = 0;

    // =========================================================================
    // Exercise 5: Safe decrement with custom error handling
    // =========================================================================
    // Implement a `decrement` instruction that:
    //   - Checks if count > 0 before decrementing
    //   - Returns a custom error `ErrorCode::Underflow` if count is 0
    //   - Uses require! macro (not a manual if/else)
    //   - Decrements count by 1
    //
    // TODO: Write the decrement function here
    // pub fn decrement(...) -> Result<()> { ... }
}

// Account size constant for Exercise 1
// 8 (discriminator) + 8 (count: u64) + 32 (authority: Pubkey)
pub const COUNTER_SIZE: usize = 8 + 8 + 32;

// Counter account with an authority field (used by exercises 1, 2, 3, 5)
#[account]
pub struct Counter {
    pub count: u64,
    pub authority: Pubkey,
}

// ---------------------------------------------------------------------------
// Exercise 1: Complete this struct by adding #[account(...)] constraints.
// ---------------------------------------------------------------------------
// Currently the constraints are missing. Add them:
//   - counter: #[account(init, payer = user, space = COUNTER_SIZE)]
//   - user: #[account(mut)]
//   - system_program: (no constraint needed, the type handles validation)
#[derive(Accounts)]
pub struct Initialize<'info> {
    // TODO: Add constraint: #[account(init, payer = user, space = COUNTER_SIZE)]
    #[account(init, payer = user, space = COUNTER_SIZE)]
    pub counter: Account<'info, Counter>,

    // TODO: Add constraint: #[account(mut)]
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// Accounts struct for the basic increment
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

// ---------------------------------------------------------------------------
// Exercise 2: Create the Reset accounts struct.
// ---------------------------------------------------------------------------
// Hint: You just need mutable access to the counter.
// It should look very similar to Increment.

// TODO: Uncomment and complete:
// #[derive(Accounts)]
// pub struct Reset<'info> {
//     ...
// }

// ---------------------------------------------------------------------------
// Exercise 3: Create the IncrementAuth accounts struct.
// ---------------------------------------------------------------------------
// This needs:
//   - counter: Account<'info, Counter> with `mut` and `has_one = authority`
//   - authority: Signer<'info>
//
// The `has_one = authority` constraint checks that
// counter.authority == authority.key()
//
// In Solidity, this is like:
//   modifier onlyOwner() { require(msg.sender == owner); _; }

// TODO: Uncomment and complete:
// #[derive(Accounts)]
// pub struct IncrementAuth<'info> {
//     ...
// }

// ---------------------------------------------------------------------------
// Exercise 4: GameState struct for space calculation.
// ---------------------------------------------------------------------------
#[account]
pub struct GameState {
    pub authority: Pubkey,
    pub score: u64,
    pub level: u16,
    pub is_active: bool,
    pub player_name: String,
}

// TODO: Calculate the correct size:
// pub const GAME_STATE_SIZE: usize = ???;

// ---------------------------------------------------------------------------
// Exercise 5: Custom error enum
// ---------------------------------------------------------------------------
// Add an `Underflow` variant for when decrement is called at count = 0.

// TODO: Uncomment and complete:
// #[error_code]
// pub enum ErrorCode {
//     ...
// }
