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
     * Set the log index.
     * 
     * This function is used for debugging purposes.
     * It sets the log index to the specified value.
     */
    pub fn debug_set_counter(&mut self, c: u16) {
        self.c = c;
    }

    /**
     * Log a message.
     * 
     * The message is logged with the specified severity level.
     * The message is output to the console with a unique log index.
     */
    pub fn log(&mut self, t: LogLevel, s: &str) {
        match t {
            I => println!(
                "[{}:{}] {}",
                format!("{:#06x}", self.c).blue().bold(),
                "*".blue().bold(),
                s.blue()
            ),
            W => println!(
                "[{}:{}] {}",
                format!("{:#06x}", self.c).yellow().bold(),
                "!".yellow().bold(),
                s.yellow().bold()
            ),
            E => println!(
                "[{}:{}] {}",
                format!("{:#06x}", self.c).red().bold(),
                "✘".red().bold(),
                s.red().bold()
            ),
            S => println!(
                "[{}:{}] {}",
                format!("{:#06x}", self.c).green().bold(),
                "✔".green().bold(),
                s.green()
            ),
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

    #[test]
    fn logger_overflow() {
        let mut l = Logger::new();
        l.debug_set_counter(0xffff);
        l.log(I, "info");
        l.log(W, "warning");
        l.log(E, "error");
        l.log(S, "success");
    }
}
