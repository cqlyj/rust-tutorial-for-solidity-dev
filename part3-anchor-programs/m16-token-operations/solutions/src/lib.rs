// Solutions for Module 16 exercises.
// Every line is commented to explain the what and why.

// Import the Anchor prelude — gives us all derive macros, types, and traits.
use anchor_lang::prelude::*;

// Import SPL token CPI helpers and account types.
// `token::mint_to` and `token::transfer` are the CPI wrapper functions.
// `MintTo`, `Transfer`, `Burn` are the account structs those helpers expect.
// `Mint`, `Token`, `TokenAccount` are Anchor's typed account wrappers.
use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount, Transfer};

// Import the Associated Token Account program type for ATA creation.
use anchor_spl::associated_token::AssociatedToken;

// Program ID placeholder. Replace with your deployed program address.
declare_id!("11111111111111111111111111111111");

// The #[program] module — each pub fn is an on-chain instruction.
#[program]
pub mod token_solutions {
    use super::*;

    // =========================================================================
    // Solution 1: Mint Tokens (account struct is the main exercise)
    // =========================================================================
    //
    // The instruction body is the same as the exercise — the real task was
    // writing the MintTokens account struct below.
    pub fn exercise1_mint(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        // Build the accounts struct for the Token Program's MintTo instruction.
        let cpi_accounts = MintTo {
            // The mint whose supply will increase.
            mint: ctx.accounts.mint.to_account_info(),
            // The token account that will receive the newly minted tokens.
            to: ctx.accounts.token_account.to_account_info(),
            // The mint authority — Token Program checks this matches mint.mint_authority.
            authority: ctx.accounts.authority.to_account_info(),
        };

        // Get a reference to the Token Program for the CPI target.
        let cpi_program = ctx.accounts.token_program.to_account_info();

        // Build the CPI context. `new` means the authority is a regular signer
        // (not a PDA). If it were a PDA, we'd use `new_with_signer`.
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Execute the CPI — calls Token Program's MintTo instruction.
        // The `?` propagates errors (like Solidity's require/revert).
        token::mint_to(cpi_ctx, amount)?;

        // Log for debugging / transaction history.
        msg!("Minted {} tokens to {}", amount, ctx.accounts.token_account.key());
        Ok(())
    }

    // =========================================================================
    // Solution 2: Transfer Tokens
    // =========================================================================
    //
    // Solidity equivalent: implementing the body of ERC-20 `transfer(to, amount)`.
    // Instead of directly modifying storage (_balances[from] -= amount),
    // we CPI into the Token Program which does the bookkeeping.
    pub fn exercise2_transfer(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        // Build the accounts struct for Token Program's Transfer instruction.
        // This is analogous to `_balances[from] -= amount; _balances[to] += amount;`
        // in Solidity, except a separate program handles the actual mutation.
        let cpi_accounts = Transfer {
            // Source token account — its `amount` field will decrease.
            from: ctx.accounts.from_ata.to_account_info(),
            // Destination token account — its `amount` field will increase.
            to: ctx.accounts.to_ata.to_account_info(),
            // The owner of the source account. Must be a signer on the transaction.
            // In Solidity, this is implicit via `msg.sender`.
            authority: ctx.accounts.owner.to_account_info(),
        };

        // Reference to the Token Program — the CPI target.
        let cpi_program = ctx.accounts.token_program.to_account_info();

        // Build the CPI context with a regular signer (not a PDA).
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Execute the transfer CPI. The Token Program verifies:
        //   - `authority` matches `from.owner`
        //   - `from.amount >= amount`
        //   - `from.mint == to.mint` (can't mix token types)
        token::transfer(cpi_ctx, amount)?;

        msg!(
            "Transferred {} tokens from {} to {}",
            amount,
            ctx.accounts.from_ata.key(),
            ctx.accounts.to_ata.key()
        );
        Ok(())
    }

