// =============================================================================
// Module 08: Macros and Attributes
// =============================================================================
// Macros generate code at compile time. Anchor is 90% macros.
// Understanding macros = understanding what Anchor does behind the scenes.
// Solidity has NO macro system — this is entirely new territory.
// =============================================================================

// -----------------------------------------------------------------------------
// Section 1: Standard Library Macros You Already Know
// -----------------------------------------------------------------------------

// This function demonstrates macros you've been using since Module 01.
// Every function call with a `!` is a macro, not a regular function.
fn standard_macros_demo() {
    // println! — prints formatted text to stdout (like console.log in JS/Solidity)
    println!("=== Section 1: Standard Library Macros ===");

    // println! accepts variable numbers of arguments — impossible for regular functions
    println!("No args");                         // 0 format arguments
    println!("One: {}", 42);                     // 1 format argument
    println!("Two: {} and {}", "hello", 3.14);   // 2 format arguments
    println!("Inline: {x}", x = 99);             // named argument

    // format! — like println! but returns a String instead of printing
    let greeting: String = format!("Hello, {}!", "Solana dev"); // creates a heap-allocated String
    println!("{}", greeting);                                    // print the formatted string

    // vec! — creates a Vec (dynamic array) with initial values
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5]; // expands to Vec::new() + push() calls
    println!("Vec: {:?}", numbers);                // {:?} uses the Debug trait to print

    // vec! with repeated values — [value; count] syntax
    let zeros: Vec<i32> = vec![0; 5]; // creates [0, 0, 0, 0, 0]
    println!("Zeros: {:?}", zeros);    // prints the five zeros

    // assert! — panics if the condition is false (used in tests)
    assert!(1 + 1 == 2); // passes silently — condition is true

    // assert_eq! — panics if two values aren't equal, showing both values
    assert_eq!(2 + 2, 4); // passes silently — values are equal

    // assert_ne! — panics if two values ARE equal (the opposite of assert_eq!)
    assert_ne!(1, 2); // passes silently — values are not equal

    // dbg! — prints the expression and its value, returns the value (great for debugging)
    let debug_val = dbg!(5 * 10); // prints: [src/main.rs:LINE] 5 * 10 = 50
    println!("debug_val = {}", debug_val); // the value is passed through

    // todo! — marks unfinished code, panics at runtime with "not yet implemented"
    // Uncomment to see the panic:
    // todo!("implement this later");

    // concat! — concatenates string literals at compile time (zero runtime cost)
    let combined: &str = concat!("Hello", " ", "World"); // becomes "Hello World" at compile time
    println!("concat!: {}", combined);                     // prints the compile-time result

    // stringify! — converts tokens to a string literal at compile time
    let code_as_string: &str = stringify!(1 + 2 + 3); // becomes "1 + 2 + 3" (the text, not 6)
    println!("stringify!: {}", code_as_string);         // prints the literal token text

    // include_str! — includes a file as a &str at compile time (like Solidity's import but for data)
    // let contents = include_str!("../Cargo.toml"); // would embed Cargo.toml as a string
    // We skip this to keep the example self-contained

    println!(); // empty line for readability
}

// -----------------------------------------------------------------------------
// Section 2: Writing Your Own Declarative Macros
// -----------------------------------------------------------------------------

// A simple macro that takes no arguments
// The empty () pattern matches a call like say_hello!()
macro_rules! say_hello {
    // () is the pattern — matches an empty argument list
    () => {
        // This block is the "expansion" — the code that replaces the macro call
        println!("Hello from a macro!");
    };
}

// A macro with one argument — $name:expr captures any expression
macro_rules! greet {
    // $name:expr means "capture an expression and call it $name"
    ($name:expr) => {
        // $name is replaced with whatever expression was passed in
        println!("Hello, {}! Welcome to Rust macros.", $name);
    };
}

// A macro with multiple pattern arms — like a match expression for syntax
macro_rules! math_op {
    // Pattern 1: matches "add <expr>, <expr>"
    (add $a:expr, $b:expr) => {
        $a + $b // expands to the sum of the two expressions
    };
    // Pattern 2: matches "mul <expr>, <expr>"
    (mul $a:expr, $b:expr) => {
        $a * $b // expands to the product of the two expressions
    };
    // Pattern 3: matches "square <expr>"
    (square $a:expr) => {
        $a * $a // expands to the expression multiplied by itself
    };
}

