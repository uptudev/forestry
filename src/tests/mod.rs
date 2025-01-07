use crate::prelude::*;
type Res = Result<(), Box<dyn std::error::Error>>;

#[allow(unused_imports)]
use Options::*;

#[test]
#[cfg(not(feature = "async"))]
fn logger_prints() {
    println!();
    let mut l = Logger::new();
    l
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical") 
        .debug("debug");
    }

#[test]
#[cfg(not(feature = "async"))]
fn logger_no_header() -> Res {
    println!();
    let mut l = Logger::new();
    l.cfg(&[NoIndex, NoSymbol])?
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
    Ok(())
}

#[test]
#[cfg(not(feature = "async"))]
fn logger_plain() -> Res {
    println!();
    let mut l = Logger::new();
    l.cfg(&[Plain]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
    Ok(())
}

#[test]
#[cfg(not(feature = "async"))]
fn logger_basic() -> Res {
    println!();
    let mut l = Logger::new();
    l.cfg(&[Basic]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
    Ok(())
}

#[test]
#[cfg(not(feature = "async"))]
fn logger_no_index() -> Res {
    println!();
    let mut l = Logger::new();
    l.cfg(&[NoIndex]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
    Ok(())
}

#[test]
#[cfg(not(feature = "async"))]
fn logger_no_symbol() -> Res {
    println!();
    let mut l = Logger::new();
    l.cfg(&[NoSymbol]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
    Ok(())
}

#[test]
#[cfg(not(feature = "async"))]
fn logger_ns_nc_nb() -> Res {
    println!();
    let mut l = Logger::new();
    l.cfg(&[NoSymbol, NoColor, NoBold]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
    Ok(())
}

#[test]
#[cfg(not(feature = "async"))]
fn logger_no_bold() -> Res {
    println!();
    let mut l = Logger::new();
    l.cfg(&[NoBold]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
    Ok(())
}

#[test]
#[cfg(not(feature = "async"))]
fn logger_file_io() -> Res {
    println!();
    let mut l = Logger::new();
    l.cfg(&[File, FileOnly]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
    Ok(())
}

#[test]
#[cfg(not(feature = "async"))]
fn logger_file_at() -> Res {
    println!();
    let mut l = Logger::new();
    l.cfg(&[FileAt(&std::fs::File::create("fileat.log").unwrap()), FileOnly]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
    Ok(())
}

#[test]
#[cfg(not(feature = "async"))]
fn logger_timer() -> Res {
    println!();
    let mut l = Logger::new();
    l.cfg(&[Timer]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
    Ok(())
}

#[test]
#[cfg(not(feature = "async"))]
fn logger_timer_at() -> Res {
    println!();
    let mut l = Logger::new();
    l.cfg(&[TimerAt(&std::time::Instant::now())]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
    Ok(())
}

#[tokio::test]
#[cfg(feature = "async")]
async fn logger_async() {
    println!();
    let mut l = Logger::new();
    l
        .info("info").await
        .warn("warning").await
        .error("error").await
        .success("success").await
        .critical("critical").await
        .debug("debug").await;
}

#[test]
#[cfg(not(feature = "async"))]
#[cfg(feature = "static")]
fn static_log() {
    println!();
    info("info");
    warn("warn");
    error("error");
    success("success");
    critical("critical");
    debug("debug");
}

#[test]
#[cfg(not(feature = "async"))]
#[cfg(feature = "static")]
fn static_cfg() -> Res {
    println!();
    cfg(&[Timer])?;
    info("info");
    warn("warn");
    error("error");
    success("success");
    critical("critical");
    debug("debug");
    Ok(())
}

#[tokio::test]
#[cfg(feature = "static")]
#[cfg(feature = "async")]
async fn static_async() {
    println!();
    info("info").await;
    warn("warn").await;
    error("error").await;
    success("success").await;
    critical("critical").await;
    debug("debug").await;
}

#[tokio::test]
#[cfg(feature = "static")]
#[cfg(feature = "async")]
async fn static_async_two() {
    println!();
    info("info").await;
    warn("warn").await;
    error("error").await;
    success("success").await;
    critical("critical").await;
    debug("debug").await;
}
