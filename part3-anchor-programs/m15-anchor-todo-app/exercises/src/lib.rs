// Module 15 Exercises: Full CRUD Todo App in Anchor
//
// Complete the TODOs in each exercise. The program structure is provided;
// you fill in the missing pieces.

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod todo_app_exercises {
    use super::*;

    // This instruction is provided for you — no changes needed.
    pub fn initialize_list(ctx: Context<InitializeList>) -> Result<()> {
        let todo_list = &mut ctx.accounts.todo_list;
        todo_list.authority = ctx.accounts.authority.key();
        todo_list.last_idx = 0;
        msg!("TodoList initialized for {}", todo_list.authority);
        Ok(())
    }

    // ===================================================================
    // EXERCISE 2: Implement the `add_todo` instruction handler.
    //
    // Requirements:
    //   - Validate that content is not empty and not longer than 256 bytes
    //   - Set the todo_item fields: authority, idx, content, completed
    //   - Increment todo_list.last_idx using checked_add
    //   - Return appropriate errors using the TodoError enum
    // ===================================================================
    pub fn add_todo(ctx: Context<AddTodo>, content: String) -> Result<()> {
        // TODO: Validate content length (max 256 bytes)

        // TODO: Validate content is not empty

        // TODO: Get mutable references to todo_list and todo_item

        // TODO: Set todo_item.authority to the signer's key

        // TODO: Set todo_item.idx to todo_list.last_idx

        // TODO: Set todo_item.content to the provided content

        // TODO: Set todo_item.completed to false

        // TODO: Increment todo_list.last_idx with checked_add

        Ok(())
    }

    // This instruction is provided for you — no changes needed.
    pub fn toggle_todo(ctx: Context<ToggleTodo>) -> Result<()> {
        let todo_item = &mut ctx.accounts.todo_item;
        todo_item.completed = !todo_item.completed;
        msg!("Todo #{} toggled to {}", todo_item.idx, todo_item.completed);
        Ok(())
    }

    // This instruction is provided for you — no changes needed.
    pub fn update_todo(ctx: Context<UpdateTodo>, new_content: String) -> Result<()> {
        require!(new_content.len() <= 256, TodoError::ContentTooLong);
        require!(!new_content.is_empty(), TodoError::ContentEmpty);
        let todo_item = &mut ctx.accounts.todo_item;
        todo_item.content = new_content;
        msg!("Todo #{} updated", todo_item.idx);
        Ok(())
    }

    // This instruction is provided for you — no changes needed.
    pub fn remove_todo(ctx: Context<RemoveTodo>) -> Result<()> {
        msg!("Todo #{} removed for {}", ctx.accounts.todo_item.idx, ctx.accounts.authority.key());
        Ok(())
    }

    // ===================================================================
    // EXERCISE 4: Add a `add_todo_with_deadline` instruction.
    //
    // This is like `add_todo`, but the TodoItemWithDeadline struct has
    // an additional `deadline: i64` field (Unix timestamp).
    //
    // Requirements:
    //   - Accept a `deadline: i64` parameter in addition to `content`
    //   - Validate content (same as add_todo)
    //   - Validate that deadline is in the future using Clock::get()?
    //   - Set all fields including the deadline
    //   - Increment todo_list.last_idx
    // ===================================================================
    pub fn add_todo_with_deadline(
        ctx: Context<AddTodoWithDeadline>,
        content: String,
        deadline: i64,
    ) -> Result<()> {
        // TODO: Validate content length (max 256 bytes)

        // TODO: Validate content is not empty

        // TODO: Get the current clock and validate deadline is in the future

        // TODO: Get mutable references to todo_list and todo_item

        // TODO: Set all todo_item fields (authority, idx, content, completed, deadline)

        // TODO: Increment todo_list.last_idx

        Ok(())
    }

    // ===================================================================
    // EXERCISE 5: Add a `transfer_todo` instruction.
    //
    // This instruction transfers ownership of a TodoItem to a new authority.
    //
    // Requirements:
    //   - Change todo_item.authority to the new_authority's key
    //   - Only the current authority can call this
    //   - The new_authority must be a different pubkey than the current one
    // ===================================================================
    pub fn transfer_todo(ctx: Context<TransferTodo>) -> Result<()> {
        // TODO: Get a mutable reference to the todo_item

        // TODO: Validate new_authority is different from current authority

        // TODO: Set todo_item.authority to the new_authority's key

        // TODO: Log the transfer

        Ok(())
    }
}

// ===========================================================================
// Account data structures
// ===========================================================================

// TodoList account: stores per-user metadata. (Provided — no changes needed.)
#[account]
pub struct TodoList {
    pub authority: Pubkey, // 32 bytes
    pub last_idx: u64,     // 8 bytes
}

const TODO_LIST_SIZE: usize = 8 + 32 + 8;

