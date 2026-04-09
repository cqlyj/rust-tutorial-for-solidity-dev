// ============================================================
// Module 03: Structs, Enums, and Pattern Matching
// ============================================================
// This module covers Rust's core data modeling tools.
// Coming from Solidity, think of structs as your contract state
// and enums as your instruction dispatch mechanism.
// ============================================================

// ---- SECTION 1: Basic Struct Definition ----

// The #[derive(Debug)] attribute auto-generates debug formatting.
// This lets us print the struct with {:?} — always derive this.
#[derive(Debug)]
// Define a struct representing a token account — preview of Solana patterns.
// In Solidity, this would be state variables in your contract.
// In Solana, account data is serialized into structs like this.
struct TokenAccount {
    // In real Solana code, this would be Pubkey (a 32-byte public key).
    owner: String,
    // Token balance, like balanceOf[address] in an ERC-20 contract.
    balance: u64,
    // Account freeze flag, similar to a paused modifier in Solidity.
    is_frozen: bool,
}

// ---- SECTION 2: Methods via impl blocks ----

// The impl block attaches methods to the TokenAccount struct.
// In Solidity, functions live at the contract level.
// In Rust, methods are scoped to a specific type via impl.
impl TokenAccount {
    // Associated function (no self) — acts like a constructor.
    // In Solidity, this is similar to the constructor() function.
    // Called with TokenAccount::new(...), not dot syntax.
    fn new(owner: String) -> Self {
        // Self refers to TokenAccount — the type we're implementing.
        Self {
            // Field init shorthand: owner is the same as owner: owner.
            owner,
            // New accounts start with zero balance.
            balance: 0,
            // New accounts start unfrozen.
            is_frozen: false,
        }
    }

    // &self means immutable borrow — read-only access to the struct.
    // In Solidity, this would be a `view` function.
    fn get_balance(&self) -> u64 {
        // Access the balance field through self.
        self.balance
    }

    // &self again — another read-only method.
    // Returns true if the account has any tokens.
    fn has_tokens(&self) -> bool {
        // Compare balance to zero and return the boolean result.
        self.balance > 0
    }

    // &mut self means mutable borrow — can modify the struct's fields.
    // In Solidity, this would be a state-changing function (not view/pure).
    fn deposit(&mut self, amount: u64) {
        // Add the deposit amount to the current balance.
        self.balance += amount;
        // Print a confirmation showing the deposited amount.
        println!("  Deposited {} tokens", amount);
    }

    // &mut self — another mutable method.
    // Returns a Result-like bool to indicate success/failure.
    fn withdraw(&mut self, amount: u64) -> bool {
        // Check if frozen — like require(!paused) in Solidity.
        if self.is_frozen {
            // Print error message and deny the withdrawal.
            println!("  ERROR: Account is frozen, cannot withdraw");
            // Return false to indicate failure.
            return false;
        }
        // Check sufficient balance — like require(balance >= amount) in Solidity.
        if self.balance < amount {
            // Print error message showing the shortfall.
            println!("  ERROR: Insufficient balance");
            // Return false to indicate failure.
            return false;
        }
        // Subtract the withdrawal amount from the balance.
        self.balance -= amount;
        // Print a confirmation showing the withdrawn amount.
        println!("  Withdrew {} tokens", amount);
        // Return true to indicate success.
        true
    }

    // &mut self — toggles the frozen state.
    fn toggle_freeze(&mut self) {
        // Flip the is_frozen flag using the NOT operator.
        self.is_frozen = !self.is_frozen;
        // Print the new state using if/else for the message.
        if self.is_frozen {
            // Account is now frozen.
            println!("  Account FROZEN");
        } else {
            // Account is now unfrozen.
            println!("  Account UNFROZEN");
        }
    }

    // self (no &) takes ownership — consumes the struct.
    // After calling this, the original variable is no longer usable.
    // Think of this like Solidity's selfdestruct (now deprecated).
    fn close(self) -> u64 {
        // Print that we're closing and returning the remaining balance.
        println!("  Closing account, returning {} tokens to owner", self.balance);
        // Return the remaining balance. After this, self is dropped.
        self.balance
    }
}

// ---- SECTION 3: Tuple Structs (Newtype Pattern) ----

// Tuple structs wrap existing types to create distinct new types.
// This is the "newtype pattern" — heavily used in Solana.
// In Solidity, you'd just use uint256 for both and risk mixing them up.

// Derive Debug for printing, Clone and Copy for value semantics.
#[derive(Debug, Clone, Copy)]
// Lamports wraps a u64 to represent SOL's smallest unit.
struct Lamports(u64);

