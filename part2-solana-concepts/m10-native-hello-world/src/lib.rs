// Import the types and macros we need from the solana_program crate.
// This is the foundational crate for all native Solana programs.
use solana_program::{
    // AccountInfo is a struct that represents a single Solana account.
    // Every account the instruction touches is passed as an AccountInfo.
    // In Solidity, account data is implicit (contract storage). In Solana, it's explicit.
    account_info::AccountInfo,
    // The entrypoint! macro generates the C-compatible entrypoint function
    // that the Solana runtime calls when an instruction targets this program.
    // Without this, the runtime wouldn't know how to invoke your code.
    entrypoint,
    // ProgramResult is a type alias for Result<(), ProgramError>.
    // Returning Ok(()) means the instruction succeeded.
    // Returning Err(ProgramError::...) means it failed and the transaction rolls back.
    entrypoint::ProgramResult,
    // msg! is a macro for logging messages to the transaction log.
    // It works like println! but writes to Solana's log system instead of stdout.
    // Visible in `solana logs`, block explorers, and test output.
    msg,
    // Pubkey is a 32-byte public key, equivalent to `address` in Solidity.
    // Every account and program on Solana is identified by a Pubkey.
    pubkey::Pubkey,
};

// Register our process_instruction function as the program's entrypoint.
// The macro expands into a C-compatible function that the Solana runtime calls.
// This is analogous to how the EVM knows to call a contract's function selector dispatcher.
entrypoint!(process_instruction);

// This is the main function of our program. Every instruction targeting this program enters here.
// Think of it as the Solidity equivalent of a contract's fallback function, except it's the ONLY
// entry point — there are no named functions. You dispatch manually based on instruction_data.
//
// Parameters:
//   program_id       - The public key of THIS program (like address(this) in Solidity)
//   accounts         - Slice of all accounts the client passed to this instruction
//   instruction_data - Raw bytes of instruction arguments (like msg.data / calldata in Solidity)
//
// Returns:
//   ProgramResult    - Ok(()) on success, Err(ProgramError) on failure (like revert in Solidity)
fn process_instruction(
    // The public key (address) of this program on-chain.
    // Use it to verify account ownership: if account.owner != program_id, reject.
    program_id: &Pubkey,
    // A slice of AccountInfo structs — one for each account the client declared.
    // The client must list every account the instruction reads or writes.
    // Order matters: your program identifies accounts by their index in this slice.
    accounts: &[AccountInfo],
    // Raw instruction data bytes. No ABI encoding, no function selectors.
    // You decide the format. Common pattern: first byte = instruction variant.
    instruction_data: &[u8],
) -> ProgramResult {
    // Log a greeting. This appears in the transaction log.
    // In Solidity, this would be like emitting an event or using Hardhat's console.log.
    msg!("Hello, Solana! This is a native program speaking.");

    // Log the program's own address (public key).
    // Equivalent to logging address(this) in Solidity.
    msg!("Program ID: {}", program_id);

    // Log how many accounts were passed to this instruction.
    // In Solidity, you don't think about this — accounts are implicit.
    // In Solana, the client explicitly passes every account the instruction needs.
    msg!("Number of accounts: {}", accounts.len());

    // Iterate over each account and log its details.
    // This demonstrates the fields available on AccountInfo.
    for (index, account) in accounts.iter().enumerate() {
        // Log the account's public key (its on-chain address).
        // Like logging an address in Solidity.
        msg!("Account {}: key = {}", index, account.key);

        // Log whether this account signed the transaction.
        // This is the Solana equivalent of checking msg.sender in Solidity.
        // If is_signer is true, the account's private key signed the transaction.
        msg!("  is_signer: {}", account.is_signer);

        // Log whether the instruction can modify this account.
        // The client declares writability upfront; the runtime enforces it.
        // There's no direct Solidity equivalent — in Solidity, any storage is writable by the contract.
        msg!("  is_writable: {}", account.is_writable);

        // Log the account's balance in lamports (1 SOL = 1,000,000,000 lamports).
        // Like address.balance in Solidity, but accessed via a RefCell.
        // The ** dereferences: first the Ref from borrow(), then the &mut u64.
        msg!("  lamports: {}", account.lamports());

        // Log the length of the account's data buffer.
        // This is the raw byte array where the account stores its state.
        // In Solidity, this would be like the size of a contract's storage.
        msg!("  data length: {}", account.data_len());

        // Log the owner of this account (which program owns it).
        // In Solidity, ownership is implicit — a contract owns its own storage.
        // In Solana, any program can be assigned as an account's owner.
        // Only the owner program can modify the account's data.
        msg!("  owner: {}", account.owner);
    }

    // Log the instruction data that was passed.
    // This is the raw calldata — equivalent to msg.data in Solidity.
    msg!("Instruction data length: {} bytes", instruction_data.len());

    // If there's any instruction data, log the raw bytes.
    // In a real program, you'd parse this into a structured instruction.
    if !instruction_data.is_empty() {
        // Log the raw bytes as a debug-formatted slice.
        msg!("Instruction data: {:?}", instruction_data);
    }

    // Return success. The transaction will be committed.
    // In Solidity, this is like reaching the end of a function without reverting.
    // If we returned Err(ProgramError::...), the transaction would roll back entirely.
    Ok(())
}
