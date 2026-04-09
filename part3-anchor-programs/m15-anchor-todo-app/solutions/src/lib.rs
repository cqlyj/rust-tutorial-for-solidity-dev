// Module 15 Solutions: Full CRUD Todo App in Anchor
//
// Every line is commented to explain its purpose.

// Import the Anchor prelude, providing all essential types and macros.
use anchor_lang::prelude::*;

// Declare the on-chain program ID (placeholder for tutorial purposes).
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// The #[program] macro marks this module as the Anchor program entry point.
// Every public function becomes a callable on-chain instruction.
#[program]
pub mod todo_app_solutions {
    // Import all items from the parent scope into this module.
    use super::*;

    // initialize_list: Creates the user's TodoList PDA account.
    // Called once per user before they can add any todos.
    pub fn initialize_list(ctx: Context<InitializeList>) -> Result<()> {
        // Get a mutable reference to the newly created TodoList account.
        let todo_list = &mut ctx.accounts.todo_list;

        // Store the signer's pubkey as the list owner.
        todo_list.authority = ctx.accounts.authority.key();

        // Start the running index counter at zero.
        todo_list.last_idx = 0;

        // Log for debugging and indexer consumption.
        msg!("TodoList initialized for {}", todo_list.authority);

        // Return Ok to indicate success.
        Ok(())
    }

    // ===================================================================
    // EXERCISE 2 SOLUTION: Implement the `add_todo` instruction handler.
    // ===================================================================
    pub fn add_todo(ctx: Context<AddTodo>, content: String) -> Result<()> {
        // Validate that the content does not exceed the 256-byte maximum.
        // This prevents Borsh serialization from overflowing the account space.
        require!(content.len() <= 256, TodoError::ContentTooLong);

        // Validate that the content is not an empty string.
        // An empty todo has no practical value and wastes rent.
        require!(!content.is_empty(), TodoError::ContentEmpty);

        // Get a mutable reference to the user's TodoList metadata account.
        let todo_list = &mut ctx.accounts.todo_list;

        // Get a mutable reference to the newly created TodoItem PDA account.
        let todo_item = &mut ctx.accounts.todo_item;

        // Set the item's owner to the transaction signer.
        todo_item.authority = ctx.accounts.authority.key();

        // Assign the current running index to this item.
        // The index is used as part of the PDA seed, ensuring uniqueness.
        todo_item.idx = todo_list.last_idx;

        // Store the user-provided content string in the account.
        todo_item.content = content;

        // New todos start as not completed.
        todo_item.completed = false;

        // Increment the running counter using checked_add to prevent overflow.
        // If overflow would occur, return the MaxTodosReached error.
        todo_list.last_idx = todo_list
            .last_idx
            .checked_add(1)
            .ok_or(TodoError::MaxTodosReached)?;

        // Log the creation for debugging.
        msg!(
            "Todo #{} added for {}",
            todo_item.idx,
            todo_item.authority
        );

        // Return Ok to indicate success.
        Ok(())
    }

    // toggle_todo: Flips the completed boolean on a TodoItem.
    pub fn toggle_todo(ctx: Context<ToggleTodo>) -> Result<()> {
        // Get a mutable reference to the existing TodoItem.
        let todo_item = &mut ctx.accounts.todo_item;

        // Flip the completed status: true becomes false, false becomes true.
        todo_item.completed = !todo_item.completed;

        // Log the new state.
        msg!(
            "Todo #{} toggled to {}",
            todo_item.idx,
            todo_item.completed
        );

        // Return success.
        Ok(())
    }

    // update_todo: Replaces the content string of an existing TodoItem.
    pub fn update_todo(ctx: Context<UpdateTodo>, new_content: String) -> Result<()> {
        // Enforce the same 256-byte max as add_todo.
        require!(new_content.len() <= 256, TodoError::ContentTooLong);

        // Enforce non-empty content.
        require!(!new_content.is_empty(), TodoError::ContentEmpty);

        // Get a mutable reference to the TodoItem.
        let todo_item = &mut ctx.accounts.todo_item;

        // Overwrite the content with the new value.
        todo_item.content = new_content;

        // Log the update.
        msg!("Todo #{} updated", todo_item.idx);

        // Return success.
        Ok(())
    }

