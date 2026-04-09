// ============================================================
// transactions.rs — The transactions module
// ============================================================
// Demonstrates cross-module usage: this module imports from
// the wallet module using `use crate::wallet::Wallet`.
// ============================================================

// Import the Wallet struct from our wallet module.
// `crate::` means "start from the crate root (main.rs)".
// This is like Solidity's: import {Wallet} from "./Wallet.sol";
use crate::wallet::Wallet;

// An enum representing transaction types.
// `pub` makes it visible outside this module.
// Enums in Rust are like Solidity enums but far more powerful
// because each variant can hold data (tagged unions).
#[derive(Debug)]
pub enum TransactionType {
    // A transfer variant holding the recipient address and amount.
    Transfer { to: String, amount: u64 },
    // A stake variant holding the amount to stake.
    Stake { amount: u64 },
    // An unstake variant holding the amount to unstake.
    Unstake { amount: u64 },
}

// A struct representing a transaction.
// All fields are public so other modules can read them.
#[derive(Debug)]
pub struct Transaction {
    // The type of transaction (Transfer, Stake, or Unstake).
    pub tx_type: TransactionType,
    // The sender's address as a string.
    pub from: String,
    // Whether the transaction has been executed.
    pub executed: bool,
}

// Implementation block for Transaction.
impl Transaction {
    // Constructor: create a new unexecuted transaction.
    pub fn new(from: &str, tx_type: TransactionType) -> Self {
        // Build the Transaction struct.
        Transaction {
            // Store the transaction type.
            tx_type,
            // Convert the sender to an owned String.
            from: from.to_string(),
            // Transactions start as not executed.
            executed: false,
        }
    }

    // Execute the transaction against a mutable wallet.
    // Takes a mutable reference to a Wallet from the wallet module.
    // Returns Result<String, String> — success message or error.
    pub fn execute(&mut self, wallet: &mut Wallet) -> Result<String, String> {
        // Prevent double-execution — like Solidity's reentrancy guard.
        if self.executed {
            // Return error if already executed.
            return Err("Transaction already executed".to_string());
        }

        // Match on the transaction type to determine behavior.
        // `match` is like a Solidity if/else chain but exhaustive.
        let result = match &self.tx_type {
            // Handle Transfer: withdraw from wallet.
            TransactionType::Transfer { to, amount } => {
                // Attempt to withdraw; the `?` would propagate errors,
                // but here we use match for clarity.
                wallet.withdraw(*amount)?;
                // Return a success message with transfer details.
                format!("Transferred {} lamports to {}", amount, to)
            }
            // Handle Stake: withdraw from wallet to stake.
            TransactionType::Stake { amount } => {
                // Withdraw the stake amount from the wallet.
                wallet.withdraw(*amount)?;
                // Return a success message.
                format!("Staked {} lamports", amount)
            }
            // Handle Unstake: deposit back into wallet.
            TransactionType::Unstake { amount } => {
                // Deposit the unstaked amount back.
                wallet.deposit(*amount);
                // Return a success message.
                format!("Unstaked {} lamports", amount)
            }
        };

        // Mark the transaction as executed.
        self.executed = true;
        // Return the success message.
        Ok(result)
    }
}

// Display trait for pretty-printing transactions.
impl std::fmt::Display for Transaction {
    // Format the transaction for display.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Show execution status as a checkmark or pending indicator.
        let status = if self.executed { "✓" } else { "pending" };
        // Write the formatted transaction info.
        write!(f, "[{}] from {} — {:?}", status, self.from, self.tx_type)
    }
}
