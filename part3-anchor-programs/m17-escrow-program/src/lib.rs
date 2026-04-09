// =============================================================================
// Module 17: Escrow Program — The Capstone
// =============================================================================
//
// This is a trustless token escrow on Solana. Two parties swap SPL tokens
// without trusting each other. The program holds tokens in a vault controlled
// by a PDA — similar to how a Solidity contract holds ERC-20 balances, but
// with explicit account ownership instead of implicit balance mappings.
//
// Three instructions:
//   make   — Maker creates escrow and deposits Token A into a vault
//   take   — Taker sends Token B to maker, receives Token A from vault
//   cancel — Maker reclaims Token A from vault
//
// This program ties together every concept from Modules 1–16:
//   - Rust ownership & borrowing (M1–M4)
//   - Error handling with custom enums (M5)
//   - Traits and derive macros (M6)
//   - Anchor program structure (M13)
//   - Account constraints and validation (M14)
//   - CRUD patterns (M15)
//   - SPL Token CPIs (M16)
// =============================================================================

// Import the Anchor framework prelude — gives us all macros, traits, and types
// we need: #[program], #[derive(Accounts)], Account, Signer, etc.
// (Recall from M13: this single import replaces dozens of native Solana imports.)
use anchor_lang::prelude::*;

// Import the Associated Token Account program — we use this to create
// deterministic token account addresses (like how CREATE2 gives deterministic
// contract addresses in Solidity).
use anchor_spl::associated_token::AssociatedToken;

// Import SPL Token types — these let us interact with token accounts and
// perform token transfers via CPI (Cross-Program Invocation).
// (Recall from M16: SPL Token is a separate program, like calling an external
// ERC-20 contract in Solidity.)
use anchor_spl::token_interface::{
    close_account,       // CPI to close a token account and reclaim its rent
    transfer_checked,    // CPI to transfer tokens with decimal verification
    CloseAccount,        // Account struct for the close_account CPI
    Mint,                // Deserialized mint account (stores decimals, supply, etc.)
    TokenAccount,        // Deserialized token account (stores balance, owner, mint)
    TokenInterface,      // The token program itself (SPL Token or Token-2022)
    TransferChecked,     // Account struct for the transfer_checked CPI
};

// Every Solana program needs a unique ID. In a real Anchor project, this comes
// from `anchor keys list` after your first build. For this tutorial module, we
// use a placeholder. (Like a contract's deployed address in Solidity, except
// you declare it before deployment.)
declare_id!("Esc1111111111111111111111111111111111111111");

// =============================================================================
// Program Module — Contains all instruction handlers
// =============================================================================
//
// The #[program] macro transforms this module into a Solana program.
// Each public function becomes an instruction that clients can call.
// (Recall from M13: Anchor auto-generates instruction routing, so you don't
// need a manual match/dispatch like in native Solana programs.)
#[program]
pub mod escrow {
    // Bring everything from the parent module into scope.
    // This is needed because #[program] creates a new module boundary.
    use super::*;

