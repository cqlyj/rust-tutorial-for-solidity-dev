// =============================================================================
// Module 01 Exercises: Variables, Types, and Functions
// =============================================================================
// Complete each exercise by replacing `todo!()` or the TODO comments with
// working code. Run with `cargo run` to test your solutions.
//
// The `todo!()` macro compiles but panics at runtime — so you can compile
// the file and work through exercises one at a time. Comment out later
// exercises if you want to run earlier ones without panicking.
// =============================================================================

fn main() {
    println!("=== Module 01 Exercises ===\n");

    // =========================================================================
    // Exercise 1: Variable Bindings and Mutability
    // =========================================================================
    // a) Declare an immutable variable `language` with the value "Rust" (&str).
    // b) Declare a mutable variable `level` with the initial value 1 (i32).
    // c) Change `level` to 2.
    // d) Print both values.
    println!("--- Exercise 1: Variable Bindings ---");

    let language: &str = todo!(); // TODO: Replace todo!() with "Rust"
    let mut level: i32 = todo!(); // TODO: Replace todo!() with 1
    // TODO: Change `level` to 2 on the next line

    println!("Language: {}, Level: {}", language, level);
    println!();

    // =========================================================================
    // Exercise 2: Integer Types and Type Annotations
    // =========================================================================
    // a) Declare a variable `lamports` of type u64 with value 1_000_000_000.
    //    (This represents 1 SOL in Solana's smallest unit.)
    // b) Declare a variable `temperature` of type i16 with value -40.
    // c) Declare a variable `byte_val` using the u8 suffix syntax with value 255.
    // d) Print all three.
    println!("--- Exercise 2: Integer Types ---");

    let lamports: u64 = todo!(); // TODO: 1_000_000_000
    let temperature: i16 = todo!(); // TODO: -40
    let byte_val: u8 = todo!(); // TODO: Use suffix syntax, e.g. 255u8

    println!("Lamports: {}, Temperature: {}, Byte: {}", lamports, temperature, byte_val);
    println!();

    // =========================================================================
    // Exercise 3: Strings — String vs &str
    // =========================================================================
    // a) Create a &str variable `greeting_slice` with value "Hello".
    // b) Create an owned String `greeting_owned` from the slice using String::from().
    // c) Create a mutable String `full_greeting` and append ", Solana!" to it using push_str().
    // d) Print the final `full_greeting`.
    println!("--- Exercise 3: Strings ---");

    let greeting_slice: &str = todo!(); // TODO: "Hello"
    let greeting_owned: String = todo!(); // TODO: Convert greeting_slice to an owned String
    let mut full_greeting: String = greeting_owned; // Start with the owned string.
    // TODO: Use full_greeting.push_str() to append ", Solana!" on the next line

    println!("Full greeting: {}", full_greeting);
    println!();

    // =========================================================================
    // Exercise 4: Tuples and Destructuring
    // =========================================================================
    // a) Create a tuple `wallet` of type (String, u64, bool) representing
    //    (address, balance_in_lamports, is_initialized).
    //    Use: ("7xKX...".to_string(), 5_000_000_000, true)
    // b) Destructure the tuple into three variables: address, balance, initialized.
    // c) Print all three.
    println!("--- Exercise 4: Tuples ---");

    let wallet: (String, u64, bool) = todo!(); // TODO: Create the tuple
    let (address, balance, initialized) = wallet; // Destructure it.

    println!("Address: {}, Balance: {} lamports, Initialized: {}", address, balance, initialized);
    println!();

    // =========================================================================
    // Exercise 5: Arrays and Indexing
    // =========================================================================
    // a) Create an array `scores` of type [u32; 5] with values [85, 92, 78, 95, 88].
    // b) Create an array `zeros` containing 8 zeros of type u8 using the [value; count] syntax.
    // c) Print the third element of `scores` (index 2).
    // d) Print the length of `zeros`.
    println!("--- Exercise 5: Arrays ---");

    let scores: [u32; 5] = todo!(); // TODO: [85, 92, 78, 95, 88]
    let zeros: [u8; 8] = todo!(); // TODO: Use [0u8; 8] syntax

    println!("Third score: {}", scores[2]);
    println!("Zeros length: {}", zeros.len());
    println!("All scores: {:?}", scores);
    println!();

    // =========================================================================
    // Exercise 6: Functions
    // =========================================================================
    // a) Complete the `lamports_to_sol` function below. It should take a u64
    //    (lamports) and return an f64 (SOL). There are 1_000_000_000 lamports per SOL.
    // b) Complete the `is_rent_exempt` function below. It should return true if
    //    the balance is >= 890_880 lamports.
    // c) Complete the `swap_pair` function that takes a tuple (i32, i32) and
    //    returns a tuple with the values swapped.
    println!("--- Exercise 6: Functions ---");

    let my_lamports: u64 = 2_500_000_000;
    let sol_amount = lamports_to_sol(my_lamports);
    println!("{} lamports = {} SOL", my_lamports, sol_amount);

    let rent_check = is_rent_exempt(1_000_000);
    println!("1_000_000 lamports rent exempt? {}", rent_check);

    let pair = (10, 20);
    let swapped = swap_pair(pair);
    println!("({}, {}) swapped = ({}, {})", pair.0, pair.1, swapped.0, swapped.1);
    println!();

    // =========================================================================
    // Exercise 7: Type Casting
    // =========================================================================
    // a) Cast the u64 value 1_000_000_000u64 to f64 and store it in `as_float`.
    // b) Cast the f64 value 3.7 to i32 and store it in `truncated`.
    // c) Cast the char 'Z' to u32 and store it in `code_point`.
    // d) Print all three.
    println!("--- Exercise 7: Type Casting ---");

    let as_float: f64 = todo!(); // TODO: Cast 1_000_000_000u64 to f64
    let truncated: i32 = todo!(); // TODO: Cast 3.7f64 to i32
    let code_point: u32 = todo!(); // TODO: Cast 'Z' to u32

    println!("u64 as f64: {}", as_float);
    println!("f64 3.7 as i32: {}", truncated);
    println!("'Z' as u32: {}", code_point);
    println!();

    // =========================================================================
    // Exercise 8: Shadowing and Constants
    // =========================================================================
    // a) Define a constant `DECIMALS` of type u32 with value 9 (SOL has 9 decimals).
    // b) Start with `let amount = "1000000000"` (a string).
    // c) Shadow `amount` by parsing it into a u64.
    //    Hint: use `amount.parse::<u64>().unwrap()`
    // d) Shadow `amount` again by dividing it by 10^DECIMALS to get the SOL amount as f64.
    // e) Print the final amount.
    println!("--- Exercise 8: Shadowing and Constants ---");

    // TODO: Change the 0 to the correct number of decimals for SOL (hint: 9)
    const DECIMALS: u32 = 0;

    let amount = "1000000000"; // Start as a &str.
    let amount: u64 = todo!(); // TODO: Parse the string into u64 using amount.parse::<u64>().unwrap()
    let amount: f64 = todo!(); // TODO: Convert to SOL (divide by 10^DECIMALS using 10f64.powi(DECIMALS as i32))

    println!("{} SOL", amount);
    println!();

    // =========================================================================
    println!("=== All exercises complete! ===");
}

// =============================================================================
// Exercise 6 Function Stubs
// =============================================================================

/// Convert lamports (u64) to SOL (f64). 1 SOL = 1_000_000_000 lamports.
fn lamports_to_sol(lamports: u64) -> f64 {
    todo!() // TODO: Return lamports as f64 divided by 1_000_000_000.0
}

/// Check if a balance meets the minimum rent-exempt threshold (890_880 lamports).
fn is_rent_exempt(balance: u64) -> bool {
    todo!() // TODO: Return true if balance >= 890_880
}

/// Swap the two elements of a tuple.
fn swap_pair(pair: (i32, i32)) -> (i32, i32) {
    todo!() // TODO: Return (pair.1, pair.0)
}
