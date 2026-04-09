// Import the Anchor framework prelude, which includes all essential types,
// macros, traits, and re-exports needed for Solana program development.
use anchor_lang::prelude::*;

// Declare the on-chain program ID. In production this comes from `anchor keys list`.
// For local development / tutorial purposes we use a placeholder.
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// ---------------------------------------------------------------------------
// Program module – every public function here becomes an on-chain instruction.
// Anchor generates the instruction discriminator (8-byte hash) automatically.
// ---------------------------------------------------------------------------
#[program]
pub mod todo_app {
    // Bring everything from the outer scope into the module.
    use super::*;

    // -----------------------------------------------------------------------
    // initialize_list: Creates the user's TodoList account.
    //
    // Solidity equivalent:
    //   constructor() { owner = msg.sender; nextId = 0; }
    //
    // Key difference: In Solidity the mapping lives inside the contract.
    // On Solana each user gets their own PDA account to store metadata.
    // -----------------------------------------------------------------------
    pub fn initialize_list(ctx: Context<InitializeList>) -> Result<()> {
        // Obtain a mutable reference to the newly created TodoList account.
        let todo_list = &mut ctx.accounts.todo_list;

        // Set the authority to the user who signed the transaction.
        todo_list.authority = ctx.accounts.authority.key();

        // Initialize the running counter to zero; incremented with each add_todo.
        todo_list.last_idx = 0;

        // Emit a log so indexers / explorers can track list creation.
        msg!("TodoList initialized for {}", todo_list.authority);

        // Return success.
        Ok(())
    }

    // -----------------------------------------------------------------------
    // add_todo: Creates a new TodoItem account linked to the user's list.
    //
    // Solidity equivalent:
    //   function addTodo(string memory _content) public { ... }
    //
    // On Solana each item is a separate PDA account derived from the user's
    // pubkey and the item index, allowing O(1) lookup by index.
    // -----------------------------------------------------------------------
    pub fn add_todo(ctx: Context<AddTodo>, content: String) -> Result<()> {
        // Enforce a maximum content length to cap account size and rent cost.
        require!(content.len() <= 256, TodoError::ContentTooLong);

        // Enforce non-empty content.
        require!(!content.is_empty(), TodoError::ContentEmpty);

        // Obtain a mutable reference to the user's TodoList metadata account.
        let todo_list = &mut ctx.accounts.todo_list;

        // Obtain a mutable reference to the newly created TodoItem account.
        let todo_item = &mut ctx.accounts.todo_item;

        // Set the item's authority to the signer (owner of the list).
        todo_item.authority = ctx.accounts.authority.key();

        // Assign the current running index to this item.
        todo_item.idx = todo_list.last_idx;

        // Store the user-provided content string.
        todo_item.content = content;

        // New items are not completed by default.
        todo_item.completed = false;

        // Increment the running counter so the next item gets a unique index.
        todo_list.last_idx = todo_list
            .last_idx
            .checked_add(1)
            .ok_or(TodoError::MaxTodosReached)?;

        // Log the creation for debugging and indexing.
        msg!(
            "Todo #{} added for {}",
            todo_item.idx,
            todo_item.authority
        );

        // Return success.
        Ok(())
    }

    // -----------------------------------------------------------------------
    // toggle_todo: Flips the completed status of a TodoItem.
    //
    // Solidity equivalent:
    //   function toggleTodo(uint256 id) public { todos[id].completed = !todos[id].completed; }
    // -----------------------------------------------------------------------
    pub fn toggle_todo(ctx: Context<ToggleTodo>) -> Result<()> {
        // Obtain a mutable reference to the existing TodoItem account.
        let todo_item = &mut ctx.accounts.todo_item;

        // Flip the boolean: true -> false, false -> true.
        todo_item.completed = !todo_item.completed;

        // Log the new state.
        msg!(
            "Todo #{} completed = {}",
            todo_item.idx,
            todo_item.completed
        );

        // Return success.
        Ok(())
    }