    // =========================================================================
    // MAKE — Create an escrow and deposit tokens into the vault
    // =========================================================================
    //
    // This is the first step of the escrow flow. The maker:
    //   1. Creates an escrow PDA account (stores the deal terms)
    //   2. Creates a vault token account (will hold the escrowed tokens)
    //   3. Transfers their Token A from their ATA into the vault
    //
    // Parameters:
    //   seed          — A unique u64 so one maker can have multiple escrows
    //                   (like a nonce — prevents PDA collisions)
    //   amount_offered — How many Token A tokens the maker is depositing
    //   amount_wanted  — How many Token B tokens the maker wants in return
    //
    // Solidity equivalent: constructor() + deposit() combined into one call.
    // On Solana, we create accounts and deposit in a single atomic transaction.
    pub fn make(
        ctx: Context<Make>,      // The validated account struct (see Make below)
        seed: u64,               // Unique seed for this escrow's PDA
        amount_offered: u64,     // How much Token A to deposit
        amount_wanted: u64,      // How much Token B to request
    ) -> Result<()> {
        // --- Validation ---
        // Prevent zero-amount escrows. This is a business logic check that
        // Anchor constraints can't express. In Solidity, you'd use require().
        // (Recall from M5: Result<()> and the ? operator for error propagation.)
        require!(amount_offered > 0, EscrowError::InvalidAmount);
        require!(amount_wanted > 0, EscrowError::InvalidAmount);

        // --- Initialize Escrow State ---
        // Write the deal terms into the escrow PDA account.
        // `ctx.accounts.escrow` was just created by the `init` constraint
        // (see the Make struct below). We're filling in its fields.
        //
        // This is like setting constructor storage variables in Solidity:
        //   maker = msg.sender;
        //   tokenA = _tokenA;
        //   amountA = _amountA;
        //
        // (Recall from M3: we access struct fields with dot notation.
        //  Recall from M14: `set_inner` replaces all fields at once.)
        ctx.accounts.escrow.set_inner(Escrow {
            // Store the maker's pubkey so we can verify identity on cancel/take.
            // Like storing `maker = msg.sender` in Solidity.
            maker: ctx.accounts.maker.key(),

            // Store which token mint the maker deposited. Used to validate
            // the vault mint in take/cancel. Like `tokenA = _tokenA`.
            mint_a: ctx.accounts.mint_a.key(),

            // Store which token mint the maker wants. Used to validate what
            // the taker sends. Like `tokenB = _tokenB`.
            mint_b: ctx.accounts.mint_b.key(),

            // How many tokens are in the vault (and will go to taker).
            amount_offered,

            // How many tokens the taker must send to the maker.
            amount_wanted,

            // Store the seed used to derive this PDA. We need it later to
            // reconstruct the PDA signer seeds in take/cancel.
            seed,

            // Store the PDA bump — the extra byte Anchor found to make the
            // seeds hash to a valid PDA (off the ed25519 curve).
            // Storing it avoids recomputing it on every instruction, saving
            // ~1000 compute units. (Recall from M12: PDA bump derivation.)
            bump: ctx.bumps.escrow,
        });

        // --- Transfer Tokens to Vault ---
        // CPI (Cross-Program Invocation) to the SPL Token program to move
        // tokens from the maker's ATA into the vault.
        //
        // In Solidity: tokenA.transferFrom(maker, address(this), amountA)
        // On Solana: we call the token program with the right accounts.
        //
        // We use `transfer_checked` instead of `transfer` because it verifies
        // the decimal count matches the mint, preventing decimal mismatch bugs.
        // (Recall from M16: always prefer transfer_checked for safety.)
        transfer_checked(
            // Build the CPI context: which accounts the token program needs.
            // CpiContext::new() takes the program to call + the account struct.
            CpiContext::new(
                // The token program we're calling — passed in as an account
                // so the runtime can verify it's the real SPL Token program.
                ctx.accounts.token_program.to_account_info(),

                // The accounts for the transfer_checked instruction:
                TransferChecked {
                    // Source: maker's Token A account (where tokens come FROM).
                    from: ctx.accounts.maker_ata_a.to_account_info(),

                    // The mint account — transfer_checked reads this to verify
                    // the decimal count. It's a safety feature: you explicitly
                    // say "I know this is a 6-decimal token" and the program
                    // confirms it matches.
                    mint: ctx.accounts.mint_a.to_account_info(),

                    // Destination: the vault token account (where tokens go TO).
                    // After this transfer, the vault holds the escrowed tokens.
                    to: ctx.accounts.vault.to_account_info(),

                    // Authority: who's authorizing this transfer? The maker,
                    // because the tokens are coming FROM the maker's account.
                    // The maker must have signed this transaction.
                    // (Like msg.sender approving a transferFrom in Solidity.)
                    authority: ctx.accounts.maker.to_account_info(),
                },
            ),
            amount_offered,                     // How many tokens to transfer
            ctx.accounts.mint_a.decimals,       // Expected decimal places
        )?; // The ? propagates any error (insufficient balance, wrong mint, etc.)

        // If we get here, everything succeeded. The escrow is created and funded.
        Ok(())
    }

