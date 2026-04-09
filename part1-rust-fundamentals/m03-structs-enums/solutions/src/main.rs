// ============================================================
// Module 03 Solutions: Structs, Enums, and Pattern Matching
// ============================================================
// Every line is commented. Run with: cargo run
// All exercises should print PASS.
// ============================================================

// ---- Exercise 1 & 2: Wallet Struct ----

// Derive Debug so we can print with {:?}.
#[derive(Debug)]
// Define the Wallet struct with three fields.
struct Wallet {
    // The wallet owner's name (String for owned data).
    owner: String,
    // The wallet balance in tokens (u64 for unsigned 64-bit integer).
    balance: u64,
    // Whether the wallet is locked (bool for true/false).
    is_locked: bool,
}

// The impl block attaches methods and associated functions to Wallet.
impl Wallet {
    // Associated function (constructor) — called with Wallet::new().
    // Takes ownership of a String for the owner name.
    // Returns a new Wallet with zero balance and unlocked.
    fn new(owner: String) -> Self {
        // Self refers to the Wallet type.
        Self {
            // Use field init shorthand: owner variable matches owner field.
            owner,
            // Initialize balance to zero.
            balance: 0,
            // Initialize is_locked to false (unlocked).
            is_locked: false,
        }
    }

    // Immutable borrow (&self) — read-only access, like a Solidity view function.
    // Returns the current balance.
    fn get_balance(&self) -> u64 {
        // Access and return the balance field.
        self.balance
    }

    // Mutable borrow (&mut self) — can modify fields.
    // Adds the given amount to the balance.
    fn deposit(&mut self, amount: u64) {
        // Add amount to the current balance using += operator.
        self.balance += amount;
    }

    // Mutable borrow — sets is_locked to true.
    fn lock(&mut self) {
        // Set the is_locked field to true.
        self.is_locked = true;
    }

    // Mutable borrow — sets is_locked to false.
    fn unlock(&mut self) {
        // Set the is_locked field to false.
        self.is_locked = false;
    }
}

// ---- Exercise 3: TradeOrder Enum ----

// Derive Debug so we can print enum variants with {:?}.
#[derive(Debug)]
// Define an enum with four variant types.
enum TradeOrder {
    // Tuple variant: carries the number of tokens to buy.
    Buy(u64),
    // Tuple variant: carries the number of tokens to sell.
    Sell(u64),
    // Unit variant: no associated data.
    Cancel,
    // Struct variant: carries named fields for price and amount.
    LimitOrder {
        // The limit price for the order.
        price: u64,
        // The number of tokens in the order.
        amount: u64,
    },
}

// Takes an immutable reference to a TradeOrder and returns a description String.
fn describe_order(order: &TradeOrder) -> String {
    // Use match to handle each variant — this is exhaustive.
    match order {
        // Match Buy variant, bind the inner u64 to `amount`.
        TradeOrder::Buy(amount) => {
            // Format and return the buy description.
            format!("Buy {} tokens", amount)
        }
        // Match Sell variant, bind the inner u64 to `amount`.
        TradeOrder::Sell(amount) => {
            // Format and return the sell description.
            format!("Sell {} tokens", amount)
        }
        // Match Cancel variant — no data to destructure.
        TradeOrder::Cancel => {
            // Return the cancel description as a String.
            String::from("Cancel order")
        }
        // Match LimitOrder variant, destructure both named fields.
        TradeOrder::LimitOrder { price, amount } => {
            // Format and return the limit order description.
            format!("Limit order: {} tokens at price {}", amount, price)
        }
    }
}

// ---- Exercise 4: Option<T> with find_wallet_owner ----

// Takes a slice of Wallets and a target balance.
// Returns Option<&str> — the owner name if found, or None.
fn find_wallet_owner(wallets: &[Wallet], target_balance: u64) -> Option<&str> {
    // Iterate over each wallet in the slice.
    for wallet in wallets.iter() {
        // Check if this wallet's balance matches the target.
        if wallet.balance == target_balance {
            // Found a match — return Some with a string slice reference.
            // &wallet.owner converts &String to &str automatically (deref coercion).
            return Some(&wallet.owner);
        }
    }
    // No wallet matched — return None explicitly.
    None
}

// ---- Exercise 5: AuctionState State Machine ----

// Derive Debug so we can print state values.
#[derive(Debug)]
// Define the auction state machine with four states.
enum AuctionState {
    // Initial state — auction hasn't started yet.
    NotStarted,
    // Active state — carries the current highest bid and bidder name.
    Active {
        // The current highest bid amount.
        highest_bid: u64,
        // The name of the current highest bidder.
        bidder: String,
    },
    // Ended state — carries the winner and final price.
    Ended {
        // The name of the auction winner.
        winner: String,
        // The final sale price.
        final_price: u64,
    },
    // Terminal state — auction was cancelled.
    Cancelled,
}