    // =========================================================================
    // Solution 3: Create ATA and Transfer in One Instruction
    // =========================================================================
    //
    // In Solidity, sending tokens to a new address "just works" because the
    // mapping entry is created implicitly. On Solana, the recipient's token
    // account must exist before tokens can arrive. This instruction creates
    // the ATA (via Anchor's `init` constraint) and transfers in one step.
    pub fn exercise3_create_ata_and_transfer(
        ctx: Context<CreateAtaAndTransfer>,
        amount: u64,
    ) -> Result<()> {
        // At this point, Anchor has already created the recipient's ATA via
        // the `init` + `associated_token::*` constraints. We just need to
        // do the transfer.

        // Build the Transfer CPI accounts.
        let cpi_accounts = Transfer {
            // Sender's existing token account — will be debited.
            from: ctx.accounts.sender_ata.to_account_info(),
            // Recipient's newly created ATA — will be credited.
            to: ctx.accounts.recipient_ata.to_account_info(),
            // The sender must sign to authorize the transfer.
            authority: ctx.accounts.sender.to_account_info(),
        };

        // CPI into the Token Program.
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        msg!(
            "Created ATA for {} and transferred {} tokens",
            ctx.accounts.recipient.key(),
            amount
        );
        Ok(())
    }

    // =========================================================================
    // Solution 4: Fee-on-Transfer
    // =========================================================================
    //
    // Like fee-on-transfer tokens in Solidity (SafeMoon pattern), but the fee
    // logic lives in our program rather than in the token contract itself.
    // We split the transfer into two CPIs: one for the net amount to the
    // recipient, one for the fee to the collector.
    pub fn exercise4_fee_transfer(ctx: Context<FeeTransfer>, amount: u64) -> Result<()> {
        // Calculate the 1% fee. Integer division rounds down.
        // `max(1)` ensures a minimum fee of 1 token unit if amount > 0.
        // In Solidity: `uint256 fee = amount / 100; if (fee == 0 && amount > 0) fee = 1;`
        let fee = (amount / 100).max(1);

        // Net amount the recipient receives after fee deduction.
        // `checked_sub` returns None on underflow — we convert to an Anchor error.
        // In Solidity, SafeMath would revert on underflow.
        let net_amount = amount.checked_sub(fee).ok_or(ProgramError::ArithmeticOverflow)?;

        // --- CPI 1: Transfer net amount to recipient ---
        // This is the main transfer, minus the fee.
        let transfer_to_recipient = Transfer {
            from: ctx.accounts.sender_ata.to_account_info(),
            to: ctx.accounts.recipient_ata.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(
            CpiContext::new(cpi_program.clone(), transfer_to_recipient),
            net_amount,
        )?;

        // --- CPI 2: Transfer fee to fee collector ---
        // A second CPI from the same sender to the fee collection account.
        let transfer_fee = Transfer {
            from: ctx.accounts.sender_ata.to_account_info(),
            to: ctx.accounts.fee_collector_ata.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        };
        token::transfer(
            CpiContext::new(cpi_program, transfer_fee),
            fee,
        )?;

        msg!(
            "Fee transfer: {} net to recipient, {} fee to collector",
            net_amount,
            fee
        );
        Ok(())
    }

    // =========================================================================
    // Solution 5: Token Faucet with PDA Authority
    // =========================================================================
    //
    // A faucet anyone can call to mint up to 1000 tokens (with 6 decimals).
    // The mint authority is a PDA owned by this program — like a Solidity
    // contract that holds the owner/admin role on an ERC-20.
    //
    // Key concept: `CpiContext::new_with_signer` lets a PDA "sign" the CPI,
    // equivalent to a contract calling another contract in Solidity where
    // `msg.sender` becomes the calling contract's address.
    pub fn exercise5_faucet(ctx: Context<FaucetMint>, amount: u64) -> Result<()> {
        // Enforce the maximum drip amount: 1000 tokens * 10^6 decimals.
        // In Solidity: `require(amount <= 1000 * 10**6, "Too much");`
        require!(
            amount <= 1_000_000_000,
            FaucetError::ExceedsMaxDrip
        );

        // Build the PDA signer seeds. The PDA was derived from:
        //   seeds = [b"faucet", mint.key()]
        // We add the bump so that `invoke_signed` can reconstruct the valid PDA.
        // This is like a contract proving its identity when calling another contract.
        let mint_key = ctx.accounts.mint.key();
        let seeds = &[
            b"faucet".as_ref(),
            mint_key.as_ref(),
            &[ctx.bumps.faucet_authority],
        ];
        // `signer_seeds` is a slice of seed slices — one per PDA signer.
        // We only have one PDA signing this CPI.
        let signer_seeds = &[&seeds[..]];

        // Build the MintTo CPI accounts.
        let cpi_accounts = MintTo {
            // The mint whose supply will increase.
            mint: ctx.accounts.mint.to_account_info(),
            // The requester's token account that will receive the tokens.
            to: ctx.accounts.token_account.to_account_info(),
            // The PDA that is the mint authority. It "signs" via seeds.
            authority: ctx.accounts.faucet_authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        // Use `new_with_signer` because the authority is a PDA, not a wallet.
        // The runtime will verify the seeds produce the PDA's address.
        // In Solidity terms: the contract IS the msg.sender when calling out.
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Execute the mint CPI.
        token::mint_to(cpi_ctx, amount)?;

        msg!("Faucet dispensed {} tokens to {}", amount, ctx.accounts.token_account.key());
        Ok(())
    }
}

// =============================================================================
// Solution 1: MintTokens Account Struct
// =============================================================================
//
// This is the main deliverable for Exercise 1. Each field is an account that
// the Token Program's MintTo instruction needs.
#[derive(Accounts)]
pub struct MintTokens<'info> {
    // The mint account. Mutable because its `supply` field will increase
    // when new tokens are created.
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    // The token account to receive the minted tokens. Must be:
    //   - Mutable: its `amount` field will increase
    //   - For the correct mint: `token::mint = mint` prevents someone from
    //     passing a token account for a different token
    //
    // In Solidity, this is like checking that you're updating the right
    // mapping in the right contract. Here, we verify explicitly.
    #[account(
        mut,
        token::mint = mint,
    )]
    pub token_account: Account<'info, TokenAccount>,

    // The mint authority — must sign the transaction.
    // The Token Program itself will verify this key matches `mint.mint_authority`.
    // Mutable because the payer's lamport balance might change if fees apply.
    #[account(mut)]
    pub authority: Signer<'info>,

    // The SPL Token Program. We CPI into this program to execute the mint.
    pub token_program: Program<'info, Token>,
}

