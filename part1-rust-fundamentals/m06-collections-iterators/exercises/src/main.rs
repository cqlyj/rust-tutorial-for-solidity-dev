// Module 06 Exercises: Collections and Iterators
// Complete each exercise by replacing `todo!()` with working code.
// Run with: cargo run
// All exercises should compile and print results when complete.

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    println!("=== Module 06 Exercises ===\n");

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
// Exercise 1: Vec Operations
// Create a vector of token prices, then:
//   a) Push 3 more prices onto it
//   b) Remove the price at index 1
//   c) Find the length
//   d) Get the last element safely (using .last())
// ============================================================
fn exercise_1() {
    println!("--- Exercise 1: Vec Operations ---");

    // Start with these prices
    let mut prices: Vec<u64> = vec![100, 200, 300];

    // TODO: Push 400, 500, and 600 onto prices
    todo!();

    // TODO: Remove the element at index 1 (should remove 200)
    todo!();

    // TODO: Store the length in a variable called `len`
    let len: usize = todo!();

    // TODO: Get the last element safely using .last()
    let last: Option<&u64> = todo!();

    println!("Prices: {:?}", prices);
    println!("Length: {}", len);
    println!("Last element: {:?}", last);

    // Expected output:
    // Prices: [100, 300, 400, 500, 600]
    // Length: 5
    // Last element: Some(600)

    println!();
}

// ============================================================
// Exercise 2: HashMap as Token Balance Ledger
// Build an ERC-20-style balance ledger:
//   a) Create a HashMap with 3 initial balances
//   b) Add a new account using the entry API
//   c) Increase an existing account's balance by 250
//   d) Find the total balance across all accounts
// ============================================================
fn exercise_2() {
    println!("--- Exercise 2: HashMap Token Ledger ---");

    // TODO: Create a HashMap with these initial balances:
    //   "Alice" => 1000, "Bob" => 500, "Charlie" => 750
    let mut ledger: HashMap<String, u64> = todo!();

    // TODO: Use the entry API to add "Dave" with balance 0 (only if not present)
    todo!();

    // TODO: Use the entry API to increase Alice's balance by 250
    todo!();

    // TODO: Calculate the total balance across all accounts using iterator .sum()
    let total: u64 = todo!();

    println!("Ledger: {:?}", ledger);
    println!("Total balance: {}", total);

    // Expected: Alice=1250, Bob=500, Charlie=750, Dave=0 => total=2500

    println!();
}

// ============================================================
// Exercise 3: Iterator Chain — Filter and Map
// Given a vector of transaction amounts (some negative = withdrawals):
//   a) Filter to keep only deposits (positive amounts)
//   b) Apply a 1% fee to each deposit (amount * 99 / 100)
//   c) Collect into a new Vec<i64>
// ============================================================
fn exercise_3() {
    println!("--- Exercise 3: Filter and Map ---");

    let transactions: Vec<i64> = vec![500, -200, 1000, -50, 300, -100, 750];

    // TODO: Chain filter (keep positive) and map (apply 1% fee) then collect
    let deposits_after_fee: Vec<i64> = todo!();

    println!("Original: {:?}", transactions);
    println!("Deposits after 1% fee: {:?}", deposits_after_fee);

    // Expected: [495, 990, 297, 742]

    println!();
}

// ============================================================
// Exercise 4: Collect into HashMap with zip
// Given two parallel vectors (accounts and balances),
// combine them into a HashMap<&str, u64>.
// ============================================================
fn exercise_4() {
    println!("--- Exercise 4: Zip into HashMap ---");

    let accounts = vec!["Alice", "Bob", "Charlie", "Dave"];
    let balances = vec![1000u64, 500, 750, 200];

    // TODO: Use .zip() and .collect() to build a HashMap<&str, u64>
    let ledger: HashMap<&str, u64> = todo!();

    println!("Ledger from zip: {:?}", ledger);

    // Expected: {"Alice": 1000, "Bob": 500, "Charlie": 750, "Dave": 200}

    println!();
}

// ============================================================
// Exercise 5: Enumerate — Numbered Instruction List
// Given a list of Solana-style instructions, use enumerate
// to print each one with its index, and also collect them
// into a Vec of formatted strings: "Step N: instruction"
// ============================================================
fn exercise_5() {
    println!("--- Exercise 5: Enumerate ---");

    let instructions = vec![
        "create_account",
        "initialize_mint",
        "mint_to",
        "transfer",
        "burn",
    ];

    // TODO: Use .enumerate() and .map() to create Vec<String>
    // Each string should be formatted as "Step {index}: {instruction}"
    let numbered: Vec<String> = todo!();

    for step in &numbered {
        println!("  {}", step);
    }

    // Expected:
    //   Step 0: create_account
    //   Step 1: initialize_mint
    //   Step 2: mint_to
    //   Step 3: transfer
    //   Step 4: burn

    println!();
}

