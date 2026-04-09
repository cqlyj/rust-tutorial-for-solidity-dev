// ============================================================================
// Module 02 Solutions: Ownership, Borrowing, and Lifetimes
// ============================================================================
// Every line is explained. This compiles and runs with all exercises passing.
// Run with: cargo run
// ============================================================================

fn main() {
    println!("=== Module 02 Solutions ===\n");

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
// Solution: Use .clone() to create a deep copy so s1 stays valid.
fn exercise_1() {
    println!("--- Exercise 1: Fix the Move Error ---");

    let s1 = String::from("hello");   // s1 owns the String "hello" on the heap.

    let s2: String = s1.clone();       // .clone() creates a full deep copy on the heap.
    // s1 is NOT moved because we cloned — both s1 and s2 own independent copies.

    println!("s1 = {}", s1);          // s1 is still valid because we cloned, not moved.
    println!("s2 = {}", s2);          // s2 owns its own independent copy.
    assert_eq!(s1, "hello");          // Verify s1 is unchanged.
    assert_eq!(s2, "hello");          // Verify s2 has the same content.
    println!("Exercise 1 passed!\n");
}

// ============================================================================
// Exercise 2: Ownership and Functions
// ============================================================================
// Solution: Print the string and return it so ownership goes back to the caller.
fn exercise_2() {
    println!("--- Exercise 2: Ownership and Functions ---");

    let message = String::from("Ownership is power"); // message owns the String.
    let message = print_and_return(message);           // message is moved into the function, then moved back.
    println!("Still have: {}", message);               // We got it back — message is valid.
    assert_eq!(message, "Ownership is power");         // Content unchanged.
    println!("Exercise 2 passed!\n");
}

/// Takes ownership of a String, prints it, then returns it.
fn print_and_return(s: String) -> String { // s takes ownership of the String.
    println!("Printing: {}", s);            // Use the String — we own it.
    s                                       // Return it — ownership moves back to the caller.
} // s is NOT dropped because it was moved out via the return.

// ============================================================================
// Exercise 3: Borrowing Instead of Moving
// ============================================================================
// Solution: Use the .len() method on the borrowed reference.
fn exercise_3() {
    println!("--- Exercise 3: Borrowing Instead of Moving ---");

    let word = String::from("borrowing");  // word owns the String.
    let length = get_length(&word);         // &word borrows — word stays valid.
    println!("'{}' has {} characters", word, length); // Both word and length are usable.
    assert_eq!(length, 9);                  // "borrowing" is 9 characters.
    println!("Exercise 3 passed!\n");
}

/// Returns the length of the borrowed string.
fn get_length(s: &String) -> usize { // s is an immutable reference — borrows, doesn't own.
    s.len()                           // .len() returns the number of bytes in the String.
} // s (the reference) goes out of scope. The String itself is NOT dropped.

// ============================================================================
// Exercise 4: Mutable References
// ============================================================================
// Solution: Use push_str() to append to the String through the mutable reference.
fn exercise_4() {
    println!("--- Exercise 4: Mutable References ---");

    let mut greeting = String::from("hello"); // Must be mut to allow mutable borrowing.
    add_world(&mut greeting);                  // &mut greeting passes a mutable reference.
    println!("{}", greeting);                  // "hello world" — it was modified in place.
    assert_eq!(greeting, "hello world");       // Verify the mutation happened.
    println!("Exercise 4 passed!\n");
}

/// Appends " world" to the given String through a mutable reference.
fn add_world(s: &mut String) {  // s is a mutable reference — can modify the String.
    s.push_str(" world");        // push_str appends a &str to the String in place.
} // s (the mutable reference) goes out of scope. The String is NOT dropped.

// ============================================================================
// Exercise 5: Choose the Right Parameter Type
// ============================================================================
// Solution: Filter characters and count spaces using the chars() iterator.
fn exercise_5() {
    println!("--- Exercise 5: Choose the Right Parameter Type ---");

    let owned = String::from("hello beautiful world"); // Owned String.
    let literal = "one two three four";                 // &str literal.

    let count1 = count_spaces(&owned);                  // &String auto-derefs to &str.
    let count2 = count_spaces(literal);                 // &str passed directly.

    println!("'{}' has {} spaces", owned, count1);     // "hello beautiful world" → 2 spaces.
    println!("'{}' has {} spaces", literal, count2);   // "one two three four" → 3 spaces.
    assert_eq!(count1, 2);                              // Verify.
    assert_eq!(count2, 3);                              // Verify.
    println!("Exercise 5 passed!\n");
}

/// Counts the number of spaces in a string slice.
fn count_spaces(s: &str) -> usize {    // &str accepts both &String and &str.
    s.chars()                           // Get an iterator over characters.
        .filter(|&c| c == ' ')         // Keep only space characters.
        .count()                        // Count how many passed the filter.
}

// ============================================================================
// Exercise 6: Working with Slices
// ============================================================================
// Solution: Iterate through the slice tracking the maximum value.
fn exercise_6() {
    println!("--- Exercise 6: Working with Slices ---");

    let vec_nums = vec![3, 7, 2, 9, 4];   // Vec<i32> on the heap.
    let arr_nums = [10, 5, 8, 1, 6];       // Array on the stack.

    let max1 = find_largest(&vec_nums);     // &Vec<i32> auto-derefs to &[i32].
    let max2 = find_largest(&arr_nums);     // &[i32; 5] auto-derefs to &[i32].

    println!("Largest in vec: {}", max1);  // 9
    println!("Largest in array: {}", max2); // 10
    assert_eq!(max1, 9);                    // Verify.
    assert_eq!(max2, 10);                   // Verify.
    println!("Exercise 6 passed!\n");
}

/// Returns the largest value in a slice of i32.
fn find_largest(numbers: &[i32]) -> i32 {   // Takes a slice — borrows the data.
    let mut largest = numbers[0];            // Start with the first element.
    for &num in &numbers[1..] {              // Iterate over the rest, dereferencing each &i32.
        if num > largest {                   // If current is bigger than our max...
            largest = num;                   // ...update the max.
        }
    }
    largest                                  // Return the largest value found.
}

// ============================================================================
// Exercise 7: String Slices
// ============================================================================
// Solution: Find the last space and return everything after it.
fn exercise_7() {
    println!("--- Exercise 7: String Slices ---");

    let sentence = String::from("the quick brown fox"); // Owned String.
    let last = last_word(&sentence);                     // Returns &str slice into sentence.
    println!("Last word: {}", last);                     // "fox"
    assert_eq!(last, "fox");                              // Verify.

    let single = String::from("hello");                  // No spaces.
    let last2 = last_word(&single);                      // Should return the whole string.
    assert_eq!(last2, "hello");                           // Verify.
    println!("Exercise 7 passed!\n");
}

/// Returns the last word of a string (everything after the last space).
fn last_word(s: &str) -> &str {           // Takes &str, returns &str — both borrow same data.
    match s.rfind(' ') {                   // rfind searches from the end for a space.
        Some(idx) => &s[idx + 1..],       // Found a space: return everything after it.
        None => s,                          // No space: the entire string is one word.
    }
}

// ============================================================================
// Exercise 8: Lifetime Annotations
// ============================================================================
// Solution: Add 'a lifetime to tie both inputs and the output together.
fn exercise_8() {
    println!("--- Exercise 8: Lifetime Annotations ---");

    let s1 = String::from("short");              // s1 owns "short".
    let s2 = String::from("much longer string"); // s2 owns "much longer string".

    let shorter = find_shorter(&s1, &s2);        // Returns the shorter of the two.
    println!("Shorter: '{}'", shorter);           // "short"
    assert_eq!(shorter, "short");                  // Verify.

    let shorter2 = find_shorter("abc", "ab");    // Works with &str literals too.
    assert_eq!(shorter2, "ab");                    // "ab" is shorter than "abc".
    println!("Exercise 8 passed!\n");
}

/// Returns the shorter of two string slices.
/// 'a means: the returned reference is valid as long as BOTH inputs are valid.
fn find_shorter<'a>(x: &'a str, y: &'a str) -> &'a str { // Lifetime 'a ties all references.
    if x.len() <= y.len() {                                 // Compare lengths.
        x                                                    // Return x if it's shorter or equal.
    } else {
        y                                                    // Return y if it's shorter.
    }
}

// ============================================================================
// Exercise 9: Struct with Lifetime
// ============================================================================
// Solution: Implement display() using format! to create a new owned String.
fn exercise_9() {
    println!("--- Exercise 9: Struct with Lifetime ---");

    let text = String::from("Rust is great for Solana development"); // Owned String.
    let highlight = Highlight { content: &text };                     // Borrow text into the struct.
    let display = highlight.display();                                // Call the method.
    println!("{}", display);                                          // ">>> Rust is great for Solana development <<<"
    assert_eq!(display, ">>> Rust is great for Solana development <<<"); // Verify.
    println!("Exercise 9 passed!\n");
}

/// A struct that holds a reference to a string slice.
/// 'a means: this struct cannot outlive the data it borrows.
struct Highlight<'a> {     // 'a is the lifetime parameter.
    content: &'a str,       // content borrows a &str that must be valid for lifetime 'a.
}