    // =========================================================================
    // TAKE — Taker completes the swap (atomic exchange)
    // =========================================================================
    //
    // The taker fulfills the escrow by:
    //   1. Sending amount_wanted of Token B to the maker
    //   2. Receiving amount_offered of Token A from the vault
    //   3. Closing the vault (rent → maker)
    //   4. Closing the escrow PDA (rent → maker)
    //
    // This all happens atomically — if step 1 fails (taker doesn't have enough
    // Token B), steps 2-4 never happen. This is like Solidity's transaction
    // atomicity: if any require() fails, everything reverts.
    //
    // Key difference from Solidity: in Solidity, the contract calls
    // token.transfer() and is implicitly authorized. Here, the escrow PDA
    // must sign the transfer from the vault using its seeds.
    pub fn take(ctx: Context<Take>) -> Result<()> {
        // --- Step 1: Taker sends Token B to Maker ---
        // CPI: transfer Token B from taker's ATA to maker's ATA.
        //
        // In Solidity: tokenB.transferFrom(taker, maker, amountB)
        //
        // The taker is the authority because tokens come from THEIR account.
        transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    // Source: taker's Token B account
                    from: ctx.accounts.taker_ata_b.to_account_info(),

                    // Mint B — for decimal verification
                    mint: ctx.accounts.mint_b.to_account_info(),

                    // Destination: maker's Token B account (maker receives payment)
                    to: ctx.accounts.maker_ata_b.to_account_info(),

                    // Authority: taker signs because it's their tokens being sent
                    authority: ctx.accounts.taker.to_account_info(),
                },
            ),
            ctx.accounts.escrow.amount_wanted,  // Amount stored when escrow was created
            ctx.accounts.mint_b.decimals,        // Decimal verification
        )?;

        // --- Step 2: Vault sends Token A to Taker ---
        // CPI: transfer Token A from vault to taker's ATA.
        //
        // THIS IS THE KEY PATTERN: the escrow PDA signs this transfer.
        // The vault's authority is the escrow PDA, so only the PDA can
        // authorize transfers out of the vault. We provide the PDA's seeds
        // so the Solana runtime can verify: sha256(seeds) == PDA address.
        //
        // In Solidity: tokenA.transfer(taker, amountA)
        // (Where the contract is implicitly authorized.)
        //
        // We use CpiContext::new_with_signer() instead of CpiContext::new()
        // because we need the PDA to sign. The signer_seeds prove to the
        // runtime that our program controls this PDA.

        // Build the signer seeds. These must exactly match the seeds used
        // to derive the escrow PDA in the account constraints.
        // (Recall from M12: PDA = sha256(seeds + program_id), bump ensures
        // the result is off the ed25519 curve.)
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"escrow",                                    // Static prefix
            ctx.accounts.escrow.maker.as_ref(),           // Maker's pubkey bytes
            &ctx.accounts.escrow.seed.to_le_bytes(),      // Seed as little-endian bytes
            &[ctx.accounts.escrow.bump],                  // The bump byte
        ]];

        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    // Source: the vault (holds the escrowed Token A)
                    from: ctx.accounts.vault.to_account_info(),

                    // Mint A — for decimal verification
                    mint: ctx.accounts.mint_a.to_account_info(),

                    // Destination: taker's Token A account (taker receives tokens)
                    to: ctx.accounts.taker_ata_a.to_account_info(),

                    // Authority: the escrow PDA (owner of the vault).
                    // Even though the PDA has no private key, the signer_seeds
                    // prove that our program created it, so the runtime allows
                    // the transfer.
                    authority: ctx.accounts.escrow.to_account_info(),
                },
                signer_seeds, // PDA signs via seeds — the "keyless signature"
            ),
            ctx.accounts.escrow.amount_offered, // All tokens in the vault
            ctx.accounts.mint_a.decimals,        // Decimal verification
        )?;

        // --- Step 3: Close the vault token account ---
        // Now that the vault is empty, close it to reclaim the rent-exempt
        // SOL back to the maker. This is good practice — don't leave empty
        // accounts on-chain wasting rent.
        //
        // The escrow PDA must sign this too, since it's the vault's authority.
        // (In Solidity, there's no equivalent — contracts don't have "rent".)
        close_account(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                // The account to close
                account: ctx.accounts.vault.to_account_info(),

                // Where to send the reclaimed rent-exempt SOL
                destination: ctx.accounts.maker.to_account_info(),

                // Authority over the vault — the escrow PDA
                authority: ctx.accounts.escrow.to_account_info(),
            },
            signer_seeds, // Same PDA signer seeds
        ))?;

        // Step 4: Close the escrow PDA account — handled by the `close`
        // constraint in the Take account struct. Anchor automatically
        // transfers the escrow PDA's lamports to the maker and zeroes
        // the account data. (See `close = maker` in the Take struct below.)

        Ok(())
    }

    // =========================================================================
    // CANCEL — Maker reclaims their tokens
    // =========================================================================
    //
    // The maker changes their mind and wants their tokens back:
    //   1. Transfer Token A from vault → maker's ATA
    //   2. Close the vault (rent → maker)
    //   3. Close the escrow PDA (rent → maker, via `close` constraint)
    //
    // Security: the `has_one = maker` constraint on the escrow account ensures
    // only the original maker can call this. The `maker` account must also be
    // a signer. (In Solidity: require(msg.sender == maker).)
    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        // Build PDA signer seeds — same pattern as in `take`.
        // We need the PDA to sign because it's the vault's authority.
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"escrow",                                    // Static prefix
            ctx.accounts.escrow.maker.as_ref(),           // Maker's pubkey
            &ctx.accounts.escrow.seed.to_le_bytes(),      // Seed bytes
            &[ctx.accounts.escrow.bump],                  // Bump byte
        ]];

        // --- Step 1: Return tokens from vault to maker ---
        // CPI: transfer all Token A from vault back to maker's ATA.
        // The escrow PDA signs because it's the vault's authority.
        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    // Source: the vault (holds maker's deposited Token A)
                    from: ctx.accounts.vault.to_account_info(),

                    // Mint A — for decimal verification
                    mint: ctx.accounts.mint_a.to_account_info(),

                    // Destination: back to maker's Token A account
                    to: ctx.accounts.maker_ata_a.to_account_info(),

                    // Authority: escrow PDA (owns the vault)
                    authority: ctx.accounts.escrow.to_account_info(),
                },
                signer_seeds, // PDA signs
            ),
            ctx.accounts.escrow.amount_offered, // Return ALL deposited tokens
            ctx.accounts.mint_a.decimals,        // Decimal verification
        )?;

        // --- Step 2: Close the vault ---
        // Reclaim vault rent-exempt SOL back to the maker.
        close_account(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                // The empty vault to close
                account: ctx.accounts.vault.to_account_info(),

                // Rent goes back to the maker
                destination: ctx.accounts.maker.to_account_info(),

                // PDA authorizes the close
                authority: ctx.accounts.escrow.to_account_info(),
            },
            signer_seeds,
        ))?;

        // Step 3: Close the escrow PDA — handled by `close = maker` constraint
        // in the Cancel account struct below. Anchor does this automatically.

        Ok(())
    }
}