    // -----------------------------------------------------------------------
    // update_todo: Replaces the content of an existing TodoItem.
    //
    // Solidity equivalent:
    //   function updateTodo(uint256 id, string memory _content) public { ... }
    //
    // Note: Because we allocated space for up to 256 bytes of content at
    // account creation, we must enforce the same limit here.
    // -----------------------------------------------------------------------
    pub fn update_todo(ctx: Context<UpdateTodo>, new_content: String) -> Result<()> {
        // Enforce the same maximum length as add_todo.
        require!(new_content.len() <= 256, TodoError::ContentTooLong);

        // Enforce non-empty content.
        require!(!new_content.is_empty(), TodoError::ContentEmpty);

        // Obtain a mutable reference to the TodoItem account.
        let todo_item = &mut ctx.accounts.todo_item;

        // Overwrite the content with the new value.
        todo_item.content = new_content;

        // Log the update.
        msg!("Todo #{} content updated", todo_item.idx);

        // Return success.
        Ok(())
    }

    // -----------------------------------------------------------------------
    // remove_todo: Closes the TodoItem account and returns its rent-exempt
    // lamports back to the authority.
    //
    // Solidity equivalent (loose):
    //   function removeTodo(uint256 id) public { delete todos[id]; }
    //
    // On Solana, closing an account zeroes its data, removes the owner, and
    // transfers the lamports to a designated address – reclaiming rent.
    // -----------------------------------------------------------------------
    pub fn remove_todo(ctx: Context<RemoveTodo>) -> Result<()> {
        // Log which todo is being removed before the account is closed.
        msg!(
            "Todo #{} removed for {}",
            ctx.accounts.todo_item.idx,
            ctx.accounts.authority.key()
        );

        // The actual closing happens via the `close = authority` constraint
        // in the RemoveTodo accounts struct. No additional logic needed here.

        // Return success.
        Ok(())
    }
}

// ===========================================================================
// Account data structures – these define what gets serialized into each PDA.
// ===========================================================================

// ---------------------------------------------------------------------------
// TodoList: Per-user metadata account.
//
// Space calculation:
//   8  (Anchor discriminator)
// + 32 (authority: Pubkey)
// +  8 (last_idx: u64)
// = 48 bytes
// ---------------------------------------------------------------------------
#[account]
pub struct TodoList {
    // The wallet address that owns this todo list.
    pub authority: Pubkey, // 32 bytes

    // Running counter used to derive unique PDA seeds for each TodoItem.
    pub last_idx: u64, // 8 bytes
}

// Constant for TodoList account size: discriminator + authority + last_idx.
const TODO_LIST_SIZE: usize = 8 + 32 + 8;

// ---------------------------------------------------------------------------
// TodoItem: Individual todo entry account.
//
// Space calculation:
//   8   (Anchor discriminator)
// + 32  (authority: Pubkey)
// +  8  (idx: u64)
// +  4  (String length prefix – Borsh encodes length as u32)
// + 256 (max content bytes)
// +  1  (completed: bool)
// = 309 bytes
// ---------------------------------------------------------------------------
#[account]
pub struct TodoItem {
    // The wallet that owns this item (must match TodoList authority).
    pub authority: Pubkey, // 32 bytes

    // The sequential index of this item within the user's list.
    pub idx: u64, // 8 bytes

    // The text content of the todo (max 256 UTF-8 bytes).
    pub content: String, // 4 + up to 256 bytes

    // Whether the todo has been marked as complete.
    pub completed: bool, // 1 byte
}

// Constant for TodoItem account size.
const TODO_ITEM_SIZE: usize = 8 + 32 + 8 + 4 + 256 + 1;

// ===========================================================================
// Instruction account validation structs.
// Each struct defines the accounts an instruction expects, with constraints.
// ===========================================================================

