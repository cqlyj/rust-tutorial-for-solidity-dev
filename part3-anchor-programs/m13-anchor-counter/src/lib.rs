// =============================================================================
// Module 13: Anchor Counter Program
// =============================================================================
// This is your first complete Anchor program. Every line is commented to explain
// what's happening and what the macros generate behind the scenes.
//
// Solidity equivalent:
//   contract Counter {
//       uint64 public count;
//       function initialize() public { count = 0; }
//       function increment() public { count += 1; }
//       function decrement() public { require(count > 0); count -= 1; }
//   }
// =============================================================================

// Import everything from anchor_lang::prelude.
// This brings in: declare_id!, #[program], #[derive(Accounts)], #[account],
// Context, Result, Account, Signer, Program, System, msg!, require!, and more.
// In Solidity, this is like `import "@openzeppelin/contracts/...";` — one import
// gives you the entire framework.
use anchor_lang::prelude::*;

// declare_id! sets the program's on-chain address (public key).
// In Solidity, this would be like knowing your contract's address at compile time.
// When you run `anchor build`, this gets updated with the actual deployed address.
// The Anchor framework checks that instructions are sent to this program ID.
// Behind the scenes, this generates:
//   pub static ID: Pubkey = Pubkey::new_from_array([...]);
//   pub fn id() -> Pubkey { ID }
//   pub fn check_id(id: &Pubkey) -> bool { *id == ID }
declare_id!("11111111111111111111111111111111");

// =============================================================================
// #[program] — Instruction Handlers
// =============================================================================
// #[program] marks this module as containing the instruction handlers.
// Think of it like the public functions in a Solidity contract.
//
// What Anchor generates from #[program]:
//   1. An `entrypoint` function that the Solana runtime calls.
//   2. A dispatcher that reads the first 8 bytes of instruction data
//      (the "discriminator", like Solidity's 4-byte function selector)
//      and routes to the correct function below.
//   3. For each function: account deserialization before the call,
//      and account serialization after it returns.
//   4. Error handling: if any check fails, the transaction reverts
//      (all account changes are rolled back, like Solidity's revert).
#[program]
pub mod counter {
    // `use super::*` imports everything from the parent scope (the crate root).
    // This is needed because #[program] creates a new module scope, but we need
    // access to our types (Counter, Initialize, etc.) defined outside.
    use super::*;

    // =========================================================================
    // initialize — Create the counter account and set count to 0
    // =========================================================================
    // Solidity equivalent: `function initialize() public { count = 0; }`
    //
    // `ctx: Context<Initialize>` means this function receives pre-validated accounts
    // matching the `Initialize` struct defined below. Before this function body runs,
    // Anchor has already:
    //   1. Verified the `user` signed the transaction (Signer check)
    //   2. Created the `counter` account via System Program CPI (init constraint)
    //   3. Allocated 16 bytes of space (8 discriminator + 8 for u64)
    //   4. Paid rent from `user`'s SOL balance (payer constraint)
    //   5. Set the account owner to this program
    //   6. Written the 8-byte discriminator to the account
    //   7. Deserialized the account data into a Counter struct
    //
    // Returns `Result<()>` — Ok(()) for success, Err for failure.
    // In Solidity, success = normal return, failure = revert.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Access the counter account from the validated context.
        // `&mut` gives us mutable access — we need to write to it.
        // Anchor already deserialized the account data into our Counter struct.
        // The `ctx.accounts` field has type `Initialize` (our Accounts struct).
        let counter = &mut ctx.accounts.counter;

        // Set the initial count to 0.
        // After this function returns, Anchor will automatically serialize
        // the Counter struct back into the account's data bytes using Borsh encoding.
        counter.count = 0;

        // msg! logs a message to the transaction log (visible in explorer/CLI).
        // In Solidity, this is like `emit` for events or `console.log` in Hardhat.
        msg!("Counter initialized! Current count: {}", counter.count);

        // Return Ok(()) to indicate success.
        // In Solidity, this is like returning without reverting.
        // If we returned Err(...), the transaction would revert and all changes
        // would be rolled back — exactly like Solidity's revert.
        Ok(())
    }

    // =========================================================================
    // increment — Add 1 to the counter
    // =========================================================================
    // Solidity equivalent: `function increment() public { count += 1; }`
    //
    // Note: anyone can call this — there's no access control.
    // The Increment accounts struct only requires `counter` to be mutable.
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        // Get mutable reference to the counter account.
        // Anchor already verified that this account is owned by our program
        // and contains a valid Counter discriminator.
        let counter = &mut ctx.accounts.counter;

        // Increment the count by 1.
        // In a production program, you might use checked_add to prevent overflow:
        //   counter.count = counter.count.checked_add(1).unwrap();
        // But for u64 (max: 18,446,744,073,709,551,615), overflow is unlikely.
        counter.count += 1;

        // Log the new count.
        msg!("Counter incremented! Current count: {}", counter.count);

        Ok(())
    }

    // =========================================================================
    // decrement — Subtract 1 from the counter
    // =========================================================================
    // Solidity equivalent:
    //   function decrement() public {
    //       require(count > 0, "Counter: cannot go below zero");
    //       count -= 1;
    //   }
    //
    // This goes beyond the basic Counter tutorial — we add a safety check
    // to prevent underflow, demonstrating Anchor's error handling.
    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        // require! is Anchor's equivalent of Solidity's require().
        // If the condition is false, it returns Err(ErrorCode::BelowZero),
        // which causes the transaction to revert (all changes rolled back).
        // The error code is logged so clients can display a meaningful message.
        require!(counter.count > 0, ErrorCode::BelowZero);

        // Safe to subtract now — we've verified count > 0.
        counter.count -= 1;

        msg!("Counter decremented! Current count: {}", counter.count);

        Ok(())
    }
}

