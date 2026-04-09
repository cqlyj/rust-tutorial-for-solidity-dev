# Module 06: Collections and Iterators

## Creating This Project

```bash
cargo new m06-collections-iterators
cd m06-collections-iterators
```

This creates a standard Rust binary project. No external dependencies needed — everything in this module comes from Rust's standard library.

---

## Solidity ↔ Rust: The Big Picture

| Concept | Solidity | Rust |
|---------|----------|------|
| Dynamic array | `uint[] balances` | `Vec<u64>` |
| Fixed array | `uint[10] slots` | `[u64; 10]` |
| Key-value map | `mapping(address => uint)` | `HashMap<Pubkey, u64>` |
| Unique set | *(no built-in)* | `HashSet<T>` |
| Iteration | `for (uint i = 0; i < arr.length; i++)` | `for item in vec.iter()` or iterator chains |
| Functional transforms | *(not available)* | `.map().filter().collect()` |

**Key insight for Solidity devs**: Solidity's `mapping` cannot be iterated — you need a separate array of keys. Rust's `HashMap` is iterable out of the box. And Rust iterators are **zero-cost abstractions**: the compiler optimizes `.map().filter().collect()` chains into the same machine code as a hand-written `for` loop. Think JavaScript's `.map()/.filter()/.reduce()` but with no runtime overhead.

---

## 1. `Vec<T>` — The Dynamic Array

`Vec<T>` is Rust's growable array, analogous to Solidity's `uint[] storage`.

### Creating Vectors

```rust
// Empty vector — type annotation required when empty
let mut balances: Vec<u64> = Vec::new();

// The vec![] macro — quick creation with initial values
let prices = vec![100, 200, 300, 400, 500];

// Create a vector of N identical elements
let zeros = vec![0u64; 10]; // ten zeros
```

### Pushing and Removing

```rust
let mut txns = Vec::new();
txns.push("transfer");    // append to end
txns.push("stake");
txns.push("unstake");

let last = txns.pop();           // removes & returns last element as Option<T>
txns.remove(0);                  // removes element at index (shifts everything left)
txns.insert(0, "initialize");   // insert at index
```

### Indexing vs Safe Access

```rust
let vals = vec![10, 20, 30];

// Direct indexing — panics if out of bounds (like Solidity revert)
let first = vals[0];

// Safe access — returns Option<&T>
match vals.get(5) {
    Some(val) => println!("Got {val}"),
    None => println!("Index out of bounds"),
}
```

**Solidity comparison**: In Solidity, out-of-bounds array access reverts the transaction. In Rust, `vec[i]` panics (crashes the program). Prefer `.get(i)` which returns `Option<&T>` — the Rust way of saying "this might not exist."

### Iterating Over Vectors

```rust
let accounts = vec!["Alice", "Bob", "Charlie"];

// Immutable iteration
for name in &accounts {
    println!("{name}");
}

// Mutable iteration
let mut scores = vec![80, 90, 70];
for score in &mut scores {
    *score += 10; // dereference to modify
}

// Consuming iteration (moves ownership — vec is gone after this)
for name in accounts {
    println!("Processing {name}");
}
// accounts is no longer usable here
```

---

## 2. `HashMap<K, V>` — The Key-Value Map

`HashMap` is like Solidity's `mapping`, but **you can iterate it**.

```rust
use std::collections::HashMap;

// Create a token ledger (like ERC-20 balances)
let mut balances: HashMap<String, u64> = HashMap::new();

// Insert entries (like Solidity: balances[msg.sender] = 1000)
balances.insert("Alice".to_string(), 1000);
balances.insert("Bob".to_string(), 500);

// Get a value — returns Option<&V>
if let Some(bal) = balances.get("Alice") {
    println!("Alice has {bal} tokens");
}

// Check existence
if balances.contains_key("Charlie") {
    println!("Charlie has tokens");
}

// Remove an entry
balances.remove("Bob");
```

### The Entry API — Insert-or-Update Pattern

This is something Solidity devs do manually all the time:

