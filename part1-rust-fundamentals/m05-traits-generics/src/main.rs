// ============================================================================
// Module 05: Traits and Generics
// ============================================================================
// Traits are Rust's version of interfaces (like Solidity's `interface`).
// Generics let you write code that works with many types — compiled to
// specialized versions for each type (zero runtime cost, unlike Solidity's
// dynamic dispatch through contract interfaces).
// ============================================================================

// Import the Display and Formatter types from the standard library's fmt module.
use std::fmt;

// ============================================================================
// Part 1: Defining and Implementing a Trait
// ============================================================================

// Define a trait called `Validate` — similar to a Solidity interface.
// Any type that implements this trait must provide a `validate` method.
// In Solidity, this would be: interface IValidate { function validate() ... }
trait Validate {
    // Required method — every implementor must define this.
    // `&self` borrows the value immutably (like `view` in Solidity).
    fn validate(&self) -> bool;

    // Default implementation — implementors get this for free but can override.
    // Solidity interfaces can't do this; only abstract contracts can.
    fn error_message(&self) -> String {
        // Return a generic error string if the implementor doesn't override.
        String::from("Validation failed")
    }

    // Default method that uses other trait methods.
    // This pattern is common — build complex behavior from simple required methods.
    fn validate_or_panic(&self) {
        // Call the required `validate` method.
        if !self.validate() {
            // If validation fails, panic with the error message.
            panic!("{}", self.error_message());
        }
    }
}

// ============================================================================
// Part 2: Implementing Traits for Structs
// ============================================================================

// Derive Debug so we can print this struct with {:?}.
// Derive Clone so we can make copies of it.
// Derive Default so we can create a zero-valued instance.
#[derive(Debug, Clone, Default)]
struct Account {
    // The account's lamport balance (like wei in Ethereum).
    balance: u64,
    // The owner's public key as a string (simplified for this example).
    owner: String,
    // Whether this account has been initialized on-chain.
    is_initialized: bool,
}

// Implement the Validate trait for Account.
// This is like `contract Account is IValidate { ... }` in Solidity.
impl Validate for Account {
    // Provide the required `validate` method.
    fn validate(&self) -> bool {
        // An account is valid if it has a positive balance,
        // a non-empty owner, and has been initialized.
        self.balance > 0 && !self.owner.is_empty() && self.is_initialized
    }

    // Override the default error_message with a more specific one.
    fn error_message(&self) -> String {
        // Build a specific error message for Account validation.
        String::from("Account must have balance > 0, a non-empty owner, and be initialized")
    }
}

// Derive common traits: Debug for {:?}, Clone for .clone(), PartialEq for ==.
#[derive(Debug, Clone, PartialEq)]
struct Transaction {
    // Who sends the tokens.
    from: String,
    // Who receives the tokens.
    to: String,
    // How many lamports to transfer.
    amount: u64,
}

// Implement Validate for Transaction too — multiple types can implement the same trait.
impl Validate for Transaction {
    // A transaction is valid if it has different sender/receiver and positive amount.
    fn validate(&self) -> bool {
        // Sender and receiver must differ, and amount must be positive.
        self.from != self.to && self.amount > 0
    }

    // Custom error message for transaction validation failures.
    fn error_message(&self) -> String {
        // Return a transaction-specific validation error.
        String::from("Transaction: sender != receiver required, amount must be > 0")
    }
}

// ============================================================================
// Part 3: Implementing Display (for human-readable printing)
// ============================================================================

// Implement the standard library's Display trait for Account.
// This lets us use {} in println! (like Solidity's toString equivalent).
impl fmt::Display for Account {
    // The `fmt` method is required by the Display trait.
    // `f` is the formatter that we write output into.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write formatted output: the write! macro returns a fmt::Result.
        write!(
            f,                                      // Write into the formatter.
            "Account(owner={}, balance={}, init={})", // Format string template.
            self.owner,                             // Insert the owner field.
            self.balance,                           // Insert the balance field.
            self.is_initialized                     // Insert the initialized flag.
        )
    }
}

// Implement Display for Transaction too.
impl fmt::Display for Transaction {
    // Format a transaction as a human-readable string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Show the transfer details in a readable format.
        write!(f, "Transfer {} lamports: {} -> {}", self.amount, self.from, self.to)
    }
}

// ============================================================================
// Part 4: Generic Functions with Trait Bounds
// ============================================================================

// A generic function that accepts any type implementing Validate.
// `impl Validate` is syntactic sugar for a generic with a trait bound.
// The compiler generates a specialized version for each concrete type used.
fn check_validity(item: &impl Validate) {
    // Call validate() — works on any type that implements the trait.
    if item.validate() {
        // Print success if the item passes validation.
        println!("  ✓ Valid");
    } else {
        // Print the error message if validation fails.
        println!("  ✗ Invalid: {}", item.error_message());
    }
}

