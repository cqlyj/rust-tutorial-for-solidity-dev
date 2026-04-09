use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount, Transfer};
use anchor_spl::associated_token::AssociatedToken;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod token_exercises {
    use super::*;

    // =========================================================================
    // Exercise 1: Write the MintTokens Account Struct
    // =========================================================================
    //
    // The instruction handler is provided. Your job is to write the `MintTokens`
    // account struct below (look for the TODO).
    //
    // Requirements:
    //   - `mint`: mutable Mint account
    //   - `token_account`: mutable TokenAccount, constrained to the correct mint
    //   - `authority`: mutable signer (the mint authority)
    //   - `token_program`: the SPL Token Program
    //
    // Hint: Use `token::mint = mint` constraint on the token_account.
    // Think about it like this: in Solidity, the ERC-20 contract inherently
    // knows which token it manages. On Solana, we must explicitly verify that
    // the token account belongs to the correct mint.
    pub fn exercise1_mint(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }

    // =========================================================================
    // Exercise 2: Implement a Transfer Instruction Handler
    // =========================================================================
    //
    // The `TransferTokens` account struct is provided below. Your job is to
    // implement the instruction handler body.
    //
    // Requirements:
    //   - Build a `Transfer` CPI accounts struct with from, to, authority
    //   - Create a `CpiContext` pointing to the token program
    //   - Call `token::transfer` with the context and amount
    //
    // Solidity comparison: This is like implementing the body of ERC-20's
    // `transfer(address to, uint256 amount)` function, except instead of
    // modifying storage directly, you CPI into the Token Program.
    pub fn exercise2_transfer(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        // TODO: Build CPI accounts, create context, call token::transfer
        // Hint: Look at how mint_tokens builds its CPI context above.
        //
        // The Transfer struct needs:
        //   from:      ctx.accounts.from_ata.to_account_info()
        //   to:        ctx.accounts.to_ata.to_account_info()
        //   authority: ctx.accounts.owner.to_account_info()
        todo!("Implement transfer CPI")
    }

    // =========================================================================
    // Exercise 3: Create ATA and Transfer in One Instruction
    // =========================================================================
    //
    // Write both the instruction handler AND the account struct for
    // `CreateAtaAndTransfer`.
    //
    // This instruction should:
    //   1. Create an Associated Token Account for the recipient (if it doesn't exist)
    //   2. Transfer tokens from the sender's ATA to the newly created ATA
    //
    // In Solidity, you never worry about "creating" a balance slot — the mapping
    // entry just exists. On Solana, someone must create and pay rent for the ATA.
    //
    // Hint: Use `init_if_needed` on the recipient's token account with
    //   `associated_token::mint` and `associated_token::authority` constraints.
    //   You'll also need the `associated_token_program` and `system_program`.
    //
    // Note: `init_if_needed` requires enabling the feature in Cargo.toml:
    //   anchor-lang = { version = "0.30", features = ["init-if-needed"] }
    //   We've left it out for safety — in practice, you'd add it.
    //   For this exercise, use `init` instead (assume ATA doesn't exist yet).
    pub fn exercise3_create_ata_and_transfer(
        ctx: Context<CreateAtaAndTransfer>,
        amount: u64,
    ) -> Result<()> {
        // TODO: Build a Transfer CPI and execute it.
        // The ATA creation is handled by Anchor's `init` constraint.
        // You just need to do the transfer.
        todo!("Implement transfer after ATA creation")
    }

    // =========================================================================
    // Exercise 4: Fee-on-Transfer
    // =========================================================================
    //
    // Implement a transfer mechanism that takes a 1% fee.
    //
    // Like fee-on-transfer tokens in Solidity (SafeMoon, etc.), but here
    // the fee logic lives in YOUR program, not in the token contract itself.
    //
    // The account struct `FeeTransfer` is provided. You need to:
    //   1. Calculate the fee (1% of amount, minimum 1 token if amount > 0)
    //   2. Transfer (amount - fee) from sender to recipient
    //   3. Transfer fee from sender to fee_collector
    //
    // Hint: You need TWO separate CPI calls to token::transfer.
    pub fn exercise4_fee_transfer(ctx: Context<FeeTransfer>, amount: u64) -> Result<()> {
        // TODO: Calculate fee, transfer net amount to recipient, transfer fee to collector
        //
        // Step 1: let fee = amount / 100; (1%)
        //         let fee = fee.max(1);   (minimum 1 if amount > 0)
        //         let net_amount = amount - fee;
        //
        // Step 2: CPI transfer of net_amount from sender_ata to recipient_ata
        //
        // Step 3: CPI transfer of fee from sender_ata to fee_collector_ata
        todo!("Implement fee-on-transfer")
    }

    // =========================================================================
    // Exercise 5: Token Faucet
    // =========================================================================
    //
    // Implement a faucet that lets anyone mint up to 1000 tokens per call.
    // The program's PDA is the mint authority (not a user wallet).
    //
    // This is like a Solidity faucet contract that holds token admin rights:
    //   function drip(uint256 amount) external {
    //       require(amount <= 1000 * 10**decimals, "Too much");
    //       token.mint(msg.sender, amount);
    //   }
    //
    // Key difference: the mint authority is a PDA, so you must use
    // `CpiContext::new_with_signer` with the PDA's seeds.
    //
    // The FaucetMint account struct is provided. You need to:
    //   1. Validate that amount <= 1_000_000_000 (1000 tokens with 6 decimals)
    //   2. Build a MintTo CPI using CpiContext::new_with_signer
    //   3. Use seeds: [b"faucet", mint.key().as_ref()] with the bump
    pub fn exercise5_faucet(ctx: Context<FaucetMint>, amount: u64) -> Result<()> {
        // TODO:
        // 1. Check amount <= 1_000_000_000
        // 2. Build seeds for the PDA signer
        // 3. Build MintTo CPI accounts
        // 4. Call token::mint_to with CpiContext::new_with_signer
        //
        // Hint for seeds:
        //   let seeds = &[b"faucet".as_ref(), ctx.accounts.mint.key().as_ref(), &[ctx.bumps.faucet_authority]];
        //   let signer_seeds = &[&seeds[..]];
        todo!("Implement faucet mint with PDA signer")
    }
}