// A macro using repetition — $(...),* matches zero or more comma-separated items
macro_rules! make_list {
    // $( $item:expr ),* means "zero or more expressions separated by commas"
    ( $( $item:expr ),* ) => {
        {
            // Create a new vector to hold the items
            let mut temp_vec = Vec::new();
            // For each captured $item, push it into the vector
            // $( ... )* repeats the block for each captured item
            $(
                temp_vec.push($item); // this line is generated once per item
            )*
            // Return the populated vector as the macro's value
            temp_vec
        }
    };
}

// A macro that creates a HashMap literal (Rust has no built-in syntax for this)
macro_rules! map {
    // $( $key:expr => $value:expr ),* matches "key => value" pairs
    // $(,)? optionally matches a trailing comma for ergonomics
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            // Create a new HashMap
            let mut m = ::std::collections::HashMap::new();
            // Insert each key-value pair
            $(
                m.insert($key, $value); // generated once per key-value pair
            )*
            // Return the populated map
            m
        }
    };
}

// A macro that simulates Solana's msg! — logs with a prefix
// This shows how Anchor's msg! could work conceptually
macro_rules! sol_log {
    // Match a format string and zero or more arguments (like println!)
    ($($arg:tt)*) => {
        // Prefix every log with [Program log:] like Solana does
        println!("[Program log:] {}", format!($($arg)*));
    };
}

// A macro that creates a struct with a constructor — reducing boilerplate
macro_rules! new_struct {
    // $name:ident captures an identifier (the struct name)
    // $( $field:ident : $ty:ty ),* captures field name-type pairs
    ($name:ident { $( $field:ident : $ty:ty ),* $(,)? }) => {
        // Generate the struct definition with Debug derive
        #[derive(Debug)] // auto-implement Debug so we can print it
        struct $name {
            $( $field: $ty, )* // generate each field declaration
        }

        // Generate an impl block with a new() constructor
        impl $name {
            // Constructor that takes all fields as parameters
            fn new($( $field: $ty ),*) -> Self {
                // Return a new instance with all fields set
                Self { $( $field, )* }
            }
        }
    };
}

// Use the new_struct! macro to define a Token struct and its constructor
new_struct!(Token {
    symbol: String,    // the token's ticker symbol
    decimals: u8,      // number of decimal places
    supply: u64,       // total supply in smallest unit
});

// This function demonstrates all the custom macros defined above
fn custom_macros_demo() {
    println!("=== Section 2: Custom Declarative Macros ===");

    // Using our no-argument macro
    say_hello!(); // expands to println!("Hello from a macro!")

    // Using our single-argument macro
    greet!("Solidity Developer"); // passes a string expression

    // Using our multi-arm math macro
    let sum = math_op!(add 10, 20);       // expands to 10 + 20
    let product = math_op!(mul 5, 6);     // expands to 5 * 6
    let squared = math_op!(square 7);     // expands to 7 * 7
    println!("add: {}, mul: {}, square: {}", sum, product, squared); // 30, 30, 49

    // Using our repetition macro to create a vector
    let colors = make_list!["red", "green", "blue"]; // creates Vec<&str>
    println!("Colors: {:?}", colors);                  // prints the vector

    // Using our HashMap macro — Rust has no built-in map literal
    let scores = map! {
        "Alice" => 100,   // key => value pairs
        "Bob" => 85,      // separated by commas
        "Charlie" => 92,  // trailing comma is okay thanks to $(,)?
    };
    println!("Scores: {:?}", scores); // prints the HashMap

    // Using our Solana-like logging macro
    sol_log!("Transfer complete: {} lamports", 1_000_000); // prefixed output
    sol_log!("Account initialized");                        // simple message

    // Using the struct created by our new_struct! macro
    let token = Token::new(          // new() was generated by the macro
        String::from("SOL"),         // symbol field
        9,                           // decimals field (SOL has 9 decimals)
        1_000_000_000,               // supply field
    );
    println!("Token: {:?}", token); // Debug was derived by the macro

    println!(); // empty line for readability
}

