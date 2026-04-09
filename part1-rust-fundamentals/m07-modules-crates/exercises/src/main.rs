// ============================================================
// Module 07 — Exercises: Modules, Crates & Cargo
// ============================================================
// These exercises use inline modules (mod { } blocks) so
// everything lives in one file for simplicity.
//
// Fix the TODO items to make each exercise compile and pass.
// Run with: cargo run
// ============================================================

fn main() {
    println!("=== Module 07 Exercises ===\n");

    exercise_1();
    exercise_2();
    exercise_3();
    exercise_4();
    exercise_5();
    exercise_6();

    println!("\n=== All exercises complete! ===");
}

// ── Exercise 1: Basic Module Declaration ──────────────────────
// Create a module named `greet` with a public function `hello`
// that returns "Hello, Solana!".
//
// Solidity equivalent:
//   library Greet {
//       function hello() internal pure returns (string memory) {
//           return "Hello, Solana!";
//       }
//   }

// TODO: Declare a module named `greet` with a pub function `hello`
// that returns a &'static str "Hello, Solana!"
// mod greet {
//     ???
// }

fn exercise_1() {
    println!("Exercise 1: Basic Module");
    // TODO: Uncomment the next line once you've defined the module
    // println!("  {}", greet::hello());
    println!("  (not yet implemented)");
}

// ── Exercise 2: Visibility Rules ──────────────────────────────
// The module below has a struct with mixed visibility.
// Fix the visibility so that:
// - `TokenAccount` is public
// - `mint` field is public
// - `owner` field is public
// - `amount` field is PRIVATE (accessed via getter)
// - `new` function is public
// - `amount` getter is public

mod token {
    // TODO: Add correct visibility modifiers
    struct TokenAccount {
        mint: String,
        owner: String,
        amount: u64,
    }

    impl TokenAccount {
        fn new(mint: &str, owner: &str, amount: u64) -> Self {
            TokenAccount {
                mint: mint.to_string(),
                owner: owner.to_string(),
                amount,
            }
        }

        fn amount(&self) -> u64 {
            self.amount
        }
    }
}

fn exercise_2() {
    println!("Exercise 2: Visibility Rules");
    // TODO: Uncomment the following once visibility is fixed
    // let account = token::TokenAccount::new("SOL_MINT", "Alice", 1000);
    // println!("  Mint: {}", account.mint);
    // println!("  Owner: {}", account.owner);
    // println!("  Amount: {}", account.amount());
    // The next line should NOT compile (amount is private):
    // println!("  Direct: {}", account.amount);
    println!("  (not yet implemented)");
}

// ── Exercise 3: Using `use` to Import ────────────────────────
// The module below defines an enum and function.
// Use the `use` keyword to bring items into scope so the
// exercise function works without full paths.

mod instructions {
    pub enum Instruction {
        Transfer { to: String, amount: u64 },
        Mint { amount: u64 },
        Burn { amount: u64 },
    }

    pub fn describe(ix: &Instruction) -> String {
        match ix {
            Instruction::Transfer { to, amount } => {
                format!("Transfer {} to {}", amount, to)
            }
            Instruction::Mint { amount } => {
                format!("Mint {}", amount)
            }
            Instruction::Burn { amount } => {
                format!("Burn {}", amount)
            }
        }
    }
}

fn exercise_3() {
    println!("Exercise 3: Using `use`");
    // TODO: Add a `use` statement above this function (or inside it)
    // so you can write `Instruction::Transfer` instead of
    // `instructions::Instruction::Transfer`.
    //
    // Then uncomment the following:
    // let ix = Instruction::Transfer {
    //     to: "Bob".to_string(),
    //     amount: 500,
    // };
    // println!("  {}", describe(&ix));
    //
    // let ix2 = Instruction::Burn { amount: 100 };
    // println!("  {}", describe(&ix2));
    println!("  (not yet implemented)");
}

// ── Exercise 4: Nested Modules and `super` ───────────────────
// Complete the nested module structure.
// The inner module should use `super::` to access the outer module's constant.

mod config {
    // A constant in the config module.
    pub const NETWORK: &str = "devnet";

    pub mod settings {
        // TODO: Create a public function `network_url` that returns
        // a String like "https://api.devnet.solana.com" by using
        // `super::NETWORK` to get the network name.
        //
        // Hint: format!("https://api.{}.solana.com", super::???)
    }
}

fn exercise_4() {
    println!("Exercise 4: Nested Modules + super");
    // TODO: Uncomment once you've implemented network_url
    // println!("  Network: {}", config::NETWORK);
    // println!("  URL: {}", config::settings::network_url());
    println!("  (not yet implemented)");
}

// ── Exercise 5: Re-exports with `pub use` ────────────────────
// The module below has a deeply nested structure.
// Add re-exports so users can access items without the full path.

mod program {
    pub mod state {
        pub struct Counter {
            pub value: u64,
        }

        impl Counter {
            pub fn new() -> Self {
                Counter { value: 0 }
            }

            pub fn increment(&mut self) {
                self.value += 1;
            }
        }
    }

    pub mod errors {
        pub struct ProgramError {
            pub message: String,
        }

        impl ProgramError {
            pub fn new(msg: &str) -> Self {
                ProgramError {
                    message: msg.to_string(),
                }
            }
        }
    }

    // TODO: Add pub use statements to re-export Counter and ProgramError
    // so they can be accessed as `program::Counter` and `program::ProgramError`
    // instead of `program::state::Counter` and `program::errors::ProgramError`.
}

fn exercise_5() {
    println!("Exercise 5: Re-exports");
    // TODO: Uncomment once re-exports are added
    // let mut counter = program::Counter::new();
    // counter.increment();
    // counter.increment();
    // println!("  Counter: {}", counter.value);
    //
    // let err = program::ProgramError::new("Overflow");
    // println!("  Error: {}", err.message);
    println!("  (not yet implemented)");
}

// ── Exercise 6: Module Tree and Paths ────────────────────────
// This exercise tests your understanding of module paths.
// Fix the function bodies to use the correct paths.

mod vault {
    pub const MAX_BALANCE: u64 = 1_000_000_000_000;

    pub mod accounts {
        pub struct VaultAccount {
            pub authority: String,
            pub balance: u64,
        }

        impl VaultAccount {
            pub fn new(authority: &str) -> Self {
                VaultAccount {
                    authority: authority.to_string(),
                    balance: 0,
                }
            }

            pub fn deposit(&mut self, amount: u64) -> Result<(), String> {
                // TODO: Use `super::MAX_BALANCE` to check if
                // the deposit would exceed the maximum.
                // If balance + amount > MAX_BALANCE, return Err("Exceeds max balance")
                // Otherwise, add amount to balance and return Ok(())
                self.balance += amount;
                Ok(())
            }
        }
    }

    pub mod display {
        // TODO: Use `super::accounts::VaultAccount` to reference VaultAccount.
        // Create a public function `format_vault` that takes a &VaultAccount
        // and returns a String like "Vault[authority=Alice, balance=1000]".
    }
}

fn exercise_6() {
    println!("Exercise 6: Module Paths");
    // TODO: Uncomment once the vault module is complete
    // let mut account = vault::accounts::VaultAccount::new("Alice");
    // account.deposit(5000).unwrap();
    // println!("  {}", vault::display::format_vault(&account));
    //
    // // Test the max balance check
    // match account.deposit(2_000_000_000_000) {
    //     Ok(()) => println!("  Unexpected success!"),
    //     Err(e) => println!("  Correctly rejected: {}", e),
    // }
    println!("  (not yet implemented)");
}
