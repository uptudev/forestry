use std::io::Write;

use colored::*;

/**
    A simple logger for an application.
    
    The logger is used to log messages to the console.
    The messages are coloured based on their severity level.
    Logs are output with a unique 16-bit log index.
    Logger also contains an 8-bit options value set by `cfg()`.
 */
pub struct Logger {
    index: u16,
    flags: u8,
    file: Option<std::io::BufWriter<std::fs::File>>,
    timer: Option<std::time::Instant>,
}

impl Logger {
    /**
        Create a new logger.
      
        The logger is initialised with a log index of 0.
     */
    pub fn new() -> Self {
        Logger {
            index: 0,
            flags: 0,
            file: None,
            timer: None,
        }
    }

    /**
        Configure the logger with options.
        
        See [Options] for more details.
        
        # Arguments
        - `opts`: an array of [Options]
     */
    pub fn cfg(&mut self, opts: &[Options]) -> &mut Self {
        for &e in opts {
            match e {
                Options::NoIndex =>   self.flags |= 0b00000001,
                Options::NoSymbol =>  self.flags |= 0b00000010,
                Options::NoColor =>   self.flags |= 0b00000100,
                Options::NoBold =>    self.flags |= 0b00001000,
                Options::Plain =>     self.flags |= 0b00001100,
                Options::Basic =>     self.flags |= 0b00001111,
                Options::File => {
                    self.flags |= 0b00010000;
                    self.file = Some(
                        std::io::BufWriter::new(
                        std::fs::File::create("forestry.log").unwrap())
                    );
                },
                Options::FileAt(f) => {
                    self.flags |= 0b00010000;
                    self.file = Some(
                        std::io::BufWriter::new(
                        f.try_clone().unwrap())
                    );
                },
                Options::FileOnly =>  self.flags |= 0b00100000,
                Options::Timer => {
                    self.flags |= 0b01000000;
                    self.timer = Some(std::time::Instant::now());
                },
                Options::TimerAt(t) => {
                    self.flags |= 0b01000000;
                    self.timer = Some(*t);
                },
                Options::Reset =>     self.flags &= 0b00000000,
            }
        }
        self
    }

}

#[cfg(not(feature = "async"))]
impl Logger {
    fn fmt_header(&self, lvl: LogLevel) -> String {
        // If neither part of the header is desired, return a blank string.
        if self.flags & 0b0011 == 0b0011 {
            return "".to_string();
        }
        let mut cnt: ColoredString = "".into();
        let mut sym: ColoredString = "".into();
        let mut tim: ColoredString = "".into();

        if self.flags & 0b0001 == 0 {
            cnt = format!("{:0>4x}", self.index).into();
        }
        if self.flags & 0b0010 == 0 {
            sym = match lvl {
                LogLevel::Info => "*".into(),
                LogLevel::Warn => "~".into(),
                LogLevel::Error => "!".into(),
                LogLevel::Success => "+".into(),
                LogLevel::Critical => "%".into(),
            };
        }
        if self.flags & 0b01000000 != 0 {
            let micros = self.timer.unwrap().elapsed().as_micros();
            let msecs = micros as f64 / 1_000.0;
            tim = format!("{:.3}ms", msecs).into();
        }
        if self.flags & 0b0100 == 0 {
            match lvl {
                LogLevel::Info => {
                    cnt = cnt.blue();
                    sym = sym.blue();
                    tim = tim.blue();
                },
                LogLevel::Warn => {
                    cnt = cnt.yellow();
                    sym = sym.yellow();
                    tim = tim.yellow();
                },
                LogLevel::Error => {
                    cnt = cnt.red();
                    sym = sym.red();
                    tim = tim.red();
                },
                LogLevel::Success => {
                    cnt = cnt.green();
                    sym = sym.green();
                    tim = tim.green();
                },
                LogLevel::Critical => {
                    cnt = cnt.on_red().white();
                    sym = sym.on_red().white();
                    tim = tim.on_red().white();
                },
            }
        }
        if self.flags & 0b1000 == 0 {
            cnt = cnt.bold();
            sym = sym.bold();
            tim = tim.bold();
        }
        #[allow(unused_assignments)]
        let mut res = "".to_string();
        if self.flags & 0b0011 == 0 {
            res = format!("[{}:{}]", cnt, sym);
        } else if self.flags & 0b0001 == 0 {
            res = format!("[{}]", cnt);
        } else {
            res = format!("[{}]", sym);
        }
        if self.flags & 0b01000000 != 0 {
            res = format!("{}({})", res, tim);
        }
        res.push_str(" ");
        res
    }

