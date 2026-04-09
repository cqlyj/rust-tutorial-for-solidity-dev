// ============================================================
// Module 07 — Solutions: Modules, Crates & Cargo
// ============================================================
// Complete solutions for all exercises.
// Every line is commented to explain the reasoning.
// Run with: cargo run
// ============================================================

// The main function calls each exercise solution.
fn main() {
    // Print header.
    println!("=== Module 07 Solutions ===\n");

    // Run each solution function.
    exercise_1();
    exercise_2();
    exercise_3();
    exercise_4();
    exercise_5();
    exercise_6();

    // Print footer.
    println!("\n=== All solutions verified! ===");
}

// ── Exercise 1 Solution: Basic Module Declaration ─────────────
// We define an inline module with `mod` and a `pub` function inside.
// In Solidity terms, this is like defining a library with an internal function,
// except we explicitly mark it `pub` to make it visible outside the module.
mod greet {
    // A public function that returns a static string slice.
    // `pub` is required so exercise_1() can call greet::hello().
    // `&'static str` is a string baked into the binary — no allocation needed.
    pub fn hello() -> &'static str {
        // Return the greeting string.
        "Hello, Solana!"
    }
}

// Exercise 1 solution function.
fn exercise_1() {
    // Print the section header.
    println!("Exercise 1: Basic Module");
    // Call the public function using the module path.
    println!("  {}", greet::hello());
}

// ── Exercise 2 Solution: Visibility Rules ─────────────────────
// The key changes:
// - `pub` on the struct so it's visible outside `token`
// - `pub` on `mint` and `owner` fields for direct access
// - `amount` stays private — only accessible via the getter
// - `pub` on `new` and `amount` methods
mod token {
    // The struct is public — visible outside the `token` module.
    pub struct TokenAccount {
        // Public field — other modules can read/write directly.
        pub mint: String,
        // Public field — accessible from outside.
        pub owner: String,
        // Private field — only this module can access it directly.
        // Other modules must use the amount() getter below.
        amount: u64,
    }

    // Implementation block for TokenAccount.
    impl TokenAccount {
        // Public constructor — callable from outside the module.
        pub fn new(mint: &str, owner: &str, amount: u64) -> Self {
            // Create and return a new TokenAccount.
            TokenAccount {
                // Convert &str to owned String for the mint field.
                mint: mint.to_string(),
                // Convert &str to owned String for the owner field.
                owner: owner.to_string(),
                // Set the private amount field.
                amount,
            }
        }

        // Public getter for the private amount field.
        // This pattern gives controlled read access to private data.
        pub fn amount(&self) -> u64 {
            // Return the amount value.
            self.amount
        }
    }
}

// Exercise 2 solution function.
fn exercise_2() {
    // Print the section header.
    println!("Exercise 2: Visibility Rules");
    // Create a new token account using the public constructor.
    let account = token::TokenAccount::new("SOL_MINT", "Alice", 1000);
    // Access the public mint field directly.
    println!("  Mint: {}", account.mint);
    // Access the public owner field directly.
    println!("  Owner: {}", account.owner);
    // Access the private amount through the public getter.
    println!("  Amount: {}", account.amount());
    // NOTE: `account.amount` (direct field access) would NOT compile
    // because the `amount` field is private.
}

// ── Exercise 3 Solution: Using `use` to Import ───────────────
// We define the module with the enum and function.
mod instructions {
    // A public enum representing different instruction types.
    pub enum Instruction {
        // Transfer variant with recipient and amount.
        Transfer { to: String, amount: u64 },
        // Mint variant with amount.
        Mint { amount: u64 },
        // Burn variant with amount.
        Burn { amount: u64 },
    }

    // A public function that describes an instruction.
    pub fn describe(ix: &Instruction) -> String {
        // Match on the instruction variant.
        match ix {
            // Format a transfer description.
            Instruction::Transfer { to, amount } => {
                format!("Transfer {} to {}", amount, to)
            }
            // Format a mint description.
            Instruction::Mint { amount } => {
                format!("Mint {}", amount)
            }
            // Format a burn description.
            Instruction::Burn { amount } => {
                format!("Burn {}", amount)
            }
        }
    }
}

// Bring the Instruction enum into scope so we can use it without the module prefix.
// This is like Solidity's: import {Instruction} from "./Instructions.sol";
use instructions::Instruction;

// Bring the describe function into scope.
use instructions::describe;

// Exercise 3 solution function.
fn exercise_3() {
    // Print the section header.
    println!("Exercise 3: Using `use`");
    // Create a Transfer instruction — no need for instructions:: prefix.
    let ix = Instruction::Transfer {
        // The recipient.
        to: "Bob".to_string(),
        // The amount to transfer.
        amount: 500,
    };
    // Call describe directly — imported via `use`.
    println!("  {}", describe(&ix));

    // Create a Mint instruction.
    let ix2 = Instruction::Mint { amount: 250 };
    // Describe it.
    println!("  {}", describe(&ix2));

    // Create a Burn instruction.
    let ix3 = Instruction::Burn { amount: 100 };
    // Describe it.
    println!("  {}", describe(&ix3));
}