/// Methods on Highlight.
impl<'a> Highlight<'a> {              // impl block carries the same lifetime parameter.
    /// Returns a new String wrapping the content with ">>> " and " <<<".
    fn display(&self) -> String {      // &self borrows the Highlight. Returns owned String.
        format!(">>> {} <<<", self.content) // format! creates a new heap-allocated String.
    }
}

// ============================================================================
// Exercise 10: Putting It All Together
// ============================================================================
// Solution: Use retain() to keep only strings >= min_len, calculate removed count.
fn exercise_10() {
    println!("--- Exercise 10: Putting It All Together ---");

    let mut words = vec![
        String::from("hi"),         // len 2 — will be removed (< 4).
        String::from("hello"),      // len 5 — will be kept (>= 4).
        String::from("hey"),        // len 3 — will be removed (< 4).
        String::from("greetings"),  // len 9 — will be kept (>= 4).
        String::from("yo"),         // len 2 — will be removed (< 4).
    ];

    let removed = remove_short_words(&mut words, 4); // Remove words shorter than 4 characters.
    println!("Removed {} words", removed);             // 3 words removed.
    println!("Remaining: {:?}", words);                // ["hello", "greetings"]
    assert_eq!(removed, 3);                             // Verify count.
    assert_eq!(words.len(), 2);                         // 2 words remain.
    assert_eq!(words[0], "hello");                      // First remaining.
    assert_eq!(words[1], "greetings");                  // Second remaining.
    println!("Exercise 10 passed!\n");
}

/// Removes all Strings from the Vec that are shorter than `min_len`.
/// Returns the number of removed items.
fn remove_short_words(words: &mut Vec<String>, min_len: usize) -> usize { // Mutable borrow of the Vec.
    let original_len = words.len();  // Record the original length before removing.
    words.retain(|w| w.len() >= min_len); // retain() keeps elements where the closure returns true.
    original_len - words.len()       // The difference is how many were removed.
}
