use super::*;
type Res = Result<(), Box<dyn std::error::Error>>;
static LOG: std::sync::Mutex<logs::Logger> = std::sync::Mutex::new(logs::Logger::new());

#[cfg(not(feature = "async"))]
pub use sync::*;
#[cfg(feature = "async")]
mod r#async;
#[cfg(feature = "async")]
pub use r#async::*;

#[cfg(not(feature = "async"))]
mod sync {
    use super::*;
    /**
      Configure the logger with options.

      See [Options] for more details.

      # Arguments
      - `opts`: an array of [Options]
      */
    pub fn cfg(c: &[logs::Options]) -> Res {
        LOG.lock().unwrap().cfg(c)?;
        Ok(())
    }

    /**
      Log a message.

      The message is logged as an INFO message.

      # Arguments
      - `s`: The message to log.

      # Example
      ```rust
      use forestry::prelude::*;
      info("info");        // Output: [0000:*] info
      ```
      */
    pub fn info(s: &str) {
        LOG.lock().unwrap().info(s);
    }

    /**
      Log a message.

      The message is logged as a warn message.

      # Arguments
      - `s`: The message to log.

      # Example
      ```rust
      use forestry::prelude::*;
      warn("warn");        // Output: [0000:~] warn
      ```
      */
    pub fn warn(s: &str) {
        LOG.lock().unwrap().warn(s);
    }

    /**
      Log a message.

      The message is logged as an ERROR message.

      # Arguments
      - `s`: The message to log.

      # Example
      ```rust
      use forestry::prelude::*;
      error("error");        // Output: [0000:!] error
      ```
      */
    pub fn error(s: &str) {
        LOG.lock().unwrap().error(s);
    }

    /**
      Log a message.

      The message is logged as a SUCCESS message.

      # Arguments
      - `s`: The message to log.

      # Example
      ```rust
      use forestry::prelude::*;
      success("success");        // Output: [0000:+] success
      ```
      */
    pub fn success(s: &str) {
        LOG.lock().unwrap().success(s);
    }

    /**
      Log a message.

      The message is logged as a DEBUG message.

      # Arguments
      - `s`: The message to log.

      # Example
      ```rust
      use forestry::prelude::*;
      debug("debug");        // Output: [0000:?] debug
      ```
      */
    pub fn debug(s: &str) {
        LOG.lock().unwrap().debug(s);
    }

    /**
      Log a message.

      The message is logged as a CRITICAL message.

      # Arguments
      - `s`: The message to log.

      # Example
      ```rust
      use forestry::prelude::*;
      critical("critical");        // Output: [0000:%] critical
      ```
      */
    pub fn critical(s: &str) {
        LOG.lock().unwrap().critical(s);
    }
}