// ---------------------------------------------------------------------------
// InitializeList: Accounts required to create a user's TodoList.
// ---------------------------------------------------------------------------
#[derive(Accounts)]
pub struct InitializeList<'info> {
    // The TodoList PDA account to be created.
    // seeds: deterministic address based on "todo-list" + user pubkey.
    // bump: Anchor finds the canonical bump automatically.
    // payer: the authority pays for rent.
    // space: use our pre-calculated constant.
    #[account(
        init,
        payer = authority,
        space = TODO_LIST_SIZE,
        seeds = [b"todo-list", authority.key().as_ref()],
        bump
    )]
    pub todo_list: Account<'info, TodoList>,

    // The user creating the list; must sign the transaction.
    #[account(mut)]
    pub authority: Signer<'info>,

    // The System Program is required for `init` to create the account.
    pub system_program: Program<'info, System>,
}

// ---------------------------------------------------------------------------
// AddTodo: Accounts required to create a new TodoItem.
// ---------------------------------------------------------------------------
#[derive(Accounts)]
pub struct AddTodo<'info> {
    // The user's TodoList – must exist and be owned by the signer.
    // `has_one = authority` ensures todo_list.authority == authority.key().
    #[account(
        mut,
        seeds = [b"todo-list", authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub todo_list: Account<'info, TodoList>,

    // The new TodoItem PDA to create.
    // The seed includes the current last_idx so every item gets a unique PDA.
    #[account(
        init,
        payer = authority,
        space = TODO_ITEM_SIZE,
        seeds = [
            b"todo-item",
            authority.key().as_ref(),
            &todo_list.last_idx.to_le_bytes()
        ],
        bump
    )]
    pub todo_item: Account<'info, TodoItem>,

    // The signer who owns the list and pays for the new account.
    #[account(mut)]
    pub authority: Signer<'info>,

    // System Program needed for account creation.
    pub system_program: Program<'info, System>,
}

// ---------------------------------------------------------------------------
// ToggleTodo: Accounts required to toggle a TodoItem's completed flag.
// ---------------------------------------------------------------------------
#[derive(Accounts)]
pub struct ToggleTodo<'info> {
    // The TodoItem to modify. `has_one = authority` ensures only the owner
    // can toggle it. `mut` because we write to the account.
    #[account(
        mut,
        has_one = authority
    )]
    pub todo_item: Account<'info, TodoItem>,

    // The owner of the todo item; must be the transaction signer.
    pub authority: Signer<'info>,
}

// ---------------------------------------------------------------------------
// UpdateTodo: Accounts required to change a TodoItem's content.
// ---------------------------------------------------------------------------
#[derive(Accounts)]
pub struct UpdateTodo<'info> {
    // The TodoItem to update. Same ownership check as ToggleTodo.
    #[account(
        mut,
        has_one = authority
    )]
    pub todo_item: Account<'info, TodoItem>,

    // The owner; must sign.
    pub authority: Signer<'info>,
}

// ---------------------------------------------------------------------------
// RemoveTodo: Accounts required to delete (close) a TodoItem.
//
// The `close = authority` constraint zeroes the account data, transfers all
// lamports to `authority`, and removes the account from state.
// ---------------------------------------------------------------------------
#[derive(Accounts)]
pub struct RemoveTodo<'info> {
    // The TodoItem to close. `close = authority` handles the lamport transfer.
    #[account(
        mut,
        has_one = authority,
        close = authority
    )]
    pub todo_item: Account<'info, TodoItem>,

    // The owner who receives the reclaimed rent lamports.
    #[account(mut)]
    pub authority: Signer<'info>,
}

// ===========================================================================
// Custom error codes.
// Each variant maps to a unique error number starting at 6000 (Anchor base).
// ===========================================================================
#[error_code]
pub enum TodoError {
    // Raised when the content string exceeds 256 bytes.
    #[msg("Todo content exceeds the 256-byte maximum.")]
    ContentTooLong,

    // Raised when the content string is empty.
    #[msg("Todo content must not be empty.")]
    ContentEmpty,

    // Raised if last_idx would overflow u64 (extremely unlikely, but safe).
    #[msg("Maximum number of todos reached.")]
    MaxTodosReached,
}