// Takes ownership of the current state and returns the next state.
// This consumes the old state — you can't use it after calling next_state.
fn next_state(state: AuctionState) -> AuctionState {
    // Match on the current state to determine the transition.
    match state {
        // NotStarted transitions to Active with initial values.
        AuctionState::NotStarted => AuctionState::Active {
            // Start with a bid of 0.
            highest_bid: 0,
            // Start with "none" as the bidder placeholder.
            bidder: String::from("none"),
        },
        // Active transitions to Ended, carrying over bid data.
        AuctionState::Active { highest_bid, bidder } => AuctionState::Ended {
            // The bidder becomes the winner.
            winner: bidder,
            // The highest bid becomes the final price.
            final_price: highest_bid,
        },
        // Ended transitions to Cancelled.
        AuctionState::Ended { .. } => AuctionState::Cancelled,
        // Cancelled stays Cancelled — it's a terminal state.
        AuctionState::Cancelled => AuctionState::Cancelled,
    }
}

// ---- Exercise 6: Config with Derive Macros ----

// Derive Debug for {:?} printing, Clone for .clone(), PartialEq for == comparison.
#[derive(Debug, Clone, PartialEq)]
// Define a Config struct for token configuration.
struct Config {
    // The token name.
    name: String,
    // Maximum token supply.
    max_supply: u64,
    // Whether the config can be changed after creation.
    is_mutable: bool,
}

// ---- Exercise 7: Tuple Structs (Newtype Pattern) ----

// Derive Debug for printing and Clone for cloning.
#[derive(Debug, Clone)]
// Tuple struct wrapping f64 to represent SOL amounts.
struct Sol(f64);

// Derive Debug for printing and Clone for cloning.
#[derive(Debug, Clone)]
// Tuple struct wrapping f64 to represent USD amounts.
struct Usd(f64);

// Implement conversion method on Sol.
impl Sol {
    // Takes an immutable reference to self and the SOL/USD price.
    // Returns a new Usd value representing the converted amount.
    fn to_usd(&self, price: f64) -> Usd {
        // Multiply the SOL amount (self.0) by the price to get USD.
        Usd(self.0 * price)
    }
}

// Implement conversion method on Usd.
impl Usd {
    // Takes an immutable reference to self and the SOL/USD price.
    // Returns a new Sol value representing the converted amount.
    fn to_sol(&self, price: f64) -> Sol {
        // Divide the USD amount (self.0) by the price to get SOL.
        Sol(self.0 / price)
    }
}

// ---- Main Function: Run All Exercises ----

// Entry point — runs all seven exercises in order.
fn main() {
    // Run exercise 1.
    exercise_1();
    // Run exercise 2.
    exercise_2();
    // Run exercise 3.
    exercise_3();
    // Run exercise 4.
    exercise_4();
    // Run exercise 5.
    exercise_5();
    // Run exercise 6.
    exercise_6();
    // Run exercise 7.
    exercise_7();
}

// ---- Exercise Runner Functions ----

// Exercise 1: Test struct definition and constructor.
fn exercise_1() {
    // Create a new Wallet using the associated function.
    let w = Wallet::new(String::from("Alice"));
    // Verify the owner field is set correctly.
    assert_eq!(w.owner, "Alice");
    // Verify balance starts at 0.
    assert_eq!(w.balance, 0);
    // Verify the wallet starts unlocked.
    assert_eq!(w.is_locked, false);
    // All assertions passed — print success.
    println!("Exercise 1: PASS — Wallet struct and constructor work!");
}

// Exercise 2: Test methods with different self receivers.
fn exercise_2() {
    // Create a mutable Wallet so we can call &mut self methods.
    let mut w = Wallet::new(String::from("Bob"));
    // Deposit 100 tokens using the &mut self method.
    w.deposit(100);
    // Verify the balance using the &self getter.
    assert_eq!(w.get_balance(), 100);
    // Deposit 50 more tokens.
    w.deposit(50);
    // Verify the updated balance.
    assert_eq!(w.get_balance(), 150);
    // Lock the wallet.
    w.lock();
    // Verify it's locked.
    assert!(w.is_locked);
    // Unlock the wallet.
    w.unlock();
    // Verify it's unlocked.
    assert!(!w.is_locked);
    // All assertions passed — print success.
    println!("Exercise 2: PASS — Methods with &self and &mut self work!");
}

// Exercise 3: Test enum definition and pattern matching.
fn exercise_3() {
    // Create a Buy variant with 100 tokens.
    let o1 = TradeOrder::Buy(100);
    // Create a Sell variant with 50 tokens.
    let o2 = TradeOrder::Sell(50);
    // Create a Cancel variant (unit variant — no data).
    let o3 = TradeOrder::Cancel;
    // Create a LimitOrder variant with named fields.
    let o4 = TradeOrder::LimitOrder { price: 42, amount: 10 };
    // Verify each description matches the expected output.
    assert_eq!(describe_order(&o1), "Buy 100 tokens");
    assert_eq!(describe_order(&o2), "Sell 50 tokens");
    assert_eq!(describe_order(&o3), "Cancel order");
    assert_eq!(describe_order(&o4), "Limit order: 10 tokens at price 42");
    // All assertions passed — print success.
    println!("Exercise 3: PASS — Enum pattern matching works!");
}

