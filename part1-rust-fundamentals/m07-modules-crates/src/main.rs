// ============================================================
// main.rs — Crate root for m07-modules-crates
// ============================================================
// This is the entry point and the root of the module tree.
// Every top-level module must be declared here with `mod`.
// Think of this as the "manifest" that tells Rust which
// modules exist in this crate.
// ============================================================

// Declare the `wallet` module.
// Rust will look for src/wallet.rs to find this module's code.
mod wallet;

// Declare the `transactions` module.
// Rust will look for src/transactions.rs.
mod transactions;

// Declare the `utils` module.
// Since utils has submodules, Rust looks for src/utils/mod.rs.
mod utils;

// Bring specific items into scope using `use`.
// `crate::` is the absolute path from this crate's root.
// This is like Solidity's: import {Wallet} from "./Wallet.sol";
use crate::wallet::Wallet;

// Import multiple items from the transactions module.
// Curly braces let us import several items at once.
use crate::transactions::{Transaction, TransactionType};

// Import the re-exported functions from utils.
// Because utils/mod.rs has `pub use helpers::format_lamports`,
// we can import directly from utils instead of utils::helpers.
use crate::utils::format_lamports;
use crate::utils::format_sol;

// Import the validation function from utils directly.
use crate::utils::is_valid_address;

// The main function — program entry point.
fn main() {
    // Print a header for the demo.
    println!("=== Module 07: Modules, Crates & Cargo ===\n");

    // ── Section 1: Creating a Wallet ──────────────────────────
    println!("── Creating Wallets ──");

    // Create a new wallet using the Wallet::new associated function.
    // We imported Wallet with `use`, so we don't need the full path.
    let mut alice = Wallet::new("Alice", 5_000_000_000);

    // Print using the Display trait we implemented on Wallet.
    println!("  Created: {}", alice);

    // Access the public `owner` field directly.
    println!("  Owner: {}", alice.owner);

    // Access the private `balance` field through the public getter.
    // We can't write `alice.balance` here — it's private!
    println!("  Balance: {}", format_lamports(alice.balance()));

    // Also show the balance in SOL using our utility function.
    println!("  Balance: {}", format_sol(alice.balance()));

    // Use the pub(crate) method — works because we're in the same crate.
    println!("  Summary: {}", alice.summary());

    // ── Section 2: Deposit ────────────────────────────────────
    println!("\n── Deposit ──");

    // Deposit 2 SOL (in lamports) into Alice's wallet.
    let deposit_amount = 2_000_000_000;
    // Call the deposit method — requires &mut self.
    alice.deposit(deposit_amount);
    // Show the updated balance.
    println!("  Deposited: {}", format_lamports(deposit_amount));
    println!("  New balance: {}", alice);

    // ── Section 3: Transactions ───────────────────────────────
    println!("\n── Executing Transactions ──");

    // Create a Transfer transaction using the Transaction constructor.
    // TransactionType::Transfer holds the recipient and amount.
    let mut tx1 = Transaction::new(
        // The sender's address.
        "Alice",
        // The transaction type with associated data.
        TransactionType::Transfer {
            // Recipient address.
            to: "Bob".to_string(),
            // Amount in lamports (1 SOL).
            amount: 1_000_000_000,
        },
    );
    // Print the pending transaction.
    println!("  Created: {}", tx1);

    // Execute the transaction, passing a mutable reference to the wallet.
    // This demonstrates cross-module interaction: Transaction uses Wallet.
    match tx1.execute(&mut alice) {
        // If execution succeeds, print the success message.
        Ok(msg) => println!("  Result: {}", msg),
        // If execution fails, print the error.
        Err(e) => println!("  Error: {}", e),
    }
    // Show the transaction is now marked as executed.
    println!("  After: {}", tx1);
    // Show updated wallet balance.
    println!("  Wallet: {}", alice);

    // Create and execute a Stake transaction.
    let mut tx2 = Transaction::new(
        // Sender.
        "Alice",
        // Stake 2 SOL.
        TransactionType::Stake {
            amount: 2_000_000_000,
        },
    );
    // Execute the stake transaction.
    match tx2.execute(&mut alice) {
        // Print success message.
        Ok(msg) => println!("  Staked: {}", msg),
        // Print error message.
        Err(e) => println!("  Stake error: {}", e),
    }
    // Show wallet after staking.
    println!("  Wallet after stake: {}", alice);

    // Try to execute the same transaction again — should fail.
    println!("\n── Double-Execution Guard ──");
    match tx2.execute(&mut alice) {
        // This shouldn't happen.
        Ok(msg) => println!("  Unexpected success: {}", msg),
        // Expected: "Transaction already executed".
        Err(e) => println!("  Correctly rejected: {}", e),
    }

    // ── Section 4: Overdraft Protection ───────────────────────
    println!("\n── Overdraft Protection ──");

    // Try to transfer more than the balance — should fail.
    let mut tx3 = Transaction::new(
        // Sender.
        "Alice",
        // Try to send 100 SOL (way more than balance).
        TransactionType::Transfer {
            to: "Charlie".to_string(),
            amount: 100_000_000_000,
        },
    );
    // Attempt execution.
    match tx3.execute(&mut alice) {
        // This shouldn't succeed.
        Ok(msg) => println!("  Unexpected: {}", msg),
        // Expected: "Insufficient funds".
        Err(e) => println!("  Correctly rejected: {}", e),
    }

    // ── Section 5: Unstaking ──────────────────────────────────
    println!("\n── Unstaking ──");

    // Unstake returns lamports to the wallet.
    let mut tx4 = Transaction::new(
        // Sender.
        "Alice",
        // Unstake 1 SOL.
        TransactionType::Unstake {
            amount: 1_000_000_000,
        },
    );
    // Execute the unstake.
    match tx4.execute(&mut alice) {
        // Print success.
        Ok(msg) => println!("  {}", msg),
        // Print error.
        Err(e) => println!("  Error: {}", e),
    }
    // Show final wallet state.
    println!("  Final wallet: {}", alice);
    println!("  Final balance: {}", format_sol(alice.balance()));

    // ── Section 6: Utils Demonstration ────────────────────────
    println!("\n── Utils Module ──");

    // Test the address validator from the utils module.
    let good_addr = "7EcDhSYGxXyscszYEp35KHN8vvw3svAuLKTzXwCFLtV";
    let bad_addr = "too_short";
    // Validate the good address.
    println!(
        "  '{}' valid: {}",
        good_addr,
        is_valid_address(good_addr)
    );
    // Validate the bad address.
    println!("  '{}' valid: {}", bad_addr, is_valid_address(bad_addr));

    // Show formatting utilities.
    let amount: u64 = 123_456_789_012;
    // Format as lamports with commas.
    println!("  Formatted: {}", format_lamports(amount));
    // Format as SOL.
    println!("  As SOL: {}", format_sol(amount));

    // ── Done ──────────────────────────────────────────────────
    println!("\n=== Module 07 Complete ===");
}
