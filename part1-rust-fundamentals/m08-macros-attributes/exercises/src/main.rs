// =============================================================================
// Module 08: Macros and Attributes — EXERCISES
// =============================================================================
// Complete each exercise by replacing `todo!()` with working code.
// Run with: cargo run
// Test with: cargo test
//
// There are 7 exercises covering:
//   1. Writing a simple declarative macro
//   2. Macro with repetition
//   3. Using derive macros
//   4. Understanding #[derive()] behavior
//   5. Conditional compilation
//   6. Reading Anchor-like macro code
//   7. Building a macro that generates an impl block
// =============================================================================

// =============================================================================
// Exercise 1: Write a Simple Declarative Macro
// =============================================================================
// Create a macro called `double!` that takes a single expression and returns
// it multiplied by 2.
//
// Example usage:
//   double!(5)    -> 10
//   double!(3+1)  -> 8
//
// Hint: Use macro_rules! with $val:expr
// Solidity analogy: imagine a preprocessor that rewrites double(x) to (x * 2)

// TODO: Define the `double` macro here
// macro_rules! double {
//     ...
// }

// =============================================================================
// Exercise 2: Macro with Repetition
// =============================================================================
// Create a macro called `sum_all!` that takes any number of expressions
// and returns their sum.
//
// Example usage:
//   sum_all!(1, 2, 3)     -> 6
//   sum_all!(10, 20)      -> 30
//   sum_all!(5)            -> 5
//
// Hint: Use $( $x:expr ),+ repetition and fold with +
// The tricky part: you need a way to add multiple values together.
// Approach: use 0 $( + $x )* to start with 0 and add each value

// TODO: Define the `sum_all` macro here
// macro_rules! sum_all {
//     ...
// }

// =============================================================================
// Exercise 3: Using Derive Macros
// =============================================================================
// Add the correct #[derive(...)] attributes to make this code compile.
// You need: Debug (for {:?} printing), Clone (for .clone()), PartialEq (for ==)
//
// Solidity analogy: this is like having the compiler auto-generate
// comparison functions and copy constructors for your structs.

// TODO: Add the correct #[derive(...)] attribute
struct TokenAccount {
    mint: String,        // the token mint address
    owner: String,       // who owns this token account
    amount: u64,         // token balance
}

// TODO: Add the correct #[derive(...)] attribute to make Debug and PartialEq work
enum TransferStatus {
    Pending,
    Completed,
    Failed(String),
}

// =============================================================================
// Exercise 4: Understanding What #[derive()] Does
// =============================================================================
// The Price struct below has #[derive(Debug, Clone, PartialEq)].
// Your task: implement the `price_operations` function that uses all three
// derived traits. Follow the instructions in the function body.

#[derive(Debug, Clone, PartialEq)]
struct Price {
    asset: String,    // e.g., "SOL", "BTC"
    usd_cents: u64,   // price in cents (e.g., 15000 = $150.00)
}

fn price_operations() -> bool {
    // Step 1: Create a Price for SOL at $150.00 (15000 cents)
    let _sol_price = todo!("Create a Price struct");

    // Step 2: Clone it to create a copy
    let _sol_copy = todo!("Clone sol_price");

    // Step 3: Verify the clone is equal to the original using assert_eq!
    // (This uses PartialEq)
    todo!("Assert sol_price equals sol_copy");

    // Step 4: Print the price using Debug formatting ({:?})
    // (This uses Debug)
    println!("  Price (Debug): {:?}", _sol_price);

    // Step 5: Create a different price and verify it's NOT equal
    let _btc_price = todo!("Create a Price struct for BTC");
    todo!("Assert sol_price does NOT equal btc_price");

    // Return true if everything passed
    true
}

// =============================================================================
// Exercise 5: Conditional Compilation
// =============================================================================
// Create two versions of a function using #[cfg(...)]:
//   - In test mode (#[cfg(test)]): `get_environment()` returns "test"
//   - In non-test mode (#[cfg(not(test))]): `get_environment()` returns "production"
//
// Also create a function `is_debug_build()` that returns true in debug builds
// and false in release builds using the cfg!() macro (not the attribute).

// TODO: Define get_environment() with two versions using #[cfg(test)] and #[cfg(not(test))]
// fn get_environment() -> &'static str { ... }
// fn get_environment() -> &'static str { ... }

fn is_debug_build() -> bool {
    // TODO: Use the cfg!() macro to check debug_assertions
    todo!("Return true if debug build, false if release")
}