// Equivalent function using explicit generic syntax.
// `T: Validate + fmt::Display` means T must implement BOTH traits.
// The `+` combines multiple trait bounds (like multiple inheritance in Solidity).
fn check_and_display<T: Validate + fmt::Display>(item: &T) {
    // Use Display (via {}) to show the item, then validate it.
    println!("  Checking: {}", item);
    // Use Validate to check if it's valid.
    if item.validate() {
        // Report valid items.
        println!("  ✓ Valid!");
    } else {
        // Report invalid items with the trait's error message.
        println!("  ✗ {}", item.error_message());
    }
}

// Using a `where` clause — identical behavior but more readable for complex bounds.
// Preferred when you have multiple generics or long bound lists.
fn validate_pair<T, U>(first: &T, second: &U) -> bool
where
    T: Validate + fmt::Display, // T must be validatable and displayable.
    U: Validate + fmt::Display, // U must also be validatable and displayable.
{
    // Print what we're validating.
    println!("  Validating pair: {} AND {}", first, second);
    // Both items must be valid for the pair to be valid.
    first.validate() && second.validate()
}

// ============================================================================
// Part 5: Returning impl Trait
// ============================================================================

// Return a type that implements Validate without exposing the concrete type.
// The caller only knows it gets "something that can be validated."
fn create_test_account() -> impl Validate + fmt::Display {
    // Construct and return an Account — but the caller sees `impl Validate`.
    Account {
        balance: 1_000_000,                         // 1 million lamports.
        owner: String::from("11111111111111111111"), // Fake public key string.
        is_initialized: true,                        // Mark as initialized.
    }
}

// ============================================================================
// Part 6: Generics on Structs
// ============================================================================

// A generic struct — `T` can be any type.
// This is like a template — the compiler fills in T with concrete types.
#[derive(Debug)]
struct Wallet<T> {
    // The wallet owner's identifier.
    owner: String,
    // The balance can be any numeric type (u64, f64, i128, etc.).
    balance: T,
}

// Implement methods for ALL Wallet<T> regardless of what T is.
impl<T> Wallet<T> {
    // Constructor that works for any T.
    fn new(owner: String, balance: T) -> Self {
        // Create and return a new Wallet with the given owner and balance.
        Wallet { owner, balance }
    }
}

// Implement methods ONLY for Wallet<T> where T implements Display.
// This is a "conditional implementation" — these methods only exist when T: Display.
impl<T: fmt::Display> Wallet<T> {
    // This method only exists when the balance type can be displayed.
    fn print_balance(&self) {
        // Print the owner and their balance using Display formatting.
        println!("  {}'s balance: {}", self.owner, self.balance);
    }
}

// ============================================================================
// Part 7: Generic Enum (custom Result-like type)
// ============================================================================

// A generic enum modeling a transaction outcome.
// Rust's standard Result<T, E> follows this exact pattern.
#[derive(Debug)]
enum TransactionResult<T> {
    // The transaction succeeded — carries a value of type T.
    Success(T),
    // The transaction failed due to insufficient funds — carries the deficit.
    InsufficientFunds { required: u64, available: u64 },
    // The transaction failed for an unknown reason — carries an error message.
    Failed(String),
}

// Implement methods on the generic enum.
impl<T: fmt::Debug> TransactionResult<T> {
    // Check whether this result represents success.
    fn is_success(&self) -> bool {
        // Use pattern matching — the idiomatic Rust way to inspect enums.
        matches!(self, TransactionResult::Success(_))
    }

    // Print the result with debug formatting.
    fn report(&self) {
        // Match on each variant and print appropriate output.
        match self {
            // Destructure the Success variant to get the inner value.
            TransactionResult::Success(val) => println!("  ✓ Success: {:?}", val),
            // Destructure InsufficientFunds to get both fields.
            TransactionResult::InsufficientFunds { required, available } => {
                // Show how much was needed vs how much was available.
                println!("  ✗ Insufficient funds: need {}, have {}", required, available);
            }
            // Destructure Failed to get the error message.
            TransactionResult::Failed(msg) => println!("  ✗ Failed: {}", msg),
        }
    }
}

// ============================================================================
// Part 8: Trait Objects and Dynamic Dispatch
// ============================================================================