// -----------------------------------------------------------------------------
// Section 3: Derive Macros — Auto-Implementing Traits
// -----------------------------------------------------------------------------

// #[derive(Debug)] — auto-generates Debug trait (enables {:?} formatting)
// #[derive(Clone)] — auto-generates Clone trait (enables .clone())
// #[derive(PartialEq)] — auto-generates == and != operators
// #[derive(Default)] — auto-generates Default::default() with zero/empty values
#[derive(Debug, Clone, PartialEq, Default)]
struct WalletInfo {
    address: String, // the wallet's public key as a string
    balance: u64,    // balance in lamports (smallest SOL unit)
    is_active: bool, // whether the wallet is currently active
}

// #[derive(Debug, PartialEq)] on enums works just like on structs
#[derive(Debug, PartialEq)]
enum NetworkStatus {
    Connected,                  // variant with no data
    Disconnected,               // variant with no data
    Error(String),              // variant carrying an error message
}

// Demonstrate how derive macros reduce boilerplate
fn derive_macros_demo() {
    println!("=== Section 3: Derive Macros ===");

    // Default::default() creates a WalletInfo with "", 0, false
    let empty_wallet = WalletInfo::default(); // all fields get their default values
    println!("Default wallet: {:?}", empty_wallet); // Debug lets us print the struct

    // Create a wallet with actual data
    let wallet = WalletInfo {
        address: String::from("7xKX...abc"),   // a fake Solana address
        balance: 5_000_000_000,                 // 5 SOL in lamports
        is_active: true,                        // wallet is active
    };
    println!("My wallet: {:?}", wallet); // {:?} works because of #[derive(Debug)]

    // Clone creates a deep copy — the original is unaffected
    let wallet_copy = wallet.clone(); // .clone() works because of #[derive(Clone)]
    println!("Copy: {:?}", wallet_copy); // identical to the original

    // PartialEq enables == comparison
    assert_eq!(wallet, wallet_copy); // passes — clone creates an equal copy
    println!("wallet == wallet_copy: {}", wallet == wallet_copy); // true

    // PartialEq also works on enums
    let status1 = NetworkStatus::Connected;           // first status
    let status2 = NetworkStatus::Connected;           // second status (same variant)
    let status3 = NetworkStatus::Error("timeout".to_string()); // different variant
    println!("Connected == Connected: {}", status1 == status2); // true
    println!("Connected == Error: {}", status1 == status3);     // false

    println!(); // empty line for readability
}

// -----------------------------------------------------------------------------
// Section 4: Attributes in Action
// -----------------------------------------------------------------------------

// #[allow(dead_code)] suppresses the "unused function" warning
// Without this, the compiler warns that this function is never called
#[allow(dead_code)]
fn unused_helper() -> u64 {
    42 // this function exists but isn't called from main
}

// #[repr(C)] forces C-compatible memory layout — fields stay in declaration order
// Critical for Solana: account data must have predictable byte layout
#[repr(C)]
#[derive(Debug)]
struct AccountData {
    is_initialized: bool, // 1 byte — first in memory
    padding: [u8; 7],     // 7 bytes — explicit padding for alignment
    balance: u64,         // 8 bytes — aligned to 8-byte boundary
    owner: [u8; 32],      // 32 bytes — a public key (like Solana's Pubkey)
}

// #[inline] suggests the compiler inline this function for performance
// The compiler may ignore this hint if inlining wouldn't help
#[inline]
fn add_lamports(current: u64, amount: u64) -> u64 {
    current.saturating_add(amount) // saturating_add prevents overflow (caps at u64::MAX)
}

