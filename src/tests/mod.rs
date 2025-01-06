use crate::prelude::*;

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
fn logger_no_header() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[NoIndex, NoSymbol]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
}

#[test]
#[cfg(not(feature = "async"))]
fn logger_plain() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[Plain]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
}

#[test]
#[cfg(not(feature = "async"))]
fn logger_basic() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[Basic]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
}

#[test]
#[cfg(not(feature = "async"))]
fn logger_no_index() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[NoIndex]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
}

#[test]
#[cfg(not(feature = "async"))]
fn logger_no_symbol() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[NoSymbol]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
}

#[test]
#[cfg(not(feature = "async"))]
fn logger_ns_nc_nb() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[NoSymbol, NoColor, NoBold]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
}

#[test]
#[cfg(not(feature = "async"))]
fn logger_no_bold() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[NoBold]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
}

#[test]
#[cfg(not(feature = "async"))]
fn logger_file_io() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[File, FileOnly]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
}

#[test]
#[cfg(not(feature = "async"))]
fn logger_file_at() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[FileAt(&std::fs::File::create("fileat.log").unwrap()), FileOnly]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
}

#[test]
#[cfg(not(feature = "async"))]
fn logger_timer() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[Timer]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
}

#[test]
#[cfg(not(feature = "async"))]
fn logger_timer_at() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[TimerAt(&std::time::Instant::now())]).unwrap()
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical")
        .debug("debug");
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
fn static_cfg() {
    println!();
    cfg(&[Timer]);
    info("info");
    warn("warn");
    error("error");
    success("success");
    critical("critical");
    debug("debug");
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