// Exercise 4: Test Option<T> usage and if let.
fn exercise_4() {
    // Create a vector of Wallets with different balances.
    let wallets = vec![
        // Alice has 100 tokens.
        Wallet { owner: String::from("Alice"), balance: 100, is_locked: false },
        // Bob has 200 tokens.
        Wallet { owner: String::from("Bob"), balance: 200, is_locked: false },
        // Charlie has 300 tokens and is locked.
        Wallet { owner: String::from("Charlie"), balance: 300, is_locked: true },
    ];
    // Search for the owner with balance 200 — should find Bob.
    assert_eq!(find_wallet_owner(&wallets, 200), Some("Bob"));
    // Search for a balance that doesn't exist — should return None.
    assert_eq!(find_wallet_owner(&wallets, 999), None);
    // Use if let to extract the value from the Option.
    if let Some(owner) = find_wallet_owner(&wallets, 100) {
        // Verify the extracted owner is Alice.
        assert_eq!(owner, "Alice");
    } else {
        // This should never execute — Alice has balance 100.
        panic!("Should have found Alice!");
    }
    // All assertions passed — print success.
    println!("Exercise 4: PASS — Option<T> and if let work!");
}

// Exercise 5: Test state machine transitions.
fn exercise_5() {
    // Start in NotStarted state.
    let s0 = AuctionState::NotStarted;
    // Transition to Active state.
    let s1 = next_state(s0);
    // Print the current state for debugging.
    println!("  After start: {:?}", s1);
    // Verify we're in Active state with initial values.
    match &s1 {
        // Destructure Active to check its fields.
        AuctionState::Active { highest_bid, bidder } => {
            // Verify the initial bid is 0.
            assert_eq!(*highest_bid, 0);
            // Verify the initial bidder is "none".
            assert_eq!(bidder, "none");
        }
        // Any other state is an error.
        _ => panic!("Expected Active state"),
    }
    // Transition from Active to Ended.
    let s2 = next_state(s1);
    // Print the current state.
    println!("  After end: {:?}", s2);
    // Verify we're in Ended state.
    match &s2 {
        // Destructure Ended to check its fields.
        AuctionState::Ended { winner, final_price } => {
            // Winner should be "none" (the initial bidder).
            assert_eq!(winner, "none");
            // Final price should be 0 (the initial bid).
            assert_eq!(*final_price, 0);
        }
        // Any other state is an error.
        _ => panic!("Expected Ended state"),
    }
    // Transition from Ended to Cancelled.
    let s3 = next_state(s2);
    // Print the current state.
    println!("  After cancel: {:?}", s3);
    // Verify we're in Cancelled state.
    match &s3 {
        // Match the Cancelled variant.
        AuctionState::Cancelled => {}
        // Any other state is an error.
        _ => panic!("Expected Cancelled state"),
    }
    // All assertions passed — print success.
    println!("Exercise 5: PASS — State machine transitions work!");
}

// Exercise 6: Test derive macros (Debug, Clone, PartialEq).
fn exercise_6() {
    // Create a Config instance.
    let config1 = Config {
        // Set the token name.
        name: String::from("MyToken"),
        // Set the maximum supply.
        max_supply: 1_000_000,
        // This config is mutable.
        is_mutable: true,
    };
    // Clone config1 into config2 — Clone derive makes this work.
    let config2 = config1.clone();
    // Verify the clone equals the original — PartialEq derive makes this work.
    assert_eq!(config1, config2);
    // Create a different Config.
    let config3 = Config {
        // Different name.
        name: String::from("OtherToken"),
        // Different max supply.
        max_supply: 500_000,
        // Different mutability.
        is_mutable: false,
    };
    // Verify different configs are not equal.
    assert_ne!(config1, config3);
    // Print configs using Debug derive.
    println!("  config1: {:?}", config1);
    // Print the cloned config.
    println!("  config2 (cloned): {:?}", config2);
    // Print comparison results.
    println!("  config1 == config2: {}", config1 == config2);
    println!("  config1 == config3: {}", config1 == config3);
    // All assertions passed — print success.
    println!("Exercise 6: PASS — Derive macros work!");
}

// Exercise 7: Test tuple structs and newtype pattern conversions.
fn exercise_7() {
    // Create a Sol value representing 2.5 SOL.
    let my_sol = Sol(2.5);
    // Set the SOL/USD price.
    let sol_price = 150.0;
    // Convert SOL to USD using the method.
    let my_usd = my_sol.to_usd(sol_price);
    // Print the conversion result.
    println!("  {:?} at ${}/SOL = {:?}", my_sol, sol_price, my_usd);
    // Verify the USD value is correct (2.5 * 150.0 = 375.0).
    assert!((my_usd.0 - 375.0).abs() < 0.001);
    // Convert USD back to SOL.
    let back_to_sol = my_usd.to_sol(sol_price);
    // Print the reverse conversion.
    println!("  {:?} at ${}/SOL = {:?}", my_usd, sol_price, back_to_sol);
    // Verify the round-trip conversion is correct (375.0 / 150.0 = 2.5).
    assert!((back_to_sol.0 - 2.5).abs() < 0.001);
    // All assertions passed — print success.
    println!("Exercise 7: PASS — Newtype pattern and methods work!");
}