// ===========================================================================
// EXERCISE 1: Define the TodoItem account struct and calculate its space.
//
// Requirements:
//   - authority: Pubkey (32 bytes)   — the owner of this todo
//   - idx: u64 (8 bytes)            — the item's index in the list
//   - content: String (4 + 256)     — the todo text (max 256 bytes)
//   - completed: bool (1 byte)      — whether the todo is done
//
// Calculate the total space including the 8-byte Anchor discriminator.
// Replace the `0` in TODO_ITEM_SIZE with the correct value.
// ===========================================================================
#[account]
pub struct TodoItem {
    // TODO: Define the fields here
    pub authority: Pubkey,
    pub idx: u64,
    pub content: String,
    pub completed: bool,
}

// TODO: Replace 0 with the correct space calculation:
//   8 (discriminator) + 32 (Pubkey) + 8 (u64) + 4 (String prefix) + 256 (max content) + 1 (bool)
const TODO_ITEM_SIZE: usize = 0; // FIX ME

// ===========================================================================
// EXERCISE 4 (cont.): Define the TodoItemWithDeadline account struct.
//
// Same as TodoItem, but with an additional `deadline: i64` field.
// Calculate the total space.
// ===========================================================================
#[account]
pub struct TodoItemWithDeadline {
    pub authority: Pubkey, // 32 bytes
    pub idx: u64,          // 8 bytes
    pub content: String,   // 4 + 256 bytes
    pub completed: bool,   // 1 byte
    // TODO: Add a `deadline` field of type i64
    pub deadline: i64,
}

// TODO: Calculate the space. Hint: it's TODO_ITEM_SIZE + the size of an i64.
const TODO_ITEM_WITH_DEADLINE_SIZE: usize = 0; // FIX ME

// ===========================================================================
// Instruction account validation structs
// ===========================================================================

// InitializeList: Provided — no changes needed.
#[derive(Accounts)]
pub struct InitializeList<'info> {
    #[account(
        init,
        payer = authority,
        space = TODO_LIST_SIZE,
        seeds = [b"todo-list", authority.key().as_ref()],
        bump
    )]
    pub todo_list: Account<'info, TodoList>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// AddTodo: Provided — no changes needed.
#[derive(Accounts)]
pub struct AddTodo<'info> {
    #[account(
        mut,
        seeds = [b"todo-list", authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub todo_list: Account<'info, TodoList>,

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

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// ToggleTodo: Provided — no changes needed.
#[derive(Accounts)]
pub struct ToggleTodo<'info> {
    #[account(mut, has_one = authority)]
    pub todo_item: Account<'info, TodoItem>,

    pub authority: Signer<'info>,
}

// UpdateTodo: Provided — no changes needed.
#[derive(Accounts)]
pub struct UpdateTodo<'info> {
    #[account(mut, has_one = authority)]
    pub todo_item: Account<'info, TodoItem>,

    pub authority: Signer<'info>,
}

// ===========================================================================
// EXERCISE 3: Write the RemoveTodo accounts struct with a `close` constraint.
//
// Requirements:
//   - todo_item must be mutable
//   - todo_item must have `has_one = authority` to verify ownership
//   - todo_item must have `close = authority` to reclaim rent
//   - authority must be a mutable Signer (receives the lamports)
// ===========================================================================
#[derive(Accounts)]
pub struct RemoveTodo<'info> {
    // TODO: Add the todo_item account with mut, has_one, and close constraints
    #[account(mut, has_one = authority, close = authority)]
    pub todo_item: Account<'info, TodoItem>,

    // TODO: Add the authority as a mutable Signer
    #[account(mut)]
    pub authority: Signer<'info>,
}

// AddTodoWithDeadline: accounts struct for Exercise 4.
#[derive(Accounts)]
pub struct AddTodoWithDeadline<'info> {
    #[account(
        mut,
        seeds = [b"todo-list", authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub todo_list: Account<'info, TodoList>,

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

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// ===========================================================================
// EXERCISE 5 (cont.): Write the TransferTodo accounts struct.
//
// Requirements:
//   - todo_item: mutable, has_one = authority (current owner)
//   - authority: Signer (current owner, must sign)
//   - new_authority: just a plain AccountInfo (the new owner, unchecked)
//     Use: `/// CHECK: new authority can be any valid pubkey`
// ===========================================================================
#[derive(Accounts)]
pub struct TransferTodo<'info> {
    // TODO: Add todo_item with mut and has_one = authority
    #[account(mut, has_one = authority)]
    pub todo_item: Account<'info, TodoItem>,

    // TODO: Add authority as Signer
    pub authority: Signer<'info>,

    // TODO: Add new_authority as AccountInfo with a CHECK doc comment
    /// CHECK: new authority can be any valid pubkey; no data is read from this account.
    pub new_authority: AccountInfo<'info>,
}

// ===========================================================================
// Custom errors
// ===========================================================================
#[error_code]
pub enum TodoError {
    #[msg("Todo content exceeds the 256-byte maximum.")]
    ContentTooLong,

    #[msg("Todo content must not be empty.")]
    ContentEmpty,

    #[msg("Maximum number of todos reached.")]
    MaxTodosReached,

    #[msg("Deadline must be in the future.")]
    DeadlineInPast,

    #[msg("New authority must differ from the current authority.")]
    SameAuthority,
}
