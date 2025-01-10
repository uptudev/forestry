#[cfg(not(feature = "async"))]
use std::{fs::File, io::{self, Write}};

#[cfg(feature = "async")]
mod r#async;
#[cfg(feature = "async")]
pub use r#async::*;
#[cfg(feature = "async")]
use tokio::{fs::File, io::{self, AsyncWriteExt}};

use colored::*;

/**
    A simple logger for an application.
    
    The logger is used to log messages to the console.
    The messages are coloured based on their severity level.
    Logs are output with a unique 16-bit log index.
    Logger also contains an 8-bit options value set by `cfg()`.
 */
pub struct Logger {
    pub index: u16,
    pub flags: u8,
    pub file: Option<io::BufWriter<File>>,
    pub timer: Option<std::time::Instant>,
}

impl Logger {
    /**
        Create a new logger.
      
        The logger is initialised with a log index of 0.
     */
    pub const fn new() -> Self {
        Logger {
            index: 0,
            flags: 0,
            file: None,
            timer: None,
        }
    }

    fn fmt_header(&self, lvl: LogLevel) -> String {
        // If no part of the header is desired, return a blank string.
        if self.flags & 0b01000011 == 0b00000011 {
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
                LogLevel::Debug => "?".into(),
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
                LogLevel::Debug => {
                    cnt = cnt.magenta();
                    sym = sym.magenta();
                    tim = tim.magenta();
                }
            }
        }
        if self.flags & 0b1000 == 0 {
            cnt = cnt.bold();
            sym = sym.bold();
            tim = tim.bold();
        }
        #[allow(unused_assignments)]
        let mut res = String::from("");
        if self.flags & 0b0011 == 0 {
            let cnt: String = cnt.to_string();
            let sym: String = sym.to_string();
            res.push('[');
            res.push_str(&cnt);
            res.push(':');
            res.push_str(&sym);
            res.push(']');
        } else if self.flags & 0b0001 == 0 {
            let cnt: String = cnt.to_string();
            res.push('[');
            res.push_str(&cnt);
            res.push(']');
        } else {
            let sym: String = sym.to_string();
            res.push('[');
            res.push_str(&sym);
            res.push(']');
        }
        if self.flags & 0b01000000 != 0 {
            let tim: String = tim.to_string();
            res.push('(');
            res.push_str(&tim);
            res.push(')');
        }
        res.push(' ');
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
                LogLevel::Debug => {
                    fmt = fmt.magenta()
                }
            }
        }
        if self.flags & 0b1000 == 0 {
            match lvl {
                LogLevel::Error => {
                    fmt = fmt.bold()
                },
                LogLevel::Success => {
                    fmt = fmt.bold()
                },
                LogLevel::Critical => {
                    fmt = fmt.bold()
                },
                _ => {},
            }
        }
        fmt.to_string()
    }
}

#[cfg(not(feature = "async"))]
impl Logger {
    /**
        Configure the logger with options.
        
        See [Options] for more details.
        
        # Arguments
        - `opts`: an array of [Options]
     */
    pub fn cfg(&mut self, opts: &[Options]) -> Result<&mut Self, io::Error> {
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
                        io::BufWriter::new(
                        File::create("forestry.log")?)
                    );
                },
                Options::FileAt(f) => {
                    self.flags |= 0b00010000;
                    self.file = Some(
                        io::BufWriter::new(
                        f.try_clone()?)
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
        Ok(self)
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    pub fn success(&mut self, s: &str) -> &mut Self {
        self.print(LogLevel::Success, s)
    }

    /**
        Log a message.

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
    #[inline(always)]
    pub fn critical(&mut self, s: &str) -> &mut Self {
        self.print(LogLevel::Critical, s)
    }

    /**
        Log a message.

        The mesage is logged as a DEBUG message.

        # Arguments
        - `s`: The message to log.

        # Example
        ```rust
         use forestry::prelude::*;
         let mut log = Logger::new();
         log.debug("debug");              // Output: [0000:?] debug
        ```
    */
    #[inline(always)]
    pub fn debug(&mut self, s: &str) -> &mut Self {
        self.print(LogLevel::Debug, s);
        self
    }

    fn print(&mut self, lvl: LogLevel, string: &str) -> &mut Self {
        let mut s: String = self.fmt_header(lvl);
        s.push_str(&self.fmt_string(lvl, string));
        s.push('\n');
        if self.flags & 0b00100000 == 0 {
            io::stderr().write_all(s.as_bytes()).unwrap();
        }

        if self.flags & 0b00010000 != 0 {
            let temp = self.flags & 0b00001100;
            self.flags |= 0b00001100;
            let plain = ansi_strip(&s);
            if let Some(inner) = &mut self.file {
                inner
                    .write(plain.as_bytes())
                    .unwrap();
                inner.flush().unwrap();
            } else {
                self.warn("File output enabled without file specified.");
            }
            self.flags &= 0b11110011;
            self.flags |= temp;
        }

        if self.flags & 0b00000001 == 0 {
            self.index = self.index.wrapping_add(1);
            if self.index == 0 {
                self.warn("Log index overflowed; log index may be inaccurate.");
            }
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
    FileAt(&'a File),
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
    Debug,
}

fn ansi_strip(s: &str) -> String {
    if !s.contains("\x1b[") { return s.to_owned() }
    let mut chars = s.chars();
    let mut buf = String::new();
    while let Some(c) = chars.next() {
        match c {
            '\x1b' => {
                // consume up to and including the next 'm'
                while let Some(ch) = chars.next() {
                    if ch == 'm' {break;}
                }
            }
            _ => buf.push(c),
        }
    }
    buf
}
