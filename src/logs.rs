use colored::*;

/**
    A simple logger for an application.
    
    The logger is used to log messages to the console.
    The messages are coloured based on their severity level.
    Logs are output with a unique 16-bit log index.
    Logger also contains an 8-bit options value set by `cfg()`.
 */
pub struct Logger(u16, u8);

impl Logger {
    /**
        Create a new logger.
      
        The logger is initialised with a log index of 0.
     */
    pub fn new() -> Self {
        Logger(0, 0)
    }

    /**
        Configure the logger with options.
        
        See `crate::logs::FormatOptions` for more details.
        
        # Arguments
        - `opts`: an array of FormatOptions
     */
    pub fn cfg(&mut self, opts: &[FormatOptions]) -> &mut Self {
        for &e in opts {
            match e {
                FormatOptions::NoIndex =>   self.1 |= 0b0001,
                FormatOptions::NoSymbol =>  self.1 |= 0b0010,
                FormatOptions::NoColor =>   self.1 |= 0b0100,
                FormatOptions::NoBold =>    self.1 |= 0b1000,
                FormatOptions::Plain =>     self.1 |= 0b1100,
                FormatOptions::Basic =>     self.1 |= 0b1111,
                FormatOptions::Reset =>     self.1 &= 0b0000,
            }
        }
        self
    }

    fn fmt_header(&self, lvl: LogLevel) -> String {
        // If neither part of the header is desired, return a blank string.
        if self.1 & 0b0011 == 0b0011 {
            return "".to_string();
        }
        let mut cnt: ColoredString = "".into();
        let mut sym: ColoredString = "".into();

        if self.1 & 0b0001 == 0 {
            cnt = format!("{:0>4x}", self.0).into();
        }
        if self.1 & 0b0010 == 0 {
            sym = match lvl {
                LogLevel::Info => "*".into(),
                LogLevel::Warn => "~".into(),
                LogLevel::Error => "!".into(),
                LogLevel::Success => "+".into(),
                LogLevel::Critical => "%".into(),
            };
        }
        if self.1 & 0b0100 == 0 {
            match lvl {
                LogLevel::Info => {
                    cnt = cnt.blue();
                    sym = sym.blue();
                },
                LogLevel::Warn => {
                    cnt = cnt.yellow();
                    sym = sym.yellow();
                },
                LogLevel::Error => {
                    cnt = cnt.red();
                    sym = sym.red();
                },
                LogLevel::Success => {
                    cnt = cnt.green();
                    sym = sym.green();
                },
                LogLevel::Critical => {
                    cnt = cnt.on_red().white();
                    sym = sym.on_red().white();
                },
            }
        }
        if self.1 & 0b1000 == 0 {
            cnt = cnt.bold();
            sym = sym.bold();
        }
        if self.1 & 0b0011 == 0 {
            return format!("[{}:{}] ", cnt, sym);
        } else if self.1 & 0b0001 == 0 {
            return format!("[{}] ", cnt);
        } else {
            return format!("[{}] ", sym);
        }
    }

    fn fmt_string(&self, lvl: LogLevel, s: &str) -> String {
        let mut fmt: ColoredString = s.into();
        if self.1 & 0b0100 == 0 {
            match lvl {
                LogLevel::Info => {
                    fmt = fmt.blue()
                },
                LogLevel::Warn => {
                    fmt = fmt.yellow()
                },
                LogLevel::Error => {
                    fmt = fmt.red()
                },
                LogLevel::Success => {
                    fmt = fmt.green()
                },
                LogLevel::Critical => {
                    fmt = fmt.on_red().white()
                },
            }
        }
        if self.1 & 0b1000 == 0 {
            match lvl {
                LogLevel::Info => {},
                LogLevel::Warn => {},
                LogLevel::Error => {
                    fmt = fmt.bold()
                },
                LogLevel::Success => {
                    fmt = fmt.bold()
                },
                LogLevel::Critical => {
                    fmt = fmt.bold()
                },
            }
        }
        fmt.to_string()
    }

