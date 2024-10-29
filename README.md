# Forestry

A CLI logging library written in Rust.

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

let l = Logger::new();
l.i("This is an info message.");
l.w("This is a warning message.");
l.e("This is an error message.");
l.s("This is a success message.");
```

## Contributing

If you would like to contribute to forestry, please open an issue or submit a pull request.

## License

This code is dual-licensed under either the MIT or Apache 2.0 license, at your option. Please see the `LICENSE` file for more information.
