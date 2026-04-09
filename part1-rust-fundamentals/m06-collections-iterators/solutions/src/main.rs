// Module 06 Solutions: Collections and Iterators
// Every line is commented explaining what it does.
// Run with: cargo run

// Import HashMap from the standard library for key-value storage
use std::collections::HashMap;
// Import HashSet from the standard library for unique value storage
use std::collections::HashSet;

fn main() {
    // Print a header for the solutions output
    println!("=== Module 06 Solutions ===\n");

    // Run each exercise solution in order
    exercise_1();
    exercise_2();
    exercise_3();
    exercise_4();
    exercise_5();
    exercise_6();
    exercise_7();
    exercise_8();
}

// ============================================================
// Exercise 1 Solution: Vec Operations
// ============================================================
fn exercise_1() {
    // Print the exercise header
    println!("--- Exercise 1: Vec Operations ---");

    // Start with a vector of 3 token prices using the vec! macro
    let mut prices: Vec<u64> = vec![100, 200, 300];

    // Push 3 more prices onto the end of the vector
    prices.push(400); // append 400 to the end
    prices.push(500); // append 500 to the end
    prices.push(600); // append 600 to the end

    // Remove the element at index 1 (the value 200)
    // All elements after index 1 shift left to fill the gap
    prices.remove(1);

    // Get the length of the vector (number of elements)
    let len: usize = prices.len();

    // Get the last element safely — returns Option<&u64>
    // Returns Some(&600) since the vector is non-empty
    let last: Option<&u64> = prices.last();

    // Print results to verify correctness
    println!("Prices: {:?}", prices);      // [100, 300, 400, 500, 600]
    println!("Length: {}", len);            // 5
    println!("Last element: {:?}", last);  // Some(600)

    // Print blank line for readability
    println!();
}

// ============================================================
// Exercise 2 Solution: HashMap as Token Balance Ledger
// ============================================================
fn exercise_2() {
    // Print the exercise header
    println!("--- Exercise 2: HashMap Token Ledger ---");

    // Create a new empty HashMap for the balance ledger
    let mut ledger: HashMap<String, u64> = HashMap::new();
    // Insert Alice with 1000 tokens
    ledger.insert("Alice".to_string(), 1000);
    // Insert Bob with 500 tokens
    ledger.insert("Bob".to_string(), 500);
    // Insert Charlie with 750 tokens
    ledger.insert("Charlie".to_string(), 750);

    // Use the entry API to insert Dave with 0 only if "Dave" is not already a key
    // or_insert returns a mutable reference to the value (new or existing)
    ledger.entry("Dave".to_string()).or_insert(0);

    // Use the entry API to get a mutable reference to Alice's balance
    // Since Alice exists, or_insert(0) returns &mut to existing value (1000)
    let alice_balance = ledger.entry("Alice".to_string()).or_insert(0);
    // Dereference the mutable reference and add 250
    *alice_balance += 250; // Alice now has 1250

    // Calculate the total balance by iterating over all values and summing
    let total: u64 = ledger.values().sum();

    // Print the ledger and total (order is not guaranteed)
    println!("Ledger: {:?}", ledger);
    println!("Total balance: {}", total); // 1250 + 500 + 750 + 0 = 2500

    // Print blank line
    println!();
}

// ============================================================
// Exercise 3 Solution: Iterator Chain — Filter and Map
// ============================================================
fn exercise_3() {
    // Print the exercise header
    println!("--- Exercise 3: Filter and Map ---");

    // A vector of transaction amounts — positive = deposits, negative = withdrawals
    let transactions: Vec<i64> = vec![500, -200, 1000, -50, 300, -100, 750];

    // Chain filter and map operations:
    let deposits_after_fee: Vec<i64> = transactions.iter() // borrow each element as &i64
        .filter(|&&amount| amount > 0)                      // keep only positive (deposit) amounts
        .map(|&amount| amount * 99 / 100)                   // apply 1% fee (keep 99%)
        .collect();                                         // collect results into a new Vec<i64>

    // Print original and processed transactions
    println!("Original: {:?}", transactions);
    println!("Deposits after 1% fee: {:?}", deposits_after_fee); // [495, 990, 297, 742]

    // Print blank line
    println!();
}