// ── Exercise 4 Solution: Nested Modules and `super` ──────────
mod config {
    // A public constant defining the network name.
    pub const NETWORK: &str = "devnet";

    // A nested public module for settings.
    pub mod settings {
        // Public function that builds a URL using the parent module's constant.
        // `super::NETWORK` reaches up to the `config` module to get NETWORK.
        pub fn network_url() -> String {
            // Format the URL using the network name from the parent module.
            // `super::` goes up one level: from settings to config.
            format!("https://api.{}.solana.com", super::NETWORK)
        }
    }
}

// Exercise 4 solution function.
fn exercise_4() {
    // Print the section header.
    println!("Exercise 4: Nested Modules + super");
    // Access the constant from the config module.
    println!("  Network: {}", config::NETWORK);
    // Call the function from the nested settings module.
    println!("  URL: {}", config::settings::network_url());
}

// ── Exercise 5 Solution: Re-exports with `pub use` ───────────
mod program {
    // A nested module containing the Counter struct.
    pub mod state {
        // A public struct with a public value field.
        pub struct Counter {
            // The counter's current value.
            pub value: u64,
        }

        // Implementation for Counter.
        impl Counter {
            // Create a new counter starting at zero.
            pub fn new() -> Self {
                // Initialize with value 0.
                Counter { value: 0 }
            }

            // Increment the counter by one.
            pub fn increment(&mut self) {
                // Add 1 to the current value.
                self.value += 1;
            }
        }
    }

    // A nested module containing error types.
    pub mod errors {
        // A public error struct with a message.
        pub struct ProgramError {
            // The error message.
            pub message: String,
        }

        // Implementation for ProgramError.
        impl ProgramError {
            // Create a new error with the given message.
            pub fn new(msg: &str) -> Self {
                // Convert the &str to an owned String.
                ProgramError {
                    message: msg.to_string(),
                }
            }
        }
    }

    // Re-export Counter from program::state so it's accessible as program::Counter.
    // This is the `pub use` pattern — it "lifts" an item to the current module's namespace.
    pub use state::Counter;
    // Re-export ProgramError similarly.
    pub use errors::ProgramError;
}

// Exercise 5 solution function.
fn exercise_5() {
    // Print the section header.
    println!("Exercise 5: Re-exports");
    // Access Counter via the re-exported path: program::Counter
    // instead of the full path: program::state::Counter.
    let mut counter = program::Counter::new();
    // Increment twice.
    counter.increment();
    counter.increment();
    // Print the counter value.
    println!("  Counter: {}", counter.value);

    // Access ProgramError via the re-exported path.
    let err = program::ProgramError::new("Overflow");
    // Print the error message.
    println!("  Error: {}", err.message);
}

// ── Exercise 6 Solution: Module Tree and Paths ───────────────
mod vault {
    // A public constant defining the maximum balance.
    pub const MAX_BALANCE: u64 = 1_000_000_000_000;

    // Nested module for account types.
    pub mod accounts {
        // A public struct representing a vault account.
        pub struct VaultAccount {
            // The authority (owner) of the vault.
            pub authority: String,
            // The current balance.
            pub balance: u64,
        }

        // Implementation for VaultAccount.
        impl VaultAccount {
            // Create a new vault account with zero balance.
            pub fn new(authority: &str) -> Self {
                // Initialize with zero balance.
                VaultAccount {
                    authority: authority.to_string(),
                    balance: 0,
                }
            }

            // Deposit funds with a max balance check.
            pub fn deposit(&mut self, amount: u64) -> Result<(), String> {
                // Use `super::MAX_BALANCE` to access the constant from the parent module.
                // `super::` goes from accounts up to vault.
                if self.balance + amount > super::MAX_BALANCE {
                    // Return an error if the deposit would exceed the limit.
                    return Err("Exceeds max balance".to_string());
                }
                // Add the amount to the balance.
                self.balance += amount;
                // Return success.
                Ok(())
            }
        }
    }

    // Nested module for display utilities.
    pub mod display {
        // Use `super::accounts::VaultAccount` to reference the struct from a sibling module.
        // `super::` goes up to vault, then `accounts::` goes into the sibling.
        use super::accounts::VaultAccount;

        // A public function that formats a vault account for display.
        pub fn format_vault(account: &VaultAccount) -> String {
            // Build a formatted string using the account's fields.
            format!(
                "Vault[authority={}, balance={}]",
                account.authority, account.balance
            )
        }
    }
}

// Exercise 6 solution function.
fn exercise_6() {
    // Print the section header.
    println!("Exercise 6: Module Paths");
    // Create a new vault account.
    let mut account = vault::accounts::VaultAccount::new("Alice");
    // Deposit 5000 lamports — should succeed.
    account.deposit(5000).unwrap();
    // Format and print using the display module.
    println!("  {}", vault::display::format_vault(&account));

    // Try to deposit way more than the max — should fail.
    match account.deposit(2_000_000_000_000) {
        // This shouldn't happen.
        Ok(()) => println!("  Unexpected success!"),
        // Expected: "Exceeds max balance".
        Err(e) => println!("  Correctly rejected: {}", e),
    }
}
