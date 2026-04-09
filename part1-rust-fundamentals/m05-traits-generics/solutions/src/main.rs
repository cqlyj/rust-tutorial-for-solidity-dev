// ============================================================================
// Module 05 Solutions: Traits and Generics
// ============================================================================
// Every line is commented to explain what it does.
// Run with: cargo run
// ============================================================================

// Import the Display and Formatter types from std::fmt for custom formatting.
use std::fmt;

// Entry point — runs all exercise solutions in sequence.
fn main() {
    // Print the header for the solutions output.
    println!("=== Module 05 Solutions: Traits and Generics ===\n");

    // Run each exercise solution function.
    exercise_1();
    exercise_2();
    exercise_3();
    exercise_4();
    exercise_5();
    exercise_6();
    exercise_7();
    exercise_8();

    // Print completion message.
    println!("\n🎉 All exercises complete!");
}

// ============================================================================
// Exercise 1 Solution: Define and Implement a Trait
// ============================================================================

// Define a struct to hold article data.
struct Article {
    // The article's title.
    title: String,
    // The article's author name.
    author: String,
    // The article's body content.
    content: String,
}

// Define the Summary trait with a required method and a default method.
trait Summary {
    // Required: every implementor must define how to summarize itself.
    fn summarize(&self) -> String;

    // Default: returns first 20 chars of summarize() plus "...".
    // Implementors get this for free but can override it.
    fn preview(&self) -> String {
        // Get the full summary string.
        let full = self.summarize();
        // If the summary is longer than 20 characters, truncate it.
        if full.len() > 20 {
            // Take the first 20 characters and append "...".
            format!("{}...", &full[..20])
        } else {
            // If 20 chars or fewer, return the whole thing.
            full
        }
    }
}

// Implement Summary for Article.
impl Summary for Article {
    // Provide the required summarize method.
    fn summarize(&self) -> String {
        // Combine title, author, and content into a single summary string.
        format!("{} by {}: {}", self.title, self.author, self.content)
    }
}

// Run and verify exercise 1.
fn exercise_1() {
    // Print the exercise header.
    println!("--- Exercise 1: Define and Implement a Trait ---");

    // Create a test article with sample data.
    let article = Article {
        title: String::from("Solana vs Ethereum"),       // Set the title.
        author: String::from("Alice"),                    // Set the author.
        content: String::from("A deep comparison of the two blockchains..."), // Set content.
    };

    // Call summarize() — our implementation concatenates all fields.
    println!("  Summary: {}", article.summarize());
    // Call preview() — the default implementation truncates to 20 chars.
    println!("  Preview: {}", article.preview());
}

// ============================================================================
// Exercise 2 Solution: Generic Function with Trait Bounds
// ============================================================================

// Define the Validate trait with a required method and a default error message.
trait Validate {
    // Required: returns whether this value is valid.
    fn validate(&self) -> bool;

    // Default error message — implementors can override.
    fn error_message(&self) -> String {
        // Return a generic validation failure string.
        String::from("Validation failed")
    }
}

// Generic function requiring both Validate and Display trait bounds.
// `T: Validate + fmt::Display` means T must implement both traits.
// The compiler generates specialized code for each concrete type (static dispatch).
fn print_if_valid<T: Validate + fmt::Display>(item: &T) {
    // Check if the item passes validation.
    if item.validate() {
        // Print the item using Display formatting (valid case).
        println!("  ✓ {}", item);
    } else {
        // Print the error message from the Validate trait (invalid case).
        println!("  ✗ Invalid: {}", item.error_message());
    }
}