    // remove_todo: Closes the TodoItem account and reclaims rent to authority.
    // The actual close is handled by the `close = authority` constraint.
    pub fn remove_todo(ctx: Context<RemoveTodo>) -> Result<()> {
        // Log which item is being removed (before close zeroes the data).
        msg!(
            "Todo #{} removed for {}",
            ctx.accounts.todo_item.idx,
            ctx.accounts.authority.key()
        );

        // No additional logic needed — the close constraint handles everything.

        // Return success.
        Ok(())
    }

    // ===================================================================
    // EXERCISE 4 SOLUTION: add_todo_with_deadline instruction.
    //
    // Like add_todo, but includes a deadline (i64 Unix timestamp) that
    // must be in the future.
    // ===================================================================
    pub fn add_todo_with_deadline(
        ctx: Context<AddTodoWithDeadline>,
        content: String,
        deadline: i64,
    ) -> Result<()> {
        // Validate content length: must not exceed 256 bytes.
        require!(content.len() <= 256, TodoError::ContentTooLong);

        // Validate content is not empty.
        require!(!content.is_empty(), TodoError::ContentEmpty);

        // Get the current on-chain clock to compare against the deadline.
        let clock = Clock::get()?;

        // Validate that the deadline is in the future.
        // clock.unix_timestamp is the current slot's Unix timestamp.
        require!(deadline > clock.unix_timestamp, TodoError::DeadlineInPast);

        // Get mutable references to the TodoList and TodoItemWithDeadline accounts.
        let todo_list = &mut ctx.accounts.todo_list;
        let todo_item = &mut ctx.accounts.todo_item;

        // Set the item's owner to the signer.
        todo_item.authority = ctx.accounts.authority.key();

        // Assign the current running index.
        todo_item.idx = todo_list.last_idx;

        // Store the content string.
        todo_item.content = content;

        // New items start as not completed.
        todo_item.completed = false;

        // Store the validated deadline timestamp.
        todo_item.deadline = deadline;

        // Increment the running counter with overflow protection.
        todo_list.last_idx = todo_list
            .last_idx
            .checked_add(1)
            .ok_or(TodoError::MaxTodosReached)?;

        // Log the creation including the deadline.
        msg!(
            "Todo #{} added with deadline {} for {}",
            todo_item.idx,
            todo_item.deadline,
            todo_item.authority
        );

        // Return success.
        Ok(())
    }

    // ===================================================================
    // EXERCISE 5 SOLUTION: transfer_todo instruction.
    //
    // Changes the authority of a TodoItem from the current owner to a
    // new pubkey. Only the current owner can initiate this.
    // ===================================================================
    pub fn transfer_todo(ctx: Context<TransferTodo>) -> Result<()> {
        // Get a mutable reference to the TodoItem whose ownership will change.
        let todo_item = &mut ctx.accounts.todo_item;

        // Get the new authority's pubkey from the accounts struct.
        let new_authority_key = ctx.accounts.new_authority.key();

        // Validate that the new authority is different from the current one.
        // Transferring to yourself is a no-op and likely a mistake.
        require!(
            new_authority_key != todo_item.authority,
            TodoError::SameAuthority
        );

        // Log the transfer details before changing ownership.
        msg!(
            "Todo #{} transferred from {} to {}",
            todo_item.idx,
            todo_item.authority,
            new_authority_key
        );

        // Update the authority field to the new owner's pubkey.
        todo_item.authority = new_authority_key;

        // Return success.
        Ok(())
    }
}

// ===========================================================================
// Account data structures
// ===========================================================================

// TodoList: Per-user metadata account tracking the owner and next item index.
// Space: 8 (discriminator) + 32 (Pubkey) + 8 (u64) = 48 bytes.
#[account]
pub struct TodoList {
    // The wallet address that owns this todo list.
    pub authority: Pubkey, // 32 bytes

    // Running counter for unique PDA derivation; never decremented.
    pub last_idx: u64, // 8 bytes
}

