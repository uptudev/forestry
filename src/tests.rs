use super::prelude::*;

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
        .critical("critical");
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
        .critical("critical");
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
        .critical("critical");
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
        .critical("critical");
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
        .critical("critical");
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
        .critical("critical");
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
        .critical("critical");
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
        .critical("critical");
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
        .critical("critical");
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
        .critical("critical");
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
        .critical("critical");
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
        .critical("critical");
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
        .critical("critical").await;
}
