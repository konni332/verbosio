#![allow(unused_imports)]
//! # verbosio
//!
//! A lightweight, macro-based logging utility for CLI tools and scripts.
//! Designed for ergonomics, speed, and minimal dependencies — with optional ANSI-colored output and terminal spinners.
//!
//! ## Features
//!
//! - Global verbosity level
//!     - Set/get via `set_verbosity!`, `get_verbosity!`, or `verbose_env!`
//! - Conditional message printing
//!     - `verbose!`, `vinfo!`, `vwarn!`, `verror!`, `vebug!`
//! - Distinct log levels: `INFO`, `WARN`, `ERROR`, `DEBUG`
//! - Optional section headers via `vsection!`
//! - Interactive terminal spinners via `status_line!` macros
//! - Terminal-safe output (no flickering) using `crossterm`
//! - All macros are verbosity-aware (`@lvl N`)
//!
//! ## Optional Features
//!
//! - `color`: Enables ANSI-colored output (via [`colored`](https://crates.io/crates/colored))
//! - `status`: Enables spinner-based status lines using [`crossterm`](https://crates.io/crates/crossterm)
//!
//! ## Dependencies
//!
//! | Name        | Purpose                          | Required by default  |
//! |-------------|----------------------------------|----------------------|
//! | `once_cell` | Global static verbosity state    |  Yes                 |
//! | `colored`   | Colored output for log levels    |  No (`color`)        |
//! | `crossterm` | Interactive terminal spinners    |  No (`status`)       |
//!
//! ## Example
//!
//! ```rust
//! use verbosio::*;
//!
//! set_verbosity!(2);
//!
//! verbose!(@lvl 1, "Hello World!");           // printed
//! vinfo!("App started.");                     // [INFO] App started.
//! vwarn!(@lvl 3, "This won't show.");         // not printed
//! verror!("Something went wrong");            // [ERROR] Something went wrong
//! ```
//!
//! ## Environment Support
//!
//! You can also set verbosity via the `VERBOSE` environment variable:
//!
//! ```rust
//! use verbosio::verbose_env;
//! unsafe {std::env::set_var("VERBOSE", "2");}
//! verbose_env!(); // Sets verbosity to 2
//! ```
//!
//! ## Status Line Example (feature = `"status"`)
//!
//! ```rust
//! use verbosio::{status_line, status_line_done};
//!
//! if let Some(spinner) = status_line!("Building project…") {
//!     // Simulate work
//!     std::thread::sleep(std::time::Duration::from_secs(2));
//!     spinner.stop();
//!     status_line_done!("Build complete.");
//! }
//! ```
//!
//! ## Section Headers
//!
//! ```rust
//! use verbosio::vsection;
//!
//! vsection!("Step 1: Preparation");
//! vsection!(@lvl 3, "Step 2: {:?}", "Details"); // only printed if verbosity ≥ 3
//! ```
//!
//! ## Debug-Only Output
//!
//! ```rust
//! use verbosio::vebug;
//!
//! vebug!("Always shown in debug mode.");
//! vebug!(@lvl 3, "Detailed: {:?}", "info"); // only if verbosity ≥ 3
//! ```
//!
//! ## Philosophy
//!
//! `verbosio` is built to be ultra-lightweight and practical for CLI tooling,
//! build scripts, and low-dependency utilities — with structured logging, and
//! configurable verbosity.
//!
//! ## Performance Note
//!
//! `vebug!` uses `#[cfg(debug_assertions)]` and is completely removed in release builds.
//! All other macros like `verbose!` and `status_line!` still check verbosity at runtime.




pub mod macros;
pub mod util;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU8};

pub static VERBOSE: Lazy<AtomicU8> = Lazy::new(|| AtomicU8::new(0));

/// Re-exports all macros for easy access.
pub use macros::verbosity::*;
pub use macros::terminal::*;
pub use util::*;