// =============================================================================
// Solution 2: TransferTokens Account Struct (was provided in exercises)
// =============================================================================
#[derive(Accounts)]
pub struct TransferTokens<'info> {
    // The mint, for validation. Not mutated during a transfer.
    pub mint: Account<'info, Mint>,

    // Source token account. Must be:
    //   - For the correct mint (prevents mixing token types)
    //   - Owned by the signer (prevents unauthorized transfers)
    #[account(
        mut,
        token::mint = mint,
        token::authority = owner,
    )]
    pub from_ata: Account<'info, TokenAccount>,

    // Destination token account. Must be for the same mint.
    // We don't check authority because anyone can receive tokens.
    #[account(
        mut,
        token::mint = mint,
    )]
    pub to_ata: Account<'info, TokenAccount>,

    // The owner of the source account. Must sign to authorize the transfer.
    pub owner: Signer<'info>,

    // The SPL Token Program.
    pub token_program: Program<'info, Token>,
}

// =============================================================================
// Solution 3: CreateAtaAndTransfer Account Struct
// =============================================================================
//
// This struct both creates an ATA for the recipient AND validates the
// sender's existing accounts for the transfer.
#[derive(Accounts)]
pub struct CreateAtaAndTransfer<'info> {
    // The mint, for reference. Both ATAs must be for this mint.
    pub mint: Account<'info, Mint>,

    // Sender's existing ATA. Must be for the correct mint and owned by sender.
    #[account(
        mut,
        token::mint = mint,
        token::authority = sender,
    )]
    pub sender_ata: Account<'info, TokenAccount>,

    // Recipient's ATA — CREATED by this instruction.
    // `init` tells Anchor to allocate the account.
    // `payer = sender` means the sender pays the rent-exempt lamports.
    // `associated_token::mint = mint` and `associated_token::authority = recipient`
    // derive the deterministic ATA address: PDA(recipient, TOKEN_PROGRAM, mint).
    //
    // In Solidity, the equivalent would be if `transfer()` had to first call
    // `createBalanceSlot(to)` before writing to `_balances[to]`. On Solana,
    // account creation is an explicit, paid operation.
    #[account(
        init,
        payer = sender,
        associated_token::mint = mint,
        associated_token::authority = recipient,
    )]
    pub recipient_ata: Account<'info, TokenAccount>,

    // The sender's wallet. Signs and pays for the ATA creation.
    #[account(mut)]
    pub sender: Signer<'info>,

    // The recipient's wallet. Does NOT need to sign — you can create an ATA
    // for someone else and send them tokens without their involvement.
    // `/// CHECK:` tells Anchor we've manually verified this is safe.
    /// CHECK: Recipient wallet address — no data read, used only for ATA derivation.
    pub recipient: UncheckedAccount<'info>,

    // System Program — required for allocating the new ATA account.
    pub system_program: Program<'info, System>,

    // Token Program — required for initializing the token account.
    pub token_program: Program<'info, Token>,

    // Associated Token Program — required for deriving the ATA address.
    pub associated_token_program: Program<'info, AssociatedToken>,
}