// Derive the same traits for TokenAmount.
#[derive(Debug, Clone, Copy)]
// TokenAmount wraps a u64 to represent token quantities.
struct TokenAmount(u64);

// ---- SECTION 4: Unit Struct ----

// Unit structs have no fields — they're zero-size type markers.
// Used for type-state patterns and trait implementations.
#[derive(Debug)]
// Represents an uninitialized state — carries no data, just identity.
struct Uninitialized;

// ---- SECTION 5: Enums — Rust's Superpower ----

// This enum models a Solana-style instruction set.
// In Solidity, you'd have separate functions: transfer(), approve(), burn().
// In Solana, all instructions come as serialized bytes that decode into this enum.
#[derive(Debug)]
enum TokenInstruction {
    // Unit variant — no associated data, like Solidity's simple enum values.
    Initialize,
    // Tuple variant — carries a u64 amount with unnamed field.
    Transfer(u64),
    // Tuple variant — carries a u64 approval amount.
    Approve(u64),
    // Struct variant — carries named fields for mint creation.
    CreateMint {
        // Number of decimal places for the token (like ERC-20 decimals()).
        decimals: u8,
        // The mint authority who can mint new tokens.
        authority: String,
    },
    // Unit variant — no data needed to close an account.
    CloseAccount,
}

// ---- SECTION 6: Enum for State Machine ----

// Enums are perfect for state machines — a pattern used everywhere in Solana.
// The compiler ensures you handle every possible state.
#[derive(Debug)]
enum ProposalState {
    // The proposal is still collecting votes.
    Voting {
        // How many yes votes have been cast.
        yes_votes: u32,
        // How many no votes have been cast.
        no_votes: u32,
    },
    // The proposal passed and was executed.
    Executed,
    // The proposal was defeated.
    Defeated,
    // The proposal was cancelled by the authority.
    Cancelled {
        // The reason for cancellation.
        reason: String,
    },
}

// ---- SECTION 7: Functions demonstrating pattern matching ----

// This function demonstrates exhaustive pattern matching on our instruction enum.
// In Solana programs, the process_instruction function looks almost exactly like this.
fn process_instruction(instruction: TokenInstruction) {
    // match is like a super-powered switch statement.
    // The compiler REQUIRES you to handle every variant — if you forget one, it won't compile.
    match instruction {
        // Match the Initialize variant — no data to extract.
        TokenInstruction::Initialize => {
            // Handle initialization logic.
            println!("  Processing: Initialize token program");
        }
        // Match Transfer and bind the inner u64 to `amount`.
        TokenInstruction::Transfer(amount) => {
            // Handle transfer logic with the extracted amount.
            println!("  Processing: Transfer {} tokens", amount);
        }
        // Match Approve and bind the inner u64 to `amount`.
        TokenInstruction::Approve(amount) => {
            // Handle approval logic.
            println!("  Processing: Approve {} tokens for delegation", amount);
        }
        // Match CreateMint and destructure both named fields.
        TokenInstruction::CreateMint { decimals, authority } => {
            // Handle mint creation with both extracted values.
            println!(
                "  Processing: Create mint with {} decimals, authority: {}",
                decimals, authority
            );
        }
        // Match CloseAccount — no data to extract.
        TokenInstruction::CloseAccount => {
            // Handle account closure.
            println!("  Processing: Close account and reclaim rent");
        }
    }
}

// This function shows pattern matching on the state machine enum.
fn describe_proposal(state: &ProposalState) {
    // Match on a reference to ProposalState.
    match state {
        // Destructure the Voting variant to extract vote counts.
        ProposalState::Voting { yes_votes, no_votes } => {
            // Calculate total votes from both counts.
            let total = yes_votes + no_votes;
            // Print the current voting status.
            println!(
                "  Proposal is in voting: {} yes, {} no ({} total)",
                yes_votes, no_votes, total
            );
        }
        // Match the Executed unit variant.
        ProposalState::Executed => {
            // Print that the proposal passed.
            println!("  Proposal has been executed successfully");
        }
        // Match the Defeated unit variant.
        ProposalState::Defeated => {
            // Print that the proposal failed.
            println!("  Proposal was defeated");
        }
        // Destructure the Cancelled variant to extract the reason.
        ProposalState::Cancelled { reason } => {
            // Print the cancellation reason.
            println!("  Proposal was cancelled: {}", reason);
        }
    }
}

// ---- SECTION 8: Option<T> demonstration ----

