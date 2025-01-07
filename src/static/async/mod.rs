use super::*;

/**
  Configure the logger with options.

  See [Options] for more details.

  # Arguments
  - `opts`: an array of [Options]
  */
#[inline(always)]
pub async fn cfg(c: &[logs::Options<'_>]) -> Res {
    LOG.lock().unwrap().cfg(c).await?;
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
pub async fn info(s: &str) {
    LOG.lock().unwrap().info(s).await;
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
pub async fn warn(s: &str) {
    LOG.lock().unwrap().warn(s).await;
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
pub async fn error(s: &str) {
    LOG.lock().unwrap().error(s).await;
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
pub async fn success(s: &str) {
    LOG.lock().unwrap().success(s).await;
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
pub async fn debug(s: &str) {
    LOG.lock().unwrap().debug(s).await;
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
pub async fn critical(s: &str) {
    LOG.lock().unwrap().critical(s).await;
}
