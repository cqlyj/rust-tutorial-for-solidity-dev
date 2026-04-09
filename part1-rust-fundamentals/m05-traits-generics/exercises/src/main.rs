// ============================================================================
// Module 05 Exercises: Traits and Generics
// ============================================================================
// Complete each exercise by replacing `todo!()` with your implementation.
// Run with: cargo run
// All exercises should compile and pass the checks when complete.
// ============================================================================

use std::fmt;

fn main() {
    println!("=== Module 05 Exercises: Traits and Generics ===\n");

    exercise_1();
    exercise_2();
    exercise_3();
    exercise_4();
    exercise_5();
    exercise_6();
    exercise_7();
    exercise_8();

    println!("\n🎉 All exercises complete!");
}

// ============================================================================
// Exercise 1: Define and Implement a Trait
// ============================================================================
// Define a trait `Summary` with:
//   - A required method: fn summarize(&self) -> String
//   - A default method: fn preview(&self) -> String
//     that returns the first 20 chars of summarize() followed by "..."
//
// Then implement Summary for the `Article` struct below.
// ============================================================================

struct Article {
    title: String,
    author: String,
    content: String,
}

// TODO: Define the `Summary` trait here.

// TODO: Implement Summary for Article.
// summarize() should return: "{title} by {author}: {content}"

fn exercise_1() {
    println!("--- Exercise 1: Define and Implement a Trait ---");

    let article = Article {
        title: String::from("Solana vs Ethereum"),
        author: String::from("Alice"),
        content: String::from("A deep comparison of the two blockchains..."),
    };

    // Uncomment these lines after implementing:
    // println!("  Summary: {}", article.summarize());
    // println!("  Preview: {}", article.preview());

    println!("  TODO: Implement Summary trait for Article");
}

// ============================================================================
// Exercise 2: Generic Function with Trait Bounds
// ============================================================================
// Write a function `print_if_valid` that:
//   - Takes a reference to any type implementing both `Validate` and `fmt::Display`
//   - If validate() returns true, prints "✓ {item}"
//   - If validate() returns false, prints "✗ Invalid: {error_message}"
// ============================================================================

trait Validate {
    fn validate(&self) -> bool;
    fn error_message(&self) -> String {
        String::from("Validation failed")
    }
}

// TODO: Write the `print_if_valid` function here.

fn exercise_2() {
    println!("--- Exercise 2: Generic Function with Trait Bounds ---");

    // This struct is set up for you — just write the function.
    struct Lamports(u64);

    impl Validate for Lamports {
        fn validate(&self) -> bool {
            self.0 > 0
        }
        fn error_message(&self) -> String {
            String::from("Lamports must be > 0")
        }
    }

    impl fmt::Display for Lamports {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} lamports", self.0)
        }
    }

    let good = Lamports(1000);
    let bad = Lamports(0);

    // Uncomment after implementing print_if_valid:
    // print_if_valid(&good);
    // print_if_valid(&bad);

    println!("  TODO: Implement print_if_valid function");
}

// ============================================================================
// Exercise 3: Derive Macros
// ============================================================================
// Add the correct derive macros to `TokenInfo` so that:
//   1. It can be printed with {:?}
//   2. It can be compared with == and !=
//   3. It can be cloned with .clone()
//   4. It can be created with TokenInfo::default()
// ============================================================================

// TODO: Add the correct #[derive(...)] attribute.
struct TokenInfo {
    symbol: String,
    decimals: u8,
    total_supply: u64,
}

fn exercise_3() {
    println!("--- Exercise 3: Derive Macros ---");

    // Uncomment these lines after adding derives:
    // let token = TokenInfo {
    //     symbol: String::from("SOL"),
    //     decimals: 9,
    //     total_supply: 1_000_000_000,
    // };
    // let token_clone = token.clone();
    // println!("  Debug: {:?}", token);
    // println!("  Equal: {}", token == token_clone);
    // let default_token = TokenInfo::default();
    // println!("  Default: {:?}", default_token);

    println!("  TODO: Add derive macros to TokenInfo");
}

