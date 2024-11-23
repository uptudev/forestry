//! A simple, efficient, concise, and elegant logging library for Rust.
//!
//! Provides a [Logger](logs::Logger) struct that can be used to log messages to the console.
//! The messages are coloured based on their severity level.
//! Logs are output with a unique 16-bit log index.
//! The log also contains a symbol to represent the severity level.
//!
//! Most of the time, you will only need to initialize a [Logger](logs::Logger) struct.
//! After this,you can use the 
//! - [info](logs::Logger::info), 
//! - [warn](logs::Logger::warn),
//! - [error](logs::Logger::error), 
//! - [success](logs::Logger::success), and
//! - [critical](logs::Logger::critical) methods to log messages.
//!
//! Asynchronous versions of the above methods are also available by enabling the `async` feature.
//!
//! All configuration can be done using the [cfg()](logs::Logger::cfg) method in conjunction
//! with the [Options](logs::Options) enumerator. Valid options include adding a timer, logging to
//! a file, and more.
//! See the [logs] module for more details.

pub mod logs;
pub mod prelude {
    pub use crate::logs::*;
}
#[cfg(test)]
mod tests;