// =============================================================================
// Exercise 6: Reading Anchor-Like Macro Code
// =============================================================================
// This exercise tests your ability to READ macro-heavy code, not write it.
// The code below simulates Anchor patterns. Your task is to implement the
// missing pieces based on the comments explaining what each macro would do.
//
// Think of this as: "Given an Anchor program, can you understand and modify it?"

// Simulated declare_id! — stores the program's public key
macro_rules! declare_program_id {
    ($id:expr) => {
        const PROGRAM_ID: &str = $id;
    };
}

// TODO: Use the macro to set the program ID to "MyProgram111111111111111111111111"
// declare_program_id!(...);

// Simulated msg! — logs with program prefix
macro_rules! sol_msg {
    ($($arg:tt)*) => {
        println!("[Program log:] {}", format!($($arg)*));
    };
}

// Simulated account validation context
#[derive(Debug)]
struct VaultAccount {
    balance: u64,         // lamports stored in the vault
    authority: String,    // who can withdraw (like Solidity's owner)
    is_initialized: bool, // whether the account has been set up
}

fn simulate_anchor_deposit(vault: &mut VaultAccount, amount: u64, signer: &str) -> Result<(), String> {
    // TODO: Check that the vault is initialized. If not, return Err("Vault not initialized")
    todo!("Check is_initialized");

    // TODO: Check that signer matches vault.authority. If not, return Err("Unauthorized")
    todo!("Check authority");

    // TODO: Add amount to vault.balance (use checked_add to prevent overflow)
    // checked_add returns Option<u64> — None on overflow
    // Return Err("Overflow") if it overflows
    todo!("Add amount with overflow check");

    // TODO: Log the deposit using sol_msg! macro
    // Format: "Deposited {amount} lamports. New balance: {balance}"
    todo!("Log with sol_msg!");

    Ok(())
}

// =============================================================================
// Exercise 7: Build a Macro That Generates an Impl Block
// =============================================================================
// Create a macro called `impl_display` that generates a Display implementation
// for a struct. The macro takes a struct name and a format string pattern.
//
// Example:
//   impl_display!(MyStruct, "MyStruct {{ value: {} }}", self.value);
//
// This should generate:
//   impl std::fmt::Display for MyStruct {
//       fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//           write!(f, "MyStruct {{ value: {} }}", self.value)
//       }
//   }
//
// Hint: $name:ident for the struct name, $fmt:expr for the format string,
//       $( $field:ident ),* for the field names.
//       Inside the macro, access fields as self.$field.
//       Note: you can't pass self.name through a macro parameter due to hygiene.
//       Instead, pass `name` as an ident and write self.$field in the expansion.

// TODO: Define the impl_display macro
// macro_rules! impl_display {
//     ...
// }

#[derive(Debug)]
struct Validator {
    name: String,
    stake: u64,
}

// TODO: Use impl_display! to make Validator printable with format:
// "Validator { name: <name>, stake: <stake> }"
// impl_display!(Validator, "Validator {{ name: {}, stake: {} }}", name, stake);

// =============================================================================
// Main — runs the exercises that are implemented
// =============================================================================