```solidity
// Solidity pattern
if (balances[addr] == 0) {
    balances[addr] = defaultValue;
}
balances[addr] += amount;
```

Rust's Entry API handles this elegantly:

```rust
// Insert default if key doesn't exist, then modify
balances.entry("Charlie".to_string()).or_insert(0);

// Insert default and get mutable reference
let counter = balances.entry("Alice".to_string()).or_insert(0);
*counter += 500; // Alice now has 1500

// or_insert_with for computed defaults
balances.entry("Dave".to_string()).or_insert_with(|| compute_default());
```

### Iterating a HashMap

```rust
// Iterate key-value pairs
for (account, balance) in &balances {
    println!("{account}: {balance} tokens");
}

// Iterate only keys or only values
for account in balances.keys() {
    println!("Account: {account}");
}
for balance in balances.values() {
    println!("Balance: {balance}");
}
```

**Solidity comparison**: In Solidity, you literally cannot iterate a `mapping`. You need a separate `address[] public holders` array to track keys. In Rust, `HashMap` gives you `.keys()`, `.values()`, and `.iter()` for free.

---

## 3. `HashSet<T>` — Unique Values

`HashSet<T>` is like a `HashMap` with only keys — no duplicates allowed.

```rust
use std::collections::HashSet;

let mut validators: HashSet<String> = HashSet::new();
validators.insert("Alice".to_string());
validators.insert("Bob".to_string());
validators.insert("Alice".to_string()); // no-op, already exists

println!("Unique validators: {}", validators.len()); // 2

// Set operations
let set_a: HashSet<i32> = [1, 2, 3].into_iter().collect();
let set_b: HashSet<i32> = [2, 3, 4].into_iter().collect();

let union: HashSet<_> = set_a.union(&set_b).collect();
let intersection: HashSet<_> = set_a.intersection(&set_b).collect();
let difference: HashSet<_> = set_a.difference(&set_b).collect();
```

---

## 4. `String` as a Collection

`String` in Rust is a collection of UTF-8 bytes. Unlike Solidity's `string` (which is mostly opaque), Rust gives you full access to characters.

```rust
let mut greeting = String::from("Hello");
greeting.push(' ');                      // push a single char
greeting.push_str("Solana!");            // push a string slice

// Iterate over characters
for ch in greeting.chars() {
    print!("{ch} ");
}

// Iterate over bytes (important for Solana: account data is &[u8])
for byte in greeting.bytes() {
    print!("{byte} ");
}

// String length vs char count
let emoji = String::from("🦀🌊");
println!("Bytes: {}", emoji.len());        // 8 (4 bytes per emoji)
println!("Chars: {}", emoji.chars().count()); // 2
```

**Solana context**: Solana account data is `&[u8]` — raw bytes. Understanding how Rust handles bytes, characters, and slices is critical for serializing/deserializing on-chain data.

---

## 5. Iterators — The Three Ways to Iterate

Every collection in Rust can produce an **iterator**. There are three kinds:

| Method | Yields | Ownership | Use When |
|--------|--------|-----------|----------|
| `.iter()` | `&T` | Borrows | You want to read items |
| `.iter_mut()` | `&mut T` | Mutably borrows | You want to modify items in place |
| `.into_iter()` | `T` | Takes ownership | You're done with the collection |

```rust
let names = vec!["Alice", "Bob", "Charlie"];

// .iter() — borrows, collection still usable after
let uppercased: Vec<String> = names.iter()
    .map(|name| name.to_uppercase())
    .collect();
println!("Original still here: {:?}", names);

// .iter_mut() — mutably borrows, modifies in place
let mut scores = vec![80, 90, 70];
scores.iter_mut().for_each(|s| *s += 10);

// .into_iter() — consumes the collection
let data = vec![1, 2, 3];
let doubled: Vec<i32> = data.into_iter().map(|x| x * 2).collect();
// data is GONE — moved into the iterator
```

