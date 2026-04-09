// ============================================================
// wallet.rs — The wallet module
// ============================================================
// This file becomes the `wallet` module because main.rs
// declares `mod wallet;` and Rust finds this file at
// src/wallet.rs.
// ============================================================

// A Wallet struct representing a Solana-like wallet.
// `pub` makes the struct visible outside this module.
// Think of `pub` like Solidity's `public` — without it,
// the struct would be private (like Solidity's `internal`).
pub struct Wallet {
    // Public field — accessible from other modules.
    pub owner: String,
    // Private field — only accessible within this module.
    // Other modules must use methods to interact with balance.
    // This is like a private state variable in Solidity.
    balance: u64,
}

// Implementation block for Wallet — like a Solidity contract's functions.
impl Wallet {
    // Associated function (constructor) — like Solidity's constructor.
    // `pub` makes this callable from other modules.
    pub fn new(owner: &str, balance: u64) -> Self {
        // Create and return a new Wallet instance.
        Wallet {
            // Convert the string slice to an owned String.
            owner: owner.to_string(),
            // Set the initial balance.
            balance,
        }
    }

    // Public method to get the balance (getter).
    // In Solidity, public state variables auto-generate getters.
    // In Rust, we write them explicitly since `balance` is private.
    // `&self` borrows the Wallet immutably — read-only access.
    pub fn balance(&self) -> u64 {
        // Return the balance value.
        self.balance
    }

    // Public method to deposit lamports into the wallet.
    // `&mut self` borrows the Wallet mutably — write access.
    // Like a Solidity function that modifies state.
    pub fn deposit(&mut self, amount: u64) {
        // Add the amount to the current balance.
        self.balance += amount;
    }

    // Public method to withdraw lamports from the wallet.
    // Returns a Result — Ok(()) on success, Err(String) on failure.
    // This is like Solidity's require() pattern but more explicit.
    pub fn withdraw(&mut self, amount: u64) -> Result<(), String> {
        // Check if the wallet has sufficient funds.
        if amount > self.balance {
            // Return an error if insufficient balance.
            // Like Solidity: require(amount <= balance, "Insufficient funds");
            return Err("Insufficient funds".to_string());
        }
        // Subtract the amount from the balance.
        self.balance -= amount;
        // Return Ok to indicate success.
        Ok(())
    }

    // A pub(crate) method — visible within this crate but not to external users.
    // Like Solidity's `internal` visibility.
    // Useful for helper functions other modules in the same crate need.
    pub(crate) fn summary(&self) -> String {
        // Format a summary string with owner and balance.
        format!("Wallet[owner={}, balance={}]", self.owner, self.balance)
    }
}

// Display trait implementation — like Solidity's toString() pattern.
// This lets us print Wallet using `{}` format specifier.
impl std::fmt::Display for Wallet {
    // The `fmt` method is required by the Display trait.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write formatted output: "Alice: 1000 lamports".
        write!(f, "{}: {} lamports", self.owner, self.balance)
    }
}
