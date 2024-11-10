use std::io::Write;

use colored::*;

/**
    A simple logger for an application.
    
    The logger is used to log messages to the console.
    The messages are coloured based on their severity level.
    Logs are output with a unique 16-bit log index.
    Logger also contains an 8-bit options value set by `cfg()`.
 */
pub struct Logger(u16, u8, Option<std::io::BufWriter<std::fs::File>>);

impl Logger {
    /**
        Create a new logger.
      
        The logger is initialised with a log index of 0.
     */
    pub fn new() -> Self {
        Logger(0, 0, None)
    }

    /**
        Configure the logger with options.
        
        See `crate::logs::FormatOptions` for more details.
        
        # Arguments
        - `opts`: an array of FormatOptions
     */
    pub fn cfg(&mut self, opts: &[Options]) -> &mut Self {
        for &e in opts {
            match e {
                Options::NoIndex =>   self.1 |= 0b00000001,
                Options::NoSymbol =>  self.1 |= 0b00000010,
                Options::NoColor =>   self.1 |= 0b00000100,
                Options::NoBold =>    self.1 |= 0b00001000,
                Options::Plain =>     self.1 |= 0b00001100,
                Options::Basic =>     self.1 |= 0b00001111,
                Options::File => {
                    self.1 |= 0b00010000;
                    self.2 = Some(
                        std::io::BufWriter::new(
                        std::fs::File::create("forestry.log").unwrap())
                    );
                },
                Options::FileAt(f) => {
                    self.1 |= 0b00010000;
                    self.2 = Some(
                        std::io::BufWriter::new(
                        f.try_clone().unwrap())
                    );
                },
                Options::FileOnly =>  self.1 |= 0b00100000,
                Options::Reset =>     self.1 &= 0b00000000,
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
        self.print(LogLevel::Info, s)
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
        self.print(LogLevel::Warn, s)
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
        self.print(LogLevel::Error, s)
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
        self.print(LogLevel::Success, s)
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
        self.print(LogLevel::Critical, s)
    }

    fn print(&mut self, lvl: LogLevel, string: &str) -> &mut Self {
        if self.1 & 0b00100000 == 0 {
            eprintln!("{}{}", self.fmt_header(lvl), self.fmt_string(lvl, string));
        }

        if self.1 & 0b00010000 != 0 {
            let temp = self.1 & 0b00001100;
            self.1 |= 0b00001100;
            // invoke buffered print here while formatting is temporarily plain
            let plain = format!("{}{}\n", self.fmt_header(lvl), self.fmt_string(lvl, string));
            if self.2.is_none() {
                self.warn("File output enabled without file specified.");
            } else {
                self.2
                    .as_mut()
                    .unwrap()
                    .write(plain.as_bytes())
                    .unwrap();
            }
            self.1 &= 0b11110011;
            self.1 |= temp;
        }

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
    - `Basic`: Turns this into a bare `eprintln!()` call.
    - `Reset`: Resets the logger's formatter to default settings.
 */
#[derive(Copy, Clone)]
pub enum Options <'a> {
    /// Logs to the default file
    File,
    /// Logs to a specified file
    FileAt(&'a std::fs::File),
    /// Only logs to the file; requires `File` or `FileAt`.
    FileOnly,
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
    /// Removes all extras; this is now just `eprintln!()`.
    Basic,
    /// Reset the logger's formatter to its default state.
    Reset,
}

#[derive(Clone, Copy)]
enum LogLevel {
    Info,
    Warn,
    Error,
    Success,
    Critical,
}
