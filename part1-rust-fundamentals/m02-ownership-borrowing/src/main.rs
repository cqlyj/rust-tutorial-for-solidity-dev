// ============================================================================
// Module 02: Ownership, Borrowing, and Lifetimes
// ============================================================================
// This is the most important concept in Rust. Every line is commented.
// Run with: cargo run
// ============================================================================

fn main() {
    // ========================================================================
    // SECTION 1: Stack vs Heap
    // ========================================================================
    // Stack: fixed-size data, fast, automatically cleaned up.
    // Heap: dynamic-size data, slower, must be explicitly managed (via ownership).

    println!("=== SECTION 1: Stack vs Heap ===\n");

    let x: i32 = 42;               // i32 is 4 bytes, lives on the stack.
    let y: f64 = 3.14;             // f64 is 8 bytes, lives on the stack.
    let flag: bool = true;         // bool is 1 byte, lives on the stack.

    println!("Stack values: x={}, y={}, flag={}", x, y, flag); // All stack values, trivially copied.

    let heap_string = String::from("hello"); // String allocates on the heap.
    // heap_string is 3 values on the stack: pointer, length (5), capacity (>=5).
    // The actual bytes "hello" live on the heap.

    println!("Heap string: '{}', len={}, capacity={}", heap_string, heap_string.len(), heap_string.capacity());
    // len() returns how many bytes are used. capacity() returns how many bytes are allocated.

    // ========================================================================
    // SECTION 2: Ownership Rules
    // ========================================================================
    // Rule 1: Each value has exactly one owner.
    // Rule 2: There can only be one owner at a time.
    // Rule 3: When the owner goes out of scope, the value is dropped.

    println!("\n=== SECTION 2: Ownership Rules ===\n");

    {
        let scoped_string = String::from("I live in this block"); // scoped_string owns the String.
        println!("Inside scope: {}", scoped_string);              // Valid — we're still in scope.
    } // scoped_string goes out of scope here. Rust calls `drop()`, freeing the heap memory.
    // println!("{}", scoped_string); // ERROR: scoped_string doesn't exist here.

    let outer = String::from("I'm in main"); // outer is valid for the rest of main().
    println!("Outer: {}", outer);             // Works fine — we're in main's scope.

    // ========================================================================
    // SECTION 3: Move Semantics
    // ========================================================================
    // When you assign a heap-allocated value to another variable, the original is MOVED.
    // This is fundamentally different from Solidity where assignment copies values.

    println!("\n=== SECTION 3: Move Semantics ===\n");

    let s1 = String::from("hello"); // s1 owns the String "hello" on the heap.
    let s2 = s1;                     // Ownership MOVES from s1 to s2. s1 is now invalid.
    // println!("{}", s1);           // COMPILE ERROR: value used after move.
    // In Solidity: `string memory s2 = s1;` would copy. In Rust, it MOVES.
    println!("s2 = {}", s2);        // Works — s2 is the new owner.

    // Moves also happen when passing values to functions:
    let s3 = String::from("world"); // s3 owns "world".
    takes_ownership(s3);             // s3 is MOVED into the function parameter.
    // println!("{}", s3);           // COMPILE ERROR: s3 was moved into takes_ownership().

    // Moves happen when returning from functions too:
    let s4 = gives_ownership();      // The function creates a String and moves it to s4.
    println!("s4 = {}", s4);        // s4 owns the returned String.

    // You can give and take back:
    let s5 = String::from("boomerang"); // s5 owns "boomerang".
    let s5 = takes_and_gives_back(s5);  // s5 is moved in, then a new String is moved back.
    println!("s5 = {}", s5);            // s5 now owns the returned value.

    // ========================================================================
    // SECTION 4: Clone and Copy
    // ========================================================================
    // Clone: explicit deep copy of heap data.
    // Copy: implicit bitwise copy for simple stack types.

    println!("\n=== SECTION 4: Clone and Copy ===\n");

    // --- Clone: explicit deep copy ---
    let original = String::from("deep copy me");  // original owns the String.
    let cloned = original.clone();                  // clone() creates a full independent copy on the heap.
    println!("original = '{}', cloned = '{}'", original, cloned); // BOTH are valid — they're independent.
    // clone() is explicit because it can be expensive (heap allocation + copy).

    // --- Copy: implicit for stack types ---
    let a: i32 = 10;   // i32 implements the Copy trait.
    let b = a;          // a is COPIED, not moved. Both a and b are valid.
    println!("a = {}, b = {}", a, b); // Both valid — i32 is Copy.

    let t1: (i32, bool) = (42, true); // Tuples of Copy types are also Copy.
    let t2 = t1;                       // Copied, not moved.
    println!("t1 = {:?}, t2 = {:?}", t1, t2); // Both valid.

    // This tuple contains a String, so it is NOT Copy — it moves:
    let t3 = (42, String::from("not copy")); // String prevents Copy on the tuple.
    let t4 = t3;                              // t3 is MOVED to t4.
    // println!("{:?}", t3);                  // COMPILE ERROR: t3 was moved.
    println!("t4 = {:?}", t4);               // Works — t4 owns it now.

    // Types that are Copy: i8, i16, i32, i64, i128, u8-u128, f32, f64, bool, char, &T.
    // Types that are NOT Copy: String, Vec<T>, Box<T>, any type owning heap data.

    // ========================================================================
    // SECTION 5: References and Borrowing
    // ========================================================================
    // A reference borrows a value without taking ownership.
    // &T  = shared (immutable) reference — can read, cannot modify.
    // &mut T = mutable reference — can read AND modify.

    println!("\n=== SECTION 5: References and Borrowing ===\n");

    // --- Immutable references (&T) ---
    let greeting = String::from("hello, world");   // greeting owns the String.
    let len = calculate_length(&greeting);          // &greeting borrows it — greeting stays valid.
    println!("'{}' has length {}", greeting, len);  // greeting is still usable because it was borrowed, not moved.

    // --- Mutable references (&mut T) ---
    let mut message = String::from("hello");  // Must be declared `mut` to allow mutable borrowing.
    append_exclamation(&mut message);          // &mut message borrows it mutably — function can modify it.
    println!("After mutation: {}", message);   // message is now "hello!" — it was modified through the reference.

    // --- Multiple immutable references are OK ---
    let data = String::from("shared data");  // data owns the String.
    let r1 = &data;                           // First immutable borrow — OK.
    let r2 = &data;                           // Second immutable borrow — OK (many readers allowed).
    println!("r1={}, r2={}", r1, r2);         // Both references are valid simultaneously.

    // --- Cannot mix immutable and mutable references ---
    let mut mutable_data = String::from("careful");  // Declared mutable.
    let r3 = &mutable_data;                           // Immutable borrow.
    // let r4 = &mut mutable_data;                    // COMPILE ERROR: can't borrow mutably while r3 exists.
    println!("r3 = {}", r3);                          // r3's lifetime ends here (last use).

    let r4 = &mut mutable_data;                       // NOW OK — r3 is no longer in use.
    r4.push_str("!");                                 // Modify through mutable reference.
    println!("After r4 mutation: {}", r4);            // Prints "careful!"

    // ========================================================================
    // SECTION 6: Borrowing Rules in Practice
    // ========================================================================
    // Rule: You can have MANY &T OR ONE &mut T, but not both at the same time.
    // This prevents data races at compile time.

    println!("\n=== SECTION 6: Borrowing Rules ===\n");

    let mut numbers = vec![1, 2, 3, 4, 5]; // A Vec<i32> — heap-allocated dynamic array.

    // Multiple immutable borrows — safe because nobody is modifying:
    let first = &numbers[0];   // Immutable borrow of the first element.
    let second = &numbers[1];  // Another immutable borrow — fine.
    println!("first={}, second={}", first, second); // Both valid.
    // first and second are no longer used after this line, so their borrows end.

    // Now we can mutably borrow:
    numbers.push(6);           // push() takes &mut self — mutable borrow. OK because no immutable borrows active.
    println!("numbers = {:?}", numbers); // [1, 2, 3, 4, 5, 6]

    // This would fail:
    // let r = &numbers[0];    // Immutable borrow.
    // numbers.push(7);        // COMPILE ERROR: can't mutably borrow while r exists.
    // println!("{}", r);      // r is still in use here, so the immutable borrow is active.

    // ========================================================================
    // SECTION 7: Dangling References
    // ========================================================================
    // Rust prevents dangling references at compile time.

    println!("\n=== SECTION 7: Dangling References ===\n");

    // This function would create a dangling reference — Rust rejects it:
    // fn dangle() -> &String {      // Return type is a reference.
    //     let s = String::from("hello");
    //     &s                          // ERROR: s is dropped at the end of this function.
    // }                               // The returned &s would point to freed memory.

    // The correct approach — return the owned value:
    let safe = no_dangle();        // The function moves the String to us.
    println!("Safe: {}", safe);    // We own it — no dangling reference.

    // ========================================================================
    // SECTION 8: String vs &str
    // ========================================================================
    // String = owned, heap-allocated, growable.
    // &str   = borrowed, read-only slice of string data.

    println!("\n=== SECTION 8: String vs &str ===\n");

    let owned_string: String = String::from("I am owned");  // Heap-allocated, owned.
    let string_literal: &str = "I am a literal";             // Baked into the binary, type is &'static str.
    let string_slice: &str = &owned_string[0..4];            // Slice into the owned String — borrows it.

    println!("owned: {}", owned_string);      // "I am owned"
    println!("literal: {}", string_literal);  // "I am a literal"
    println!("slice: {}", string_slice);      // "I am" — first 4 bytes.

    // Functions should prefer &str as parameter type for maximum flexibility:
    print_greeting("literal works");                // &str — works directly.
    print_greeting(&owned_string);                  // &String auto-derefs to &str — works too.
    print_greeting(&String::from("temporary"));     // Even temporary Strings work.

    // Building strings:
    let mut built = String::new();            // Empty String — 0 capacity.
    built.push_str("Hello");                  // Append a &str slice.
    built.push(' ');                           // Append a single char.
    built.push_str("World");                  // Append another &str.
    println!("Built: {}", built);             // "Hello World"

    // Converting between String and &str:
    let s: String = "convert me".to_string();  // &str -> String via to_string().
    let s2: String = String::from("or this");  // &str -> String via String::from().
    let _slice: &str = &s;                     // String -> &str via & (auto-deref).
    let _slice2: &str = s2.as_str();           // String -> &str via as_str().

    // ========================================================================
    // SECTION 9: Slices
    // ========================================================================
    // A slice is a reference to a contiguous section of a collection.
    // &[T] = slice of T values. &str = slice of string bytes.

    println!("\n=== SECTION 9: Slices ===\n");

    // --- Array slices ---
    let arr = [10, 20, 30, 40, 50];          // Fixed-size array on the stack.
    let full_slice: &[i32] = &arr;            // Slice of the entire array.
    let partial: &[i32] = &arr[1..4];         // Elements at index 1, 2, 3 → [20, 30, 40].
    let first_three: &[i32] = &arr[..3];      // Elements at index 0, 1, 2 → [10, 20, 30].
    let last_two: &[i32] = &arr[3..];         // Elements at index 3, 4 → [40, 50].

    println!("full: {:?}", full_slice);       // [10, 20, 30, 40, 50]
    println!("partial [1..4]: {:?}", partial); // [20, 30, 40]
    println!("first three: {:?}", first_three); // [10, 20, 30]
    println!("last two: {:?}", last_two);     // [40, 50]

    // --- Vec slices ---
    let vec = vec![100, 200, 300, 400];       // Heap-allocated vector.
    let vec_slice: &[i32] = &vec[1..3];       // Slice into the vector: [200, 300].
    println!("vec slice: {:?}", vec_slice);   // [200, 300]

    // --- String slices are &str ---
    let sentence = String::from("hello world"); // Owned String.
    let word: &str = &sentence[0..5];            // String slice: "hello".
    println!("first word: {}", word);            // "hello"

    // Slices borrow the original — you can't modify the original while a slice exists:
    // let mut v = vec![1, 2, 3];
    // let s = &v[..];         // Immutable borrow via slice.
    // v.push(4);              // COMPILE ERROR: can't mutably borrow while slice exists.
    // println!("{:?}", s);    // Slice is still in use.

    // Functions that work with slices:
    let values = vec![3, 7, 1, 9, 4];        // A Vec<i32>.
    let sum = sum_slice(&values);              // Pass &Vec<i32>, which auto-derefs to &[i32].
    println!("Sum of {:?} = {}", values, sum); // "Sum of [3, 7, 1, 9, 4] = 24"

    // Find first word using slices:
    let text = String::from("hello beautiful world"); // Owned String.
    let first = first_word(&text);                     // Returns &str — a slice into text.
    println!("First word: {}", first);                 // "hello"

    // ========================================================================
    // SECTION 10: Lifetimes
    // ========================================================================
    // Lifetimes ensure references are always valid.
    // Syntax: 'a (read: "lifetime a").
    // Most lifetimes are inferred. You annotate when the compiler needs help.

    println!("\n=== SECTION 10: Lifetimes ===\n");

    // --- Basic lifetime inference ---
    // This function has one reference input and one reference output.
    // The compiler infers the output lifetime matches the input. No annotation needed.
    let my_string = String::from("hello world");    // Owned String.
    let first_w = first_word(&my_string);           // Compiler knows: first_w lives as long as &my_string.
    println!("First word: {}", first_w);            // "hello"

    // --- When you NEED lifetime annotations ---
    // When there are MULTIPLE reference inputs and a reference output,
    // the compiler can't tell which input the output borrows from.

    let string1 = String::from("long string");    // string1 owns "long string".
    let result;                                     // Declare result here so it lives long enough.
    {
        let string2 = String::from("xyz");        // string2 owns "xyz".
        result = longest(&string1, &string2);      // longest() returns a reference.
        // The returned reference is valid as long as BOTH inputs are valid.
        println!("Longest: {}", result);           // Works — both string1 and string2 are still alive.
    } // string2 is dropped here.
    // println!("{}", result);                     // WOULD ERROR if uncommented — result might point to string2.

    // --- 'static lifetime ---
    // &'static str means the reference lives for the entire program.
    // All string literals are 'static — they're baked into the binary.
    let static_str: &'static str = "I live forever"; // This data is in the binary, never freed.
    println!("Static: {}", static_str);               // Valid anywhere in the program.

    // --- Lifetime annotations in structs ---
    // If a struct holds a reference, it needs a lifetime parameter.
    let novel = String::from("Call me Ishmael. Some years ago...");  // Owned String.
    let excerpt = ImportantExcerpt {
        part: first_sentence(&novel),  // Borrow a slice of novel.
    };
    println!("Excerpt: {}", excerpt.part);  // "Call me Ishmael."
    // excerpt cannot outlive novel, because it borrows from novel.
    // The lifetime annotation on ImportantExcerpt enforces this.

    // --- Lifetime in method (impl block) ---
    let announcement = excerpt.announce_and_return("Big news!");  // Method with lifetimes.
    println!("Announced: {}", announcement);                       // Returns the excerpt's part.

    // ========================================================================
    // SECTION 11: The 'info Lifetime in Solana (Preview)
    // ========================================================================
    // In Solana/Anchor, you'll see 'info everywhere.
    // It means: "these references are valid for the duration of instruction processing."

    println!("\n=== SECTION 11: Solana 'info Preview ===\n");

    // Simulating the Solana pattern:
    let account_data = vec![0u8; 64];                // Simulated account data bytes.
    let context = SimulatedContext {
        data: &account_data,                          // The context borrows account data.
    };
    println!("Simulated account data length: {}", context.data.len()); // 64

    // In real Anchor code, it looks like:
    // pub struct MyInstruction<'info> {
    //     pub my_account: Account<'info, MyData>,
    //     pub signer: Signer<'info>,
    // }
    // All references share the 'info lifetime — they're all valid for the instruction.

    // ========================================================================
    // SECTION 12: Best Practices Demonstrated
    // ========================================================================

    println!("\n=== SECTION 12: Best Practices ===\n");

    // GOOD: Borrow when you only need to read.
    let data_vec = vec![1, 2, 3, 4, 5];              // Owned Vec.
    let total = sum_slice(&data_vec);                  // Borrow — data_vec stays valid.
    println!("Sum: {}, original: {:?}", total, data_vec); // Both usable.

    // BAD: Cloning when borrowing would suffice.
    let original_str = String::from("don't clone me needlessly");
    let _wasted_clone = original_str.clone();          // Unnecessary heap allocation!
    // If you only need to read, just borrow with &original_str.

    // GOOD: Use &str parameters for flexibility.
    print_greeting("works with literals");             // &str.
    print_greeting(&String::from("works with String")); // &String auto-derefs to &str.

    // GOOD: Return owned types when creating new data.
    let new_string = create_greeting("Rustacean");     // Returns owned String — caller decides what to do.
    println!("{}", new_string);                        // "Hello, Rustacean!"

    // GOOD: Keep mutable borrows short.
    let mut buffer = String::from("start");           // Mutable String.
    {
        let r = &mut buffer;                           // Mutable borrow starts.
        r.push_str(" end");                            // Modify.
    }                                                  // Mutable borrow ends here.
    println!("Buffer: {}", buffer);                    // Can use buffer again — "start end".

    println!("\n=== All sections complete! ===");
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Takes ownership of a String. The caller can no longer use it.
fn takes_ownership(s: String) {  // s takes ownership of the String passed in.
    println!("I own: {}", s);    // We can use s freely.
} // s goes out of scope and is dropped. The String is freed.

/// Creates and returns a new String. Ownership transfers to the caller.
fn gives_ownership() -> String {          // Return type is an owned String.
    let s = String::from("gift");         // s owns "gift".
    s                                      // Ownership moves to the caller.
} // s is NOT dropped here — it was moved out.

/// Takes a String, modifies it, and returns it. Ownership goes in and out.
fn takes_and_gives_back(mut s: String) -> String { // s takes ownership; mut lets us modify.
    s.push_str(" returned");                        // Modify the String.
    s                                               // Move it back to the caller.
}

/// Borrows a String immutably. The caller keeps ownership.
fn calculate_length(s: &String) -> usize { // s is a reference to a String (borrowed, not owned).
    s.len()                                 // Return the length. We can read but not modify.
} // s (the reference) goes out of scope. The String it points to is NOT dropped.

/// Borrows a String mutably. Can modify the original.
fn append_exclamation(s: &mut String) { // s is a mutable reference — can modify the String.
    s.push('!');                         // Append '!' to the original String.
} // s (the mutable reference) goes out of scope. The String is NOT dropped.

/// Takes &str for maximum flexibility — accepts &str, &String, and slices.
fn print_greeting(name: &str) {        // &str is the most flexible string parameter type.
    println!("Hello, {}!", name);      // Print a greeting.
}

/// Returns an owned String — the caller gets full ownership.
fn create_greeting(name: &str) -> String {  // Takes borrowed &str, returns owned String.
    format!("Hello, {}!", name)              // format! creates a new heap-allocated String.
}

/// Sums a slice of i32 values. Accepts &Vec<i32> and &[i32] equally.
fn sum_slice(numbers: &[i32]) -> i32 {  // &[i32] is a slice — borrows the data.
    let mut total = 0;                   // Accumulator.
    for &n in numbers {                  // Iterate, dereferencing each &i32 to i32.
        total += n;                      // Add to total.
    }
    total                                // Return the sum.
}

/// Returns the first word of a string (up to the first space).
fn first_word(s: &str) -> &str {      // Takes &str, returns &str — both borrow the same data.
    let bytes = s.as_bytes();          // Get the raw bytes of the string.
    for (i, &byte) in bytes.iter().enumerate() { // Iterate with index.
        if byte == b' ' {             // Found a space.
            return &s[..i];           // Return a slice from start to the space.
        }
    }
    s                                  // No space found — the entire string is one word.
}

/// Returns the first sentence (up to and including the first period).
fn first_sentence(s: &str) -> &str {  // Borrows a &str and returns a slice of it.
    match s.find('.') {                // Find the index of the first period.
        Some(i) => &s[..=i],          // Return slice up to and including the period.
        None => s,                     // No period — return the whole thing.
    }
}

// ============================================================================
// LIFETIME ANNOTATIONS
// ============================================================================

/// Returns the longer of two string slices.
/// The lifetime 'a means: the returned reference lives as long as BOTH inputs.
/// Without 'a, the compiler can't know which input the return value borrows from.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // 'a ties all three lifetimes together.
    if x.len() > y.len() {                            // Compare lengths.
        x                                              // Return x if it's longer.
    } else {
        y                                              // Return y otherwise.
    }
}