// =============================================================================
// Escrow State Account
// =============================================================================
//
// This is the data stored in the escrow PDA account. It represents the "deal"
// — who created it, what tokens are involved, and how much.
//
// In Solidity, these would be state variables in the Escrow contract.
// On Solana, they're serialized into a dedicated account using Borsh encoding.
//
// #[account] derives:
//   - BorshSerialize / BorshDeserialize (binary encoding, like abi.encode)
//   - AccountSerialize / AccountDeserialize (Anchor's wrapper)
//   - Owner (sets the owner to this program's ID)
//   - Discriminator (8-byte prefix for account type identification)
//
// (Recall from M6: derive macros auto-implement traits.
//  Recall from M13: #[account] is Anchor's magic derive.)
#[account]
pub struct Escrow {
    // The maker's wallet address. Used to:
    //   - Verify identity on cancel (has_one = maker)
    //   - Send Token B to during take
    //   - Return rent to on close
    // 32 bytes (Pubkey is 32 bytes on Solana, like address is 20 bytes on EVM)
    pub maker: Pubkey,

    // Mint address of the token the maker deposited (Token A).
    // Used to validate vault mint in take/cancel.
    // In Solidity: IERC20 public tokenA;
    pub mint_a: Pubkey,

    // Mint address of the token the maker wants (Token B).
    // Used to validate what the taker sends.
    // In Solidity: IERC20 public tokenB;
    pub mint_b: Pubkey,

