use tokio::{fs::File, io::{self, AsyncWriteExt}};
use super::*;

#[cfg(feature = "async")]
impl Logger {
    /**
      Configure the logger with options.

      See [Options] for more details.

      # Arguments
      - `opts`: an array of [Options]
      */
    pub async fn cfg(&mut self, opts: &[Options<'_>]) -> Result<&mut Self, io::Error> {
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
                            File::create("forestry.log").await?)
                    );
                },
                Options::FileAt(f) => {
                    self.flags |= 0b00010000;
                    self.file = Some(
                        io::BufWriter::new(
                            f.try_clone().await?)
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
    pub async fn critical(&mut self, s: &str) -> &mut Self {
        self.print(LogLevel::Critical, s).await;
        self
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
    pub async fn debug(&mut self, s: &str) -> &mut Self {
        self.print(LogLevel::Debug, s).await;
        self
    }

    async fn print(&mut self, lvl: LogLevel, string: &str) -> &mut Self {
        if self.flags & 0b00100000 == 0 {
            let mut s: String = self.fmt_header(lvl);
            s.push_str(&self.fmt_string(lvl, string));
            s.push('\n');
            io::stderr().write_all(s.as_bytes()).await.unwrap();
        }

        if self.flags & 0b00010000 != 0 {
            let temp = self.flags & 0b00001100;
            self.flags |= 0b00001100;
            // invoke buffered print here while formatting is temporarily plain
            let mut plain = String::from("");
            plain.push_str(&self.fmt_header(lvl)); 
            plain.push_str(&self.fmt_string(lvl, string));
            plain.push('\n');
            if self.file.is_none() {
                eprintln!("File output enabled without file specified.");
            } else {
                self.file
                    .as_mut()
                    .unwrap()
                    .write(plain.as_bytes())
                    .await
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