// This function demonstrates various attributes and their effects
fn attributes_demo() {
    println!("=== Section 4: Attributes in Action ===");

    // Using our #[repr(C)] struct — memory layout is predictable
    let account = AccountData {
        is_initialized: true,     // first byte is 1 (true)
        padding: [0; 7],          // next 7 bytes are zeros
        balance: 1_000_000_000,   // next 8 bytes: 1 SOL in lamports
        owner: [1; 32],           // next 32 bytes: fake public key (all 1s)
    };
    println!("Account: {:?}", account); // Debug shows all fields

    // std::mem::size_of shows the byte size of a type
    println!(
        "AccountData size: {} bytes",      // should be 1 + 7 + 8 + 32 = 48
        std::mem::size_of::<AccountData>() // compile-time size calculation
    );

    // Using our #[inline] function
    let new_balance = add_lamports(account.balance, 500_000_000); // add 0.5 SOL
    println!("New balance: {} lamports", new_balance); // 1_500_000_000

    // Demonstrating saturating_add prevents overflow
    let max_balance = add_lamports(u64::MAX, 1); // would overflow without saturating
    println!("Saturated max: {}", max_balance);    // stays at u64::MAX

    println!(); // empty line for readability
}

// -----------------------------------------------------------------------------
// Section 5: Simulated Anchor Patterns
// -----------------------------------------------------------------------------
// We can't use real Anchor (it requires the solana SDK), but we can simulate
// the patterns to understand what macros generate behind the scenes.

// Simulate what #[account] does — adds serialization trait implementations
// In real Anchor, #[account] generates BorshSerialize + BorshDeserialize
#[derive(Debug, Clone)]
struct SimulatedAccount {
    discriminator: [u8; 8], // Anchor uses 8-byte discriminators to identify account types
    value: u64,             // the actual data stored in the account
}

// Simulate what #[derive(Accounts)] generates — validation logic
// In real Anchor, this is all generated by the derive macro
struct SimulatedContext {
    account: SimulatedAccount,    // the deserialized account data
    signer_is_valid: bool,        // whether the signer check passed
    account_is_owned: bool,       // whether the account is owned by this program
}

// Simulate account validation — what Anchor's derive macro generates
impl SimulatedContext {
    // This is roughly what #[derive(Accounts)] generates for validation
    fn validate(&self) -> Result<(), String> {
        // Check 1: is the signer valid? (Anchor checks Signer type)
        if !self.signer_is_valid {
            return Err("Invalid signer".to_string()); // Anchor returns AccountNotSigner error
        }
        // Check 2: is the account owned by our program? (Anchor checks ownership)
        if !self.account_is_owned {
            return Err("Account not owned by program".to_string()); // Anchor returns AccountOwnedByWrongProgram
        }
        // All checks passed
        Ok(()) // Anchor proceeds to deserialize and call the instruction handler
    }
}

// Simulate what declare_id! does — stores the program's public key
const PROGRAM_ID: &str = "11111111111111111111111111111111"; // in real Anchor, this is a Pubkey type

// Simulate what msg! does — logs with Solana's log format
macro_rules! program_log {
    ($($arg:tt)*) => {
        // In real Solana, this calls sol_log_ syscall
        println!("Program log: {}", format!($($arg)*));
    };
}

// Simulate an Anchor instruction handler
fn simulated_initialize(ctx: &SimulatedContext, initial_value: u64) -> Result<String, String> {
    // Step 1: Anchor validates accounts (generated by #[derive(Accounts)])
    ctx.validate()?; // the ? operator propagates errors (covered in Module 04)

    // Step 2: Our instruction logic (what we write inside #[program] functions)
    program_log!("Initializing account with value: {}", initial_value);

    // Step 3: Return success (in real Anchor, account changes are auto-serialized)
    Ok(format!("Account initialized with value {}", initial_value))
}

