// Import HashMap from the standard library's collections module
use std::collections::HashMap;
// Import HashSet for storing unique values
use std::collections::HashSet;

// ============================================================
// A simple Transaction struct to model on-chain transfers
// Similar to how you'd think about ERC-20 Transfer events
// ============================================================
#[derive(Debug)] // Derive Debug so we can print transactions with {:?}
struct Transaction {
    from: String,   // sender address (String for simplicity)
    to: String,     // receiver address
    amount: u64,    // token amount (u64 matches Solana's lamport type)
}

fn main() {
    // Print a section header for readability
    println!("=== Module 06: Collections and Iterators ===\n");

    // ----------------------------------------------------------
    // PART 1: Vec<T> — Rust's dynamic array (like Solidity uint[])
    // ----------------------------------------------------------
    println!("--- Part 1: Vec<T> ---");

    // Create an empty vector with explicit type annotation
    let mut token_ids: Vec<u32> = Vec::new();

    // Push elements onto the end of the vector (like Solidity .push())
    token_ids.push(1001); // first NFT token ID
    token_ids.push(1002); // second NFT token ID
    token_ids.push(1003); // third NFT token ID
    token_ids.push(1004); // fourth NFT token ID

    // Print the entire vector using Debug formatting
    println!("Token IDs: {:?}", token_ids);

    // Use the vec! macro for quick initialization with values
    let prices: Vec<u64> = vec![100, 250, 75, 300, 180];

    // Print prices vector
    println!("Prices: {:?}", prices);

    // Access by index — panics if out of bounds (like Solidity revert)
    let first_price = prices[0]; // grab element at index 0
    println!("First price: {}", first_price);

    // Safe access with .get() — returns Option<&T> instead of panicking
    match prices.get(99) {
        Some(price) => println!("Price at 99: {}", price),   // would print if index existed
        None => println!("Index 99 is out of bounds (safe!)"), // gracefully handle missing
    }

    // Create a vector of zeros using repeat syntax: vec![value; count]
    let zeroed_balances: Vec<u64> = vec![0; 5]; // five zero balances
    println!("Zeroed: {:?}", zeroed_balances);

    // Pop the last element — returns Option<T>
    let mut stack = vec![10, 20, 30]; // create a mutable vector
    let popped = stack.pop(); // removes and returns the last element
    println!("Popped: {:?}, Remaining: {:?}", popped, stack);

    // Get the length of a vector (like Solidity .length)
    println!("Token count: {}", token_ids.len());

    // Check if vector is empty
    println!("Is empty: {}", token_ids.is_empty());

    // Slice a vector — borrow a contiguous portion without copying
    let first_two = &prices[0..2]; // elements at index 0 and 1
    println!("First two prices: {:?}", first_two);

    println!(); // blank line for readability

    // ----------------------------------------------------------
    // PART 2: HashMap<K, V> — Like Solidity's mapping
    // Building a token balance ledger (ERC-20 style)
    // ----------------------------------------------------------
    println!("--- Part 2: HashMap (Token Ledger) ---");

    // Create a new empty HashMap to store account balances
    let mut ledger: HashMap<String, u64> = HashMap::new();

    // Insert balances — like Solidity: balances[address] = amount
    ledger.insert("Alice".to_string(), 1000);   // Alice starts with 1000 tokens
    ledger.insert("Bob".to_string(), 500);       // Bob starts with 500 tokens
    ledger.insert("Charlie".to_string(), 750);   // Charlie starts with 750 tokens
    ledger.insert("Dave".to_string(), 200);      // Dave starts with 200 tokens

    // Print the entire ledger (order is not guaranteed in HashMap)
    println!("Ledger: {:?}", ledger);

    // Look up a balance using .get() — returns Option<&V>
    if let Some(balance) = ledger.get("Alice") {
        println!("Alice's balance: {} tokens", balance); // prints 1000
    }

    // Check if an account exists in the ledger
    let has_eve = ledger.contains_key("Eve"); // returns false
    println!("Eve exists: {}", has_eve);

    // Entry API: insert a default value only if key is missing
    ledger.entry("Eve".to_string()).or_insert(0); // Eve gets 0 if not present
    println!("Eve after or_insert: {:?}", ledger.get("Eve")); // Some(0)

    // Entry API: get mutable reference to value, inserting default if needed
    let alice_bal = ledger.entry("Alice".to_string()).or_insert(0); // Alice exists, returns &mut 1000
    *alice_bal += 500; // dereference and add 500 — Alice now has 1500
    println!("Alice after bonus: {:?}", ledger.get("Alice")); // Some(1500)

    // Iterate over all key-value pairs in the ledger
    println!("\nAll balances:");
    for (account, balance) in &ledger {
        // Print each account and its balance
        println!("  {}: {} tokens", account, balance);
    }

    // Collect only the keys into a vector
    let accounts: Vec<&String> = ledger.keys().collect(); // gather all account names
    println!("\nAll accounts: {:?}", accounts);

    // Remove an account from the ledger
    ledger.remove("Dave"); // Dave leaves the system
    println!("After removing Dave: {} accounts", ledger.len());

    println!(); // blank line

    // ----------------------------------------------------------
    // PART 3: HashSet<T> — Unique values only
    // ----------------------------------------------------------
    println!("--- Part 3: HashSet ---");

    // Create a HashSet to track unique validator addresses
    let mut validators: HashSet<String> = HashSet::new();

    // Insert validators — duplicates are silently ignored
    validators.insert("Validator_A".to_string()); // first insert succeeds
    validators.insert("Validator_B".to_string()); // second insert succeeds
    validators.insert("Validator_A".to_string()); // duplicate — returns false, no change

    // Only 2 unique validators despite 3 inserts
    println!("Unique validators: {}", validators.len());

    // Check membership
    let is_validator = validators.contains("Validator_A"); // returns true
    println!("Is Validator_A active: {}", is_validator);

    // Set operations: union, intersection, difference
    let set_a: HashSet<i32> = vec![1, 2, 3, 4].into_iter().collect();     // {1, 2, 3, 4}
    let set_b: HashSet<i32> = vec![3, 4, 5, 6].into_iter().collect();     // {3, 4, 5, 6}
    let intersection: Vec<&i32> = set_a.intersection(&set_b).collect();    // elements in both
    println!("Intersection of A and B: {:?}", intersection);               // [3, 4] (order may vary)

    println!(); // blank line

    // ----------------------------------------------------------
    // PART 4: String as a collection
    // ----------------------------------------------------------
    println!("--- Part 4: String as Collection ---");

    // Create a String (heap-allocated, growable)
    let mut message = String::from("Hello"); // start with "Hello"
    message.push(' ');                        // push a single character (space)
    message.push_str("Solana!");              // push a string slice
    println!("Message: {}", message);         // "Hello Solana!"

    // Iterate over characters
    print!("Characters: ");
    for ch in message.chars() {
        print!("[{}]", ch); // print each character in brackets
    }
    println!(); // newline after characters

    // String byte length vs character count (important for UTF-8)
    let emoji_str = String::from("🦀🌊"); // two emoji characters
    println!("Emoji byte length: {}", emoji_str.len());          // 8 bytes (4 per emoji)
    println!("Emoji char count: {}", emoji_str.chars().count()); // 2 characters

    println!(); // blank line

    // ----------------------------------------------------------
    // PART 5: The three types of iteration
    // ----------------------------------------------------------
    println!("--- Part 5: iter(), iter_mut(), into_iter() ---");

    // .iter() borrows immutably — collection is still usable after
    let names = vec!["Alice", "Bob", "Charlie"]; // a vector of string slices
    let upper: Vec<String> = names.iter()            // borrow each element as &&str
        .map(|name| name.to_uppercase())             // transform to uppercase String
        .collect();                                   // collect into Vec<String>
    println!("Uppercased: {:?}", upper);             // ["ALICE", "BOB", "CHARLIE"]
    println!("Original still exists: {:?}", names);  // names is still valid

    // .iter_mut() borrows mutably — modify elements in place
    let mut scores = vec![70, 85, 92, 60, 78]; // mutable vector of scores
    scores.iter_mut()                            // get mutable references to each element
        .for_each(|score| *score += 5);          // add 5 to each score (curve!)
    println!("Curved scores: {:?}", scores);     // [75, 90, 97, 65, 83]

    // .into_iter() takes ownership — collection is consumed
    let data = vec![10, 20, 30];                     // vector that will be consumed
    let doubled: Vec<i32> = data.into_iter()         // takes ownership of each element
        .map(|x| x * 2)                              // double each value
        .collect();                                   // collect into a new Vec
    println!("Doubled: {:?}", doubled);              // [20, 40, 60]
    // data is no longer available here — it was moved into the iterator

    println!(); // blank line

    // ----------------------------------------------------------
    // PART 6: Iterator adaptors (lazy transforms)
    // ----------------------------------------------------------
    println!("--- Part 6: Iterator Adaptors ---");

    // map — transform each element
    let amounts = vec![100, 200, 300, 400, 500]; // token amounts
    let fees: Vec<u64> = amounts.iter()           // borrow each amount
        .map(|&a| a / 100 * 2)                    // calculate 2% fee on each
        .collect();                                // collect fees into a new vector
    println!("2% fees: {:?}", fees);              // [2, 4, 6, 8, 10]

    // filter — keep only elements matching a predicate
    let all_balances = vec![0, 500, 0, 1000, 250, 0]; // some accounts have zero balance
    let active: Vec<&u64> = all_balances.iter()        // borrow each balance
        .filter(|&&b| b > 0)                            // keep only non-zero balances
        .collect();                                      // collect active balances
    println!("Active balances: {:?}", active);          // [500, 1000, 250]

    // enumerate — get (index, value) pairs
    let instructions = vec!["initialize", "transfer", "close"]; // program instructions
    println!("Instructions:");
    for (i, inst) in instructions.iter().enumerate() {
        // enumerate gives (index, &element) tuples
        println!("  [{i}] {inst}"); // print index and instruction name
    }

    // zip — pair elements from two iterators
    let senders = vec!["Alice", "Bob", "Charlie"]; // sender list
    let receivers = vec!["Dave", "Eve", "Frank"];   // receiver list
    let pairs: Vec<(&&str, &&str)> = senders.iter() // borrow senders
        .zip(receivers.iter())                       // pair with borrowed receivers
        .collect();                                  // collect into vector of tuples
    println!("Transfer pairs: {:?}", pairs);

    // take and skip — slice the iterator
    let numbers: Vec<i32> = (1..=10).collect();       // [1, 2, 3, ..., 10]
    let first_three: Vec<&i32> = numbers.iter()       // borrow elements
        .take(3)                                       // take only first 3
        .collect();                                    // [1, 2, 3]
    let after_seven: Vec<&i32> = numbers.iter()       // borrow elements again
        .skip(7)                                       // skip first 7
        .collect();                                    // [8, 9, 10]
    println!("First 3: {:?}", first_three);
    println!("After skipping 7: {:?}", after_seven);

    // chain — concatenate two iterators into one
    let batch1 = vec![100, 200];   // first batch of amounts
    let batch2 = vec![300, 400];   // second batch of amounts
    let all: Vec<&i32> = batch1.iter()  // iterate first batch
        .chain(batch2.iter())            // chain second batch onto it
        .collect();                      // collect all into one vector
    println!("Chained batches: {:?}", all); // [100, 200, 300, 400]

    println!(); // blank line

    // ----------------------------------------------------------
    // PART 7: Consuming adaptors (produce a final result)
    // ----------------------------------------------------------
    println!("--- Part 7: Consuming Adaptors ---");

    // sum — add up all elements
    let deposits = vec![100u64, 200, 300, 400]; // deposit amounts
    let total: u64 = deposits.iter().sum();      // sum all deposits
    println!("Total deposits: {}", total);       // 1000

    // count — how many elements
    let num_deposits = deposits.iter().count(); // count elements
    println!("Number of deposits: {}", num_deposits); // 4

    // any — does any element match the predicate?
    let has_large = deposits.iter().any(|&d| d >= 400); // any deposit >= 400?
    println!("Has large deposit (>=400): {}", has_large); // true

    // all — do all elements match the predicate?
    let all_positive = deposits.iter().all(|&d| d > 0); // all deposits positive?
    println!("All positive: {}", all_positive); // true

    // find — return the first matching element as Option<&T>
    let first_big = deposits.iter().find(|&&d| d > 250); // first deposit over 250
    println!("First deposit > 250: {:?}", first_big); // Some(300)

    // fold — accumulate a value (like JavaScript reduce)
    let product = deposits.iter()
        .fold(1u64, |acc, &x| acc.saturating_mul(x)); // multiply all together safely
    println!("Product of deposits: {}", product); // 100 * 200 * 300 * 400 = 2_400_000_000

    // min and max
    let smallest = deposits.iter().min(); // returns Option<&u64>
    let largest = deposits.iter().max();  // returns Option<&u64>
    println!("Smallest deposit: {:?}", smallest); // Some(100)
    println!("Largest deposit: {:?}", largest);   // Some(400)

    println!(); // blank line

    // ----------------------------------------------------------
    // PART 8: Chaining operations — functional style
    // Building a transaction processing pipeline
    // ----------------------------------------------------------
    println!("--- Part 8: Transaction Processing Pipeline ---");

    // Create a list of transactions (like on-chain transfer events)
    let transactions = vec![
        Transaction { from: "Alice".to_string(), to: "Bob".to_string(), amount: 500 },
        Transaction { from: "Bob".to_string(), to: "Charlie".to_string(), amount: 100 },
        Transaction { from: "Alice".to_string(), to: "Dave".to_string(), amount: 200 },
        Transaction { from: "Charlie".to_string(), to: "Alice".to_string(), amount: 50 },
        Transaction { from: "Alice".to_string(), to: "Eve".to_string(), amount: 1000 },
        Transaction { from: "Bob".to_string(), to: "Alice".to_string(), amount: 75 },
        Transaction { from: "Alice".to_string(), to: "Frank".to_string(), amount: 25 },
    ];

    // Query 1: Total amount sent by Alice
    let alice_total: u64 = transactions.iter()     // borrow each transaction
        .filter(|tx| tx.from == "Alice")            // keep only Alice's outgoing
        .map(|tx| tx.amount)                        // extract the amount field
        .sum();                                     // sum all amounts
    println!("Alice sent total: {} tokens", alice_total); // 500 + 200 + 1000 + 25 = 1725

    // Query 2: Alice's transfers over 100 tokens
    let alice_big: Vec<&Transaction> = transactions.iter() // borrow each transaction
        .filter(|tx| tx.from == "Alice")                    // only Alice's sends
        .filter(|tx| tx.amount > 100)                       // only amounts over 100
        .collect();                                         // collect matching transactions
    println!("Alice's big transfers (>100):");
    for tx in &alice_big {
        // Print each qualifying transaction
        println!("  {} -> {}: {} tokens", tx.from, tx.to, tx.amount);
    }

    // Query 3: Unique senders in all transactions
    let unique_senders: HashSet<&String> = transactions.iter() // borrow each transaction
        .map(|tx| &tx.from)                                     // extract sender reference
        .collect();                                             // HashSet deduplicates automatically
    println!("Unique senders: {:?}", unique_senders);

    // Query 4: Transaction summary — (from, to) with formatted string
    let summaries: Vec<String> = transactions.iter()         // borrow each transaction
        .enumerate()                                          // get (index, &transaction)
        .map(|(i, tx)| {                                      // transform into summary string
            format!("#{}: {} → {} ({} tokens)", i, tx.from, tx.to, tx.amount)
        })
        .collect();                                           // collect formatted strings
    println!("\nTransaction summaries:");
    for summary in &summaries {
        println!("  {}", summary); // print each summary line
    }

    println!(); // blank line

    // ----------------------------------------------------------
    // PART 9: Ranges and collect with turbofish
    // ----------------------------------------------------------
    println!("--- Part 9: Ranges and Turbofish ---");

    // Exclusive range: 0..5 gives 0, 1, 2, 3, 4
    let indices: Vec<i32> = (0..5).collect(); // collect range into Vec
    println!("Range 0..5: {:?}", indices);    // [0, 1, 2, 3, 4]

    // Inclusive range: 1..=5 gives 1, 2, 3, 4, 5
    let slots: Vec<i32> = (1..=5).collect(); // inclusive range collected
    println!("Range 1..=5: {:?}", slots);    // [1, 2, 3, 4, 5]

    // Turbofish syntax — specify the collection type inline
    let even_numbers = (0..20)                       // range 0 to 19
        .filter(|n| n % 2 == 0)                      // keep only even numbers
        .collect::<Vec<i32>>();                       // turbofish: collect into Vec<i32>
    println!("Even numbers (turbofish): {:?}", even_numbers);

    // Turbofish with wildcard — let Rust infer the element type
    let squares = (1..=5)                            // range 1 to 5 inclusive
        .map(|x: i32| x * x)                         // square each number
        .collect::<Vec<_>>();                         // _ means "you figure out the element type"
    println!("Squares: {:?}", squares);              // [1, 4, 9, 16, 25]

    // Simulating Solana byte slicing with ranges
    let account_data: Vec<u8> = vec![                // simulate raw account data bytes
        0x01, 0x02, 0x03, 0x04,                      // first 4 bytes: discriminator
        0xE8, 0x03, 0x00, 0x00,                      // next 4 bytes: amount (1000 in little-endian)
        0xFF, 0x00,                                   // remaining data
    ];
    let discriminator = &account_data[0..4];          // slice first 4 bytes
    let amount_bytes = &account_data[4..8];           // slice next 4 bytes
    println!("Discriminator bytes: {:?}", discriminator);
    // Convert 4 bytes to u32 using little-endian byte order (Solana standard)
    let amount = u32::from_le_bytes(amount_bytes.try_into().unwrap());
    println!("Decoded amount: {}", amount);           // 1000

    println!(); // blank line

    // ----------------------------------------------------------
    // PART 10: Slices — views into contiguous data
    // ----------------------------------------------------------
    println!("--- Part 10: Slices ---");

    // A function that accepts a slice works with both Vec and array
    let vec_data = vec![10, 20, 30, 40, 50]; // data in a Vec
    let array_data = [60, 70, 80, 90, 100];  // data in a fixed-size array

    // Call the same function with different collection types
    println!("Vec sum: {}", sum_of_slice(&vec_data));     // Vec auto-derefs to &[i32]
    println!("Array sum: {}", sum_of_slice(&array_data)); // array auto-derefs to &[i32]

    // Partial slices
    let middle = &vec_data[1..4];                          // borrow elements at index 1, 2, 3
    println!("Middle slice: {:?}", middle);                // [20, 30, 40]
    println!("Middle sum: {}", sum_of_slice(middle));      // 90

    println!(); // blank line

    // ----------------------------------------------------------
    // PART 11: Collect into HashMap (advanced)
    // ----------------------------------------------------------
    println!("--- Part 11: Collect into HashMap ---");

    // Build a balance map from parallel vectors using zip + collect
    let owners = vec!["Alice", "Bob", "Charlie"];     // account owners
    let amounts_list = vec![1500u64, 800, 600];       // corresponding balances
    let balance_map: HashMap<&str, u64> = owners.iter()  // borrow owners
        .copied()                                         // copy &&str to &str
        .zip(amounts_list.iter().copied())                // pair with copied amounts
        .collect();                                       // collect into HashMap
    println!("Balance map: {:?}", balance_map);

    // Collect into HashMap using enumerate (index as key)
    let items = vec!["sword", "shield", "potion"];          // inventory items
    let inventory: HashMap<usize, &&str> = items.iter()     // borrow each item
        .enumerate()                                         // get (index, &item)
        .collect();                                          // collect index => item map
    println!("Inventory: {:?}", inventory);

    println!(); // blank line

    // ----------------------------------------------------------
    // Final summary
    // ----------------------------------------------------------
    println!("=== Module 06 Complete! ===");
    println!("Key takeaways:");
    println!("  - Vec<T> is your dynamic array (like Solidity uint[])");
    println!("  - HashMap<K,V> is your mapping (but iterable!)");
    println!("  - Iterator chains are zero-cost functional programming");
    println!("  - collect() transforms iterators into any collection");
    println!("  - Ranges and slices are essential for Solana byte parsing");
}

// A function that accepts a slice — works with Vec, array, or any contiguous data
fn sum_of_slice(data: &[i32]) -> i32 {
    data.iter().sum() // sum all elements via iterator
}