// Run and verify exercise 2.
fn exercise_2() {
    // Print the exercise header.
    println!("--- Exercise 2: Generic Function with Trait Bounds ---");

    // Define a simple wrapper type for lamport amounts.
    struct Lamports(u64);

    // Implement Validate — lamports must be positive.
    impl Validate for Lamports {
        // Return true only if the lamport amount is greater than zero.
        fn validate(&self) -> bool {
            self.0 > 0
        }
        // Custom error message for zero lamports.
        fn error_message(&self) -> String {
            String::from("Lamports must be > 0")
        }
    }

    // Implement Display so the type can be printed with {}.
    impl fmt::Display for Lamports {
        // Format lamports as "N lamports".
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Write the amount followed by the unit.
            write!(f, "{} lamports", self.0)
        }
    }

    // Create a valid lamport amount.
    let good = Lamports(1000);
    // Create an invalid lamport amount (zero).
    let bad = Lamports(0);

    // Call our generic function with both values.
    print_if_valid(&good);
    // This should print the error message.
    print_if_valid(&bad);
}

// ============================================================================
// Exercise 3 Solution: Derive Macros
// ============================================================================

// Derive Debug (for {:?}), PartialEq (for ==), Clone (for .clone()), Default (for ::default()).
#[derive(Debug, PartialEq, Clone, Default)]
struct TokenInfo {
    // The token's ticker symbol.
    symbol: String,
    // Number of decimal places (9 for SOL).
    decimals: u8,
    // Total supply of the token.
    total_supply: u64,
}

// Run and verify exercise 3.
fn exercise_3() {
    // Print the exercise header.
    println!("--- Exercise 3: Derive Macros ---");

    // Create a token with specific values.
    let token = TokenInfo {
        symbol: String::from("SOL"),  // Set ticker to SOL.
        decimals: 9,                   // SOL has 9 decimal places.
        total_supply: 1_000_000_000,   // 1 billion total supply.
    };

    // Clone the token — works because we derived Clone.
    let token_clone = token.clone();
    // Print with Debug formatting — works because we derived Debug.
    println!("  Debug: {:?}", token);
    // Compare with PartialEq — works because we derived PartialEq.
    println!("  Equal: {}", token == token_clone);
    // Create a default instance — works because we derived Default.
    let default_token = TokenInfo::default();
    // Print the default (all fields zero/empty).
    println!("  Default: {:?}", default_token);
}

// ============================================================================
// Exercise 4 Solution: Implement Display for a Custom Type
// ============================================================================

// A struct representing a balance in lamports (smallest SOL unit).
struct SolBalance {
    // Balance stored as lamports (1 SOL = 1,000,000,000 lamports).
    lamports: u64,
}

// Implement Display to show SOL balance with 9 decimal places.
impl fmt::Display for SolBalance {
    // Format lamports as "X.YYYYYYYYY SOL".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Divide by 1 billion to get the whole SOL amount.
        let whole = self.lamports / 1_000_000_000;
        // Modulo 1 billion gives the fractional part in lamports.
        let frac = self.lamports % 1_000_000_000;
        // Write formatted output: whole part, then 9-digit zero-padded fraction.
        write!(f, "{}.{:09} SOL", whole, frac)
    }
}

// Run and verify exercise 4.
fn exercise_4() {
    // Print the exercise header.
    println!("--- Exercise 4: Implement Display ---");

    // Create a balance of 2.5 SOL.
    let balance = SolBalance { lamports: 2_500_000_000 };
    // Create a zero balance.
    let zero = SolBalance { lamports: 0 };
    // Create a fractional balance less than 1 SOL.
    let fractional = SolBalance { lamports: 123_456_789 };

    // Print each balance — uses our Display implementation.
    println!("  {}", balance);       // Prints: 2.500000000 SOL
    // Print zero balance.
    println!("  {}", zero);          // Prints: 0.000000000 SOL
    // Print fractional balance.
    println!("  {}", fractional);    // Prints: 0.123456789 SOL
}

// ============================================================================
// Exercise 5 Solution: Trait Objects vs Generics
// ============================================================================

// A struct representing account data with a balance.
struct AccountData {
    // The account's lamport balance.
    balance: u64,
}

// Implement Validate for AccountData — balance must be positive.
impl Validate for AccountData {
    // Account is valid if balance is greater than zero.
    fn validate(&self) -> bool {
        self.balance > 0
    }
}

