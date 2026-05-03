# forestry

[![crate version number](https://img.shields.io/crates/v/forestry)](https://crates.io/crates/forestry/)
[![crate download count](https://img.shields.io/crates/d/forestry?label=downloads&color=blue)](https://crates.io/crates/forestry/)
[![GitHub Actions status](https://img.shields.io/github/actions/workflow/status/uptudev/forestry/rust.yml)](https://github.com/uptudev/forestry/actions/workflows/rust.yml)

A simple, efficient, concise, and elegant logging library for Rust.

## Table of Contents

* [Installation](#installation)
* [Dependencies](#dependencies)
* [Usage](#usage)
* [Contributing](#contributing)
* [License](#license)

## Installation

To install forestry, simply add it to your Rust project via Cargo:

```bash
cargo add forestry
```

## Dependencies

Forestry depends on the `textfmt` crate for colorized output as well as the `supports-color` 
crate. If the `async` feature is enabled, it also depends on the `tokio` crate
for an async runtime

## Usage

Forestry is a simple logging library that allows you to log messages to the console 
with different levels of severity. Here's an example of how to use it:

`src/main.rs`
```rust
use forestry::prelude::*;

fn main() {
    let log = Logger::new();
    log.info("This is an info message.");
    log.warn("This is a warning message.");
    log.error("This is an error message.");
    log.success("This is a success message.");
    log.critical("This is a critical message.");
}
```

### Inlining

These calls can also be inlined as follows

`src/main.rs`
```rust
use forestry::prelude::*

fn main() {
    let log = Logger::new();
    log.info("This is an info message.")
        .warn("This is a warning message.")
        .error("This is an error message.")
        .success("This is a success message.")
        .critical("This is a critical message.");
}
```

### Static Logging

Using the `static` feature allows for the following to be run on a single static 
`Logger` instance

`src/main.rs`
```rust
use forestry::prelude::*;

fn main() {
    info("This is an info message.");
    warn("This is a warning message.");
    error("This is an error message.");
    success("This is a success message.");
    critical("This is a critical message.");
}
```

This requires your `Cargo.toml` to have the feature enabled, which is available 
on versions after and including `1.8.0`. To do so, ensure your `Cargo.toml` has either

`Cargo.toml`
```toml
[dependencies]
forestry = { version = "1.8.0", features = ["static"] }
```

or

`Cargo.toml`
```toml
[dependencies.forestry]
version = "1.8.0"
features = ["static"]
```

---

These will all output the following to the console:

```
[0000:*] This is an info message.
[0001:~] This is a warning message.
[0002:!] This is an error message.
[0003:+] This is a success message.
[0004:%] This is a critical message.
```

It will also be coloured in most terminals.

All formatting is optional; please see the documentation at 
[Docs.rs](https://docs.rs/forestry/latest/forestry/index.html), 
specifically for `crate::logs::Options`. Optional file output and timer inclusion 
is also supported via the same `crate::logs::Options` enum.

### Async

Forestry also supports asynchronous logging. To enable this feature, simply add 
the `async` feature to `forestry`'s declaration in your `Cargo.toml` file.

#### Example

First, add the `async` feature to `forestry` in your `Cargo.toml` file by changing 
the default declaration to either of the following:

`Cargo.toml`
```toml
[dependencies]
forestry = { version = ">=1.5", features = ["async"] }
```

or

`Cargo.toml`
```toml
[dependencies.forestry]
version = ">=1.5"
features = ["async"]
```

Then, the logger's internal print calls will be asynchronous futures. This is useful 
for logging in async functions or in async contexts. `await`ing the logger's methods 
will return the same `&mut Logger` as before, so chaining is still possible (although 
only by adding `await` to every call).

`async` is compatible with `static` as of version `1.9.0`, and will allow for `.await`ing 
on the static calls if both features are enabled.

## Contributing

If you would like to contribute to forestry, please open an issue or submit a pull 
request.

## License

This code is dual-licensed under either the MIT or Apache 2.0 license, at your option. 
Please see the `LICENSE` file for more information.