// Pre-calculated space constant for TodoList accounts.
const TODO_LIST_SIZE: usize = 8 + 32 + 8; // = 48

// ===========================================================================
// EXERCISE 1 SOLUTION: TodoItem account struct with correct space.
//
// Space breakdown:
//   8   bytes — Anchor discriminator (SHA-256 hash of "account:TodoItem")
//   32  bytes — authority (Pubkey)
//   8   bytes — idx (u64)
//   4   bytes — String length prefix (Borsh encodes length as u32)
//   256 bytes — max content (we enforce this limit in validation)
//   1   byte  — completed (bool)
//   ─────────
//   309 bytes total
// ===========================================================================
#[account]
pub struct TodoItem {
    // The wallet that owns this todo item.
    pub authority: Pubkey, // 32 bytes

    // The sequential index within the user's list.
    pub idx: u64, // 8 bytes

    // The text content (max 256 UTF-8 bytes).
    pub content: String, // 4 (length prefix) + up to 256 bytes

    // Whether the todo has been marked complete.
    pub completed: bool, // 1 byte
}

// Correct space: 8 + 32 + 8 + 4 + 256 + 1 = 309.
const TODO_ITEM_SIZE: usize = 8 + 32 + 8 + 4 + 256 + 1; // = 309

// ===========================================================================
// EXERCISE 4 SOLUTION (cont.): TodoItemWithDeadline struct.
//
// Same as TodoItem plus an i64 deadline field.
// Space: 309 (TodoItem) + 8 (i64) = 317 bytes.
// ===========================================================================
#[account]
pub struct TodoItemWithDeadline {
    // The wallet that owns this todo item.
    pub authority: Pubkey, // 32 bytes

    // The sequential index within the user's list.
    pub idx: u64, // 8 bytes

    // The text content (max 256 UTF-8 bytes).
    pub content: String, // 4 + up to 256 bytes

    // Whether the todo has been marked complete.
    pub completed: bool, // 1 byte

    // Unix timestamp deadline; must be in the future at creation time.
    pub deadline: i64, // 8 bytes
}

// Correct space: 8 + 32 + 8 + 4 + 256 + 1 + 8 = 317.
const TODO_ITEM_WITH_DEADLINE_SIZE: usize = 8 + 32 + 8 + 4 + 256 + 1 + 8; // = 317

// ===========================================================================
// Instruction account validation structs
// ===========================================================================

// InitializeList: Creates a new TodoList PDA for the user.
#[derive(Accounts)]
pub struct InitializeList<'info> {
    // The TodoList PDA to create.
    // `init` creates the account; `payer` pays rent; `space` sets size.
    // Seeds derive a deterministic address from "todo-list" and the user pubkey.
    #[account(
        init,
        payer = authority,
        space = TODO_LIST_SIZE,
        seeds = [b"todo-list", authority.key().as_ref()],
        bump
    )]
    pub todo_list: Account<'info, TodoList>,

    // The user creating the list; must sign and pay.
    #[account(mut)]
    pub authority: Signer<'info>,

    // System Program required by the `init` constraint to create the account.
    pub system_program: Program<'info, System>,
}

// AddTodo: Creates a new TodoItem PDA linked to the user's list.
#[derive(Accounts)]
pub struct AddTodo<'info> {
    // The user's TodoList; must exist and be owned by the signer.
    // `has_one = authority` ensures todo_list.authority == authority.key().
    #[account(
        mut,
        seeds = [b"todo-list", authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub todo_list: Account<'info, TodoList>,

    // The new TodoItem PDA to create.
    // Seeds include the current last_idx for uniqueness.
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

    // The signer who owns the list and pays rent for the new item.
    #[account(mut)]
    pub authority: Signer<'info>,

    // System Program for account creation.
    pub system_program: Program<'info, System>,
}

// ToggleTodo: Modifies the completed flag on an existing TodoItem.
#[derive(Accounts)]
pub struct ToggleTodo<'info> {
    // The TodoItem to toggle; `has_one` verifies the signer is the owner.
    #[account(mut, has_one = authority)]
    pub todo_item: Account<'info, TodoItem>,

    // The owner; must be the signer.
    pub authority: Signer<'info>,
}

