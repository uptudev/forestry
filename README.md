# forestry

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

Forestry depends on the `colored` crate for colorized output.

## Usage

Forestry is a simple logging library that allows you to log messages to the console with different levels of severity. Here's an example of how to use it:

```rust
use forestry::prelude::*;

let log = Logger::new();
log.info("This is an info message.");
log.warn("This is a warning message.");
log.error("This is an error message.");
log.success("This is a success message.");
log.critical("This is a critical message.");
```

These calls can also be inlined as follows

```rust
use forestry::prelude::*

let log = Logger::new();
log.info("This is an info message.")
    .warn("This is a warning message.")
    .error("This is an error message.")
    .success("This is a success message.")
    .critical("This is a critical message.");
```

This will output the following to the console:

```
[0000:*] This is an info message.
[0001:~] This is a warning message.
[0002:!] This is an error message.
[0003:+] This is a success message.
[0004:%] This is a critical message.
```

It will also be coloured in most terminals.

All formatting is optional; please see the documentation at [Docs.rs](https://docs.rs/forestry/latest/forestry/index.html), specifically for `crate::logs::Options`. File output is also supported via the same `crate::logs::Options` enum.

## Contributing

If you would like to contribute to forestry, please open an issue or submit a pull request.

## License

This code is dual-licensed under either the MIT or Apache 2.0 license, at your option. Please see the `LICENSE` file for more information.