// This function demonstrates Option<T> — Rust's null safety mechanism.
// In Solidity, uninitialized values are 0/address(0). In Rust, we use Option.
fn find_account_balance(accounts: &[TokenAccount], owner_name: &str) -> Option<u64> {
    // Iterate over the slice of accounts using iter().
    for account in accounts.iter() {
        // Check if this account's owner matches the search name.
        if account.owner == owner_name {
            // Found it — wrap the balance in Some and return.
            return Some(account.balance);
        }
    }
    // No account found — return None (not 0, not null — explicitly "nothing").
    None
}

// ---- SECTION 9: Derive macro showcase ----

// Derive multiple traits at once to get rich functionality for free.
// Debug: enables {:?} printing.
// Clone: enables .clone() for explicit copies.
// PartialEq: enables == and != comparisons.
#[derive(Debug, Clone, PartialEq)]
struct VaultConfig {
    // The maximum deposit allowed in one transaction.
    max_deposit: u64,
    // The fee in basis points (100 = 1%).
    fee_bps: u16,
    // Whether the vault accepts new deposits.
    is_open: bool,
}

// Implement Default manually to show custom default values.
impl Default for VaultConfig {
    // The default() function returns a VaultConfig with sensible defaults.
    fn default() -> Self {
        // Construct with default values appropriate for a new vault.
        Self {
            // Default max deposit of 1 million tokens.
            max_deposit: 1_000_000,
            // Default fee of 30 basis points (0.3%).
            fee_bps: 30,
            // Vaults start open by default.
            is_open: true,
        }
    }
}

// ---- MAIN FUNCTION ----