// ============================================================
// Exercise 4 Solution: Collect into HashMap with zip
// ============================================================
fn exercise_4() {
    // Print the exercise header
    println!("--- Exercise 4: Zip into HashMap ---");

    // Parallel vectors: account names and their corresponding balances
    let accounts = vec!["Alice", "Bob", "Charlie", "Dave"];
    let balances = vec![1000u64, 500, 750, 200];

    // Zip the two iterators together and collect into a HashMap
    let ledger: HashMap<&str, u64> = accounts.iter() // borrow each account name as &&str
        .copied()                                      // copy &&str to &str for cleaner types
        .zip(balances.iter().copied())                 // pair each &str with a copied u64
        .collect();                                    // collect (key, value) pairs into HashMap

    // Print the resulting HashMap (order not guaranteed)
    println!("Ledger from zip: {:?}", ledger);

    // Print blank line
    println!();
}

// ============================================================
// Exercise 5 Solution: Enumerate — Numbered Instruction List
// ============================================================
fn exercise_5() {
    // Print the exercise header
    println!("--- Exercise 5: Enumerate ---");

    // A list of Solana-style program instructions
    let instructions = vec![
        "create_account",   // step 0
        "initialize_mint",  // step 1
        "mint_to",          // step 2
        "transfer",         // step 3
        "burn",             // step 4
    ];

    // Use enumerate to get (index, &element) pairs, then map to formatted strings
    let numbered: Vec<String> = instructions.iter()  // borrow each instruction as &&str
        .enumerate()                                  // wrap in (index, &&str) tuples
        .map(|(i, inst)| {                            // destructure the tuple
            format!("Step {}: {}", i, inst)           // format into "Step N: instruction"
        })
        .collect();                                   // collect formatted strings into Vec

    // Print each numbered instruction
    for step in &numbered {
        println!("  {}", step); // print with indentation
    }

    // Print blank line
    println!();
}

// ============================================================
// Exercise 6 Solution: Process a Transaction List
// ============================================================

// A struct representing a token transfer transaction
struct Transaction {
    from: String,   // sender account name
    to: String,     // receiver account name
    amount: u64,    // number of tokens transferred
}

fn exercise_6() {
    // Print the exercise header
    println!("--- Exercise 6: Transaction Processing ---");

    // Create a vector of test transactions
    let transactions = vec![
        Transaction { from: "Alice".to_string(), to: "Bob".to_string(), amount: 500 },       // Alice sends 500 to Bob
        Transaction { from: "Bob".to_string(), to: "Charlie".to_string(), amount: 100 },      // Bob sends 100 to Charlie
        Transaction { from: "Alice".to_string(), to: "Dave".to_string(), amount: 200 },       // Alice sends 200 to Dave
        Transaction { from: "Charlie".to_string(), to: "Alice".to_string(), amount: 50 },     // Charlie sends 50 to Alice
        Transaction { from: "Alice".to_string(), to: "Eve".to_string(), amount: 1000 },       // Alice sends 1000 to Eve
    ];

    // Calculate the total amount across all transactions
    let total: u64 = transactions.iter()  // borrow each transaction
        .map(|tx| tx.amount)               // extract the amount field
        .sum();                            // sum all amounts

    // Filter to find Alice's transactions with amount > 100
    let alice_big: Vec<&Transaction> = transactions.iter()  // borrow each transaction
        .filter(|tx| tx.from == "Alice")                     // keep only where sender is Alice
        .filter(|tx| tx.amount > 100)                        // keep only amounts over 100
        .collect();                                          // collect matching references

    // Collect all unique sender names into a HashSet (deduplicates automatically)
    let unique_senders: HashSet<&String> = transactions.iter() // borrow each transaction
        .map(|tx| &tx.from)                                     // extract reference to sender
        .collect();                                             // HashSet removes duplicates

    // Find the transaction with the maximum amount using max_by_key
    let max_tx: Option<&Transaction> = transactions.iter()  // borrow each transaction
        .max_by_key(|tx| tx.amount);                         // compare by amount field

    // Print the total amount
    println!("Total amount: {}", total); // 1850

    // Print Alice's large transactions
    println!("Alice's big sends: {} transactions", alice_big.len()); // 3
    for tx in &alice_big {
        // Print details of each qualifying transaction
        println!("  {} -> {}: {}", tx.from, tx.to, tx.amount);
    }

    // Print the set of unique senders
    println!("Unique senders: {:?}", unique_senders); // {"Alice", "Bob", "Charlie"}

    // Print the largest transaction if one exists
    if let Some(tx) = max_tx {
        // Destructure the Option to get the transaction reference
        println!("Largest tx: {} -> {} ({} tokens)", tx.from, tx.to, tx.amount);
    }

    // Print blank line
    println!();
}

