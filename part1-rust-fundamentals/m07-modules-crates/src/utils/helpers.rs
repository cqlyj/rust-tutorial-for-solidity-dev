// ============================================================
// utils/helpers.rs — The helpers submodule
// ============================================================
// This is a nested module: utils::helpers.
// It's declared by `pub mod helpers;` in utils/mod.rs.
// We use `super::` to access items from the parent (utils) module.
// ============================================================

// Import the constant from our parent module (utils/mod.rs).
// `super::` means "go up one level" — from helpers to utils.
// This is like using `../` in file paths.
use super::LAMPORTS_PER_SOL;

// Format a lamport amount with commas for readability.
// pub makes this callable from outside the helpers module.
pub fn format_lamports(lamports: u64) -> String {
    // Convert the number to a string.
    let s = lamports.to_string();
    // Collect characters into a vector for processing.
    let chars: Vec<char> = s.chars().collect();
    // Build the formatted string with comma separators.
    let mut result = String::new();
    // Iterate over each character with its index.
    for (i, ch) in chars.iter().enumerate() {
        // Add a comma every 3 digits from the right (not at the start).
        if i > 0 && (chars.len() - i) % 3 == 0 {
            // Push a comma separator.
            result.push(',');
        }
        // Push the digit character.
        result.push(*ch);
    }
    // Append the unit and return.
    format!("{} lamports", result)
}

// Convert lamports to SOL and format as a string.
// Uses the LAMPORTS_PER_SOL constant from the parent module.
pub fn format_sol(lamports: u64) -> String {
    // Divide lamports by 1 billion to get SOL amount.
    let sol = lamports as f64 / LAMPORTS_PER_SOL as f64;
    // Format to 9 decimal places (lamport precision).
    format!("{:.9} SOL", sol)
}

// A private helper function — only visible within this module.
// Demonstrates that not everything needs to be pub.
// Other modules cannot call this, even within the same crate.
#[allow(dead_code)]
fn truncate_address(address: &str, chars: usize) -> String {
    // If the address is short enough, return it as-is.
    if address.len() <= chars * 2 + 3 {
        // No truncation needed.
        return address.to_string();
    }
    // Take the first `chars` characters.
    let start: String = address.chars().take(chars).collect();
    // Take the last `chars` characters.
    let end: String = address.chars().rev().take(chars).collect::<Vec<_>>()
        // Reverse them back to the correct order.
        .into_iter().rev().collect();
    // Return truncated format: "Abc1...xyz9".
    format!("{}...{}", start, end)
}