// This function ties together the simulated Anchor patterns
fn anchor_simulation_demo() {
    println!("=== Section 5: Simulated Anchor Patterns ===");

    // Show what declare_id! stores
    println!("Program ID: {}", PROGRAM_ID); // in real code, this is a Pubkey

    // Create a valid context (all checks will pass)
    let valid_ctx = SimulatedContext {
        account: SimulatedAccount {
            discriminator: [0x01; 8], // 8-byte type discriminator
            value: 0,                  // initial value before our instruction
        },
        signer_is_valid: true,  // the transaction was properly signed
        account_is_owned: true, // the account belongs to our program
    };

    // Call the instruction handler with a valid context
    match simulated_initialize(&valid_ctx, 42) {
        Ok(msg) => println!("Success: {}", msg),    // prints the success message
        Err(e) => println!("Error: {}", e),          // would print the error
    }

    // Create an invalid context (signer check will fail)
    let invalid_ctx = SimulatedContext {
        account: SimulatedAccount {
            discriminator: [0x01; 8], // same account type
            value: 0,                  // same initial value
        },
        signer_is_valid: false, // NO valid signer — this will fail validation
        account_is_owned: true, // account ownership is fine
    };

    // Call with invalid context — demonstrates Anchor's validation behavior
    match simulated_initialize(&invalid_ctx, 100) {
        Ok(msg) => println!("Success: {}", msg),    // won't reach here
        Err(e) => println!("Error (expected): {}", e), // prints the signer error
    }

    println!(); // empty line for readability
}

// -----------------------------------------------------------------------------
// Section 6: Conditional Compilation with #[cfg]
// -----------------------------------------------------------------------------

// This function only exists in debug builds (cargo build, cargo run)
// In release builds (cargo build --release), it's completely removed
#[cfg(debug_assertions)]
fn debug_only_check() {
    println!("  [DEBUG] Running extra validation checks..."); // only in debug mode
    println!("  [DEBUG] This code is removed in --release builds"); // zero cost in prod
}

// This function only exists in release builds
// We won't see this when running cargo run (debug mode)
#[cfg(not(debug_assertions))]
fn debug_only_check() {
    println!("  [RELEASE] Running in release mode, debug checks skipped");
}

// Demonstrate conditional compilation
fn conditional_compilation_demo() {
    println!("=== Section 6: Conditional Compilation ===");

    // This calls whichever version of debug_only_check exists for our build mode
    debug_only_check(); // debug version when using `cargo run`, release version for `cargo run --release`

    // cfg! macro — evaluates a cfg condition as a bool at compile time
    let is_debug = cfg!(debug_assertions); // true in debug builds, false in release
    println!("Debug mode: {}", is_debug);   // prints true when using `cargo run`

    // cfg! with target OS — check what platform we're compiling for
    let os_name = if cfg!(target_os = "linux") {
        "Linux"     // we're on Linux (or WSL)
    } else if cfg!(target_os = "macos") {
        "macOS"     // we're on macOS
    } else if cfg!(target_os = "windows") {
        "Windows"   // we're on Windows
    } else {
        "Unknown"   // some other OS
    };
    println!("Target OS: {}", os_name); // prints the detected OS

    // cfg! with architecture
    let arch = if cfg!(target_arch = "x86_64") {
        "x86_64 (64-bit Intel/AMD)" // most desktop/server CPUs
    } else if cfg!(target_arch = "aarch64") {
        "aarch64 (64-bit ARM)"      // Apple Silicon, some servers
    } else {
        "other architecture"         // less common architectures
    };
    println!("Target arch: {}", arch); // prints the detected architecture

    println!(); // empty line for readability
}

// -----------------------------------------------------------------------------
// Section 7: Macro Patterns for Solana Development
// -----------------------------------------------------------------------------

// A macro that generates error enums — common pattern in Solana programs
// Anchor's #[error_code] attribute macro does something similar
macro_rules! define_errors {
    // Match a list of: variant_name => error_code, "message"
    ( $( $variant:ident => $code:expr, $msg:expr );* $(;)? ) => {
        // Generate an error enum with Debug and PartialEq
        #[derive(Debug, PartialEq)]
        enum ProgramError {
            $( $variant, )* // one variant per error defined
        }

        // Generate a Display implementation for human-readable messages
        impl std::fmt::Display for ProgramError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $( ProgramError::$variant => write!(f, "Error {}: {}", $code, $msg), )*
                }
            }
        }

        // Generate a method to get the numeric error code
        impl ProgramError {
            #[allow(dead_code)] // some variants might not be used in this demo
            fn code(&self) -> u32 {
                match self {
                    $( ProgramError::$variant => $code, )*
                }
            }
        }
    };
}

