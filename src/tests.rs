use super::prelude::*;
use FormatOptions::*;

#[test]
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

#[test]
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

#[test]
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

#[test]
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

#[test]
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

#[test]
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

#[test]
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
