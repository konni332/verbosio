//! # verbosio
//!
//! A lightweight, zero-dependency macro-based logging utility for CLI tools and scripts.
//! Designed for ergonomics and speed, with optional color support and verbosity levels.
//!
//! ## Features
//! - Set and get verbosity level globally
//! - Print messages conditionally based on verbosity
//! - Supports info/warn/error levels
//! - Optional ANSI-colored output via the `color` feature
//!
//! ## Example
//! ```rust
//! use verbosio::*;
//!
//! set_verbosity!(2);
//! verbose!(1, "Hello World!"); // Hello World!
//! vinfo!("App started."); // [INFO] App started.
//! vwarn!(3, "This won't show."); // not printed (would show: [WARN] This won't show.)
//! verror!("Something went wrong"); // [ERROR] Something went wrong
//! ```

pub mod macros;
pub mod util;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU8};

pub static VERBOSE: Lazy<AtomicU8> = Lazy::new(|| AtomicU8::new(0));

/// Re-exports all macros for easy access.
pub use macros::*;
pub use util::*;