// ============================================================
// Exercise 6: Process a Transaction List
// Given a list of Transaction structs:
//   a) Find the total amount of all transactions
//   b) Find all transactions FROM "Alice" with amount > 100
//   c) Collect unique senders into a HashSet
//   d) Find the transaction with the maximum amount
// ============================================================

struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

fn exercise_6() {
    println!("--- Exercise 6: Transaction Processing ---");

    let transactions = vec![
        Transaction { from: "Alice".to_string(), to: "Bob".to_string(), amount: 500 },
        Transaction { from: "Bob".to_string(), to: "Charlie".to_string(), amount: 100 },
        Transaction { from: "Alice".to_string(), to: "Dave".to_string(), amount: 200 },
        Transaction { from: "Charlie".to_string(), to: "Alice".to_string(), amount: 50 },
        Transaction { from: "Alice".to_string(), to: "Eve".to_string(), amount: 1000 },
    ];

    // TODO: Total amount of all transactions
    let total: u64 = todo!();

    // TODO: Filter transactions from Alice with amount > 100, collect into Vec
    let alice_big: Vec<&Transaction> = todo!();

    // TODO: Collect unique senders into a HashSet<&String>
    let unique_senders: HashSet<&String> = todo!();

    // TODO: Find the transaction with the maximum amount (hint: .max_by_key())
    let max_tx: Option<&Transaction> = todo!();

    println!("Total amount: {}", total);
    println!("Alice's big sends: {} transactions", alice_big.len());
    for tx in &alice_big {
        println!("  {} -> {}: {}", tx.from, tx.to, tx.amount);
    }
    println!("Unique senders: {:?}", unique_senders);
    if let Some(tx) = max_tx {
        println!("Largest tx: {} -> {} ({} tokens)", tx.from, tx.to, tx.amount);
    }

    // Expected:
    // Total amount: 1850
    // Alice's big sends: 3 transactions (500, 200, 1000)
    // Unique senders: {"Alice", "Bob", "Charlie"}
    // Largest tx: Alice -> Eve (1000 tokens)

    println!();
}

// ============================================================
// Exercise 7: fold — Running Balance Calculator
// Given a starting balance and a list of changes (+/-),
// use .fold() to compute the final balance.
// Also track the minimum balance seen during processing.
// ============================================================
fn exercise_7() {
    println!("--- Exercise 7: fold ---");

    let starting_balance: i64 = 1000;
    let changes: Vec<i64> = vec![200, -500, 300, -100, -400, 600];

    // TODO: Use .fold() to compute the final balance
    // Start from starting_balance, apply each change
    let final_balance: i64 = todo!();

    // TODO: Use .fold() to find the minimum balance during processing
    // Hint: Track (current_balance, min_seen) as the accumulator
    let (_, min_balance): (i64, i64) = todo!();

    println!("Starting balance: {}", starting_balance);
    println!("Changes: {:?}", changes);
    println!("Final balance: {}", final_balance);
    println!("Minimum balance during processing: {}", min_balance);

    // Expected:
    // Final balance: 1100  (1000 + 200 - 500 + 300 - 100 - 400 + 600)
    // Minimum balance: 500  (after -500 and -100 and -400: 1000+200-500=700, then 700+300-100-400=500)

    println!();
}

// ============================================================
// Exercise 8: Ranges and Byte Slicing
// Simulate Solana account data parsing:
//   a) Create a Vec<u8> of 16 bytes
//   b) Slice bytes 0..8 as a "discriminator"
//   c) Slice bytes 8..16 and convert to u64 (little-endian)
//   d) Use a range iterator to create a Vec of indices
// ============================================================
fn exercise_8() {
    println!("--- Exercise 8: Ranges and Byte Slicing ---");

    // Simulated account data: 8-byte discriminator + 8-byte u64 balance
    let account_data: Vec<u8> = vec![
        0xAA, 0xBB, 0xCC, 0xDD, 0x11, 0x22, 0x33, 0x44,  // discriminator
        0xE8, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,   // balance: 1000 in little-endian u64
    ];

    // TODO: Slice the discriminator (first 8 bytes)
    let discriminator: &[u8] = todo!();

    // TODO: Slice the balance bytes (bytes 8..16) and convert to u64
    // Hint: use u64::from_le_bytes() and .try_into().unwrap()
    let balance: u64 = todo!();

    // TODO: Use a range to create a Vec of indices from 0 to 15 (inclusive)
    let indices: Vec<usize> = todo!();

    println!("Discriminator: {:02X?}", discriminator);
    println!("Balance: {}", balance);
    println!("Indices: {:?}", indices);

    // Expected:
    // Discriminator: [AA, BB, CC, DD, 11, 22, 33, 44]
    // Balance: 1000
    // Indices: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]

    println!();
}
