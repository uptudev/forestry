# libforestry

`forestry` is a simplistic logging library meant to quickly add logging to your C project.

## Contents

* [Installation](#installation)
* [Usage](#usage)
  * [Logging](#logging)
  * [Closing scope](#closing-scope)
  * [Configuration](#configuration)
    * [Formatting](#formatting)
* [Contributing](#contributing)
* [License](#license)
* [Miscellaneous](#miscellaneous)

## Installation

This library can be installed bu cloning the repository, then running build commands to generate the shared library:

```bash
# Clone the 'c' branch of the repository (the default Rust branch is 'main')
git clone -b c https://github.com/uptudev/forestry.git --single-branch
cd forestry
# Configure the build with CMake
cmake . -DCMAKE_BUILD_TYPE=Release
# Build the shared library
make forestry
# OR
# Build a static library
make forestry_static
```

To ensure the library is installed correctly, you can run the test script by running the following commands:

```bash
# Build the test program
make forestry_test
# Run the test program
./forestry_test
```

There should now be a `.log` file in the current directory with the following log messages (the log file name is dependent when the test runs):

```plaintext
[0000:*] INFO
[0001:~] WARNING
[0002:!] ERROR
[0003:+] SUCCESS
[0004:%] CRITICAL ERROR
[0005:?] DEBUG
```

## Usage

### Logging

* `void log_info(char *message)`: Logs an info message.
* `void log_warning(char *message)`: Logs a warning message.
* `void log_error(char *message)`: Logs an error message.
* `void log_success(char *message)`: Logs a success message.
* `void log_critical(char *message)`: Logs a critical message.
* `void log_debug(char *message)`: Logs a debug message.

### Closing scope

* `void log_deinit()`: Closes the logger and frees its resources.

Programmers **must** call this at the end of the program to avoid memory leaks,
as it frees the heap buffer and any file handles given to `set_log_file()`.

### Configuration

* `void set_log_timer(clock_t start)`: Sets the optional timer for the log messages.
* `void set_log_file(FILE *file)`: Sets the optional file to log to.
* `void set_log_opt(const LogOptions opt)`: Enables a specific logging option.

#### Formatting

`LogOptions` is an enumerator with the following members:

* `NO_INDEX`: Disables the index in the log message.
* `NO_SYMBOL`: Disables the log level symbol ('*', '~', etc.) in the log message.
* `NO_COLOR`: Disables the ANSI colouring in the log message.
* `NO_BOLD`: Disables the ANSI bold text in the log message.
* `PLAIN`: Removes all ANSI escape sequences from the log message.
* `BASIC`: Removes all extra formatting from the log message.
* `TIMER`: Adds a C clock to count elapsed CPU clocks (not necessarily time).
* `LOG_FILE`: Logs the message to a file.
* `ONLY_FILE`: Logs the message to a file only. Recommended to use with `PLAIN`.
* `RESET`: Resets the format options to the default.

## Contributing

**Outline your contribution guidelines.**
Explain how users can contribute to your project,
whether through code, bug reports, or documentation improvements.
Specify preferred code style, pull request format, and testing procedures.

## License

**Specify the license that you plan to distribute your project under.**
Use clear and concise language, and link to the full license text.

## Miscellaneous

**Include any other relevant information you want to share.**
This could be links to related projects, documentation,
support channels, or your contact information.

**Remember:**

* Keep your README.md file concise and focused.
* Use clear headings, formatting, and visuals for readability.
* Update your README.md file regularly to reflect changes in your project.