    /**
        Log a message.
        
        The message is logged as an INFO message.
        
        # Arguments
        - `s`: The message to log.
        
        # Example
        ```rust
         use forestry::prelude::*;
         let mut log = Logger::new();
         log.info("info");            // Output: [0000:*] info
        ```
     */
    pub fn info(&mut self, s: &str) -> &mut Self {
        let header = self.fmt_header(LogLevel::Info);
        let string = self.fmt_string(LogLevel::Info, s);
        println!("{}{}", header, string);
        self.0 = self.0.wrapping_add(1);
        if self.0 == 0 {
            self.warn("Log index overflowed; log index may be inaccurate.");
        }
        self
    }

    /**
        Log a message.
        
        The message is logged as a WARN message.
        
        # Arguments
        - `s`: The message to log.
        
        # Example
        ```rust
         use forestry::prelude::*;
         let mut log = Logger::new();
         log.warn("warn");            // Output: [0000:~] warn
        ```
     */
    pub fn warn(&mut self, s: &str) -> &mut Self {
        let header = self.fmt_header(LogLevel::Warn);
        let string = self.fmt_string(LogLevel::Warn, s);
        println!("{}{}", header, string);
        self.0 = self.0.wrapping_add(1);
        if self.0 == 0 {
            self.warn("Log index overflowed; log index may be inaccurate.");
        }
        self
    }

    /**
        Log a message.
        
        The message is logged as an ERROR message.
        
        # Arguments
        - `s`: The message to log.
        
        # Example
        ```rust
         use forestry::prelude::*;
         let mut log = Logger::new();
         log.error("error");           // Output: [0000:!] error
        ```
     */
    pub fn error(&mut self, s: &str) -> &mut Self {
        let header = self.fmt_header(LogLevel::Error);
        let string = self.fmt_string(LogLevel::Error, s);
        println!("{}{}", header, string);
        self.0 = self.0.wrapping_add(1);
        if self.0 == 0 {
            self.warn("Log index overflowed; log index may be inaccurate.");
        }
        self
    }

    /**
        Log a message.
        
        The message is logged as a SUCCESS message.
        
        # Arguments
        - `s`: The message to log.
        
        # Example
        ```rust
         use forestry::prelude::*;
         let mut log = Logger::new();
         log.success("success");         // Output: [0000:+] success
        ```
     */
    pub fn success(&mut self, s: &str) -> &mut Self {
        let header = self.fmt_header(LogLevel::Success);
        let string = self.fmt_string(LogLevel::Success, s);
        println!("{}{}", header, string);
        self.0 = self.0.wrapping_add(1);
        if self.0 == 0 {
            self.warn("Log index overflowed; log index may be inaccurate.");
        }
        self
    }

    /**
        Log a message,

        The message is logged as a CRITICAL message.

        # Arguments
        - `s`: The message to log.

        # Example
        ```rust
         use forestry::prelude::*;
         let mut log = Logger::new();
         log.critical("critical");        // Output: [0000:%] critical
        ```
    */
    pub fn critical(&mut self, s: &str) -> &mut Self {
        let header = self.fmt_header(LogLevel::Critical);
        let string = self.fmt_string(LogLevel::Critical, s);
        println!("{}{}", header, string);
        self.0 = self.0.wrapping_add(1);
        if self.0 == 0 {
            self.warn("Log index overflowed; log index may be inaccurate.");
        }
        self
    }
}

/**
    Formatting options for the `Logger`.
    
    - `NoIndex`: Removes the incrementing log index.
    - `NoSymbol`: Removes the log type symbol.
    - `NoColor`: Removes all colour sequences.
    - `NoBold`: Removes all bold sequences.
    - `Plain`: Removes all formatting escape characters.
    - `Basic`: Turns this into a bare `println!()` call.
    - `Reset`: Resets the logger's formatter to default settings.
 */
#[derive(Copy, Clone)]
pub enum FormatOptions {
    /// Removes the incrementing log index.
    NoIndex,
    /// Removes the log type symbol.
    NoSymbol,
    /// Removes all colours.
    NoColor,
    /// Removes bold/highlighting.
    NoBold,
    /// Removes all formatting escape characters.
    Plain,
    /// Removes all extras; this is now just `println!()`.
    Basic,
    /// Reset the logger's formatter to its default state.
    Reset,
}

enum LogLevel {
    Info,
    Warn,
    Error,
    Success,
    Critical,
}