    // How many Token A the maker deposited into the vault.
    // This is what the taker will receive.
    // In Solidity: uint256 public amountA;
    // (u64 because SPL Token amounts are u64, not u256 like EVM.)
    pub amount_offered: u64,

    // How many Token B the maker wants from the taker.
    // In Solidity: uint256 public amountB;
    pub amount_wanted: u64,

    // A unique seed chosen by the maker to distinguish multiple escrows.
    // Without this, a maker could only have one active escrow (since PDA
    // derivation would always produce the same address for the same maker).
    // Think of it as a nonce or ID.
    pub seed: u64,

    // The PDA bump — stored so we don't have to recompute it.
    // (Recall from M12: find_program_address returns (address, bump),
    // and storing the bump saves ~1000 compute units per instruction.)
    pub bump: u8,
}

// Space calculation for the Escrow account.
// Anchor needs to know how much space to allocate when creating the account.
// (Recall from M14: the `space` parameter in the `init` constraint.)
//
// Layout:
//   8 bytes  — Anchor discriminator (account type tag, always 8 bytes)
//   32 bytes — maker (Pubkey)
//   32 bytes — mint_a (Pubkey)
//   32 bytes — mint_b (Pubkey)
//   8 bytes  — amount_offered (u64)
//   8 bytes  — amount_wanted (u64)
//   8 bytes  — seed (u64)
//   1 byte   — bump (u8)
//   ─────────
//   129 bytes total
impl Escrow {
    // DISCRIMINATOR (8) + Pubkey (32) * 3 + u64 (8) * 3 + u8 (1)
    pub const INIT_SPACE: usize = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 1;
}