// ============================================================================
// Exercise 4: Implement Display for a Custom Type
// ============================================================================
// Implement fmt::Display for `SolBalance` so that:
//   - It prints as "X.YYYYYYYYY SOL" (9 decimal places)
//   - Example: 1_500_000_000 lamports -> "1.500000000 SOL"
//
// Hint: SOL has 9 decimal places. Divide lamports by 1_000_000_000 for the
// whole part, use modulo for the fractional part.
// ============================================================================

struct SolBalance {
    lamports: u64,
}

// TODO: Implement fmt::Display for SolBalance.

fn exercise_4() {
    println!("--- Exercise 4: Implement Display ---");

    let balance = SolBalance { lamports: 2_500_000_000 };
    let zero = SolBalance { lamports: 0 };
    let fractional = SolBalance { lamports: 123_456_789 };

    // Uncomment after implementing Display:
    // println!("  {}", balance);       // Should print: 2.500000000 SOL
    // println!("  {}", zero);          // Should print: 0.000000000 SOL
    // println!("  {}", fractional);    // Should print: 0.123456789 SOL

    println!("  TODO: Implement Display for SolBalance");
}

// ============================================================================
// Exercise 5: Trait Objects vs Generics
// ============================================================================
// Part A: Write a GENERIC function `validate_generic<T: Validate>(item: &T) -> bool`
//         that returns item.validate()
//
// Part B: Write a DYNAMIC dispatch function `validate_dynamic(item: &dyn Validate) -> bool`
//         that returns item.validate()
//
// Part C: Create a Vec<Box<dyn Validate>> containing at least two different types
//         and validate each one using validate_dynamic.
// ============================================================================

struct AccountData {
    balance: u64,
}

impl Validate for AccountData {
    fn validate(&self) -> bool {
        self.balance > 0
    }
}

struct ProgramData {
    is_executable: bool,
}

impl Validate for ProgramData {
    fn validate(&self) -> bool {
        self.is_executable
    }
}

// TODO: Write validate_generic function.

// TODO: Write validate_dynamic function.

fn exercise_5() {
    println!("--- Exercise 5: Trait Objects vs Generics ---");

    let account = AccountData { balance: 100 };
    let program = ProgramData { is_executable: true };

    // Uncomment after implementing:
    // println!("  Generic - account valid: {}", validate_generic(&account));
    // println!("  Generic - program valid: {}", validate_generic(&program));
    // println!("  Dynamic - account valid: {}", validate_dynamic(&account));
    // println!("  Dynamic - program valid: {}", validate_dynamic(&program));

    // Part C: Create a Vec<Box<dyn Validate>> and validate each item.
    // let items: Vec<Box<dyn Validate>> = vec![
    //     Box::new(AccountData { balance: 50 }),
    //     Box::new(ProgramData { is_executable: false }),
    //     Box::new(AccountData { balance: 0 }),
    // ];
    // for (i, item) in items.iter().enumerate() {
    //     println!("  Item {} valid: {}", i, validate_dynamic(item.as_ref()));
    // }

    println!("  TODO: Implement validate_generic and validate_dynamic");
}

// ============================================================================
// Exercise 6: Generic Struct with Trait Bounds
// ============================================================================
// Create a generic struct `Vault<T>` where T: Display + Validate.
// It should have:
//   - fields: `name: String` and `contents: T`
//   - method `inspect(&self)` that prints the name, displays the contents,
//     and says whether the contents are valid
//
// Hint: You'll need an `impl` block with trait bounds.
// ============================================================================

// TODO: Define the Vault<T> struct.

// TODO: Implement methods on Vault<T>.