fn main() {
    // Print a section header for struct basics.
    println!("=== 1. Struct Basics ===");

    // Create a new TokenAccount using the associated function (constructor).
    let mut alice = TokenAccount::new(String::from("Alice"));
    // Print the account using Debug formatting ({:?}) — works because we derived Debug.
    println!("Created account: {:?}", alice);

    // Deposit tokens using the mutable method.
    alice.deposit(500);
    // Deposit more tokens.
    alice.deposit(300);
    // Print the balance using the immutable getter method.
    println!("Balance: {}", alice.get_balance());
    // Check if the account has tokens using the boolean method.
    println!("Has tokens: {}", alice.has_tokens());

    // Withdraw tokens — this should succeed.
    alice.withdraw(200);
    // Print updated balance.
    println!("Balance after withdrawal: {}", alice.get_balance());

    // Freeze the account.
    alice.toggle_freeze();
    // Try to withdraw from a frozen account — this should fail.
    alice.withdraw(100);
    // Unfreeze the account.
    alice.toggle_freeze();

    // Print a blank line for readability.
    println!();

    // ---- Field init shorthand demonstration ----
    // Print a section header.
    println!("=== 2. Field Init Shorthand ===");

    // Create variables with names matching struct fields.
    let owner = String::from("Bob");
    // Balance variable matches the field name.
    let balance: u64 = 1000;
    // Create struct using shorthand — owner and balance don't need `field: value`.
    let bob = TokenAccount {
        // Shorthand: uses the `owner` variable for the `owner` field.
        owner,
        // Shorthand: uses the `balance` variable for the `balance` field.
        balance,
        // This field doesn't match a variable name, so we use normal syntax.
        is_frozen: false,
    };
    // Print Bob's account to verify the shorthand worked.
    println!("Bob's account: {:?}", bob);

    // Print a blank line for readability.
    println!();

    // ---- Tuple structs (newtype pattern) ----
    // Print a section header.
    println!("=== 3. Tuple Structs (Newtype Pattern) ===");

    // Create a Lamports value — wraps u64 but is a distinct type.
    let rent = Lamports(890_880);
    // Create a TokenAmount value — also wraps u64 but is a DIFFERENT type.
    let tokens = TokenAmount(1_000);
    // Print both values using Debug formatting.
    println!("Rent: {:?}", rent);
    println!("Tokens: {:?}", tokens);
    // Access the inner value using .0 (tuple index syntax).
    println!("Rent in lamports: {}", rent.0);
    // Access the TokenAmount inner value the same way.
    println!("Token count: {}", tokens.0);
    // These are different types! You can't accidentally mix them.
    // rent == tokens would fail to compile — this is the whole point.

    // Print a blank line for readability.
    println!();

    // ---- Unit struct ----
    // Print a section header.
    println!("=== 4. Unit Struct ===");

    // Create a unit struct instance — no fields, no parentheses, no braces.
    let _state = Uninitialized;
    // Print the unit struct using Debug formatting.
    println!("State marker: {:?}", _state);

    // Print a blank line for readability.
    println!();

    // ---- Enums and pattern matching ----
    // Print a section header.
    println!("=== 5. Enums & Pattern Matching ===");

    // Create various instruction variants to demonstrate enum capabilities.
    // Unit variant — no associated data.
    let ix1 = TokenInstruction::Initialize;
    // Tuple variant — carries a u64 amount.
    let ix2 = TokenInstruction::Transfer(500);
    // Another tuple variant with different data.
    let ix3 = TokenInstruction::Approve(1000);
    // Struct variant — carries named fields.
    let ix4 = TokenInstruction::CreateMint {
        // Set decimal places to 9 (like SOL).
        decimals: 9,
        // Set the mint authority.
        authority: String::from("MintAuthority"),
    };
    // Unit variant — no data needed.
    let ix5 = TokenInstruction::CloseAccount;

    // Process each instruction through pattern matching.
    process_instruction(ix1);
    process_instruction(ix2);
    process_instruction(ix3);
    process_instruction(ix4);
    process_instruction(ix5);

    // Print a blank line for readability.
    println!();

    // ---- State machine with enums ----
    // Print a section header.
    println!("=== 6. State Machine with Enums ===");

    // Create a proposal in the Voting state with initial vote counts.
    let proposal1 = ProposalState::Voting {
        // Start with 42 yes votes.
        yes_votes: 42,
        // Start with 18 no votes.
        no_votes: 18,
    };
    // Create a proposal in the Executed state.
    let proposal2 = ProposalState::Executed;
    // Create a proposal in the Defeated state.
    let proposal3 = ProposalState::Defeated;
    // Create a cancelled proposal with a reason string.
    let proposal4 = ProposalState::Cancelled {
        // Provide the cancellation reason.
        reason: String::from("Quorum not reached"),
    };

    // Pass references to describe_proposal — we borrow, not move.
    describe_proposal(&proposal1);
    describe_proposal(&proposal2);
    describe_proposal(&proposal3);
    describe_proposal(&proposal4);

    // Print a blank line for readability.
    println!();

    // ---- Option<T> — null safety ----
    // Print a section header.
    println!("=== 7. Option<T> — Null Safety ===");

    // Create a vector of accounts to search through.
    let accounts = vec![
        // Alice's account with 500 tokens.
        TokenAccount::new(String::from("Alice")),
        // Bob's account (we'll modify it below).
        TokenAccount::new(String::from("Bob")),
    ];

    // Note: We need mutable access to deposit into these accounts.
    // Let's create fresh mutable accounts for the search demo.
    let search_accounts = vec![
        // Create Alice's account.
        TokenAccount {
            owner: String::from("Alice"),
            balance: 500,
            is_frozen: false,
        },
        // Create Bob's account.
        TokenAccount {
            owner: String::from("Bob"),
            balance: 1200,
            is_frozen: false,
        },
    ];

    // Search for Alice's balance — should return Some(500).
    let alice_balance = find_account_balance(&search_accounts, "Alice");
    // Search for Charlie's balance — should return None (doesn't exist).
    let charlie_balance = find_account_balance(&search_accounts, "Charlie");

    // Use match to safely handle the Option returned for Alice.
    match alice_balance {
        // Some(balance) means we found Alice's account.
        Some(balance) => println!("  Alice's balance: {}", balance),
        // None means no account was found (won't happen here).
        None => println!("  Alice's account not found"),
    }

    // Use match to safely handle the Option returned for Charlie.
    match charlie_balance {
        // Some(balance) means we found the account.
        Some(balance) => println!("  Charlie's balance: {}", balance),
        // None means no account was found — this will match.
        None => println!("  Charlie's account not found"),
    }

    // Print a blank line for readability.
    println!();

    // ---- if let for single-pattern matching ----
    // Print a section header.
    println!("=== 8. if let — Concise Pattern Matching ===");

    // Create an Option with a value.
    let maybe_owner: Option<String> = Some(String::from("Alice"));
    // Create an Option with no value.
    let no_owner: Option<String> = None;

    // if let extracts the value only if the pattern matches.
    // This is shorter than a full match when you only care about one variant.
    if let Some(owner) = maybe_owner {
        // This block runs because maybe_owner is Some.
        println!("  Found owner: {}", owner);
    }

    // if let with None — the block won't execute.
    if let Some(owner) = no_owner {
        // This block is skipped because no_owner is None.
        println!("  Found owner: {}", owner);
    } else {
        // The else block runs when the pattern doesn't match.
        println!("  No owner found (None)");
    }

    // Print a blank line for readability.
    println!();

    // ---- Option methods ----
    // Print a section header.
    println!("=== 9. Option Methods ===");

    // Create Some and None values to demonstrate Option methods.
    let some_value: Option<u64> = Some(42);
    // Create a None variant with explicit type annotation.
    let none_value: Option<u64> = None;

    // unwrap_or returns the inner value, or a default if None.
    println!("  some_value.unwrap_or(0) = {}", some_value.unwrap_or(0));
    // For None, it returns the default value (0).
    println!("  none_value.unwrap_or(0) = {}", none_value.unwrap_or(0));

    // is_some() returns true if the Option contains a value.
    println!("  some_value.is_some() = {}", some_value.is_some());
    // is_none() returns true if the Option is None.
    println!("  none_value.is_none() = {}", none_value.is_none());

    // map transforms the inner value if present, returns None if None.
    let doubled = some_value.map(|v| v * 2);
    // Print the mapped result — should be Some(84).
    println!("  some_value.map(|v| v * 2) = {:?}", doubled);
    // Map on None produces None — no transformation happens.
    let doubled_none = none_value.map(|v| v * 2);
    // Print the mapped None result — should be None.
    println!("  none_value.map(|v| v * 2) = {:?}", doubled_none);

    // Print a blank line for readability.
    println!();

    // ---- Match with numeric patterns ----
    // Print a section header.
    println!("=== 10. Match with Guards & Ranges ===");

    // Define some transfer amounts to match against.
    let amounts = vec![0, 50, 500, 2_000_000];

    // Iterate over each amount in the vector.
    for amount in amounts {
        // Match on the amount with ranges and guards.
        match amount {
            // Match exactly zero.
            0 => println!("  {} -> Zero transfer (no-op)", amount),
            // Match the range 1 through 100 (inclusive).
            1..=100 => println!("  {} -> Small transfer", amount),
            // Match the range 101 through 10_000 (inclusive).
            101..=10_000 => println!("  {} -> Medium transfer", amount),
            // Match guard: bind to n and check a condition.
            n if n > 1_000_000 => println!("  {} -> Whale alert!", n),
            // Wildcard: matches anything not caught above.
            _ => println!("  {} -> Large transfer", amount),
        }
    }

    // Print a blank line for readability.
    println!();

    // ---- Derive macros showcase ----
    // Print a section header.
    println!("=== 11. Derive Macros ===");

    // Create a VaultConfig using the Default implementation we wrote.
    let default_config = VaultConfig::default();
    // Print the default config — Debug derive makes this work.
    println!("  Default config: {:?}", default_config);

    // Create a custom config with different values.
    let custom_config = VaultConfig {
        // Set a lower max deposit.
        max_deposit: 500_000,
        // Set a higher fee of 50 basis points (0.5%).
        fee_bps: 50,
        // This vault is closed to new deposits.
        is_open: false,
    };
    // Print the custom config.
    println!("  Custom config: {:?}", custom_config);

    // Clone the default config — Clone derive makes this work.
    let cloned = default_config.clone();
    // Compare using == — PartialEq derive makes this work.
    println!(
        "  default == cloned: {}",
        default_config == cloned
    );
    // Compare different configs — should be false.
    println!(
        "  default == custom: {}",
        default_config == custom_config
    );

    // Print a blank line for readability.
    println!();

    // ---- Close an account (consuming self) ----
    // Print a section header.
    println!("=== 12. Consuming self (Close Account) ===");

    // Create a new account with some balance.
    let mut closing_account = TokenAccount::new(String::from("Charlie"));
    // Deposit some tokens so there's something to return.
    closing_account.deposit(750);
    // Close the account — this consumes `closing_account`.
    let returned = closing_account.close();
    // Print the returned balance.
    println!("  Returned balance: {}", returned);
    // After close(), `closing_account` is no longer usable.
    // Uncommenting the next line would cause a compile error:
    // println!("{:?}", closing_account);  // ERROR: value used after move

    // Print a blank line for readability.
    println!();

    // Print a final summary message.
    println!("=== Module 03 Complete! ===");
    println!("Key takeaways:");
    println!("  - Structs model data; impl blocks add behavior");
    println!("  - Enums carry data with variants (unlike Solidity)");
    println!("  - Option<T> eliminates null — the compiler enforces safety");
    println!("  - match is exhaustive — the compiler catches missing cases");
    println!("  - #[derive()] gives you Debug, Clone, PartialEq, etc. for free");

    // Suppress unused variable warnings for the demo accounts vector.
    drop(accounts);
    // Suppress unused variable warning for search_accounts.
    drop(search_accounts);
}
