// ============================================================
// Module 03 Exercises: Structs, Enums, and Pattern Matching
// ============================================================
// Complete each exercise by replacing `todo!()` with your code.
// Run with: cargo run
// Each exercise prints PASS or FAIL.
// ============================================================

fn main() {
    exercise_1();
    exercise_2();
    exercise_3();
    exercise_4();
    exercise_5();
    exercise_6();
    exercise_7();
}

// ---- Exercise 1: Define a Struct and Constructor ----
// Create a `Wallet` struct with fields:
//   - owner: String
//   - balance: u64
//   - is_locked: bool
// Derive Debug on it.
// Implement a `new` associated function that takes an owner name
// and returns a Wallet with balance 0 and is_locked false.

// TODO: Define the Wallet struct here with #[derive(Debug)]

// TODO: Implement the `new` associated function in an impl block

fn exercise_1() {
    // Uncomment the lines below once you've defined Wallet and its new() function.
    // let w = Wallet::new(String::from("Alice"));
    // assert_eq!(w.owner, "Alice");
    // assert_eq!(w.balance, 0);
    // assert_eq!(w.is_locked, false);
    // println!("Exercise 1: PASS — Wallet struct and constructor work!");
    println!("Exercise 1: TODO — Define the Wallet struct and new() function");
}

// ---- Exercise 2: Methods with &self and &mut self ----
// Add these methods to your Wallet impl block:
//   - `get_balance(&self) -> u64` — returns the balance
//   - `deposit(&mut self, amount: u64)` — adds amount to balance
//   - `lock(&mut self)` — sets is_locked to true
//   - `unlock(&mut self)` — sets is_locked to false

fn exercise_2() {
    // Uncomment the lines below once you've added the methods.
    // let mut w = Wallet::new(String::from("Bob"));
    // w.deposit(100);
    // assert_eq!(w.get_balance(), 100);
    // w.deposit(50);
    // assert_eq!(w.get_balance(), 150);
    // w.lock();
    // assert!(w.is_locked);
    // w.unlock();
    // assert!(!w.is_locked);
    // println!("Exercise 2: PASS — Methods with &self and &mut self work!");
    println!("Exercise 2: TODO — Add methods to Wallet");
}

// ---- Exercise 3: Pattern Matching on an Enum ----
// Define an enum `TradeOrder` with these variants:
//   - Buy(u64)        — buy this many tokens
//   - Sell(u64)       — sell this many tokens
//   - Cancel          — cancel the order (no data)
//   - LimitOrder { price: u64, amount: u64 }  — struct variant
//
// Then implement the function `describe_order` below that returns
// a String description of the order using `match`.

// TODO: Define the TradeOrder enum here

// TODO: Implement describe_order
// fn describe_order(order: &TradeOrder) -> String {
//     todo!()
// }

fn exercise_3() {
    // Uncomment the lines below once you've defined TradeOrder and describe_order.
    // let o1 = TradeOrder::Buy(100);
    // let o2 = TradeOrder::Sell(50);
    // let o3 = TradeOrder::Cancel;
    // let o4 = TradeOrder::LimitOrder { price: 42, amount: 10 };
    // assert_eq!(describe_order(&o1), "Buy 100 tokens");
    // assert_eq!(describe_order(&o2), "Sell 50 tokens");
    // assert_eq!(describe_order(&o3), "Cancel order");
    // assert_eq!(describe_order(&o4), "Limit order: 10 tokens at price 42");
    // println!("Exercise 3: PASS — Enum pattern matching works!");
    println!("Exercise 3: TODO — Define TradeOrder enum and describe_order function");
}

// ---- Exercise 4: Using Option<T> ----
// Implement `find_wallet_owner` that takes a slice of Wallets and a balance,
// and returns Option<&str> — the owner of the first wallet with that exact balance,
// or None if no wallet matches.

// TODO: Implement find_wallet_owner
// fn find_wallet_owner(wallets: &[Wallet], target_balance: u64) -> Option<&str> {
//     todo!()
// }

fn exercise_4() {
    // Uncomment the lines below once you've implemented find_wallet_owner.
    // let wallets = vec![
    //     Wallet { owner: String::from("Alice"), balance: 100, is_locked: false },
    //     Wallet { owner: String::from("Bob"), balance: 200, is_locked: false },
    //     Wallet { owner: String::from("Charlie"), balance: 300, is_locked: true },
    // ];
    // assert_eq!(find_wallet_owner(&wallets, 200), Some("Bob"));
    // assert_eq!(find_wallet_owner(&wallets, 999), None);
    // // Test if let with the result
    // if let Some(owner) = find_wallet_owner(&wallets, 100) {
    //     assert_eq!(owner, "Alice");
    // } else {
    //     panic!("Should have found Alice!");
    // }
    // println!("Exercise 4: PASS — Option<T> and if let work!");
    println!("Exercise 4: TODO — Implement find_wallet_owner with Option<T>");
}