fn exercise_6() {
    println!("--- Exercise 6: Generic Struct ---");

    // This helper type is provided for testing.
    struct Token {
        symbol: String,
        amount: u64,
    }

    impl fmt::Display for Token {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} {}", self.amount, self.symbol)
        }
    }

    impl Validate for Token {
        fn validate(&self) -> bool {
            self.amount > 0 && !self.symbol.is_empty()
        }
    }

    // Uncomment after implementing Vault:
    // let vault = Vault {
    //     name: String::from("Treasury"),
    //     contents: Token { symbol: String::from("SOL"), amount: 1000 },
    // };
    // vault.inspect();

    println!("  TODO: Implement Vault<T> struct");
}

// ============================================================================
// Exercise 7: Solana Account Validation Pattern
// ============================================================================
// Create a trait `AccountConstraint` with:
//   - fn check(&self) -> Result<(), String>
//
// Implement it for these structs:
//   - `HasMinBalance { balance: u64, minimum: u64 }` — fails if balance < minimum
//   - `IsInitialized { initialized: bool }` — fails if not initialized
//   - `IsOwnedBy { owner: String, expected_owner: String }` — fails if owners don't match
//
// Then write a function `check_all_constraints(constraints: &[&dyn AccountConstraint])`
// that checks all constraints and prints results.
// ============================================================================

// TODO: Define the AccountConstraint trait.

// TODO: Define the three constraint structs and implement AccountConstraint for each.

// TODO: Write check_all_constraints function.

fn exercise_7() {
    println!("--- Exercise 7: Solana Account Validation Pattern ---");

    // Uncomment after implementing:
    // let balance_check = HasMinBalance { balance: 1000, minimum: 500 };
    // let init_check = IsInitialized { initialized: true };
    // let owner_check = IsOwnedBy {
    //     owner: String::from("Alice"),
    //     expected_owner: String::from("Alice"),
    // };
    //
    // let constraints: Vec<&dyn AccountConstraint> = vec![
    //     &balance_check,
    //     &init_check,
    //     &owner_check,
    // ];
    // check_all_constraints(&constraints);
    //
    // // Now test a failing case:
    // let bad_balance = HasMinBalance { balance: 10, minimum: 500 };
    // let bad_owner = IsOwnedBy {
    //     owner: String::from("Eve"),
    //     expected_owner: String::from("Alice"),
    // };
    // let failing_constraints: Vec<&dyn AccountConstraint> = vec![
    //     &bad_balance,
    //     &bad_owner,
    // ];
    // check_all_constraints(&failing_constraints);

    println!("  TODO: Implement AccountConstraint pattern");
}

// ============================================================================
// Exercise 8: Where Clause and Multiple Generics
// ============================================================================
// Write a function `transfer` that:
//   - Has two generic parameters: F (from) and T (to)
//   - Both F and T must implement `Validate` and `Display`
//   - Takes `from: &F`, `to: &T`, and `amount: u64`
//   - Returns Result<String, String>
//   - Validates both accounts. If either is invalid, return Err with the error.
//   - If both valid, return Ok("Transferred {amount} from {from} to {to}")
//
// Use a `where` clause for the bounds.
// ============================================================================

// TODO: Write the `transfer` function with a `where` clause.

impl fmt::Display for AccountData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Account(balance={})", self.balance)
    }
}

fn exercise_8() {
    println!("--- Exercise 8: Where Clause and Multiple Generics ---");

    let alice = AccountData { balance: 1000 };
    let bob = AccountData { balance: 500 };
    let empty = AccountData { balance: 0 };

    // Uncomment after implementing:
    // match transfer(&alice, &bob, 100) {
    //     Ok(msg) => println!("  ✓ {}", msg),
    //     Err(e) => println!("  ✗ {}", e),
    // }
    // match transfer(&alice, &empty, 100) {
    //     Ok(msg) => println!("  ✓ {}", msg),
    //     Err(e) => println!("  ✗ {}", e),
    // }

    println!("  TODO: Implement transfer function with where clause");
}
