/// Sets the global verbosity level.
///
/// # Usage
///
/// - `set_verbosity!();` sets verbosity to `1`.
/// - `set_verbosity!(3);` sets verbosity to level `3`.
///
/// # Example
/// ```rust
/// use verbosio::set_verbosity;
///
/// set_verbosity!();       // same as set_verbosity!(1);
/// set_verbosity!(2);      // sets verbosity to 2
/// ```
#[macro_export]
macro_rules! set_verbosity {
    () => {
        $crate::VERBOSE.store(1, std::sync::atomic::Ordering::Relaxed);
    };
    ($lvl:expr) => {
        $crate::VERBOSE.store($lvl, std::sync::atomic::Ordering::Relaxed);
    };
}

/// Retrieves the current global verbosity level.
///
/// # Returns
/// A `u8` value representing the current verbosity.
///
/// # Example
/// ```rust
/// use verbosio::get_verbosity;
/// let level = get_verbosity!();
/// println!("Current verbosity: {level}");
/// ```
#[macro_export]
macro_rules! get_verbosity {
    () => {
        $crate::VERBOSE.load(std::sync::atomic::Ordering::Relaxed)
    };
}

/// Sets verbosity level based on the `VERBOSE` environment variable.
///
/// If the variable is not set or cannot be parsed as a number,
/// it defaults to `0`.
///
/// # Example
/// ```rust
/// use verbosio::verbose_env;
/// unsafe {std::env::set_var("VERBOSE", "2");}
/// verbose_env!(); // sets verbosity to 2
/// ```
#[macro_export]
macro_rules! verbose_env {
    () => {
        let val = std::env::var("VERBOSE").unwrap_or_default();
        $crate::set_verbosity!(val.parse::<u8>().unwrap_or(0));
    };
}

/// Prints a message to stdout if the verbosity is high enough.
///
/// # Syntax
///
/// - `verbose!(@lvl 1, "Message: {}", value);` → prints if verbosity ≥ 1
/// - `verbose!("Message");`               → prints if verbosity ≥ 1 (shorthand)
///
/// # Example
/// ```rust
/// use verbosio::{set_verbosity, verbose};
///
/// set_verbosity!(2);
/// verbose!(@lvl 1, "This shows");    // printed
/// verbose!(@lvl 2, "This will show {}", "too"); // printed
/// verbose!("And so will {}", "this"); // printed
/// verbose!(@lvl 3, "This won't");    // not printed
/// ```
#[macro_export]
macro_rules! verbose {
    (@lvl $lvl:expr, $($arg:tt)+) => {
        if $crate::VERBOSE.load(std::sync::atomic::Ordering::Relaxed) >= $lvl {
            let msg = format!("{}{}", $crate::format_time(), format!($($arg)+));
            println!("{}", msg);
        }
    };
    ($( $arg:tt )+) => {
        if $crate::VERBOSE.load(std::sync::atomic::Ordering::Relaxed) >= 1 {
            let msg = format!("{}{}", $crate::format_time(), format!($($arg)+));
            println!("{}", msg);
        }
    };
}

/// Prints an `[INFO]` message to stdout if the verbosity is high enough.
///
/// # Syntax
///
/// - `vinfo!(@lvl 2, "Loaded {} items", count);` → prints if verbosity ≥ 2
/// - `vinfo!("Starting...");`               → prints if verbosity ≥ 1 (default)
///
/// # Example
/// ```rust
/// use verbosio::{set_verbosity, vinfo};
///
/// set_verbosity!(2);
/// vinfo!("App started");       // printed
/// vinfo!(@lvl 3, "Details...");     // not printed
/// ```
///
/// # Output Format
/// Outputs messages like `[INFO] your message...`
///
/// # Features
/// If the `"colors"` feature is enabled, the `[INFO]` tag may appear colored.
#[macro_export]
macro_rules! vinfo {
    (@lvl $lvl:expr, $($arg:tt)+) => {
        if $crate::VERBOSE.load(std::sync::atomic::Ordering::Relaxed) >= $lvl {
            println!("{}{}{}", $crate::format_level("INFO"), $crate::format_time(), format!($($arg)+));
        }
    };
    ($( $arg:tt )+) => {
        if $crate::VERBOSE.load(std::sync::atomic::Ordering::Relaxed) >= 1 {
            println!("{}{}{}", $crate::format_level("INFO"), $crate::format_time(), format!($($arg)+));
        }
    };
}