// Use the macro to define program errors — similar to Anchor's #[error_code]
define_errors! {
    InsufficientFunds => 6000, "Insufficient funds for transfer";
    InvalidOwner => 6001, "Account owner does not match expected program";
    AlreadyInitialized => 6002, "Account is already initialized";
    Overflow => 6003, "Arithmetic overflow detected";
}

// A macro that generates a simple instruction dispatcher — like Anchor's #[program]
macro_rules! instruction_dispatch {
    // Match instruction name => handler pairs
    ( $( $name:ident => $handler:expr ),* $(,)? ) => {
        // Generate a dispatch function that routes by instruction name
        fn dispatch(instruction: &str) -> Result<String, String> {
            match instruction {
                $( stringify!($name) => $handler, )* // stringify! converts the ident to "&str"
                _ => Err(format!("Unknown instruction: {}", instruction)),
            }
        }
    };
}

// Use the dispatch macro to create an instruction router
instruction_dispatch! {
    initialize => Ok("Account initialized".to_string()),              // simulated init instruction
    transfer => Ok("Transfer complete".to_string()),                  // simulated transfer instruction
    close => Ok("Account closed and lamports returned".to_string()),  // simulated close instruction
}

// Demonstrate Solana-oriented macro patterns
fn solana_patterns_demo() {
    println!("=== Section 7: Macro Patterns for Solana ===");

    // Using our custom error enum (generated by define_errors!)
    let err = ProgramError::InsufficientFunds;          // create an error value
    println!("Error: {}", err);                          // Display shows code + message
    println!("Error code: {}", err.code());              // numeric code for client handling
    println!("Is InsufficientFunds: {}", err == ProgramError::InsufficientFunds); // PartialEq

    // Using our instruction dispatcher (generated by instruction_dispatch!)
    println!("\nDispatching instructions:");
    for instruction in &["initialize", "transfer", "close", "unknown"] {
        match dispatch(instruction) {
            Ok(msg) => println!("  {} -> Success: {}", instruction, msg),   // handler returned Ok
            Err(e) => println!("  {} -> Error: {}", instruction, e),        // handler returned Err
        }
    }

    println!(); // empty line for readability
}

// -----------------------------------------------------------------------------
// Main — runs all the demos
// -----------------------------------------------------------------------------

fn main() {
    // Section 1: macros you already know from previous modules
    standard_macros_demo();

    // Section 2: writing your own declarative macros with macro_rules!
    custom_macros_demo();

    // Section 3: derive macros auto-implement traits (Debug, Clone, etc.)
    derive_macros_demo();

    // Section 4: attributes control compiler behavior (#[repr(C)], #[inline], etc.)
    attributes_demo();

    // Section 5: simulated Anchor patterns to preview Solana development
    anchor_simulation_demo();

    // Section 6: conditional compilation with #[cfg(...)]
    conditional_compilation_demo();

    // Section 7: macro patterns commonly used in Solana programs
    solana_patterns_demo();

    // Final message
    println!("=== All demos complete! ===");
    println!("Run `cargo test` to see the test module (Section 8).");
}

// -----------------------------------------------------------------------------
// Section 8: Test Module with #[cfg(test)]
// -----------------------------------------------------------------------------

// #[cfg(test)] ensures this entire module is only compiled when running `cargo test`
// It is completely stripped from release builds — zero cost
#[cfg(test)]
mod tests {
    // Import everything from the parent module so tests can access our types/functions
    use super::*;

    // #[test] marks this function as a test case — `cargo test` discovers and runs it
    #[test]
    fn test_math_op_macro() {
        // Verify our math_op! macro produces correct results
        assert_eq!(math_op!(add 2, 3), 5);    // 2 + 3 = 5
        assert_eq!(math_op!(mul 4, 5), 20);   // 4 * 5 = 20
        assert_eq!(math_op!(square 6), 36);   // 6 * 6 = 36
    }

    #[test]
    fn test_make_list_macro() {
        // Verify make_list! creates a vector with the right elements
        let list = make_list![10, 20, 30];        // create a 3-element vector
        assert_eq!(list.len(), 3);                 // should have 3 elements
        assert_eq!(list, vec![10, 20, 30]);        // elements should match
    }