// A struct representing program data.
struct ProgramData {
    // Whether this program is executable.
    is_executable: bool,
}

// Implement Validate for ProgramData — must be executable.
impl Validate for ProgramData {
    // Program is valid if it's marked as executable.
    fn validate(&self) -> bool {
        self.is_executable
    }
}

// Part A: Generic function using static dispatch.
// The compiler creates one copy per concrete type — fastest at runtime.
fn validate_generic<T: Validate>(item: &T) -> bool {
    // Call validate() — resolved at compile time.
    item.validate()
}

// Part B: Dynamic dispatch function using a trait object.
// Uses a vtable pointer — slower but allows heterogeneous collections.
fn validate_dynamic(item: &dyn Validate) -> bool {
    // Call validate() — resolved at runtime via vtable.
    item.validate()
}

// Run and verify exercise 5.
fn exercise_5() {
    // Print the exercise header.
    println!("--- Exercise 5: Trait Objects vs Generics ---");

    // Create test instances.
    let account = AccountData { balance: 100 };     // Valid account.
    let program = ProgramData { is_executable: true }; // Valid program.

    // Part A & B: Compare generic vs dynamic dispatch — same results.
    println!("  Generic - account valid: {}", validate_generic(&account));
    // Validate program with generics.
    println!("  Generic - program valid: {}", validate_generic(&program));
    // Validate account with dynamic dispatch.
    println!("  Dynamic - account valid: {}", validate_dynamic(&account));
    // Validate program with dynamic dispatch.
    println!("  Dynamic - program valid: {}", validate_dynamic(&program));

    // Part C: Create a vector of different types behind trait objects.
    // This is only possible with dyn — generics require uniform types.
    let items: Vec<Box<dyn Validate>> = vec![
        Box::new(AccountData { balance: 50 }),          // Valid account (boxed).
        Box::new(ProgramData { is_executable: false }),  // Invalid program (boxed).
        Box::new(AccountData { balance: 0 }),            // Invalid account (boxed).
    ];

    // Iterate and validate each trait object.
    for (i, item) in items.iter().enumerate() {
        // as_ref() converts &Box<dyn Validate> to &dyn Validate.
        println!("  Item {} valid: {}", i, validate_dynamic(item.as_ref()));
    }
}

// ============================================================================
// Exercise 6 Solution: Generic Struct with Trait Bounds
// ============================================================================

// Define a generic struct where T must implement Display and Validate.
// The bounds here mean you can only create a Vault with validatable, displayable types.
struct Vault<T: fmt::Display + Validate> {
    // A human-readable name for this vault.
    name: String,
    // The contents of the vault — constrained by trait bounds.
    contents: T,
}

// Implement methods on Vault. The bounds must be repeated on the impl block.
impl<T: fmt::Display + Validate> Vault<T> {
    // Inspect the vault: show its name, contents, and validity.
    fn inspect(&self) {
        // Print the vault name.
        println!("  Vault '{}' contains: {}", self.name, self.contents);
        // Check and print whether the contents are valid.
        if self.contents.validate() {
            // Contents passed validation.
            println!("  Status: ✓ Valid");
        } else {
            // Contents failed validation.
            println!("  Status: ✗ Invalid");
        }
    }
}

// Run and verify exercise 6.
fn exercise_6() {
    // Print the exercise header.
    println!("--- Exercise 6: Generic Struct ---");

    // Define a helper Token type for testing inside the function.
    struct Token {
        // The token's ticker symbol.
        symbol: String,
        // The token amount.
        amount: u64,
    }

    // Implement Display for Token so it satisfies the Vault's bound.
    impl fmt::Display for Token {
        // Format as "AMOUNT SYMBOL".
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Write amount followed by symbol.
            write!(f, "{} {}", self.amount, self.symbol)
        }
    }

    // Implement Validate for Token so it satisfies the Vault's bound.
    impl Validate for Token {
        // Token is valid if amount > 0 and symbol is not empty.
        fn validate(&self) -> bool {
            self.amount > 0 && !self.symbol.is_empty()
        }
    }

    // Create a vault containing a valid token.
    let vault = Vault {
        name: String::from("Treasury"),                              // Vault name.
        contents: Token { symbol: String::from("SOL"), amount: 1000 }, // Valid token.
    };

    // Inspect the vault — prints name, contents, and validity status.
    vault.inspect();

    // Create a vault with an invalid token (zero amount).
    let empty_vault = Vault {
        name: String::from("Empty"),                                 // Vault name.
        contents: Token { symbol: String::from("SOL"), amount: 0 },  // Invalid: zero amount.
    };

    // Inspect the invalid vault.
    empty_vault.inspect();
}

