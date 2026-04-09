// =============================================================================
// Module 01: Variables, Types, and Functions
// =============================================================================
// This program demonstrates every concept from the tutorial with runnable
// examples. Each section matches a section in tutorial.md.
// Run with: cargo run
// =============================================================================

fn main() {
    // =========================================================================
    // SECTION 1: Variable Bindings and Immutability
    // =========================================================================
    // In Rust, variables are immutable by default — the opposite of Solidity
    // where all variables are mutable. This prevents accidental state changes.

    println!("=== SECTION 1: Variable Bindings and Immutability ===\n");

    let x = 5; // Declare an immutable variable `x`. Type inferred as i32.
               // x = 6; // ERROR! Uncommenting this would fail: "cannot assign twice to immutable variable"
    println!("x = {} (immutable, cannot be changed)", x); // Print the value of x.

    let mut y = 10; // `mut` makes the variable mutable, like Solidity's default behavior.
    println!("y = {} (before mutation)", y); // Print y before changing it.
    y = 20; // This works because `y` was declared with `mut`.
    println!("y = {} (after mutation)", y); // Print y after changing it.

    // Naming convention: Rust uses snake_case for variables (Solidity uses camelCase).
    let my_balance = 1000; // Rust style: snake_case.
                           // uint256 myBalance = 1000;  // Solidity style: camelCase (for comparison).
    println!("my_balance = {} (snake_case naming)", my_balance); // Show the naming convention.

    println!(); // Print a blank line for readability.

    // =========================================================================
    // SECTION 2: Primitive Types — Integers
    // =========================================================================
    // Rust gives you explicit control over integer sizes. Solidity defaults to
    // uint256 because the EVM is 256-bit. Rust targets real hardware.

    println!("=== SECTION 2: Primitive Types — Integers ===\n");

    let a: u8 = 255; // Unsigned 8-bit: 0 to 255. Like Solidity's uint8.
    let b: u16 = 65_535; // Unsigned 16-bit. Underscores improve readability.
    let c: u32 = 4_294_967_295; // Unsigned 32-bit. Like Solidity's uint32.
    let d: u64 = 18_446_744_073_709_551_615; // Unsigned 64-bit. Used for lamports in Solana.
    let e: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455; // Unsigned 128-bit max.
    println!("u8  max: {}", a); // Print u8 max value.
    println!("u16 max: {}", b); // Print u16 max value.
    println!("u32 max: {}", c); // Print u32 max value.
    println!("u64 max: {}", d); // Print u64 max value.
    println!("u128 max: {}", e); // Print u128 max value.

    let f: i8 = -128; // Signed 8-bit: -128 to 127. Like Solidity's int8.
    let g: i32 = -2_147_483_648; // Signed 32-bit. This is Rust's default integer type.
    let h: i64 = -1_000_000; // Signed 64-bit. Like Solidity's int64.
    println!("i8  min: {}", f); // Print i8 min value.
    println!("i32 min: {}", g); // Print i32 min value.
    println!("i64 val: {}", h); // Print i64 value.

    let index: usize = 42; // Pointer-sized unsigned int. Used for array indexing.
    let offset: isize = -10; // Pointer-sized signed int. Used for pointer arithmetic.
    println!("usize: {} (used for array indices)", index); // Show usize usage.
    println!("isize: {} (pointer-sized signed)", offset); // Show isize usage.

    // Default inference: bare numbers default to i32 (not uint256 like Solidity).
    let default_int = 42; // Type inferred as i32, Rust's default integer type.
    println!("default_int = {} (inferred as i32)", default_int); // Demonstrate default inference.

    println!(); // Blank line separator.

    // =========================================================================
    // SECTION 3: Primitive Types — Floats, Bools, Chars
    // =========================================================================
    // Solidity has NO native floats. Rust has f32 and f64.
    // Avoid floats in on-chain Solana programs (non-deterministic).

    println!("=== SECTION 3: Floats, Bools, Chars ===\n");

    let pi: f64 = 3.141592653589793; // 64-bit float. Default float type in Rust.
    let half: f32 = 0.5; // 32-bit float. Less precision, rarely used.
    let default_float = 2.718; // Type inferred as f64, Rust's default float type.
    println!("f64 pi = {}", pi); // Print the f64 value.
    println!("f32 half = {}", half); // Print the f32 value.
    println!("default float = {} (inferred as f64)", default_float); // Show default inference.

    let is_active: bool = true; // Boolean type — same concept as Solidity's `bool`.
    let is_paused = false; // Type inferred as bool.
    println!("is_active = {}, is_paused = {}", is_active, is_paused); // Print both booleans.

    let letter: char = 'A'; // A char is a 4-byte Unicode scalar. Single quotes required.
    let emoji: char = '🦀'; // Rust's mascot — chars support full Unicode.
    let chinese: char = '中'; // Unicode characters from any language.
    println!(
        "letter = {}, emoji = {}, chinese = {}",
        letter, emoji, chinese
    ); // Print chars.

    println!(); // Blank line separator.

    // =========================================================================
    // SECTION 4: Type Annotations vs Inference
    // =========================================================================
    // Rust can infer types. You only need annotations when the compiler can't
    // figure it out, or when you want a specific non-default type.

    println!("=== SECTION 4: Type Annotations vs Inference ===\n");

    let inferred = 100; // Compiler infers i32 (the default integer type).
    let annotated: u64 = 100; // Explicit: we want unsigned 64-bit, not i32.
    let annotated_128: i128 = 100; // Explicit: we want signed 128-bit.
    println!("inferred (i32): {}", inferred); // Show the inferred type's value.
    println!("annotated (u64): {}", annotated); // Show the explicitly typed value.
    println!("annotated (i128): {}", annotated_128); // Show another explicitly typed value.

    // Suffix syntax: an alternative way to specify integer types.
    let suffix_u8 = 255u8; // Append the type as a suffix. Equivalent to `let x: u8 = 255;`.
    let suffix_i64 = -50i64; // Signed 64-bit via suffix.
    let suffix_f32 = 3.14f32; // Float 32 via suffix.
    println!("suffix u8: {}", suffix_u8); // Print the suffix-annotated u8.
    println!("suffix i64: {}", suffix_i64); // Print the suffix-annotated i64.
    println!("suffix f32: {}", suffix_f32); // Print the suffix-annotated f32.

    println!(); // Blank line separator.

    // =========================================================================
    // SECTION 5: Strings — String vs &str
    // =========================================================================
    // Rust has TWO string types. This has no direct Solidity equivalent because
    // Solidity's string is garbage-collected (sort of). Rust manages memory manually.

    println!("=== SECTION 5: Strings — String vs &str ===\n");

    let string_literal: &str = "hello, world"; // A string slice — stored in the binary, immutable.
    println!("&str literal: {}", string_literal); // Print the borrowed string slice.

    let owned_string: String = String::from("hello, Solana"); // Heap-allocated, owned string.
    println!("String (owned): {}", owned_string); // Print the owned String.

    let also_owned: String = "hello, Rust".to_string(); // Another way to create an owned String.
    println!("String (to_string): {}", also_owned); // Print the other owned String.

    // Converting between String and &str:
    let borrowed: &str = &owned_string; // Borrow a &str from a String (cheap, no copy).
    println!("Borrowed from String: {}", borrowed); // Print the borrowed reference.

    let from_borrowed: String = string_literal.to_owned(); // Create an owned String from &str (copies data).
    println!("Owned from &str: {}", from_borrowed); // Print the newly owned String.

    // String concatenation:
    let mut greeting = String::from("Hello"); // Create a mutable owned String.
    greeting.push_str(", world!"); // Append a &str to the String. push_str takes a &str.
    greeting.push('!'); // Push a single char with push().
    println!("Concatenated: {}", greeting); // Print the concatenated result.

    // String length:
    let msg = "Hello! 🦀"; // String literal with a multi-byte emoji.
    println!(
        "'{}' has {} bytes and {} chars",
        msg,
        msg.len(),
        msg.chars().count()
    ); // len() = bytes, chars().count() = Unicode chars.

    println!(); // Blank line separator.

    // =========================================================================
    // SECTION 6: Tuples
    // =========================================================================
    // Tuples group values of different types. Solidity's closest equivalent is
    // multi-return values from functions.

    println!("=== SECTION 6: Tuples ===\n");

    let point: (i32, f64, char) = (10, 3.14, 'A'); // A tuple with three different types.
    println!("Tuple: ({}, {}, {})", point.0, point.1, point.2); // Access elements by index (0-based).

    let (x_val, y_val, z_val) = point; // Destructure the tuple into individual variables.
    println!("Destructured: x={}, y={}, z={}", x_val, y_val, z_val); // Print destructured values.

    let single: (i32,) = (42,); // A single-element tuple requires a trailing comma.
    println!("Single-element tuple: {}", single.0); // Access the only element.

    let unit: () = (); // The unit type — an empty tuple. Like void.
    println!("Unit type: {:?}", unit); // Debug print the unit value.

    // Nested tuples:
    let nested = ((1, 2), (3, 4)); // Tuples can contain other tuples.
    println!(
        "Nested: ({}, {}), ({}, {})",
        (nested.0).0,
        (nested.0).1,
        (nested.1).0,
        (nested.1).1
    ); // Access nested elements.

    println!(); // Blank line separator.

    // =========================================================================
    // SECTION 7: Arrays (Fixed-Size)
    // =========================================================================
    // Rust arrays are fixed-size and stack-allocated, like Solidity's fixed arrays.
    // For dynamic arrays, Rust uses Vec<T> (covered in a later module).

    println!("=== SECTION 7: Arrays (Fixed-Size) ===\n");

    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // An array of 5 i32 values. [type; length].
    println!("Array: {:?}", numbers); // {:?} debug-prints the entire array.
    println!("First element: {}", numbers[0]); // Access by index, zero-based.
    println!("Last element: {}", numbers[4]); // Access the last element by index.
    println!("Array length: {}", numbers.len()); // .len() returns the number of elements.

    let zeros = [0u8; 10]; // Create an array of 10 zeros. [value; count] syntax.
    println!("Zeros array: {:?}", zeros); // Print the array of zeros.

    let booleans: [bool; 3] = [true, false, true]; // An array of booleans.
    println!("Booleans: {:?}", booleans); // Print the boolean array.

    // Iterating over an array:
    print!("Doubled: "); // Print without newline so numbers follow on same line.
    for num in numbers.iter() {
        // .iter() creates an iterator over references to each element.
        print!("{} ", num * 2); // Print each element doubled, separated by spaces.
    }
    println!(); // End the line after the loop.

    // Array slices:
    let slice: &[i32] = &numbers[1..4]; // A slice referencing elements at index 1, 2, 3.
    println!("Slice [1..4]: {:?}", slice); // Print the slice.

    println!(); // Blank line separator.

    // =========================================================================
    // SECTION 8: Functions
    // =========================================================================
    // Rust uses `fn` instead of Solidity's `function`. Parameters are name: Type
    // (reversed from Solidity's Type name). Return type uses `->`.

    println!("=== SECTION 8: Functions ===\n");

    let sum = add(3, 7); // Call the add function defined below main().
    println!("add(3, 7) = {}", sum); // Print the return value.

    let product = multiply(4, 5); // Call the multiply function.
    println!("multiply(4, 5) = {}", product); // Print the return value.

    greet("Solana Developer"); // Call a function that returns nothing (unit type).

    let (quot, rem) = divide_with_remainder(17, 5); // Destructure the tuple return value.
    println!("17 / 5 = {} remainder {}", quot, rem); // Print quotient and remainder.

    let result = apply_twice(square, 3); // Pass a function as an argument.
    println!("square applied twice to 3 = {}", result); // Print: square(square(3)) = 81.

    // Demonstrating expressions vs statements:
    let expr_result = {
        // A block `{}` is an expression if the last line has no semicolon.
        let a = 10; // Statement: declares a variable (has semicolon).
        let b = 20; // Statement: declares another variable.
        a + b // Expression: no semicolon — this value (30) is the block's result.
    };
    println!("Block expression result: {}", expr_result); // Print 30.

    println!(); // Blank line separator.

    // =========================================================================
    // SECTION 9: Type Casting with `as`
    // =========================================================================
    // Solidity uses uint64(x) syntax. Rust uses `x as u64`.
    // Warning: `as` can silently truncate. Use .try_into() for checked casts.

    println!("=== SECTION 9: Type Casting with `as` ===\n");

    let small: i32 = 42; // Start with an i32 value.
    let widened: u64 = small as u64; // Widen: i32 → u64 (safe, no data loss).
    println!("i32 {} as u64 = {}", small, widened); // Print the widened value.

    let big: u32 = 300; // A u32 value larger than u8 can hold.
    let truncated: u8 = big as u8; // Narrow: u32 → u8 (truncates! 300 % 256 = 44).
    println!("u32 {} as u8 = {} (truncated!)", big, truncated); // Show the truncation.

    let float_val: f64 = 3.99; // A float value.
    let as_int: i32 = float_val as i32; // Float → int truncates toward zero (not rounding).
    println!(
        "f64 {} as i32 = {} (truncated, not rounded)",
        float_val, as_int
    ); // Print 3.

    let negative: i32 = -1; // A negative i32.
    let as_unsigned: u32 = negative as u32; // Negative → unsigned wraps around.
    println!("i32 {} as u32 = {} (wraps around!)", negative, as_unsigned); // Print 4294967295.

    let char_val: char = 'A'; // A char value.
    let as_u32: u32 = char_val as u32; // Char → u32 gives the Unicode code point.
    println!(
        "char '{}' as u32 = {} (Unicode code point)",
        char_val, as_u32
    ); // Print 65.

    let code_point: u8 = 66; // A u8 representing ASCII 'B'.
    let as_char: char = code_point as char; // u8 → char converts the code point to a character.
    println!("u8 {} as char = '{}'", code_point, as_char); // Print 'B'.

    println!(); // Blank line separator.

    // =========================================================================
    // SECTION 10: Constants and Statics
    // =========================================================================
    // `const` is like Solidity's `constant` — inlined at compile time.
    // `static` is a global with a fixed memory address.

    println!("=== SECTION 10: Constants and Statics ===\n");

    // Constants are defined outside or inside functions. They're inlined by the compiler.
    println!("MAX_SUPPLY = {}", MAX_SUPPLY); // Access the const defined at module level below.
    println!("MIN_BALANCE = {}", MIN_BALANCE); // Access another module-level const.
    println!("PROGRAM_VERSION = {}", PROGRAM_VERSION); // Access the static string slice.
    println!("PI = {}", PI); // Access the float constant.

    // Constants can also be defined inside a function (local scope):
    const LOCAL_CONST: u32 = 999; // This const is only visible in this scope.
    println!("LOCAL_CONST = {}", LOCAL_CONST); // Print the local constant.

    println!(); // Blank line separator.

    // =========================================================================
    // SECTION 11: Shadowing
    // =========================================================================
    // Rust lets you re-declare a variable with the same name. The new binding
    // "shadows" the old one. Solidity does NOT allow this.

    println!("=== SECTION 11: Shadowing ===\n");

    let shadow = 5; // First binding: shadow is i32 with value 5.
    println!("shadow = {} (original i32)", shadow); // Print: 5.

    let shadow = shadow + 10; // Second binding: shadows the first. Now value is 15.
    println!("shadow = {} (after + 10)", shadow); // Print: 15.

    let shadow = shadow * 2; // Third binding: shadows again. Now value is 30.
    println!("shadow = {} (after * 2)", shadow); // Print: 30.

    let shadow = "now I'm a string!"; // Fourth binding: TYPE CHANGES from i32 to &str!
    println!("shadow = {} (now a &str!)", shadow); // Print the string value.

    // Shadowing vs mut — they're different:
    let spaces = "   "; // A &str with 3 spaces.
    let spaces = spaces.len(); // Shadow with the length (usize). Type changes &str → usize.
    println!("spaces (length) = {}", spaces); // Print: 3.

    // With mut, you CAN'T change the type:
    // let mut spaces = "   ";
    // spaces = spaces.len();  // ERROR: expected `&str`, found `usize`

    println!(); // Blank line separator.

    // =========================================================================
    // SECTION 12: The Unit Type ()
    // =========================================================================
    // () is Rust's "void". Functions that don't return a value return ().
    // Very common in Solana: Result<(), ProgramError>.

    println!("=== SECTION 12: The Unit Type () ===\n");

    let nothing: () = (); // The unit value assigned to a variable.
    println!("Unit value: {:?}", nothing); // Debug print: ().

    let result = print_greeting("Rustacean"); // This function returns () implicitly.
    println!("print_greeting returned: {:?}", result); // Print: ().

    // if/else is an expression in Rust, and branches returning () is common:
    let is_valid = true; // A boolean for the condition.
    let _check: () = if is_valid {
        // The block returns () because println! returns ().
        println!("Valid!"); // Side effect: prints to console.
    } else {
        println!("Invalid!"); // Side effect: prints to console.
    };

    println!(); // Blank line separator.

    // =========================================================================
    // SECTION 13: Printing with println! and Format Strings
    // =========================================================================
    // println! is a macro (the ! is the giveaway). Similar to Hardhat's console.log.

    println!("=== SECTION 13: Printing with println! and Format Strings ===\n");

    let name = "Ferris"; // A &str for the name.
    let age = 7; // An i32 for the age.

    println!("Simple: Hello, world!"); // No placeholders — just a literal string.
    println!("Positional: {} is {} years old", name, age); // {} placeholders filled in order.
    println!("Inline: {name} is {age} years old"); // Inline variable names (Rust 1.58+).
    println!("Debug: {:?}", (1, "two", 3.0)); // {:?} uses the Debug trait for complex types.
    println!("Pretty debug:\n{:#?}", [1, 2, 3, 4, 5]); // {:#?} pretty-prints with indentation.
    println!("Hex: 255 = 0x{:x}", 255); // {:x} formats as lowercase hexadecimal.
    println!("HEX: 255 = 0x{:X}", 255); // {:X} formats as uppercase hexadecimal.
    println!("Octal: 255 = 0o{:o}", 255); // {:o} formats as octal.
    println!("Binary: 255 = 0b{:b}", 255); // {:b} formats as binary.
    println!("Padded right: '{:>10}'", "right"); // {:>10} right-aligns in a 10-char field.
    println!("Padded left:  '{:<10}'", "left"); // {:<10} left-aligns in a 10-char field.
    println!("Padded center: '{:^10}'", "center"); // {:^10} centers in a 10-char field.
    println!("Zero-padded: {:05}", 42); // {:05} pads with zeros to 5 digits.
    println!("Two decimals: {:.2}", 3.14159); // {:.2} limits to 2 decimal places.
    println!("Numbered: {0} then {1} then {0}", "first", "second"); // Numbered positions for reuse.

    // eprintln! prints to stderr instead of stdout (useful for error messages):
    eprintln!("This goes to stderr (not captured by `> file`)"); // Prints to standard error.

    println!(); // Blank line separator.

    // =========================================================================
    // DONE!
    // =========================================================================
    println!("=== All sections complete! ===");
    println!("Now try the exercises in exercises/src/main.rs");
}

