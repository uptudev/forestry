use tokio::{fs::File, io::{self, AsyncWriteExt}};
use super::*;

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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    pub async fn debug(&mut self, s: &str) -> &mut Self {
        self.print(LogLevel::Debug, s).await;
        self
    }

    async fn print(&mut self, lvl: LogLevel, string: &str) -> &mut Self {
        let mut s: String = self.fmt_header(lvl);
        s.push_str(&self.fmt_string(lvl, string));
        s.push('\n');

        if self.flags & 0b00100000 == 0 {
            io::stderr().write_all(s.as_bytes()).await.unwrap();
        }

        if self.flags & 0b00010000 != 0 {
            let temp = self.flags & 0b00001100;
            self.flags |= 0b00001100;
            // invoke buffered print here while formatting is temporarily plain
            let mut plain = ansi_strip(&s);
            if let Some(inner) = &mut self.file {
                inner
                    .write(plain.as_bytes())
                    .await
                    .unwrap();
                inner.flush().await.unwrap();

            } else {
                eprintln!("File output enabled without file specified.");
            }
            self.flags &= 0b11110011;
            self.flags |= temp;
        }

        if self.flags & 0b00000001 == 0 {
            self.index = self.index.wrapping_add(1);
            if self.index == 0 {
                eprintln!("Log index overflowed; log index may be inaccurate.");
            }
        }
        self
    }
}