    #[test]
    fn test_make_list_empty() {
        // Verify make_list! handles zero elements (the * means zero-or-more)
        let empty: Vec<i32> = make_list![];        // create an empty vector
        assert!(empty.is_empty());                  // should have no elements
    }

    #[test]
    fn test_map_macro() {
        // Verify our map! macro creates a working HashMap
        let m = map! { "a" => 1, "b" => 2 };      // create a 2-entry map
        assert_eq!(m.get("a"), Some(&1));           // key "a" maps to 1
        assert_eq!(m.get("b"), Some(&2));           // key "b" maps to 2
        assert_eq!(m.get("c"), None);               // key "c" doesn't exist
    }

    #[test]
    fn test_wallet_default() {
        // Verify WalletInfo::default() creates sensible zero values
        let w = WalletInfo::default();              // all fields get defaults
        assert_eq!(w.address, "");                  // String default is ""
        assert_eq!(w.balance, 0);                   // u64 default is 0
        assert!(!w.is_active);                      // bool default is false
    }

    #[test]
    fn test_wallet_clone_and_eq() {
        // Verify Clone and PartialEq work together correctly
        let w1 = WalletInfo {
            address: String::from("test"),          // a test address
            balance: 100,                           // 100 lamports
            is_active: true,                        // active wallet
        };
        let w2 = w1.clone();                        // clone creates a deep copy
        assert_eq!(w1, w2);                         // cloned values should be equal
    }

    #[test]
    fn test_account_data_size() {
        // Verify #[repr(C)] produces the expected memory layout
        let size = std::mem::size_of::<AccountData>(); // get byte size
        assert_eq!(size, 48);                           // 1 + 7 + 8 + 32 = 48 bytes
    }

    #[test]
    fn test_add_lamports_normal() {
        // Verify normal addition works
        assert_eq!(add_lamports(100, 50), 150);    // 100 + 50 = 150
    }

    #[test]
    fn test_add_lamports_saturating() {
        // Verify overflow is handled by saturating at u64::MAX
        assert_eq!(add_lamports(u64::MAX, 1), u64::MAX); // caps at max, doesn't wrap
    }

    #[test]
    fn test_simulated_context_valid() {
        // Verify validation passes with a valid context
        let ctx = SimulatedContext {
            account: SimulatedAccount {
                discriminator: [1; 8],              // valid discriminator
                value: 0,                            // initial value
            },
            signer_is_valid: true,                   // valid signer
            account_is_owned: true,                  // correct ownership
        };
        assert!(ctx.validate().is_ok());             // should pass validation
    }

    #[test]
    fn test_simulated_context_invalid_signer() {
        // Verify validation fails when signer is invalid
        let ctx = SimulatedContext {
            account: SimulatedAccount {
                discriminator: [1; 8],              // valid discriminator
                value: 0,                            // initial value
            },
            signer_is_valid: false,                  // INVALID signer
            account_is_owned: true,                  // correct ownership
        };
        assert!(ctx.validate().is_err());            // should fail validation
    }

    #[test]
    fn test_program_error_codes() {
        // Verify error codes match what we defined in the macro
        assert_eq!(ProgramError::InsufficientFunds.code(), 6000); // first error code
        assert_eq!(ProgramError::InvalidOwner.code(), 6001);      // second error code
        assert_eq!(ProgramError::Overflow.code(), 6003);          // fourth error code
    }

    #[test]
    fn test_dispatch_known_instructions() {
        // Verify the dispatcher routes known instructions correctly
        assert!(dispatch("initialize").is_ok());    // known instruction -> Ok
        assert!(dispatch("transfer").is_ok());      // known instruction -> Ok
        assert!(dispatch("close").is_ok());         // known instruction -> Ok
    }

    #[test]
    fn test_dispatch_unknown_instruction() {
        // Verify the dispatcher rejects unknown instructions
        let result = dispatch("hack");              // unknown instruction
        assert!(result.is_err());                    // should return Err
    }

    #[test]
    fn test_network_status_eq() {
        // Verify enum PartialEq works correctly
        assert_eq!(NetworkStatus::Connected, NetworkStatus::Connected);       // same variant
        assert_ne!(NetworkStatus::Connected, NetworkStatus::Disconnected);    // different variants
    }
}