// ============================================================================
// Exercise 7 Solution: Solana Account Validation Pattern
// ============================================================================

// Define a trait for account constraints — returns Ok(()) on success, Err on failure.
// This pattern mirrors how Anchor validates accounts.
trait AccountConstraint {
    // Check this constraint. Returns Ok if satisfied, Err with a message if not.
    fn check(&self) -> Result<(), String>;
}

// Constraint: account must have at least `minimum` lamports.
struct HasMinBalance {
    // The account's current balance.
    balance: u64,
    // The minimum balance required.
    minimum: u64,
}

// Implement the constraint for minimum balance.
impl AccountConstraint for HasMinBalance {
    // Check that balance meets the minimum requirement.
    fn check(&self) -> Result<(), String> {
        // Compare balance against the minimum.
        if self.balance >= self.minimum {
            // Constraint satisfied — return success.
            Ok(())
        } else {
            // Constraint violated — return an error with details.
            Err(format!(
                "Insufficient balance: have {}, need {}",
                self.balance, self.minimum
            ))
        }
    }
}

// Constraint: account must be initialized.
struct IsInitialized {
    // Whether the account has been initialized.
    initialized: bool,
}

// Implement the constraint for initialization check.
impl AccountConstraint for IsInitialized {
    // Check that the account is initialized.
    fn check(&self) -> Result<(), String> {
        // If initialized, return success.
        if self.initialized {
            Ok(())
        } else {
            // If not initialized, return an error.
            Err(String::from("Account is not initialized"))
        }
    }
}

// Constraint: account must be owned by the expected owner.
struct IsOwnedBy {
    // The actual owner of the account.
    owner: String,
    // The expected owner (who should own it).
    expected_owner: String,
}

// Implement the constraint for owner check.
impl AccountConstraint for IsOwnedBy {
    // Check that the actual owner matches the expected owner.
    fn check(&self) -> Result<(), String> {
        // Compare the two owner strings.
        if self.owner == self.expected_owner {
            // Owners match — constraint satisfied.
            Ok(())
        } else {
            // Owners don't match — return error with details.
            Err(format!(
                "Wrong owner: expected {}, got {}",
                self.expected_owner, self.owner
            ))
        }
    }
}

// Check all constraints in a slice — uses dynamic dispatch (dyn trait references).
// This mirrors Anchor's pattern of composing multiple account validations.
fn check_all_constraints(constraints: &[&dyn AccountConstraint]) {
    // Track whether all constraints pass.
    let mut all_passed = true;
    // Iterate over each constraint reference in the slice.
    for (i, constraint) in constraints.iter().enumerate() {
        // Call check() through the vtable (dynamic dispatch).
        match constraint.check() {
            // Constraint passed — print success.
            Ok(()) => println!("  Constraint {}: ✓ Passed", i),
            // Constraint failed — print the error message.
            Err(e) => {
                // Print the failure with its error.
                println!("  Constraint {}: ✗ {}", i, e);
                // Mark that at least one constraint failed.
                all_passed = false;
            }
        }
    }
    // Print the overall result.
    if all_passed {
        // All constraints passed.
        println!("  Result: All constraints satisfied!");
    } else {
        // At least one constraint failed.
        println!("  Result: Some constraints failed!");
    }
}