// ============================================================================
// STRUCTS WITH LIFETIMES
// ============================================================================

/// A struct that holds a borrowed string slice.
/// The lifetime 'a means: this struct cannot outlive the data it borrows.
struct ImportantExcerpt<'a> { // 'a = the struct borrows data with this lifetime.
    part: &'a str,             // part is a reference that must be valid for lifetime 'a.
}

/// Methods on a struct with lifetime annotations.
impl<'a> ImportantExcerpt<'a> {        // impl block also needs the lifetime parameter.
    /// Returns the excerpt's part with an announcement.
    /// The return lifetime is tied to 'a (self's lifetime), not the announcement.
    fn announce_and_return(&self, announcement: &str) -> &'a str { // Returns with lifetime 'a.
        println!("Attention: {}", announcement);                    // Print the announcement.
        self.part                                                   // Return the borrowed part.
    }
}

// ============================================================================
// SIMULATING SOLANA'S 'info PATTERN
// ============================================================================

/// Simulates how Solana/Anchor uses lifetimes for account data.
/// The 'info lifetime means: this context borrows data valid for the instruction.
struct SimulatedContext<'info> { // 'info = all borrowed data shares this lifetime.
    data: &'info [u8],           // Byte slice — like Solana account data.
}

// no_dangle returns an owned String instead of a dangling reference.
fn no_dangle() -> String {              // Returns owned String, not a reference.
    let s = String::from("safe");       // Create a String.
    s                                    // Move it to the caller — no dangling reference.
}