// =============================================================================
// Account Data Structs
// =============================================================================

// #[account] marks this struct as a program account — data that lives on-chain.
// In Solidity, this is like the contract's storage variables.
//
// What #[account] generates:
//   1. `#[derive(BorshSerialize, BorshDeserialize)]` — binary encoding for
//      efficient on-chain storage (like Solidity's ABI encoding but more compact).
//   2. An 8-byte discriminator (SHA-256 of "account:Counter", truncated) —
//      written at the start of the account data. This ensures you can't
//      accidentally deserialize the wrong account type.
//   3. Owner check implementation — Anchor verifies the account is owned by
//      this program before deserializing. Prevents other programs from spoofing.
//   4. `impl AccountSerialize` and `impl AccountDeserialize` — serialization
//      that includes the discriminator.
//
// On-chain layout (16 bytes total):
//   [0..8]   = discriminator (8 bytes, auto-generated hash)
//   [8..16]  = count (8 bytes, u64 in little-endian)
#[account]
pub struct Counter {
    // The counter value. u64 = unsigned 64-bit integer.
    // Equivalent to Solidity's uint64.
    // Takes 8 bytes of on-chain storage.
    pub count: u64,
}

// =============================================================================
// Account Validation Structs (the "Accounts" pattern)
// =============================================================================
// Each instruction has a corresponding Accounts struct that declares:
//   - Which accounts the instruction needs
//   - What type each account is (Account, Signer, Program, etc.)
//   - What constraints to enforce (init, mut, has_one, etc.)
//
// #[derive(Accounts)] generates the validation code. Before your instruction
// handler runs, Anchor processes every field and its constraints.
//
// In Solidity, this is like parameter validation + access control combined:
//   modifier onlyOwner() { require(msg.sender == owner); _; }
//   function transfer(address to, uint amount) public onlyOwner { ... }
// But in Anchor, the validation is declarative, not imperative.

// =========================================================================
// Initialize Accounts — for the `initialize` instruction
// =========================================================================
// The <'info> lifetime parameter is required by Anchor. It means "these
// references borrow from the transaction's account data." Every Accounts
// struct needs this lifetime. Don't overthink it — just always include it.
#[derive(Accounts)]
pub struct Initialize<'info> {
    // The counter account to create and initialize.
    //
    // Constraints:
    //   `init`  — Create this account. Anchor calls system_program.create_account()
    //             behind the scenes. The account must not already exist.
    //   `payer = user` — The `user` account pays the SOL rent for this new account.
    //             In Solidity, this is like msg.sender paying gas + deployment cost.
    //   `space = 8 + 8` — Allocate 16 bytes total:
    //             8 bytes for the Anchor discriminator (always required)
    //             + 8 bytes for our u64 `count` field.
    //             In Solidity, you never specify storage size — the EVM handles it.
    //             On Solana, you must calculate and allocate upfront.
    //
    // Type: Account<'info, Counter>
    //   `Account` is Anchor's typed account wrapper. It:
    //   1. Checks the account is owned by this program.
    //   2. Checks the 8-byte discriminator matches Counter.
    //   3. Deserializes the data into a Counter struct.
    //   After your function runs, it serializes any changes back.
    #[account(init, payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,

    // The user creating (and paying for) the counter.
    //
    // `#[account(mut)]` — marks this account as mutable. Required because
    // SOL will be deducted from this account to pay rent.
    //
    // Type: Signer<'info>
    //   Verifies this account signed the transaction.
    //   In Solidity, this is like checking msg.sender — but explicit.
    //   If this account didn't sign, the transaction fails before your code runs.
    #[account(mut)]
    pub user: Signer<'info>,

    // The System Program — required for account creation.
    //
    // Type: Program<'info, System>
    //   Verifies this is actually the System Program (address 11111...1111).
    //   The `init` constraint calls System Program to create the account.
    //   In Solidity, there's no equivalent — the EVM handles account creation internally.
    //   On Solana, it's a separate program you must explicitly include.
    pub system_program: Program<'info, System>,
}

// =========================================================================
// Increment Accounts — for the `increment` instruction
// =========================================================================
#[derive(Accounts)]
pub struct Increment<'info> {
    // The counter account to modify.
    // `#[account(mut)]` — we need write access to update the count.
    // Anchor automatically verifies:
    //   1. This account is owned by our program (owner check).
    //   2. The discriminator matches Counter (type check).
    //   3. The account is writable in the transaction (mut check).
    //
    // Note: No `init` here — the account already exists.
    // Note: No Signer required — anyone can increment. To restrict access,
    // you'd add a `has_one` constraint (see exercises).
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

// =========================================================================
// Decrement Accounts — for the `decrement` instruction
// =========================================================================
// Same structure as Increment — we just need mutable access to the counter.
// The underflow check happens in the instruction body with require!.
#[derive(Accounts)]
pub struct Decrement<'info> {
    // Mutable counter account — same validation as Increment.
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

// =============================================================================
// Custom Errors
// =============================================================================
// #[error_code] generates error types with unique numeric codes.
// Each variant gets an auto-incremented error code starting from 6000.
// The #[msg("...")] attribute provides a human-readable error message.
//
// In Solidity, this is like:
//   error BelowZero();
//   // or
//   require(count > 0, "Cannot decrement below zero");
//
// Anchor errors are much richer — they include:
//   - A unique numeric code (e.g., 6000)
//   - The error name ("BelowZero")
//   - The message ("Cannot decrement below zero")
//   - File and line number in logs
// Clients can match on the code to display localized messages.
#[error_code]
pub enum ErrorCode {
    #[msg("Cannot decrement below zero")]
    BelowZero,
}
