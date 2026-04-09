// ============================================================================
// Module 02 Exercises: Ownership, Borrowing, and Lifetimes
// ============================================================================
// Replace every `todo!()` with your implementation.
// The program should compile and run with all exercises passing.
// Run with: cargo run
// ============================================================================

fn main() {
    println!("=== Module 02 Exercises ===\n");

    exercise_1();
    exercise_2();
    exercise_3();
    exercise_4();
    exercise_5();
    exercise_6();
    exercise_7();
    exercise_8();
    exercise_9();
    exercise_10();

    println!("\n=== All exercises passed! ===");
}

// ============================================================================
// Exercise 1: Fix the Move Error
// ============================================================================
// This code has a move error. Fix it so both println! statements work.
// Hint: You need to keep s1 valid after creating s2.
fn exercise_1() {
    println!("--- Exercise 1: Fix the Move Error ---");

    let s1 = String::from("hello");

    // TODO: Fix this line so that s1 remains valid after this assignment.
    // Currently `let s2 = s1;` would move s1. Change it so both work.
    let s2: String = todo!();

    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
    assert_eq!(s1, "hello");
    assert_eq!(s2, "hello");
    println!("Exercise 1 passed!\n");
}

// ============================================================================
// Exercise 2: Ownership and Functions
// ============================================================================
// Write a function that takes a String, prints it, and returns it back
// so the caller can keep using it.
fn exercise_2() {
    println!("--- Exercise 2: Ownership and Functions ---");

    let message = String::from("Ownership is power");
    let message = print_and_return(message);
    println!("Still have: {}", message);
    assert_eq!(message, "Ownership is power");
    println!("Exercise 2 passed!\n");
}

/// Takes ownership of a String, prints it, then returns it.
fn print_and_return(s: String) -> String {
    todo!()
}

// ============================================================================
// Exercise 3: Borrowing Instead of Moving
// ============================================================================
// Write a function that borrows a String (immutable reference) and returns its length.
// The caller should still be able to use the String afterward.
fn exercise_3() {
    println!("--- Exercise 3: Borrowing Instead of Moving ---");

    let word = String::from("borrowing");
    let length = get_length(&word);
    println!("'{}' has {} characters", word, length);
    assert_eq!(length, 9);
    println!("Exercise 3 passed!\n");
}

/// Returns the length of the borrowed string.
fn get_length(s: &String) -> usize {
    todo!()
}

// ============================================================================
// Exercise 4: Mutable References
// ============================================================================
// Write a function that takes a mutable reference to a String
// and appends " world" to it.
fn exercise_4() {
    println!("--- Exercise 4: Mutable References ---");

    let mut greeting = String::from("hello");
    add_world(&mut greeting);
    println!("{}", greeting);
    assert_eq!(greeting, "hello world");
    println!("Exercise 4 passed!\n");
}

/// Appends " world" to the given String through a mutable reference.
fn add_world(s: &mut String) {
    todo!()
}

// ============================================================================
// Exercise 5: Choose the Right Parameter Type
// ============================================================================
// Write a function that counts the number of spaces in a string.
// Choose &str as the parameter type so it works with both String and &str.
fn exercise_5() {
    println!("--- Exercise 5: Choose the Right Parameter Type ---");

    let owned = String::from("hello beautiful world");
    let literal = "one two three four";

    let count1 = count_spaces(&owned);
    let count2 = count_spaces(literal);

    println!("'{}' has {} spaces", owned, count1);
    println!("'{}' has {} spaces", literal, count2);
    assert_eq!(count1, 2);
    assert_eq!(count2, 3);
    println!("Exercise 5 passed!\n");
}

/// Counts the number of spaces in a string slice.
fn count_spaces(s: &str) -> usize {
    todo!()
}

// ============================================================================
// Exercise 6: Working with Slices
// ============================================================================
// Write a function that takes a slice of i32 and returns the largest value.
// The function should work with both Vec<i32> and arrays.
fn exercise_6() {
    println!("--- Exercise 6: Working with Slices ---");

    let vec_nums = vec![3, 7, 2, 9, 4];
    let arr_nums = [10, 5, 8, 1, 6];

    let max1 = find_largest(&vec_nums);
    let max2 = find_largest(&arr_nums);

    println!("Largest in vec: {}", max1);
    println!("Largest in array: {}", max2);
    assert_eq!(max1, 9);
    assert_eq!(max2, 10);
    println!("Exercise 6 passed!\n");
}