// =============================================================================
// Make — Account Struct (Account Validation)
// =============================================================================
//
// #[derive(Accounts)] tells Anchor to generate account deserialization and
// validation code. Each field has constraints that Anchor checks BEFORE your
// instruction handler runs. If any constraint fails, the transaction is
// rejected — your handler code never executes.
//
// This is like Solidity modifiers (onlyOwner, nonReentrant) but more powerful
// because it validates account relationships, not just msg.sender.
//
// (Recall from M14: constraints are Anchor's declarative validation system.)
#[derive(Accounts)]
#[instruction(seed: u64)]
// ^^ This gives us access to instruction parameters in our constraints.
// We need `seed` to derive the escrow PDA. (Recall from M14: #[instruction]
// lets account constraints reference function parameters.)
pub struct Make<'info> {
    // The maker's wallet. Must sign this transaction (hence Signer).
    // `mut` because they pay for account creation (lamports are deducted).
    // In Solidity: msg.sender (but explicitly declared and validated).
    #[account(mut)]
    pub maker: Signer<'info>,

    // The mint for Token A (what the maker is offering).
    // InterfaceAccount<Mint> works with both SPL Token and Token-2022 mints.
    // We don't need `mut` — we're only reading the mint's decimals.
    // In Solidity: IERC20(tokenAAddress) — but here we validate it's a real mint.
    pub mint_a: InterfaceAccount<'info, Mint>,

    // The mint for Token B (what the maker wants in return).
    // Also read-only. Stored in the escrow so takers know what to send.
    pub mint_b: InterfaceAccount<'info, Mint>,

    // The maker's Associated Token Account for Token A.
    // This is where the maker's Token A tokens currently sit.
    // Constraints:
    //   mut — balance will decrease (tokens transferred out)
    //   token::mint = mint_a — must hold Token A (not some other token)
    //   token::authority = maker — must belong to the maker
    //   token::token_program = token_program — matches the token program
    //
    // In Solidity: tokenA.balanceOf(maker) — but validated at the account level.
    #[account(
        mut,
        token::mint = mint_a,
        token::authority = maker,
        token::token_program = token_program,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    // The escrow PDA account — stores the deal terms.
    // Constraints:
    //   init — create this account (allocate space, pay rent, set owner)
    //   payer = maker — maker pays the rent for this account
    //   space = Escrow::INIT_SPACE — how many bytes to allocate
    //   seeds — derive the PDA address from these values:
    //     b"escrow" — static string prefix (namespace)
    //     maker.key() — maker's pubkey (ties escrow to maker)
    //     seed.to_le_bytes() — unique seed (allows multiple escrows per maker)
    //   bump — Anchor finds and stores the PDA bump automatically
    //
    // In Solidity: this is like deploying a new Escrow contract with CREATE2
    // (deterministic address from salt + creator).
    //
    // (Recall from M12: PDAs are deterministic addresses derived from seeds.
    //  Recall from M14: `init` creates and pays for the account.)
    #[account(
        init,
        payer = maker,
        space = Escrow::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    // The vault token account — holds the escrowed Token A.
    // Constraints:
    //   init — create this token account
    //   payer = maker — maker pays rent
    //   associated_token::mint = mint_a — this vault holds Token A
    //   associated_token::authority = escrow — the escrow PDA owns this vault
    //   associated_token::token_program = token_program — matches token program
    //
    // KEY INSIGHT: The vault's authority is the ESCROW PDA, not the maker.
    // This means only the escrow program (via PDA signing) can transfer
    // tokens out of the vault. The maker can't directly withdraw — they
    // must go through the cancel instruction.
    //
    // In Solidity: this is like the contract's internal token balance.
    // But on Solana, it's an explicit account with explicit ownership.
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // Required system programs. Anchor uses these to create accounts and
    // perform token operations. You must pass them explicitly (unlike Solidity
    // where contract creation and token calls are implicit).

    // The Associated Token Account program — knows how to derive and create ATAs.
    pub associated_token_program: Program<'info, AssociatedToken>,

    // The SPL Token program — processes transfer_checked, close_account, etc.
    pub token_program: Interface<'info, TokenInterface>,

    // The System program — handles account creation and SOL transfers.
    // (Like the EVM's built-in account creation, but explicit.)
    pub system_program: Program<'info, System>,
}