/// Prints a `[WARN]` message to stdout if the verbosity is high enough.
///
/// # Syntax
///
/// - `vwarn!(@lvl 2, "Low memory");`      → prints if verbosity ≥ 2
/// - `vwarn!("Disk almost full");`   → prints if verbosity ≥ 1
///
/// # Output Format
/// Outputs messages like `[WARN] your message...`
///
/// # Example
/// ```rust
/// use verbosio::{set_verbosity, vwarn};
///
/// set_verbosity!(1);
/// vwarn!("Something might be wrong");
/// ```
///
/// # Features
/// With `"colors"` feature enabled, the `[WARN]` tag may be yellow.
#[macro_export]
macro_rules! vwarn {
    (@lvl $lvl:expr, $($arg:tt)+) => {
        if $crate::VERBOSE.load(std::sync::atomic::Ordering::Relaxed) >= $lvl {
            println!("{}{}{}", $crate::format_level("WARN"), $crate::format_time(), format!($($arg)+));
        }
    };
    ($( $arg:tt )+) => {
        if $crate::VERBOSE.load(std::sync::atomic::Ordering::Relaxed) >= 1 {
            println!("{}{}{}", $crate::format_level("WARN"), $crate::format_time(), format!($($arg)+));
        }
    };
}

/// Prints an `[ERROR]` message to stderr if the verbosity is high enough.
///
/// # Syntax
///
/// - `verror!(@lvl 3, "Critical: {}", reason);` → prints if verbosity ≥ 3
/// - `verror!("Oops");`                    → prints if verbosity ≥ 1
///
/// # Output Format
/// Messages appear as `[ERROR] ...` and are printed to `stderr`.
///
/// # Example
/// ```rust
/// use verbosio::{set_verbosity, verror};
///
/// set_verbosity!(2);
/// verror!("An error occurred"); // printed
/// verror!(@lvl 3, "Only for debug"); // not printed
/// ```
///
/// # Features
/// If the `"colors"` feature is enabled, the `[ERROR]` tag may be red.
#[macro_export]
macro_rules! verror {
    (@lvl $lvl:expr, $($arg:tt)+) => {
        if $crate::VERBOSE.load(std::sync::atomic::Ordering::Relaxed) >= $lvl {
            eprintln!("{}{}{}", $crate::format_level("ERROR"), $crate::format_time(), format!($($arg)+));
        }
    };
    ($( $arg:tt )+) => {
        if $crate::VERBOSE.load(std::sync::atomic::Ordering::Relaxed) >= 1 {
            eprintln!("{}{}{}", $crate::format_level("ERROR"), $crate::format_time(), format!($($arg)+));
        }
    };
}


/// Prints a `[DEBUG]` message to stdout if in debug mode.
///
/// # Syntax
///
/// - `vebug!(@lvl 3, "Critical: {}", reason);` → prints if verbosity ≥ 3 and in debug mode
/// - `vebug!("Oops");`                    → prints if in debug mode
///
/// # Output Format
/// Messages appear as `[DEBUG] ...` and are printed to `stdout`.
///
/// # Example
/// ```rust
/// use verbosio::{set_verbosity, vebug};
///
/// vebug!("Always in debug mode!"); // printed
/// vebug!(@lvl 3, "Only for debug when level is high enough"); // not printed
/// ```
///
/// # Features
/// If the `"colors"` feature is enabled, the `[DEBUG]` tag may be yellow.
#[macro_export]
macro_rules! vebug {
    (@lvl $lvl:expr, $($arg:tt)+) => {
        #[cfg(debug_assertions)]
        {
            if $crate::VERBOSE.load(std::sync::atomic::Ordering::Relaxed) >= $lvl {
                println!("{}{}{}", $crate::format_level("DEBUG"), $crate::format_time(), format!($($arg)+));
            }
        }
    };
    ($($arg:tt)+) => {
        #[cfg(debug_assertions)]
        {
            println!("{}{}{}", $crate::format_level("DEBUG"), $crate::format_time(), format!($($arg)+));
        }
    };
}