    fn fmt_string(&self, lvl: LogLevel, s: &str) -> String {
        let mut fmt: ColoredString = s.into();
        if self.flags & 0b0100 == 0 {
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
        if self.flags & 0b1000 == 0 {
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
        if self.flags & 0b00100000 == 0 {
            eprintln!("{}{}", self.fmt_header(lvl), self.fmt_string(lvl, string));
        }

        if self.flags & 0b00010000 != 0 {
            let temp = self.flags & 0b00001100;
            self.flags |= 0b00001100;
            // invoke buffered print here while formatting is temporarily plain
            let plain = format!("{}{}\n", self.fmt_header(lvl), self.fmt_string(lvl, string));
            if self.file.is_none() {
                self.warn("File output enabled without file specified.");
            } else {
                self.file
                    .as_mut()
                    .unwrap()
                    .write(plain.as_bytes())
                    .unwrap();
            }
            self.flags &= 0b11110011;
            self.flags |= temp;
        }

        self.index = self.index.wrapping_add(1);
        if self.index == 0 {
            self.warn("Log index overflowed; log index may be inaccurate.");
        }
        self
    }
}

#[cfg(feature = "async")]
impl Logger {
    async fn fmt_header(&self, lvl: LogLevel) -> String {
        // If neither part of the header is desired, return a blank string.
        if self.flags & 0b0011 == 0b0011 {
            return "".to_string();
        }
        let mut cnt: ColoredString = "".into();
        let mut sym: ColoredString = "".into();
        let mut tim: ColoredString = "".into();

        if self.flags & 0b0001 == 0 {
            cnt = format!("{:0>4x}", self.index).into();
        }
        if self.flags & 0b0010 == 0 {
            sym = match lvl {
                LogLevel::Info => "*".into(),
                LogLevel::Warn => "~".into(),
                LogLevel::Error => "!".into(),
                LogLevel::Success => "+".into(),
                LogLevel::Critical => "%".into(),
            };
        }
        if self.flags & 0b01000000 != 0 {
            let micros = self.timer.unwrap().elapsed().as_micros();
            let msecs = micros as f64 / 1_000.0;
            tim = format!("{:.3}ms", msecs).into();
        }
        if self.flags & 0b0100 == 0 {
            match lvl {
                LogLevel::Info => {
                    cnt = cnt.blue();
                    sym = sym.blue();
                    tim = tim.blue();
                },
                LogLevel::Warn => {
                    cnt = cnt.yellow();
                    sym = sym.yellow();
                    tim = tim.yellow();
                },
                LogLevel::Error => {
                    cnt = cnt.red();
                    sym = sym.red();
                    tim = tim.red();
                },
                LogLevel::Success => {
                    cnt = cnt.green();
                    sym = sym.green();
                    tim = tim.green();
                },
                LogLevel::Critical => {
                    cnt = cnt.on_red().white();
                    sym = sym.on_red().white();
                    tim = tim.on_red().white();
                },
            }
        }
        if self.flags & 0b1000 == 0 {
            cnt = cnt.bold();
            sym = sym.bold();
            tim = tim.bold();
        }
        #[allow(unused_assignments)]
        let mut res = "".to_string();
        if self.flags & 0b0011 == 0 {
            res = format!("[{}:{}]", cnt, sym);
        } else if self.flags & 0b0001 == 0 {
            res = format!("[{}]", cnt);
        } else {
            res = format!("[{}]", sym);
        }
        if self.flags & 0b01000000 != 0 {
            res = format!("{}({})", res, tim);
        }
        res.push_str(" ");
        res
    }

    async fn fmt_string(&self, lvl: LogLevel, s: &str) -> String {
        let mut fmt: ColoredString = s.into();
        if self.flags & 0b0100 == 0 {
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
        if self.flags & 0b1000 == 0 {
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
    pub async fn info(&mut self, s: &str) -> &mut Self {
        self.print(LogLevel::Info, s).await;
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
    pub async fn warn(&mut self, s: &str) -> &mut Self {
        self.print(LogLevel::Warn, s).await;
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
    pub async fn error(&mut self, s: &str) -> &mut Self {
        self.print(LogLevel::Error, s).await;
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
    pub async fn success(&mut self, s: &str) -> &mut Self {
        self.print(LogLevel::Success, s).await;
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
    pub async fn critical(&mut self, s: &str) -> &mut Self {
        self.print(LogLevel::Critical, s).await;
        self
    }

    async fn print(&mut self, lvl: LogLevel, string: &str) -> &mut Self {
        if self.flags & 0b00100000 == 0 {
            eprintln!("{}{}", self.fmt_header(lvl).await, self.fmt_string(lvl, string).await);
        }

        if self.flags & 0b00010000 != 0 {
            let temp = self.flags & 0b00001100;
            self.flags |= 0b00001100;
            // invoke buffered print here while formatting is temporarily plain
            let plain = format!("{}{}\n", self.fmt_header(lvl).await, self.fmt_string(lvl, string).await);
            if self.file.is_none() {
                eprintln!("File output enabled without file specified.");
            } else {
                self.file
                    .as_mut()
                    .unwrap()
                    .write(plain.as_bytes())
                    .unwrap();
            }
            self.flags &= 0b11110011;
            self.flags |= temp;
        }

        self.index = self.index.wrapping_add(1);
        if self.index == 0 {
            eprintln!("Log index overflowed; log index may be inaccurate.");
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
    - `File`: Logs to the default file (`forestry.log`).
    - `FileAt(&'a std::fs::File)`: Logs to a specified file.
    - `FileOnly`: Only logs to the file; requires `File` or `FileAt`.
    - `Time`: Include a timestamp in the log.
    - `TimerAt (&'a std::time::Instant)`: Attach an existing timestamp to the log (to allow the use of a runtime timer within one's own program as the timer).
    - `Reset`: Resets the logger's formatter to default settings.
 */
#[derive(Copy, Clone)]
pub enum Options <'a> {
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
    /// Logs to the default file
    File,
    /// Logs to a specified file
    FileAt(&'a std::fs::File),
    /// Only logs to the file; requires `File` or `FileAt`.
    FileOnly,
    /// Include a timestamp in the log.
    Timer,
    /// Attach an existing timestamp to the log (to allow the use of a runtime timer within one's own program as the timer).
    TimerAt(&'a std::time::Instant),
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
