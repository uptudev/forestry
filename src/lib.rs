//! A simple, efficient, concise, and elegant logging library for Rust.
//!
//! Provides a `Logger` struct that can be used to log messages to the console.
//! The messages are coloured based on their severity level.
//! Logs are output with a unique 16-bit log index.
//! The log also contains a symbol to represent the severity level.
//!
//! All formatting can be configured using the `cfg()` method in conjunction with the
//! `FormatOptions` enumerator. See the `logs` module for more details.

pub mod logs;
pub mod prelude {
    pub use crate::logs::*;
}
#[cfg(test)]
mod tests;
