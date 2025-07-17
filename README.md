# verbosio

A minimal macro-based logging utility for CLI tools and developer scripts.  
Supports verbosity levels, optional color output, and has **zero dependencies by default**.

##  Features

- ✅ Global verbosity level (`u8`) via `AtomicU8`
- ✅ Macros like `vinfo!`, `vwarn!`, `verror!` with level support
- ✅ Optional colored output via the `color` feature
- ✅ Configure via `set_verbosity!()` or environment variable (`VERBOSE`)
- ✅ Simple and fast. No formatting overhead when verbosity is too low

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
verbosio = "0.1"
```

Optional: enable colored output
```toml
verbosio = { version = "0.1", features = ["color"] }
```

---

Then in your code:

````rust
use verbose_log::*;

fn main() {
    set_verbosity!(2);

    vinfo!("App started");
    vwarn!(3, "This will only show if verbosity >= 3");
    verror!("Something went wrong");
}
````

---

## Macro Overview

| Macro                  | Description                                          |
|------------------------|------------------------------------------------------|
| `set_verbosity!(lvl?)` | Set the global verbosity level (`u8`). Defaults to 1 |
| `get_verbosity!()`     | Get current verbosity level                          |
| `verbose_env!()`       | Read verbosity from `VERBOSE` env var                |
| `verbose!(lvl?, ...)`  | Print only if verbosity ≥ `lvl`                      |
| `vinfo!(lvl?, ...)`    | Print `[INFO]` message                               |
| `vwarn!(lvl?, ...)`    | Print `[WARN]` message                               |
| `verror!(lvl?, ...)`   | Print `[ERROR]` to stderr                            |

*All ``lvl?`` are optional. If no level is given, it will be treated like a boolean value!*  
*This means, ``verbose!("foo")`` will print if the verbosity is >= 1*

---

## Examples

You can find runnable examples in the [examples](./examples) directory.

## Notes

- verbose!() macros are compatible with async code
- Uses std::sync::atomic::AtomicU8 and once_cell internally
- Designed for CLI tools, dev utilities, and quick experiments

---

## License

Licensed under either of:

- MIT
- Apache 2.0

Your choice

---