// Run and verify exercise 7.
fn exercise_7() {
    // Print the exercise header.
    println!("--- Exercise 7: Solana Account Validation Pattern ---");

    // Create passing constraints.
    let balance_check = HasMinBalance { balance: 1000, minimum: 500 }; // Passes: 1000 >= 500.
    let init_check = IsInitialized { initialized: true };                // Passes: initialized.
    let owner_check = IsOwnedBy {
        owner: String::from("Alice"),           // Actual owner.
        expected_owner: String::from("Alice"),  // Expected owner — matches.
    };

    // Collect constraints as trait object references.
    let constraints: Vec<&dyn AccountConstraint> = vec![
        &balance_check,  // First constraint.
        &init_check,     // Second constraint.
        &owner_check,    // Third constraint.
    ];

    // Check all — should all pass.
    println!("  --- Passing constraints ---");
    check_all_constraints(&constraints);

    // Create failing constraints to test error paths.
    let bad_balance = HasMinBalance { balance: 10, minimum: 500 }; // Fails: 10 < 500.
    let bad_owner = IsOwnedBy {
        owner: String::from("Eve"),            // Actual owner: Eve.
        expected_owner: String::from("Alice"), // Expected: Alice — mismatch.
    };

    // Collect the failing constraints.
    let failing_constraints: Vec<&dyn AccountConstraint> = vec![
        &bad_balance, // Will fail.
        &bad_owner,   // Will also fail.
    ];

    // Check all — should report failures.
    println!("  --- Failing constraints ---");
    check_all_constraints(&failing_constraints);
}

// ============================================================================
// Exercise 8 Solution: Where Clause and Multiple Generics
// ============================================================================

// Generic transfer function with a `where` clause for readability.
// F and T are two separate type parameters — they can be different concrete types.
fn transfer<F, T>(from: &F, to: &T, amount: u64) -> Result<String, String>
where
    F: Validate + fmt::Display, // The sender must be validatable and displayable.
    T: Validate + fmt::Display, // The receiver must be validatable and displayable.
{
    // First validate the sender.
    if !from.validate() {
        // Sender is invalid — return an error.
        return Err(format!("Sender invalid: {}", from));
    }
    // Then validate the receiver.
    if !to.validate() {
        // Receiver is invalid — return an error.
        return Err(format!("Receiver invalid: {}", to));
    }
    // Both valid — return success message with transfer details.
    Ok(format!("Transferred {} from {} to {}", amount, from, to))
}

// Implement Display for AccountData so it works with our transfer function.
impl fmt::Display for AccountData {
    // Format as "Account(balance=N)".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write the account with its balance.
        write!(f, "Account(balance={})", self.balance)
    }
}

// Run and verify exercise 8.
fn exercise_8() {
    // Print the exercise header.
    println!("--- Exercise 8: Where Clause and Multiple Generics ---");

    // Create test accounts.
    let alice = AccountData { balance: 1000 }; // Valid: balance > 0.
    let bob = AccountData { balance: 500 };    // Valid: balance > 0.
    let empty = AccountData { balance: 0 };    // Invalid: balance == 0.

    // Transfer between two valid accounts — should succeed.
    match transfer(&alice, &bob, 100) {
        // Print the success message.
        Ok(msg) => println!("  ✓ {}", msg),
        // Print the error (shouldn't happen here).
        Err(e) => println!("  ✗ {}", e),
    }

    // Transfer to an invalid account — should fail.
    match transfer(&alice, &empty, 100) {
        // Print success (shouldn't happen here).
        Ok(msg) => println!("  ✓ {}", msg),
        // Print the error message about the invalid receiver.
        Err(e) => println!("  ✗ {}", e),
    }

    // Transfer from an invalid account — should fail.
    match transfer(&empty, &bob, 100) {
        // Print success (shouldn't happen here).
        Ok(msg) => println!("  ✓ {}", msg),
        // Print the error message about the invalid sender.
        Err(e) => println!("  ✗ {}", e),
    }
}