// =============================================================================
// Module-Level Constants and Statics (Section 10)
// =============================================================================

const MAX_SUPPLY: u64 = 1_000_000_000; // A compile-time constant. Like Solidity's `constant`. Inlined everywhere.
const MIN_BALANCE: u64 = 890_880; // Minimum rent-exempt balance in lamports (Solana concept).
const PI: f64 = 3.14159265358979; // A float constant.
static PROGRAM_VERSION: &str = "1.0.0"; // A static string — has a fixed memory address, unlike const.

// =============================================================================
// Functions (Section 8)
// =============================================================================

/// Adds two i32 values and returns the result.
/// In Solidity: function add(int32 a, int32 b) pure returns (int32)
fn add(a: i32, b: i32) -> i32 {
    // Parameters are name: Type. Return type after `->`.
    a + b // No semicolon = expression = implicit return value.
}

/// Multiplies two i32 values using explicit `return`.
/// Both styles work; the expression form (no return keyword) is more idiomatic.
fn multiply(a: i32, b: i32) -> i32 {
    // Same signature pattern as add().
    return a * b; // Explicit return — works but not idiomatic Rust.
}

/// Prints a greeting. Returns () (unit type) — like Solidity's function with no returns.
fn greet(name: &str) {
    // Takes a &str (borrowed string slice) parameter. No `-> Type` = returns ().
    println!("Hello, {}! Welcome to Rust.", name); // Print the greeting.
}

/// Returns a tuple of (quotient, remainder). Like Solidity's multi-return.
fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    // Returns a tuple of two i32s.
    let quotient = dividend / divisor; // Integer division truncates toward zero.
    let remainder = dividend % divisor; // Modulo operator gives the remainder.
    (quotient, remainder) // Return both as a tuple (expression, no semicolon).
}

/// Takes a function as a parameter and applies it twice.
/// Demonstrates that functions are first-class values in Rust (not possible in Solidity).
fn apply_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    // `fn(i32) -> i32` is a function pointer type.
    f(f(x)) // Apply f to x, then apply f to the result.
}

/// Squares a number. Used as an argument to apply_twice().
fn square(n: i32) -> i32 {
    // A simple function that squares its input.
    n * n // Return n squared.
}

/// A function that explicitly returns the unit type.
fn print_greeting(name: &str) -> () {
    // Explicit `-> ()` return type (normally omitted).
    println!("Greetings, {}!", name); // Print the greeting as a side effect.
                                      // No expression at the end — implicitly returns ().
}
