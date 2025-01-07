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
        LOG.lock()?.cfg(c)?;
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    pub fn critical(s: &str) {
        LOG.lock().unwrap().critical(s);
    }
}