// =============================================================================
// Exercise 1: TODO — Write this struct
// =============================================================================
//
// #[derive(Accounts)]
// pub struct MintTokens<'info> {
//     TODO: Define the accounts needed for minting tokens.
//     You need: mint, token_account, authority, token_program
//     Use the correct constraints on each account.
// }

#[derive(Accounts)]
pub struct MintTokens<'info> {
    // TODO: Add a mutable Mint account
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    // TODO: Add a mutable TokenAccount with token::mint constraint
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    // TODO: Add a mutable signer for the authority
    #[account(mut)]
    pub authority: Signer<'info>,

    // TODO: Add the Token program
    pub token_program: Program<'info, Token>,
}

// =============================================================================
// Exercise 2: TransferTokens struct (provided for you)
// =============================================================================
#[derive(Accounts)]
pub struct TransferTokens<'info> {
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = owner,
    )]
    pub from_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint,
    )]
    pub to_ata: Account<'info, TokenAccount>,

    pub owner: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

// =============================================================================
// Exercise 3: TODO — Write this struct
// =============================================================================
//
// Hint: You need:
//   - mint (immutable, for reference)
//   - sender_ata (mutable, token::mint + token::authority constraints)
//   - recipient_ata (init, payer, associated_token::mint, associated_token::authority)
//   - sender (mutable signer, pays for ATA creation)
//   - recipient (the wallet that will own the new ATA — NOT a signer)
//   - system_program, token_program, associated_token_program
#[derive(Accounts)]
pub struct CreateAtaAndTransfer<'info> {
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = sender,
    )]
    pub sender_ata: Account<'info, TokenAccount>,

    // TODO: Initialize recipient's ATA using associated_token constraints
    // Hint:
    //   #[account(
    //       init,
    //       payer = sender,
    //       associated_token::mint = mint,
    //       associated_token::authority = recipient,
    //   )]
    #[account(
        mut,
        token::mint = mint,
    )]
    pub recipient_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub sender: Signer<'info>,

    /// CHECK: Recipient wallet. Not a signer — anyone can receive tokens.
    pub recipient: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

// =============================================================================
// Exercise 4: FeeTransfer struct (provided for you)
// =============================================================================
#[derive(Accounts)]
pub struct FeeTransfer<'info> {
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = sender,
    )]
    pub sender_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint,
    )]
    pub recipient_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint,
    )]
    pub fee_collector_ata: Account<'info, TokenAccount>,

    pub sender: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

// =============================================================================
// Exercise 5: FaucetMint struct (provided for you)
// =============================================================================
#[derive(Accounts)]
pub struct FaucetMint<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
    )]
    pub token_account: Account<'info, TokenAccount>,

    /// The PDA that serves as the mint authority.
    /// Seeds: ["faucet", mint.key()]
    /// In Solidity, this would be like a contract address that has minting rights.
    #[account(
        seeds = [b"faucet", mint.key().as_ref()],
        bump,
    )]
    /// CHECK: PDA used as mint authority — verified by seeds constraint.
    pub faucet_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
}
