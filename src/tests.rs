use super::prelude::*;

#[cfg(feature = "async")]
extern crate futures;

#[allow(unused_imports)]
use Options::*;

#[test]#[cfg(not(feature = "async"))]
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

#[test]#[cfg(not(feature = "async"))]
fn logger_no_header() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[NoIndex, NoSymbol])
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical");
}

#[test]#[cfg(not(feature = "async"))]
fn logger_plain() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[Plain])
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical");
}

#[test]#[cfg(not(feature = "async"))]
fn logger_basic() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[Basic])
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical");
}

#[test]#[cfg(not(feature = "async"))]
fn logger_no_index() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[NoIndex])
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical");
}

#[test]#[cfg(not(feature = "async"))]
fn logger_no_symbol() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[NoSymbol])
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical");
}

#[test]#[cfg(not(feature = "async"))]
fn logger_ns_nc_nb() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[NoSymbol, NoColor, NoBold])
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical");
}

#[test]#[cfg(not(feature = "async"))]
fn logger_no_bold() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[NoBold])
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical");
}

#[test]#[cfg(not(feature = "async"))]
fn logger_file_io() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[File, FileOnly])
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical");
}

#[test]#[cfg(not(feature = "async"))]
fn logger_file_at() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[FileAt(&std::fs::File::create("fileat.log").unwrap()), FileOnly])
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical");
}

#[test]#[cfg(not(feature = "async"))]
fn logger_timer() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[Timer])
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical");
}

#[test]#[cfg(not(feature = "async"))]
fn logger_timer_at() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[TimerAt(&std::time::Instant::now())])
        .info("info")
        .warn("warning")
        .error("error")
        .success("success")
        .critical("critical");
}

#[test]#[cfg(feature = "async")]
fn logger_async() {
    println!();
    let mut l = Logger::new();
    futures::executor::block_on(logger_async_inner(&mut l));
}

#[cfg(feature = "async")]
async fn logger_async_inner(l: &mut Logger) {
    l.info("info").await
        .warn("warning").await
        .error("error").await
        .success("success").await
        .critical("critical").await;
}