// ---- Exercise 5: State Machine with Enums ----
// Define an enum `AuctionState` with these variants:
//   - NotStarted
//   - Active { highest_bid: u64, bidder: String }
//   - Ended { winner: String, final_price: u64 }
//   - Cancelled
//
// Implement `next_state` that transitions:
//   - NotStarted -> Active (with bid=0, bidder="none")
//   - Active -> Ended (winner is the current bidder, final_price is highest_bid)
//   - Ended -> Cancelled (any ended auction can be cancelled/disputed)
//   - Cancelled -> Cancelled (stays cancelled)
//
// Return the new state.

// TODO: Define AuctionState enum with #[derive(Debug)]

// TODO: Implement next_state
// fn next_state(state: AuctionState) -> AuctionState {
//     todo!()
// }

fn exercise_5() {
    // Uncomment the lines below once you've defined AuctionState and next_state.
    // let s0 = AuctionState::NotStarted;
    // let s1 = next_state(s0);
    // println!("  After start: {:?}", s1);
    // // s1 should be Active
    // match &s1 {
    //     AuctionState::Active { highest_bid, bidder } => {
    //         assert_eq!(*highest_bid, 0);
    //         assert_eq!(bidder, "none");
    //     }
    //     _ => panic!("Expected Active state"),
    // }
    // let s2 = next_state(s1);
    // println!("  After end: {:?}", s2);
    // match &s2 {
    //     AuctionState::Ended { winner, final_price } => {
    //         assert_eq!(winner, "none");
    //         assert_eq!(*final_price, 0);
    //     }
    //     _ => panic!("Expected Ended state"),
    // }
    // let s3 = next_state(s2);
    // println!("  After cancel: {:?}", s3);
    // match &s3 {
    //     AuctionState::Cancelled => {}
    //     _ => panic!("Expected Cancelled state"),
    // }
    // println!("Exercise 5: PASS — State machine transitions work!");
    println!("Exercise 5: TODO — Define AuctionState enum and next_state function");
}

// ---- Exercise 6: Derive Macros ----
// Create a `Config` struct with fields:
//   - name: String
//   - max_supply: u64
//   - is_mutable: bool
//
// Derive: Debug, Clone, PartialEq
//
// Then verify that cloning and comparing work.

// TODO: Define Config struct with derives

fn exercise_6() {
    // Uncomment the lines below once you've defined Config.
    // let config1 = Config {
    //     name: String::from("MyToken"),
    //     max_supply: 1_000_000,
    //     is_mutable: true,
    // };
    // let config2 = config1.clone();
    // assert_eq!(config1, config2);
    // let config3 = Config {
    //     name: String::from("OtherToken"),
    //     max_supply: 500_000,
    //     is_mutable: false,
    // };
    // assert_ne!(config1, config3);
    // println!("  config1: {:?}", config1);
    // println!("  config2 (cloned): {:?}", config2);
    // println!("  config1 == config2: {}", config1 == config2);
    // println!("  config1 == config3: {}", config1 == config3);
    // println!("Exercise 6: PASS — Derive macros work!");
    println!("Exercise 6: TODO — Define Config struct with derive macros");
}

// ---- Exercise 7: Tuple Structs and Newtype Pattern ----
// Create two tuple structs:
//   - `Sol(f64)` — represents SOL amount
//   - `Usd(f64)` — represents USD amount
// Derive Debug and Clone on both.
//
// Implement a method on Sol:
//   - `to_usd(&self, price: f64) -> Usd` — converts SOL to USD at the given price
//
// And a method on Usd:
//   - `to_sol(&self, price: f64) -> Sol` — converts USD to SOL at the given price

// TODO: Define Sol and Usd tuple structs
// TODO: Implement to_usd on Sol and to_sol on Usd

fn exercise_7() {
    // Uncomment the lines below once you've defined Sol, Usd, and their methods.
    // let my_sol = Sol(2.5);
    // let sol_price = 150.0;
    // let my_usd = my_sol.to_usd(sol_price);
    // println!("  {:?} at ${}/SOL = {:?}", my_sol, sol_price, my_usd);
    // assert!((my_usd.0 - 375.0).abs() < 0.001);
    // let back_to_sol = my_usd.to_sol(sol_price);
    // println!("  {:?} at ${}/SOL = {:?}", my_usd, sol_price, back_to_sol);
    // assert!((back_to_sol.0 - 2.5).abs() < 0.001);
    // println!("Exercise 7: PASS — Newtype pattern and methods work!");
    println!("Exercise 7: TODO — Define Sol/Usd tuple structs with conversion methods");
}
