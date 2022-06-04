//! # Rust Crate
//!
//! `rust` is a lib which contains `add_one` function

pub mod extra;
pub mod learn;

pub use extra::smartpoint;

/// Adds one to the number given.
///
/// # Examples
/// ```
/// let arg = 5;
/// let answer = learn_rust::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