fn main() {
    println!("=== Module 08 Exercises: Macros and Attributes ===\n");

    // Uncomment each section as you complete the exercises:

    // Exercise 1: double! macro
    // println!("Exercise 1:");
    // println!("  double!(5) = {}", double!(5));
    // println!("  double!(3+1) = {}", double!(3 + 1));
    // assert_eq!(double!(5), 10);
    // assert_eq!(double!(3 + 1), 8);
    // println!("  ✓ Exercise 1 passed!\n");

    // Exercise 2: sum_all! macro
    // println!("Exercise 2:");
    // println!("  sum_all!(1, 2, 3) = {}", sum_all!(1, 2, 3));
    // println!("  sum_all!(10, 20) = {}", sum_all!(10, 20));
    // assert_eq!(sum_all!(1, 2, 3), 6);
    // assert_eq!(sum_all!(10, 20), 30);
    // assert_eq!(sum_all!(5), 5);
    // println!("  ✓ Exercise 2 passed!\n");

    // Exercise 3: derive macros
    // println!("Exercise 3:");
    // let account = TokenAccount {
    //     mint: String::from("So11111111111111111111111111111112"),
    //     owner: String::from("7xKXxyz..."),
    //     amount: 1_000_000_000,
    // };
    // let account_copy = account.clone();
    // println!("  Account: {:?}", account);
    // assert_eq!(account, account_copy);
    // let status = TransferStatus::Completed;
    // println!("  Status: {:?}", status);
    // assert_eq!(status, TransferStatus::Completed);
    // println!("  ✓ Exercise 3 passed!\n");

    // Exercise 4: using derived traits
    // println!("Exercise 4:");
    // assert!(price_operations());
    // println!("  ✓ Exercise 4 passed!\n");

    // Exercise 5: conditional compilation
    // println!("Exercise 5:");
    // println!("  Environment: {}", get_environment());
    // println!("  Debug build: {}", is_debug_build());
    // println!("  ✓ Exercise 5 passed!\n");

    // Exercise 6: Anchor-like patterns
    // println!("Exercise 6:");
    // println!("  Program ID: {}", PROGRAM_ID);
    // let mut vault = VaultAccount {
    //     balance: 1_000_000,
    //     authority: String::from("admin"),
    //     is_initialized: true,
    // };
    // assert!(simulate_anchor_deposit(&mut vault, 500_000, "admin").is_ok());
    // assert_eq!(vault.balance, 1_500_000);
    // assert!(simulate_anchor_deposit(&mut vault, 100, "hacker").is_err());
    // let mut uninit_vault = VaultAccount {
    //     balance: 0,
    //     authority: String::from("admin"),
    //     is_initialized: false,
    // };
    // assert!(simulate_anchor_deposit(&mut uninit_vault, 100, "admin").is_err());
    // println!("  ✓ Exercise 6 passed!\n");

    // Exercise 7: impl_display macro
    // println!("Exercise 7:");
    // let validator = Validator {
    //     name: String::from("SuperValidator"),
    //     stake: 10_000_000,
    // };
    // println!("  {}", validator);  // Uses Display trait
    // println!("  ✓ Exercise 7 passed!\n");

    println!("Uncomment exercises in main() as you complete them.");
    println!("Run `cargo test` to verify your solutions.");
}

// =============================================================================
// Tests — verify exercise solutions
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1 tests
    // #[test]
    // fn test_double_macro() {
    //     assert_eq!(double!(5), 10);
    //     assert_eq!(double!(0), 0);
    //     assert_eq!(double!(3 + 1), 8);
    //     assert_eq!(double!(100), 200);
    // }

    // Exercise 2 tests
    // #[test]
    // fn test_sum_all_macro() {
    //     assert_eq!(sum_all!(1, 2, 3), 6);
    //     assert_eq!(sum_all!(10, 20), 30);
    //     assert_eq!(sum_all!(5), 5);
    //     assert_eq!(sum_all!(1, 1, 1, 1, 1), 5);
    // }

    // Exercise 3 tests
    // #[test]
    // fn test_token_account_derive() {
    //     let a = TokenAccount {
    //         mint: String::from("mint1"),
    //         owner: String::from("owner1"),
    //         amount: 100,
    //     };
    //     let b = a.clone();
    //     assert_eq!(a, b);
    //     println!("{:?}", a);  // Debug must work
    // }

    // Exercise 4 tests
    // #[test]
    // fn test_price_operations() {
    //     assert!(price_operations());
    // }

    // Exercise 5 tests
    // #[test]
    // fn test_conditional_compilation() {
    //     assert_eq!(get_environment(), "test");
    //     // In test mode, get_environment should return "test"
    // }

    // Exercise 6 tests
    // #[test]
    // fn test_anchor_deposit_valid() {
    //     let mut vault = VaultAccount {
    //         balance: 1_000_000,
    //         authority: String::from("admin"),
    //         is_initialized: true,
    //     };
    //     assert!(simulate_anchor_deposit(&mut vault, 500_000, "admin").is_ok());
    //     assert_eq!(vault.balance, 1_500_000);
    // }

    // #[test]
    // fn test_anchor_deposit_unauthorized() {
    //     let mut vault = VaultAccount {
    //         balance: 1_000_000,
    //         authority: String::from("admin"),
    //         is_initialized: true,
    //     };
    //     assert!(simulate_anchor_deposit(&mut vault, 100, "hacker").is_err());
    // }

    // #[test]
    // fn test_anchor_deposit_uninitialized() {
    //     let mut vault = VaultAccount {
    //         balance: 0,
    //         authority: String::from("admin"),
    //         is_initialized: false,
    //     };
    //     assert!(simulate_anchor_deposit(&mut vault, 100, "admin").is_err());
    // }
}