// ============================================================
// Exercise 7 Solution: fold — Running Balance Calculator
// ============================================================
fn exercise_7() {
    // Print the exercise header
    println!("--- Exercise 7: fold ---");

    // Starting balance before any changes are applied
    let starting_balance: i64 = 1000;
    // List of balance changes: positive = deposit, negative = withdrawal
    let changes: Vec<i64> = vec![200, -500, 300, -100, -400, 600];

    // Use fold to compute the final balance
    // fold takes an initial accumulator (starting_balance) and a closure
    // The closure receives (accumulator, current_element) and returns new accumulator
    let final_balance: i64 = changes.iter()              // borrow each change as &i64
        .fold(starting_balance, |balance, &change| {      // start from starting_balance
            balance + change                               // apply each change to running balance
        });

    // Use fold to track both current balance and minimum balance seen
    // Accumulator is a tuple: (current_balance, min_balance_seen)
    let (_, min_balance): (i64, i64) = changes.iter()    // borrow each change as &i64
        .fold(
            (starting_balance, starting_balance),          // initial: (current=1000, min=1000)
            |(current, min_seen), &change| {               // destructure accumulator tuple
                let new_balance = current + change;        // apply the change
                let new_min = min_seen.min(new_balance);   // update minimum if new balance is lower
                (new_balance, new_min)                     // return updated tuple
            },
        );

    // Print the starting balance and list of changes
    println!("Starting balance: {}", starting_balance);
    println!("Changes: {:?}", changes);
    // Print the computed final balance
    println!("Final balance: {}", final_balance); // 1100
    // Print the minimum balance observed during processing
    println!("Minimum balance during processing: {}", min_balance); // 500

    // Print blank line
    println!();
}

// ============================================================
// Exercise 8 Solution: Ranges and Byte Slicing
// ============================================================
fn exercise_8() {
    // Print the exercise header
    println!("--- Exercise 8: Ranges and Byte Slicing ---");

    // Simulated Solana account data: 8-byte discriminator followed by 8-byte u64 balance
    let account_data: Vec<u8> = vec![
        0xAA, 0xBB, 0xCC, 0xDD, 0x11, 0x22, 0x33, 0x44,  // bytes 0-7: discriminator
        0xE8, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,   // bytes 8-15: balance (1000 LE)
    ];

    // Slice the first 8 bytes as the discriminator
    // &account_data[0..8] creates a slice reference &[u8] to bytes at indices 0 through 7
    let discriminator: &[u8] = &account_data[0..8];

    // Slice bytes 8 through 15 for the balance
    let balance_bytes: &[u8] = &account_data[8..16];
    // Convert the 8-byte slice to a u64 using little-endian byte order
    // .try_into() converts &[u8] to [u8; 8] (fixed-size array required by from_le_bytes)
    // .unwrap() is safe here because we know the slice is exactly 8 bytes
    let balance: u64 = u64::from_le_bytes(balance_bytes.try_into().unwrap());

    // Use an inclusive range (0..=15) to create a Vec of all indices
    // The range 0..=15 produces 0, 1, 2, ..., 15
    let indices: Vec<usize> = (0..=15).collect();

    // Print the discriminator bytes in uppercase hex format
    println!("Discriminator: {:02X?}", discriminator); // [AA, BB, CC, DD, 11, 22, 33, 44]
    // Print the decoded balance
    println!("Balance: {}", balance); // 1000
    // Print the vector of indices
    println!("Indices: {:?}", indices); // [0, 1, 2, ..., 15]

    // Print blank line
    println!();
}