// =============================================================================
// Solution 4: FeeTransfer Account Struct (was provided in exercises)
// =============================================================================
#[derive(Accounts)]
pub struct FeeTransfer<'info> {
    // The mint, for validation.
    pub mint: Account<'info, Mint>,

    // Sender's token account — total `amount` is debited from here.
    #[account(
        mut,
        token::mint = mint,
        token::authority = sender,
    )]
    pub sender_ata: Account<'info, TokenAccount>,

    // Recipient's token account — receives (amount - fee).
    #[account(
        mut,
        token::mint = mint,
    )]
    pub recipient_ata: Account<'info, TokenAccount>,

    // Fee collector's token account — receives the fee.
    // This could be a protocol treasury, a DAO, etc.
    #[account(
        mut,
        token::mint = mint,
    )]
    pub fee_collector_ata: Account<'info, TokenAccount>,

    // The sender. Signs to authorize both transfers.
    pub sender: Signer<'info>,

    // The SPL Token Program.
    pub token_program: Program<'info, Token>,
}

// =============================================================================
// Solution 5: FaucetMint Account Struct (was provided in exercises)
// =============================================================================
#[derive(Accounts)]
pub struct FaucetMint<'info> {
    // The mint whose tokens the faucet dispenses. Mutable because supply increases.
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    // The requester's token account. Receives the faucet drip.
    // Must be for the correct mint.
    #[account(
        mut,
        token::mint = mint,
    )]
    pub token_account: Account<'info, TokenAccount>,

    // The PDA that serves as the mint authority. Derived from:
    //   seeds = ["faucet", mint_pubkey]
    //   program_id = this program
    //
    // In Solidity, this is like a contract address that has been granted
    // the minter role on an ERC-20. The PDA "is" our program's identity
    // for signing CPIs.
    //
    // `bump` tells Anchor to store the bump in ctx.bumps for use in signer seeds.
    #[account(
        seeds = [b"faucet", mint.key().as_ref()],
        bump,
    )]
    /// CHECK: PDA used as mint authority — validated by seeds constraint above.
    pub faucet_authority: UncheckedAccount<'info>,

    // The user calling the faucet. Pays transaction fees but doesn't need
    // token authority — the PDA handles that.
    #[account(mut)]
    pub payer: Signer<'info>,

    // The SPL Token Program.
    pub token_program: Program<'info, Token>,
}

// =============================================================================
// Custom error for the faucet
// =============================================================================
//
// Anchor's #[error_code] generates an enum that maps to program-specific error
// codes. Clients see the message in transaction logs and can match on the code.
// In Solidity: `error ExceedsMaxDrip();` or `require(amount <= max, "Exceeds max drip");`
#[error_code]
pub enum FaucetError {
    // Error code 6000 (Anchor starts custom errors at 6000).
    // Returned when someone tries to mint more than the per-call limit.
    #[msg("Requested amount exceeds the maximum faucet drip of 1,000,000,000 (1000 tokens with 6 decimals)")]
    ExceedsMaxDrip,
}