// =============================================================================
// Take — Account Struct
// =============================================================================
//
// The taker completes the swap. This is the most complex account struct because
// it involves SIX token accounts (maker's A and B, taker's A and B, vault, and
// the escrow PDA) plus two mints.
//
// Every account is carefully validated to prevent attacks:
//   - Mints must match what the escrow expects
//   - Token accounts must belong to the right owners
//   - The escrow's maker field must match the maker account passed in
#[derive(Accounts)]
pub struct Take<'info> {
    // The taker's wallet. Must sign (they're authorizing Token B transfer).
    // Mutable because they might receive rent from closed accounts
    // (though in this design, rent goes to the maker).
    #[account(mut)]
    pub taker: Signer<'info>,

    // The maker's wallet. NOT a signer — the maker doesn't need to be present
    // for the taker to complete the swap. That's the whole point of an escrow!
    // Mutable because they receive rent from closed accounts.
    //
    // CHECK: This is validated indirectly via `has_one = maker` on the escrow
    // account. The maker pubkey stored in the escrow must match this account.
    /// CHECK: Validated by escrow's has_one constraint.
    #[account(mut)]
    pub maker: UncheckedAccount<'info>,

    // Mint A — the token the maker deposited (and taker will receive).
    pub mint_a: InterfaceAccount<'info, Mint>,

    // Mint B — the token the taker will send to the maker.
    pub mint_b: InterfaceAccount<'info, Mint>,

    // Taker's ATA for Token A — where the taker receives the escrowed tokens.
    // init_if_needed: creates the ATA if it doesn't exist yet. The taker might
    // not have ever held Token A before, so their ATA might not exist.
    // payer = taker: taker pays for their own ATA creation if needed.
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,

    // Taker's ATA for Token B — source of the tokens taker sends to maker.
    // Must hold Token B (token::mint = mint_b) and belong to the taker.
    #[account(
        mut,
        token::mint = mint_b,
        token::authority = taker,
        token::token_program = token_program,
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    // Maker's ATA for Token B — where the maker receives payment from taker.
    // init_if_needed: the maker might not have an ATA for Token B yet.
    // payer = taker: taker pays for this creation too (they're initiating).
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,

    // The escrow PDA — read the deal terms, then close it after the swap.
    // Constraints:
    //   mut — we'll close it
    //   close = maker — when closed, rent goes to the maker
    //   has_one = maker — escrow.maker must equal the maker account above
    //   has_one = mint_a — escrow.mint_a must match the mint_a account above
    //   has_one = mint_b — escrow.mint_b must match the mint_b account above
    //   seeds + bump — verify this is the correct PDA (not a fake account)
    //
    // `has_one` is Anchor's way of saying "this field in the account data must
    // match this account in the instruction." It's like:
    //   require(escrow.maker == maker, "Wrong maker");
    //   require(escrow.mint_a == mint_a, "Wrong mint A");
    //
    // (Recall from M14: has_one, seeds, and bump are constraint superpowers.)
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

    // The vault — holds the escrowed Token A. We'll transfer from it, then close it.
    // associated_token constraints verify it's the correct vault for this escrow.
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // Required programs (same as Make, plus AssociatedToken for init_if_needed).
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

// =============================================================================
// Cancel — Account Struct
// =============================================================================
//
// Simpler than Take — only involves the maker and their Token A.
// No taker, no Token B, no init_if_needed.
#[derive(Accounts)]
pub struct Cancel<'info> {
    // Maker's wallet — must sign (only the maker can cancel).
    // Receives rent from closed accounts.
    #[account(mut)]
    pub maker: Signer<'info>,

    // Mint A — needed for transfer_checked (decimal verification).
    pub mint_a: InterfaceAccount<'info, Mint>,

    // Maker's ATA for Token A — destination for reclaimed tokens.
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    // The escrow PDA — read terms, then close it.
    // has_one = maker ensures only the original maker can cancel.
    // close = maker sends rent back to the maker.
    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = mint_a,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,

    // The vault — transfer tokens out, then close it.
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

// =============================================================================
// Custom Errors
// =============================================================================
//
// Anchor's error system lets us define descriptive errors that clients can
// decode. Each variant gets a unique error code (starting at 6000).
//
// In Solidity: custom errors like `error InvalidAmount();`
// In Anchor: #[error_code] enum with #[msg] descriptions.
//
// (Recall from M5: Rust enums for error types.
//  Recall from M13: Anchor's error_code macro.)
#[error_code]
pub enum EscrowError {
    // Returned when maker tries to create an escrow with zero tokens.
    // A zero-amount escrow is meaningless and wastes rent.
    #[msg("Amount must be greater than zero")]
    InvalidAmount,
}