**Important**: `for item in &collection` is sugar for `for item in collection.iter()`, and `for item in collection` is sugar for `for item in collection.into_iter()`.

---

## 6. Iterator Adaptors — Transform Without Consuming

Adaptors are **lazy** — they don't do anything until you consume the iterator.

### `map` — Transform Each Element

```rust
let amounts = vec![100, 200, 300];
let doubled: Vec<i32> = amounts.iter().map(|x| x * 2).collect();
// [200, 400, 600]
```

### `filter` — Keep Only Matching Elements

```rust
let balances = vec![0, 500, 0, 1000, 250];
let active: Vec<&i32> = balances.iter().filter(|&&b| b > 0).collect();
// [500, 1000, 250]
```

### `enumerate` — Get Index + Value

```rust
let instructions = vec!["init", "transfer", "close"];
for (i, instruction) in instructions.iter().enumerate() {
    println!("Instruction {i}: {instruction}");
}
```

### `zip` — Pair Two Iterators

```rust
let accounts = vec!["Alice", "Bob", "Charlie"];
let balances = vec![1000, 500, 750];

let ledger: Vec<(&&str, &i32)> = accounts.iter()
    .zip(balances.iter())
    .collect();
// [("Alice", 1000), ("Bob", 500), ("Charlie", 750)]
```

### `take` and `skip` — Slice the Iterator

```rust
let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let first_three: Vec<&i32> = data.iter().take(3).collect();  // [1, 2, 3]
let skip_five: Vec<&i32> = data.iter().skip(5).collect();    // [6, 7, 8, 9, 10]
let middle: Vec<&i32> = data.iter().skip(2).take(3).collect(); // [3, 4, 5]
```

### `chain` — Concatenate Two Iterators

```rust
let batch1 = vec![100, 200];
let batch2 = vec![300, 400];

let all: Vec<&i32> = batch1.iter().chain(batch2.iter()).collect();
// [100, 200, 300, 400]
```

---

## 7. Consuming Adaptors — Produce a Final Value

These "consume" the iterator, producing a single result.

### `collect` — The Magic Method

`collect()` transforms an iterator into **any compatible collection**:

```rust
// Into a Vec
let nums: Vec<i32> = (0..5).collect();

// Into a HashMap
let pairs: HashMap<&str, i32> = vec![("Alice", 100), ("Bob", 200)]
    .into_iter()
    .collect();

// Into a String
let hello: String = vec!['H', 'e', 'l', 'l', 'o'].into_iter().collect();

// Into a HashSet
let unique: HashSet<i32> = vec![1, 1, 2, 2, 3].into_iter().collect();
```

### Turbofish Syntax `::<>`

When Rust can't infer the collection type, use turbofish:

```rust
// These are equivalent:
let nums: Vec<i32> = (0..5).collect();
let nums = (0..5).collect::<Vec<i32>>();

// Turbofish is essential in method chains
let result = some_iter
    .map(|x| x * 2)
    .collect::<Vec<_>>(); // _ lets Rust infer the element type
```

### Other Consuming Adaptors

```rust
let vals = vec![10, 20, 30, 40, 50];

let total: i32 = vals.iter().sum();              // 150
let count = vals.iter().count();                  // 5
let any_big = vals.iter().any(|&&v| v > 100);    // false (note: iter gives &i32)
let all_pos = vals.iter().all(|&&v| v > 0);      // true

// find — returns first match as Option<&T>
let first_big = vals.iter().find(|&&v| v > 25);  // Some(&30)

// fold — like JavaScript's reduce
let sum = vals.iter().fold(0, |acc, &x| acc + x); // 150

// max, min
let biggest = vals.iter().max();   // Some(&50)
let smallest = vals.iter().min();  // Some(&10)
```

---

## 8. Chaining Iterator Operations

The real power comes from chaining. This is **functional programming** in Rust:

