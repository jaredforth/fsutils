[![Build Status](https://travis-ci.com/jaredforth/fsutils.svg?token=mH2pScYxqRkBEzpBQAu6&branch=master)](https://travis-ci.com/jaredforth/fsutils)
[![Build status](https://ci.appveyor.com/api/projects/status/w75cp0q4qr0hngf8?svg=true)](https://ci.appveyor.com/project/jaredforth/fsutils)
[![Crate](https://img.shields.io/crates/v/fsutils.svg)](https://crates.io/crates/fsutils)
![Crates.io](https://img.shields.io/crates/d/fsutils)
[![API](https://docs.rs/fsutils/badge.svg)](https://docs.rs/fsutils)

# fsutils

Utilities for common filesystem operations.

**fsutils** provides an API based on Bash commands and includes a number
of utility functions to make interacting with the filesystem simpler and more ergonomic.

Documentation:
-   [API Reference](https://docs.rs/fsutils)


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
fsutils = "0.1"
```

## Error Logging 

This creates uses the `log` and `env_logger` crates. To enable logging in your application, add `env_logger::init();` to your `main()` function and set the log level to *info* with `RUST_LOG="info" ./yourapp`.


## License

**fsutils** is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
