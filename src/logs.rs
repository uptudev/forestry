use colored::*;

/**
 * A simple logger for an application.
 * 
 * The logger is used to log messages to the console.
 * The messages are coloured based on their severity level.
 * Logs are output with a unique 16-bit log index.
 */
pub struct Logger {
    c: u16,
}

/**
 * The severity level of the log message.
 * 
 * I: Info
 * W: Warn
 * E: Error
 * S: Success
 */
pub enum LogLevel {
    /**
     * Info
     * 
     * The log message is for general informational purposes.
     * It is prefaced with an asterisk and coloured blue.
     */
    I,
    W,
    E,
    S,
}
pub use LogLevel::*;

impl Logger {
    /**
     * Create a new logger.
     * 
     * The logger is initialised with a log index of 0.
     */
    pub fn new() -> Self {
        Logger { c: 0 }
    }

    /**
     * Log a message.
     * 
     * The message is logged with the specified severity level.
     * The message is output to the console with a unique log index.
     *
     * # Arguments
     * - `t`: The severity level of the log message.
     * - `s`: The message to log.
     *
     * # Example
     * ```rust
        use forestry::prelude::*;
        let mut l = Logger::new();
        l.log(I, "info");       // Output: [0000:*] info
        l.log(W, "warning");    // Output: [0001:!] warning
     * ```
     */
    pub fn log(&mut self, t: LogLevel, s: &str) {
        let count = format!("{:0>4x}", self.c);
        match t {
            I => {
                println!(
                    "[{}:{}] {}",
                    count.blue().bold(),
                    "*".blue().bold(),
                    s.blue());
            },
            W => {
                println!(
                    "[{}:{}] {}",
                    count.yellow().bold(),
                    "!".yellow().bold(),
                    s.yellow().bold())
            },
            E => {
                println!(
                    "[{}:{}] {}",
                    count.red().bold(),
                    "✘".red().bold(),
                    s.red().bold())
            },
            S => {
                println!(
                    "[{}:{}] {}",
                    count.green().bold(),
                    "✔".green().bold(),
                    s.green())
            },
        }
        self.c = self.c.wrapping_add(1);
        if self.c == 0 {
            self.log(W, "Log index overflowed; log index may be inaccurate.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logger_prints() {
        let mut l = Logger::new();
        l.log(I, "info");
        l.log(W, "warning");
        l.log(E, "error");
        l.log(S, "success");
    }
}