// This function uses dynamic dispatch via `dyn Validate`.
// It accepts a SLICE of boxed trait objects — each element can be a DIFFERENT type.
// In Solidity, all interface calls are dynamically dispatched; in Rust, it's opt-in.
fn validate_all(items: &[Box<dyn Validate>]) {
    // Iterate over each boxed trait object in the slice.
    for (i, item) in items.iter().enumerate() {
        // Call validate() through the vtable (runtime lookup).
        if item.validate() {
            // Print valid status with the item's index.
            println!("  Item {}: ✓ Valid", i);
        } else {
            // Print invalid status with the error message.
            println!("  Item {}: ✗ {}", i, item.error_message());
        }
    }
}

// ============================================================================
// Part 9: Standard Trait Derivations in Action
// ============================================================================

// Derive many standard traits at once — the compiler writes all the boilerplate.
// This is one of Rust's biggest productivity features.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct ProgramId {
    // The program's address as a byte array (simplified from Solana's [u8; 32]).
    address: [u8; 8],
    // Human-readable label for this program.
    label: String,
}

// Implement Validate for ProgramId to demonstrate trait composition.
impl Validate for ProgramId {
    // A program ID is valid if its address is not all zeros.
    fn validate(&self) -> bool {
        // Check that at least one byte in the address is non-zero.
        self.address.iter().any(|&b| b != 0)
    }

    // Custom error for program ID validation.
    fn error_message(&self) -> String {
        // Return a descriptive error message.
        String::from("ProgramId address must not be all zeros")
    }
}

// Implement Display for ProgramId.
impl fmt::Display for ProgramId {
    // Format the program ID as a hex string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Map each byte to a two-character hex string and collect into one string.
        let hex: String = self.address.iter().map(|b| format!("{:02x}", b)).collect();
        // Write the label and hex address.
        write!(f, "Program({}: {})", self.label, hex)
    }
}

// ============================================================================
// Main Function — Run All Demonstrations
// ============================================================================

