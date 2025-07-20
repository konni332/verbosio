# verbosio

A minimal, macro-based logging utility for CLI tools and developer scripts.  
Supports verbosity levels, optional colored output, and has **zero dependencies by default**.

---

## Features

- ✅ Global verbosity level (`u8`) via `AtomicU8`
- ✅ Macros like `vinfo!`, `vwarn!`, `verror!`, `verbose!`, and `vebug!`
- ✅ Optional colored output via the `color` feature
- ✅ Structured status lines with spinners (`status_line!`) via the `status` feature
- ✅ Zero-cost `vebug!` in release builds
- ✅ Configure via `set_verbosity!()` or environment variable (`VERBOSE`)
- ✅ Fast, minimal overhead when verbosity is too low

---

## Quick Start

Add `verbosio` to your `Cargo.toml`:

```toml
[dependencies]
verbosio = "0.2"
```

Optional: enable colored output
```toml
verbosio = { version = "0.2", features = ["color", "status", "time"] }
```

---

Then in your code:

````rust
use verbose_log::*;

fn main() {
    set_verbosity!(2);

    vinfo!("App started");
    vwarn!(@lvl 3, "This will only show if verbosity >= 3");
    verror!("Something went wrong");
}
````

---

## Macro Overview

| Macro                      | Description                                                             |
|----------------------------|-------------------------------------------------------------------------|
| `set_verbosity!(lvl?)`     | Set global verbosity (`u8`). Defaults to 1                              |
| `get_verbosity!()`         | Get current verbosity level                                             |
| `verbose_env!()`           | Set verbosity from `VERBOSE` env var                                    |
| `verbose!(@lvl?, ...)`     | Print raw message if verbosity ≥ level                                  |
| `vinfo!(@lvl?, ...)`       | Print `[INFO]` message if verbosity ≥ level                             |
| `vwarn!(@lvl?, ...)`       | Print `[WARN]` message if verbosity ≥ level                             |
| `verror!(@lvl?, ...)`      | Print `[ERROR]` message to stderr if verbosity ≥ level                  |
| `vebug!(@lvl?, ...)`       | Debug-only output — compiled out completely in release builds           |
| `status_line!(@lvl?, msg)` | Start live spinner with message if verbosity ≥ level (`status` feature) |
| `status_line_done!(msg)`   | Stop spinner and print final message (`status` feature)                 |
| `status_line_clear!()`     | Clear the current spinner line (`status` feature)                       |


*All ``lvl?`` are optional. If no level is given, it will be treated like a boolean value!*  
*This means, ``verbose!("foo")`` will print if the verbosity is >= 1*
*All lvl have to be explicit using `@lvl`: `verbose!(@lvl 2, "foo")`*

---

## Examples

You can find runnable examples in the [examples](./examples) directory.

## Notes

- verbose!() macros are compatible with async code
- Uses std::sync::atomic::AtomicU8 and once_cell internally
- Designed for CLI tools, dev utilities, and quick experiments

---

## Optional Features


| Feature   | Description                         | Default |
|-----------|-------------------------------------|---------|
| `color`   | ANSI-colored log levels             | No      |
| `status`  | Animated spinner via `status_line!` | No      |
| `time`    | Timestamps for verbose!, vinfo!,... | No      |  

## License

Licensed under either of:

- MIT
- Apache 2.0

Your choice

---