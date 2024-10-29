use colored::*;

/**
 * A simple logger for an application.
 * 
 * The logger is used to log messages to the console.
 * The messages are coloured based on their severity level.
 * Logs are output with a unique 16-bit log index.
 */
pub struct Logger(u16);

impl Logger {
    /**
     * Create a new logger.
     * 
     * The logger is initialised with a log index of 0.
     */
    pub fn new() -> Self {
        Logger(0)
    }

    /**
     * Log a message.
     *
     * The message is logged as an INFO message.
     *
     * # Arguments
     * - `s`: The message to log.
     *
     * # Example
     * ```rust
        use forestry::prelude::*;
        let mut l = Logger::new();
        l.i("info");            // Output: [0000:*] info
     * ```
     */
    pub fn i(&mut self, s: &str) {
        let count = format!("{:0>4x}", self.0);
        println!(
            "[{}:{}] {}",
            count.blue().bold(),
            "*".blue().bold(),
            s.blue());
        self.0 = self.0.wrapping_add(1);
        if self.0 == 0 {
            self.w("Log index overflowed; log index may be inaccurate.");
        }
    }

    /**
     * Log a message.
     * 
     * The message is logged as a WARN message.
     *
     * # Arguments
     * - `s`: The message to log.
     *
     * # Example
     * ```rust
        use forestry::prelude::*;
        let mut l = Logger::new();
        l.w("warn");            // Output: [0000:!] warn
     * ```
     */
    pub fn w(&mut self, s: &str) {
        let count = format!("{:0>4x}", self.0);
        println!(
            "[{}:{}] {}",
            count.yellow().bold(),
            "!".yellow().bold(),
            s.yellow().bold());
        self.0 = self.0.wrapping_add(1);
        if self.0 == 0 {
            self.w("Log index overflowed; log index may be inaccurate.");
        }
    }

    /**
     * Log a message.
     * 
     * The message is logged as an ERROR message.
     *
     * # Arguments
     * - `s`: The message to log.
     *
     * # Example
     * ```rust
        use forestry::prelude::*;
        let mut l = Logger::new();
        l.e("error");           // Output: [0000:✘] error
     * ```
     */
    pub fn e(&mut self, s: &str) {
        let count = format!("{:0>4x}", self.0);
        println!(
            "[{}:{}] {}",
            count.red().bold(),
            "✘".red().bold(),
            s.red().bold());
        self.0 = self.0.wrapping_add(1);
        if self.0 == 0 {
            self.w("Log index overflowed; log index may be inaccurate.");
        }
    }

    /**
     * Log a message.
     * 
     * The message is logged as a SUCCESS message.
     *
     * # Arguments
     * - `s`: The message to log.
     *
     * # Example
     * ```rust
        use forestry::prelude::*;
        let mut l = Logger::new();
        l.s("success");         // Output: [0000:✔] success
     * ```
     */
    pub fn s(&mut self, s: &str) {
        let count = format!("{:0>4x}", self.0);
        println!(
            "[{}:{}] {}",
            count.green().bold(),
            "✔".green().bold(),
            s.green());
        self.0 = self.0.wrapping_add(1);
        if self.0 == 0 {
            self.w("Log index overflowed; log index may be inaccurate.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logger_prints() {
        let mut l = Logger::new();
        l.i("info");
        l.w("warning");
        l.e("error");
        l.s("success");
    }
}
