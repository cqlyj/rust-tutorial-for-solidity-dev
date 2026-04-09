// ============================================================
// utils/mod.rs — The utils module
// ============================================================
// This file exists at src/utils/mod.rs, which makes it the
// `utils` module. The directory form (utils/mod.rs) is used
// when a module has submodules — in our case, `helpers`.
//
// Alternative: Rust also supports src/utils.rs with the
// submodule at src/utils/helpers.rs, but mod.rs is the
// traditional pattern you'll see in many Solana projects.
// ============================================================

// Declare the `helpers` submodule.
// Rust will look for src/utils/helpers.rs to find this module.
// `pub` makes the helpers module visible outside of utils.
pub mod helpers;

// Re-export commonly used items from helpers.
// This means users can write `utils::format_lamports()`
// instead of `utils::helpers::format_lamports()`.
// This is the same pattern Anchor uses with its prelude.
pub use helpers::format_lamports;
pub use helpers::format_sol;

// A constant defined directly in the utils module.
// pub(crate) makes it visible throughout our crate
// but not to external consumers.
pub(crate) const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

// A utility function defined in the utils module itself.
// Validates that a wallet address string looks reasonable.
pub fn is_valid_address(address: &str) -> bool {
    // Solana addresses are base58-encoded and 32-44 characters long.
    // This is a simplified check for demonstration.
    let len = address.len();
    // Check that the length is within the expected range.
    (32..=44).contains(&len)
        // Check that all characters are valid base58 characters.
        && address.chars().all(|c| {
            // Base58 excludes 0, O, I, l to avoid ambiguity.
            c.is_ascii_alphanumeric() && c != '0' && c != 'O' && c != 'I' && c != 'l'
        })
}