fn main() {
    // Print a section header for trait basics.
    println!("=== Part 1: Basic Trait Implementation ===");

    // Create a valid account with all fields populated.
    let valid_account = Account {
        balance: 1_000_000,                       // 1 million lamports.
        owner: String::from("Alice"),             // Owner's name.
        is_initialized: true,                      // Account is initialized.
    };

    // Create an invalid account with zero balance.
    let invalid_account = Account {
        balance: 0,                               // Zero balance — will fail validation.
        owner: String::from("Bob"),               // Owner is set.
        is_initialized: false,                     // Not initialized — another failure.
    };

    // Use Display trait (println with {}) to show the accounts.
    println!("Valid:   {}", valid_account);
    // Display the invalid account.
    println!("Invalid: {}", invalid_account);
    // Call validate() directly on each account.
    println!("Valid account passes validation: {}", valid_account.validate());
    // Show that the invalid account fails validation.
    println!("Invalid account passes validation: {}", invalid_account.validate());

    // Print a section header for generic functions.
    println!("\n=== Part 2: Generic Functions with Trait Bounds ===");

    // Create a valid transaction.
    let tx = Transaction {
        from: String::from("Alice"),   // Sender.
        to: String::from("Bob"),       // Receiver.
        amount: 500_000,               // 500k lamports.
    };

    // Create an invalid transaction (sending to yourself).
    let bad_tx = Transaction {
        from: String::from("Alice"),   // Sender.
        to: String::from("Alice"),     // Same as sender — invalid!
        amount: 100,                   // Small amount.
    };

    // Call the generic function with an Account — compiler generates Account-specific code.
    println!("Checking valid account:");
    check_validity(&valid_account);
    // Call the same generic function with a Transaction — different generated code.
    println!("Checking valid transaction:");
    check_validity(&tx);
    // Check an invalid transaction.
    println!("Checking bad transaction:");
    check_validity(&bad_tx);

    // Print a section header for multiple trait bounds.
    println!("\n=== Part 3: Multiple Trait Bounds (Validate + Display) ===");

    // Call check_and_display which requires both Validate AND Display.
    println!("Check and display account:");
    check_and_display(&valid_account);
    // Same function, different type — demonstrates generic flexibility.
    println!("Check and display transaction:");
    check_and_display(&tx);

    // Print a section header for the where clause demo.
    println!("\n=== Part 4: Where Clause — Validate Pair ===");

    // Call validate_pair with two different types (Account and Transaction).
    let pair_valid = validate_pair(&valid_account, &tx);
    // Print whether the pair passed validation.
    println!("  Pair valid: {}", pair_valid);

    // Try with an invalid combination.
    let pair_invalid = validate_pair(&invalid_account, &bad_tx);
    // This should be false since both items are invalid.
    println!("  Pair valid: {}", pair_invalid);

    // Print a section header for returning impl Trait.
    println!("\n=== Part 5: Returning impl Trait ===");

    // Get a value whose concrete type is hidden behind `impl Validate`.
    let test_account = create_test_account();
    // We can call Validate methods on it.
    println!("Test account valid: {}", test_account.validate());
    // We can also use Display since the return type is `impl Validate + Display`.
    println!("Test account: {}", test_account);

    // Print a section header for generic structs.
    println!("\n=== Part 6: Generic Structs ===");

    // Create a Wallet with u64 balance.
    let wallet_u64 = Wallet::new(String::from("Alice"), 1_000_000u64);
    // Print the balance — works because u64 implements Display.
    wallet_u64.print_balance();

    // Create a Wallet with f64 balance — same struct, different type parameter.
    let wallet_f64 = Wallet::new(String::from("Bob"), 3.14159f64);
    // Print this wallet's balance too.
    wallet_f64.print_balance();

    // Create a Wallet with a String balance — generics accept any type.
    let wallet_str = Wallet::new(String::from("Charlie"), String::from("∞ SOL"));
    // print_balance works because String implements Display.
    wallet_str.print_balance();

    // Print a section header for generic enums.
    println!("\n=== Part 7: Generic Enums ===");

    // Create a successful transaction result containing a confirmation string.
    let result1: TransactionResult<String> =
        TransactionResult::Success(String::from("tx_abc123"));
    // Create a failed result showing insufficient funds.
    let result2: TransactionResult<String> =
        TransactionResult::InsufficientFunds { required: 1000, available: 500 };
    // Create another failure with a generic error message.
    let result3: TransactionResult<String> =
        TransactionResult::Failed(String::from("Network timeout"));

    // Report each result — the method works on all variants.
    result1.report();
    result2.report();
    result3.report();

    // Check if results represent success.
    println!("  result1 is success: {}", result1.is_success());
    // This one should be false.
    println!("  result2 is success: {}", result2.is_success());

    // Print a section header for trait objects.
    println!("\n=== Part 8: Trait Objects (Dynamic Dispatch) ===");

    // Create a ProgramId for the demonstration.
    let program = ProgramId {
        address: [1, 2, 3, 4, 5, 6, 7, 8],       // Non-zero address bytes.
        label: String::from("TokenProgram"),        // Human-readable label.
    };

    // Create a vector of DIFFERENT types, all behind Box<dyn Validate>.
    // This is only possible with trait objects — generics require a single type.
    let items: Vec<Box<dyn Validate>> = vec![
        Box::new(valid_account.clone()),   // Clone the account and box it.
        Box::new(invalid_account.clone()), // Box an invalid account.
        Box::new(tx.clone()),              // Box a transaction.
        Box::new(bad_tx.clone()),          // Box an invalid transaction.
        Box::new(program.clone()),         // Box a program ID.
    ];

    // Pass the heterogeneous collection to validate_all.
    // Each call to validate() goes through a vtable (dynamic dispatch).
    validate_all(&items);

    // Print a section header for derived traits.
    println!("\n=== Part 9: Derived Standard Traits ===");

    // Create two ProgramIds to demonstrate derived traits.
    let prog_a = ProgramId {
        address: [1, 0, 0, 0, 0, 0, 0, 0],        // Only first byte is non-zero.
        label: String::from("ProgramA"),             // Label for program A.
    };

    // Create another ProgramId with a different address.
    let prog_b = ProgramId {
        address: [2, 0, 0, 0, 0, 0, 0, 0],        // Different first byte.
        label: String::from("ProgramB"),             // Label for program B.
    };

    // Clone prog_a — this works because we derived Clone.
    let prog_a_clone = prog_a.clone();

    // Use PartialEq (derived) to compare.
    println!("prog_a == prog_a_clone: {}", prog_a == prog_a_clone);
    // Compare two different programs — should be false.
    println!("prog_a == prog_b: {}", prog_a == prog_b);

    // Use Debug (derived) to print with {:?} formatting.
    println!("Debug format: {:?}", prog_a);
    // Use our custom Display implementation.
    println!("Display format: {}", prog_a);

    // Use Default (derived) to create a zeroed-out ProgramId.
    let default_prog = ProgramId::default();
    // Show what the default looks like.
    println!("Default ProgramId: {}", default_prog);
    // Validate it — should fail because address is all zeros.
    println!("Default valid: {}", default_prog.validate());

    // Use the default-or-panic pattern from our trait's default method.
    println!("\n=== Part 10: Default Method (validate_or_panic) ===");
    // This will succeed because valid_account passes validation.
    valid_account.validate_or_panic();
    // Print confirmation that we survived the panic check.
    println!("  valid_account passed validate_or_panic!");

    // Print a summary of what we covered.
    println!("\n=== Summary ===");
    println!("Covered: trait definition, implementation, default methods,");
    println!("  generic functions, trait bounds, where clauses, impl Trait,");
    println!("  generic structs/enums, trait objects, derive macros, Display.");
}
