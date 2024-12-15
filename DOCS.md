# forestry

`forestry` is a simplistic logging library meant to quickly add logging to your C project.

## Features

It provides the following:

### Logging

- `void log_info(char *message)`: Logs an info message.
- `void log_warning(char *message)`: Logs a warning message.
- `void log_error(char *message)`: Logs an error message.
- `void log_success(char *message)`: Logs a success message.
- `void log_critical(char *message)`: Logs a critical message.
- `void log_debug(char *message)`: Logs a debug message.

### Closing scope

- `void log_deinit()`: Closes the logger and frees its resources.

Programmers **must** call this at the end of the program to avoid memory leaks,
as it frees the heap buffer and any file handles given to `set_log_file()`.

### Configuration

- `void set_log_timer(clock_t start)`: Sets the optional timer for the log messages.
- `void set_log_file(FILE *file)`: Sets the optional file to log to.
- `void set_log_opt(const FormatOptions opt)`: Enables a specific format option.

#### Formatting

`FormatOptions` is an enumerator with the following members:

- `NO_INDEX`: Disables the index in the log message.
- `NO_SYMBOL`: Disables the log level symbol ('*', '~', etc.) in the log message.
- `NO_COLOR`: Disables the ANSI colouring in the log message.
- `NO_BOLD`: Disables the ANSI bold text in the log message.
- `PLAIN`: Removes all ANSI escape sequences from the log message.
- `BASIC`: Removes all extra formatting from the log message.
- `TIMER`: Adds a C clock to count elapsed CPU clocks (not necessarily time).
- `LOG_FILE`: Logs the message to a file.
- `ONLY_FILE`: Logs the message to a file only. Recommended to use with `PLAIN`.
- `RESET`: Resets the format options to the default.
