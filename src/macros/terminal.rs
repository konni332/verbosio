
/// Prints a formatted section header if the verbosity level is high enough.
///
/// # Syntax
///
/// - vsection!(@lvl 2, "Header: {}", task); → prints if verbosity ≥ 2
/// - vsection!("Title"); → prints if verbosity ≥ 1
///
/// # Output Format
/// Each section appears as:
/// /// === Your Title Here === ///
///
/// # Example
/// ```rust
/// use verbosio::{vsection};
/// vsection!("Main Phase"); // printed
/// vsection!(@lvl 3, "Subtask: {}", "init"); // not printed
///```
///
/// # Features
/// If the "colors" feature is enabled, the section title may be colored.
#[macro_export]
macro_rules! vsection {
    (@lvl $lvl:expr, $($arg:tt)+) => {
        if $crate::VERBOSE.load(std::sync::atomic::Ordering::Relaxed) >= $lvl {
            let msg = format!("=== {} ===\n", format!($($arg)+));
            println!("{}", msg);
        }
    };
    ( $($arg:tt)+) => {
        if $crate::VERBOSE.load(std::sync::atomic::Ordering::Relaxed) >= 1 {
            let msg = format!("=== {} ===\n", format!($($arg)+));
            println!("{}", msg);
        }
    };
}

/// Starts a status spinner in the terminal if verbosity is high enough.
///
/// # Syntax
///
/// - status_line!(@lvl 2, "Working on task...") → spinner if verbosity ≥ 2
/// - status_line!("Processing…") → spinner if verbosity ≥ 1
///
/// # Return
/// Returns Some(SpinnerHandle) if spinner was shown, otherwise None.
/// Call .stop() on the handle to stop the spinner manually.
///
/// # Example
///```rust
/// use verbosio::{status_line};
///
/// if let Some(spinner) = status_line!("Building…") {
/// // Do some work
/// std::thread::sleep(std::time::Duration::from_secs(2));
/// spinner.stop();
/// }
///```
///
/// # Notes
/// The spinner runs in a background thread and redraws every 100ms.
/// You must manually .stop() it to avoid ghosting.
///
/// # Features
/// Only available if the "status" feature is enabled.
#[cfg(feature = "status")]
#[macro_export]
macro_rules! status_line {
    (@lvl $lvl:expr, $msg:expr) => {{
        if $crate::VERBOSE.load(std::sync::atomic::Ordering::Relaxed) >= $lvl {
            Some($crate::status::start_spinner(|| $msg.to_string()))
        } else {
            None
        }
    }};
    ($msg:expr) => {
        $crate::status_line!(@lvl 1, $msg)
    };
}


/// Clears the current status line from the terminal.
///
/// # Syntax
///
/// - status_line_clear!();
///
/// # Effect
/// Clears any visible spinner or status line from the terminal output.
/// Has no effect if no spinner is active.
///
/// # Example
///```rust
/// use verbosio::status_line_clear;
/// status_line_clear!();
///```
///
/// # Features
/// Only available if the "status" feature is enabled.
#[cfg(feature = "status")]
#[macro_export]
macro_rules! status_line_clear {
    () => {{
        $crate::status::clear_status_line();
    }};
}

/// Finishes a status line and prints a final message in its place.
///
/// # Syntax
///
/// - status_line_done!("Done");
///
/// # Effect
/// Stops any active spinner and replaces it with the given message.
///
/// # Example
///```rust
/// use verbosio::{status_line, status_line_done};
///
/// if let Some(spinner) = status_line!("Compiling") {
/// // Do work...
/// spinner.stop();
/// status_line_done!("Compiled successfully!");
/// }
///```
///
/// # Features
/// Only available if the "status" feature is enabled.
#[cfg(feature = "status")]
#[macro_export]
macro_rules! status_line_done {
    ($msg:expr) => {{
        #[cfg(feature = "status")]
        $crate::status::finish_status_line($msg);
    }};
}



#[cfg(feature = "status")]
pub mod status {
    use std::io::{stdout, Write};
    use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
    use std::thread;
    use std::time::Duration;
    use crossterm::{execute, terminal::{ClearType, Clear}, cursor::{MoveToColumn}};

    static SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];



    pub struct SpinnerHandle {
        stop: Arc<AtomicBool>,
    }

    impl SpinnerHandle {
        pub fn stop(self){
            self.stop.store(true, Ordering::Relaxed);
        }
    }

    pub fn start_spinner<F: Fn() -> String + Send + 'static>(msg_fn: F) -> SpinnerHandle {
        let stop = Arc::new(AtomicBool::new(false));
        let stop_clone = stop.clone();

        thread::spawn(move || {
            let mut idx = 0;
            while !stop_clone.load(Ordering::Relaxed) {
                let spinner = SPINNER_FRAMES[idx % SPINNER_FRAMES.len()];
                let msg = format!("{} {}", spinner, msg_fn());

                execute!(
                    stdout(),
                    MoveToColumn(0),
                    Clear(ClearType::CurrentLine),
                ).unwrap();
                print!("{}", msg);
                stdout().flush().unwrap();

                idx += 1;
                thread::sleep(Duration::from_millis(100));
            }
        });

        SpinnerHandle {stop}
    }
    pub fn clear_status_line(){
        let _ = execute!(
            stdout(),
            MoveToColumn(0),
            Clear(ClearType::CurrentLine),
        );
        let _ = stdout().flush();
    }
    pub fn finish_status_line(msg: &str){
        clear_status_line();
        println!("{}", msg);
    }
}