// UpdateTodo: Modifies the content of an existing TodoItem.
#[derive(Accounts)]
pub struct UpdateTodo<'info> {
    // The TodoItem to update; ownership verified via `has_one`.
    #[account(mut, has_one = authority)]
    pub todo_item: Account<'info, TodoItem>,

    // The owner; must be the signer.
    pub authority: Signer<'info>,
}

// ===========================================================================
// EXERCISE 3 SOLUTION: RemoveTodo accounts struct with close constraint.
//
// The `close = authority` constraint:
//   1. Transfers all lamports from todo_item to authority
//   2. Zeroes the account data (prevents revival attacks)
//   3. Sets the owner to the System Program
// ===========================================================================
#[derive(Accounts)]
pub struct RemoveTodo<'info> {
    // The TodoItem to close. Three constraints:
    //   mut:              we modify (zero) the account data
    //   has_one:          verify the signer is the owner
    //   close = authority: transfer lamports and delete the account
    #[account(
        mut,
        has_one = authority,
        close = authority
    )]
    pub todo_item: Account<'info, TodoItem>,

    // The owner who receives the reclaimed rent SOL. Must be mutable
    // because their lamport balance increases when the account closes.
    #[account(mut)]
    pub authority: Signer<'info>,
}

// AddTodoWithDeadline: Creates a TodoItemWithDeadline PDA.
#[derive(Accounts)]
pub struct AddTodoWithDeadline<'info> {
    // The user's TodoList; verified with seeds and has_one.
    #[account(
        mut,
        seeds = [b"todo-list", authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub todo_list: Account<'info, TodoList>,

    // The new TodoItemWithDeadline PDA.
    // Uses a different seed prefix ("todo-item-dl") to avoid collision
    // with regular TodoItem PDAs.
    #[account(
        init,
        payer = authority,
        space = TODO_ITEM_WITH_DEADLINE_SIZE,
        seeds = [
            b"todo-item-dl",
            authority.key().as_ref(),
            &todo_list.last_idx.to_le_bytes()
        ],
        bump
    )]
    pub todo_item: Account<'info, TodoItemWithDeadline>,

    // The signer who owns the list and pays rent.
    #[account(mut)]
    pub authority: Signer<'info>,

    // System Program for account creation.
    pub system_program: Program<'info, System>,
}

// ===========================================================================
// EXERCISE 5 SOLUTION (cont.): TransferTodo accounts struct.
//
// Allows the current owner to transfer a TodoItem to a new authority.
// The new_authority is an unchecked AccountInfo because we only need
// its pubkey — we don't read or write any data from it.
// ===========================================================================
#[derive(Accounts)]
pub struct TransferTodo<'info> {
    // The TodoItem whose ownership will change.
    // `has_one = authority` ensures the current signer is the owner.
    #[account(mut, has_one = authority)]
    pub todo_item: Account<'info, TodoItem>,

    // The current owner; must sign the transaction.
    pub authority: Signer<'info>,

    // The new owner. We use AccountInfo (unchecked) because we only
    // need the pubkey — no data is read from this account.
    /// CHECK: new authority can be any valid pubkey; no data is read from this account.
    pub new_authority: AccountInfo<'info>,
}

// ===========================================================================
// Custom error codes
// ===========================================================================
#[error_code]
pub enum TodoError {
    // Raised when content exceeds 256 bytes.
    #[msg("Todo content exceeds the 256-byte maximum.")]
    ContentTooLong, // 6000

    // Raised when content is an empty string.
    #[msg("Todo content must not be empty.")]
    ContentEmpty, // 6001

    // Raised if the last_idx counter would overflow u64.
    #[msg("Maximum number of todos reached.")]
    MaxTodosReached, // 6002

    // Raised when the deadline timestamp is not in the future.
    #[msg("Deadline must be in the future.")]
    DeadlineInPast, // 6003

    // Raised when trying to transfer a todo to the same authority.
    #[msg("New authority must differ from the current authority.")]
    SameAuthority, // 6004
}