```rust
// Process a list of transactions
struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

let transactions: Vec<Transaction> = get_transactions();

// Find total amount sent by Alice in transactions over 100 tokens
let alice_total: u64 = transactions.iter()
    .filter(|tx| tx.from == "Alice")    // only Alice's sends
    .filter(|tx| tx.amount > 100)       // only significant ones
    .map(|tx| tx.amount)                // extract amounts
    .sum();                             // add them up
```

**Solidity comparison**: In Solidity, you'd write a for loop with if statements and manual accumulation. In Rust, the iterator chain is more readable AND compiles to equally efficient machine code.

---

## 9. Ranges

Ranges are iterators built into the language:

```rust
// Exclusive range: 0, 1, 2, ..., 9
for i in 0..10 {
    println!("{i}");
}

// Inclusive range: 0, 1, 2, ..., 10
for i in 0..=10 {
    println!("{i}");
}

// Collect a range into a Vec
let indices: Vec<usize> = (0..5).collect();

// Ranges for byte slicing (critical for Solana)
let data: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7];
let slice = &data[0..4];    // first 4 bytes
let rest = &data[4..];      // bytes 4 onwards
```

**Solana context**: Account data deserialization uses ranges constantly:

```rust
// Typical Solana pattern
let account_data: &[u8] = &ctx.accounts.my_account.data.borrow();
let discriminator = &account_data[0..8];
let balance_bytes = &account_data[8..16];
let balance = u64::from_le_bytes(balance_bytes.try_into().unwrap());
```

---

## 10. Slices Recap: `&[T]`

A slice is a **view** into a contiguous sequence — it doesn't own the data.

```rust
let vec = vec![10, 20, 30, 40, 50];

// Slice of the whole vector
let all: &[i32] = &vec;

// Slice of a portion
let middle: &[i32] = &vec[1..4]; // [20, 30, 40]

// Functions that accept slices work with both Vec and arrays
fn sum_slice(data: &[i32]) -> i32 {
    data.iter().sum()
}

let from_vec = vec![1, 2, 3];
let from_array = [4, 5, 6];

sum_slice(&from_vec);    // Vec auto-derefs to &[i32]
sum_slice(&from_array);  // array auto-derefs to &[i32]
```

**Best practice**: Write functions that accept `&[T]` instead of `&Vec<T>`. This makes them work with any contiguous data — Vec, array, or another slice.

---

## 11. Why Iterators Matter for Solana

Solana programs process raw byte arrays, account lists, and instruction data. Iterators are your primary tool:

```rust
// Processing multiple accounts
for account in ctx.remaining_accounts.iter() {
    // validate each account
}

// Deserializing a list of items from account data
let items: Vec<Item> = account_data[8..]
    .chunks(ITEM_SIZE)
    .map(|chunk| Item::deserialize(chunk))
    .collect::<Result<Vec<_>, _>>()?;

// Finding a specific account
let target = ctx.remaining_accounts.iter()
    .find(|acc| acc.key == &expected_pubkey)
    .ok_or(ProgramError::InvalidAccountData)?;
```

---

## Best Practices Summary

1. **Prefer iterators over manual indexing**: `for item in &vec` not `for i in 0..vec.len()`. Iterators are bounds-checked at compile time.

2. **Use `collect()` idiomatically**: Let type inference work for you with `collect::<Vec<_>>()`.

3. **Chain operations**: Instead of multiple loops, chain `.filter().map().collect()`. The compiler optimizes this into a single pass.

4. **Accept slices in function signatures**: `fn process(data: &[u8])` not `fn process(data: &Vec<u8>)`.

5. **Use the Entry API** for insert-or-update patterns on `HashMap`.

6. **Remember iterator laziness**: Adaptors like `map` and `filter` do nothing until consumed by `collect`, `sum`, `for_each`, etc.

7. **Use `enumerate` instead of manual counters**: `for (i, item) in collection.iter().enumerate()`.

---

## What's Next

In **Module 07**, we'll cover **Error Handling** — `Result<T, E>`, `Option<T>`, the `?` operator, and custom error types. This is where Rust really diverges from Solidity's `require()` / `revert()` pattern and becomes essential for writing robust Solana programs.
