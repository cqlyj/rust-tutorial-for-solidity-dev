// =============================================================================
// Module 01 Solutions: Variables, Types, and Functions
// =============================================================================
// These are the completed exercises with explanations on every line.
// Run with `cargo run` to verify everything compiles and produces correct output.
// =============================================================================

fn main() {
    println!("=== Module 01 Solutions ===\n");

    // =========================================================================
    // Exercise 1: Variable Bindings and Mutability
    // =========================================================================
    println!("--- Exercise 1: Variable Bindings ---");

    let language: &str = "Rust"; // Immutable &str binding with value "Rust".
    let mut level: i32 = 1; // Mutable i32 binding starting at 1. `mut` allows reassignment.
    println!("Level before mutation: {}", level); // Read the value before reassigning to avoid unused-assignment warning.
    level = 2; // Reassign level to 2. This works because `level` is declared `mut`.

    println!("Language: {}, Level: {}", language, level); // Print: Language: Rust, Level: 2
    println!(); // Blank line separator.

    // =========================================================================
    // Exercise 2: Integer Types and Type Annotations
    // =========================================================================
    println!("--- Exercise 2: Integer Types ---");

    let lamports: u64 = 1_000_000_000; // 1 SOL in lamports. u64 is the standard for Solana amounts.
    let temperature: i16 = -40; // Signed 16-bit integer. Negative values need signed types (i-prefix).
    let byte_val = 255u8; // u8 suffix syntax. Same as `let byte_val: u8 = 255;`.

    println!("Lamports: {}, Temperature: {}, Byte: {}", lamports, temperature, byte_val); // Print all three values.
    println!(); // Blank line separator.

    // =========================================================================
    // Exercise 3: Strings — String vs &str
    // =========================================================================
    println!("--- Exercise 3: Strings ---");

    let greeting_slice: &str = "Hello"; // A string slice — a reference to a string literal in the binary.
    let greeting_owned: String = String::from(greeting_slice); // Convert &str to owned String using String::from().
    let mut full_greeting: String = greeting_owned; // Move the owned String into full_greeting (now mutable).
    full_greeting.push_str(", Solana!"); // Append a &str to the mutable String. push_str borrows the argument.

    println!("Full greeting: {}", full_greeting); // Print: Full greeting: Hello, Solana!
    println!(); // Blank line separator.

    // =========================================================================
    // Exercise 4: Tuples and Destructuring
    // =========================================================================
    println!("--- Exercise 4: Tuples ---");

    let wallet: (String, u64, bool) = ( // Create a tuple with three different types.
        "7xKXxQR2pYGF".to_string(), // First element: an owned String (address).
        5_000_000_000,              // Second element: u64 (balance in lamports = 5 SOL).
        true,                       // Third element: bool (account is initialized).
    );
    let (address, balance, initialized) = wallet; // Destructure the tuple into three named variables.

    println!("Address: {}, Balance: {} lamports, Initialized: {}", address, balance, initialized); // Print all three.
    println!(); // Blank line separator.

    // =========================================================================
    // Exercise 5: Arrays and Indexing
    // =========================================================================
    println!("--- Exercise 5: Arrays ---");

    let scores: [u32; 5] = [85, 92, 78, 95, 88]; // Fixed-size array of 5 u32 values.
    let zeros: [u8; 8] = [0u8; 8]; // Create 8 zeros using [value; count] repeat syntax.

    println!("Third score: {}", scores[2]); // Print element at index 2 (0-based), which is 78.
    println!("Zeros length: {}", zeros.len()); // .len() returns 8.
    println!("All scores: {:?}", scores); // {:?} debug-prints the entire array.
    println!(); // Blank line separator.

    // =========================================================================
    // Exercise 6: Functions
    // =========================================================================
    println!("--- Exercise 6: Functions ---");

    let my_lamports: u64 = 2_500_000_000; // 2.5 SOL in lamports.
    let sol_amount = lamports_to_sol(my_lamports); // Convert lamports to SOL using our function.
    println!("{} lamports = {} SOL", my_lamports, sol_amount); // Print: 2500000000 lamports = 2.5 SOL

    let rent_check = is_rent_exempt(1_000_000); // Check if 1M lamports is rent-exempt.
    println!("1_000_000 lamports rent exempt? {}", rent_check); // Print: true (1M > 890_880).

    let pair = (10, 20); // A tuple of two i32 values.
    let swapped = swap_pair(pair); // Swap the elements.
    println!("({}, {}) swapped = ({}, {})", pair.0, pair.1, swapped.0, swapped.1); // Print: (10, 20) swapped = (20, 10).
    println!(); // Blank line separator.

    // =========================================================================
    // Exercise 7: Type Casting
    // =========================================================================
    println!("--- Exercise 7: Type Casting ---");

    let as_float: f64 = 1_000_000_000u64 as f64; // Cast u64 to f64 using the `as` keyword.
    let truncated: i32 = 3.7f64 as i32; // Cast f64 to i32 — truncates toward zero, so 3.7 becomes 3.
    let code_point: u32 = 'Z' as u32; // Cast char to u32 — gives the Unicode code point (90 for 'Z').

    println!("u64 as f64: {}", as_float); // Print: 1000000000 (as a float).
    println!("f64 3.7 as i32: {}", truncated); // Print: 3 (truncated, not rounded).
    println!("'Z' as u32: {}", code_point); // Print: 90 (Unicode/ASCII code for 'Z').
    println!(); // Blank line separator.

    // =========================================================================
    // Exercise 8: Shadowing and Constants
    // =========================================================================
    println!("--- Exercise 8: Shadowing and Constants ---");

    const DECIMALS: u32 = 9; // SOL has 9 decimal places. Constants must have explicit type annotations.

    let amount = "1000000000"; // Start as a &str string literal.
    let amount: u64 = amount.parse::<u64>().unwrap(); // Shadow: parse the string into u64. unwrap() panics if parsing fails.
    let amount: f64 = amount as f64 / 10f64.powi(DECIMALS as i32); // Shadow: convert to SOL by dividing by 10^9. Type changes to f64.

    println!("{} SOL", amount); // Print: 1 SOL
    println!(); // Blank line separator.

    // =========================================================================
    println!("=== All solutions verified! ===");
}

// =============================================================================
// Exercise 6 Function Solutions
// =============================================================================

/// Convert lamports (u64) to SOL (f64). 1 SOL = 1_000_000_000 lamports.
fn lamports_to_sol(lamports: u64) -> f64 {
    lamports as f64 / 1_000_000_000.0 // Cast lamports to f64 then divide by 1 billion. Expression form (no semicolon).
}

/// Check if a balance meets the minimum rent-exempt threshold (890_880 lamports).
fn is_rent_exempt(balance: u64) -> bool {
    balance >= 890_880 // Compare balance to the rent-exempt minimum. Returns true or false. Expression form.
}

/// Swap the two elements of a tuple.
fn swap_pair(pair: (i32, i32)) -> (i32, i32) {
    (pair.1, pair.0) // Return a new tuple with the elements in reverse order. Expression form.
}