/// Returns the largest value in a slice of i32.
/// You can assume the slice is non-empty.
fn find_largest(numbers: &[i32]) -> i32 {
    todo!()
}

// ============================================================================
// Exercise 7: String Slices
// ============================================================================
// Write a function that returns the last word of a sentence as a &str slice.
// The last word is everything after the last space (or the whole string if no space).
fn exercise_7() {
    println!("--- Exercise 7: String Slices ---");

    let sentence = String::from("the quick brown fox");
    let last = last_word(&sentence);
    println!("Last word: {}", last);
    assert_eq!(last, "fox");

    let single = String::from("hello");
    let last2 = last_word(&single);
    assert_eq!(last2, "hello");
    println!("Exercise 7 passed!\n");
}

/// Returns the last word of a string (everything after the last space).
fn last_word(s: &str) -> &str {
    todo!()
}

// ============================================================================
// Exercise 8: Lifetime Annotations
// ============================================================================
// The function signature below uses lifetime 'a to say:
// "the returned reference lives as long as BOTH inputs."
// Implement the body: return whichever string is shorter.
fn exercise_8() {
    println!("--- Exercise 8: Lifetime Annotations ---");

    let s1 = String::from("short");
    let s2 = String::from("much longer string");

    let shorter = find_shorter(&s1, &s2);
    println!("Shorter: '{}'", shorter);
    assert_eq!(shorter, "short");

    let shorter2 = find_shorter("abc", "ab");
    assert_eq!(shorter2, "ab");
    println!("Exercise 8 passed!\n");
}

/// Returns the shorter of two string slices.
/// The lifetime 'a means: the return value lives as long as both x and y.
/// TODO: Implement the body — return whichever slice is shorter.
fn find_shorter<'a>(x: &'a str, y: &'a str) -> &'a str {
    todo!()
}

// ============================================================================
// Exercise 9: Struct with Lifetime
// ============================================================================
// Create a struct that holds a borrowed string slice and implement a method on it.
fn exercise_9() {
    println!("--- Exercise 9: Struct with Lifetime ---");

    let text = String::from("Rust is great for Solana development");
    let highlight = Highlight { content: &text };
    let display = highlight.display();
    println!("{}", display);
    assert_eq!(display, ">>> Rust is great for Solana development <<<");
    println!("Exercise 9 passed!\n");
}

/// A struct that holds a reference to a string slice.
/// TODO: Add the correct lifetime parameter.
struct Highlight<'a> {
    content: &'a str,
}

/// TODO: Implement a `display` method that returns a new String
/// wrapping the content with ">>> " prefix and " <<<" suffix.
impl<'a> Highlight<'a> {
    fn display(&self) -> String {
        todo!()
    }
}

// ============================================================================
// Exercise 10: Putting It All Together
// ============================================================================
// Write a function that takes a mutable reference to a Vec<String>,
// removes all strings shorter than `min_len`, and returns the count of removed items.
// This exercises ownership (Vec owns Strings), borrowing (&mut Vec), and slices.
fn exercise_10() {
    println!("--- Exercise 10: Putting It All Together ---");

    let mut words = vec![
        String::from("hi"),
        String::from("hello"),
        String::from("hey"),
        String::from("greetings"),
        String::from("yo"),
    ];

    let removed = remove_short_words(&mut words, 4);
    println!("Removed {} words", removed);
    println!("Remaining: {:?}", words);
    assert_eq!(removed, 3);
    assert_eq!(words.len(), 2);
    assert_eq!(words[0], "hello");
    assert_eq!(words[1], "greetings");
    println!("Exercise 10 passed!\n");
}

/// Removes all Strings from the Vec that are shorter than `min_len`.
/// Returns the number of removed items.
fn remove_short_words(words: &mut Vec<String>, min_len: usize) -> usize {
    todo!()
}
