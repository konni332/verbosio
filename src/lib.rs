//! # verbosio
//!
//! A lightweight, macro-based logging utility for CLI tools and scripts.
//! Designed for ergonomics, speed, and optional colored output — with minimal dependencies.
//!
//! ## Features
//! - Global verbosity level (via `set_verbosity!`, `get_verbosity!`, `verbose_env!`)
//! - Print messages conditionally based on verbosity (`verbose!`, `vinfo!`, `vwarn!`, `verror!`)
//! - Distinct log levels: `INFO`, `WARN`, `ERROR`
//! - Optional ANSI-colored output via the `color` feature
//!
//! ## Dependencies
//! - [`once_cell`](https://crates.io/crates/once_cell): For global static verbosity state (always included)
//! - [`colored`](https://crates.io/crates/colored): Optional, behind the `color` feature flag
//!
//! ## Example
//! ```rust
//! use verbosio::*;
//!
//! set_verbosity!(2);
//!
//! verbose!(1, "Hello World!");         // printed
//! vinfo!("App started.");              // [INFO] App started.
//! vwarn!(3, "This won't show.");       // not printed
//! verror!("Something went wrong");     // [ERROR] Something went wrong
//! ```
//!
//! ## Environment Support
//! You can also set verbosity based on the `VERBOSE` environment variable:
//!
//! ```rust
//! use verbosio::verbose_env;
//! unsafe {std::env::set_var("VERBOSE", "2");}
//! verbose_env!(); // Sets verbosity to 2
//! ```


pub mod macros;
pub mod util;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU8};

pub static VERBOSE: Lazy<AtomicU8> = Lazy::new(|| AtomicU8::new(0));

/// Re-exports all macros for easy access.
#[allow(unused_imports)]
pub use macros::*;
pub use util::*;