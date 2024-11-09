use super::prelude::*;
use FormatOptions::*;

#[test]
fn logger_prints() {
    println!();
    let mut l = Logger::new();
    l
        .i("info")
        .w("warning")
        .e("error")
        .s("success")
        .c("critical");
    }

#[test]
fn logger_no_header() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[NoIndex, NoSymbol])
        .i("info")
        .w("warning")
        .e("error")
        .s("success")
        .c("critical");
}

#[test]
fn logger_plain() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[Plain])
        .i("info")
        .w("warning")
        .e("error")
        .s("success")
        .c("critical");
}

#[test]
fn logger_basic() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[Basic])
        .i("info")
        .w("warning")
        .e("error")
        .s("success")
        .c("critical");
}

#[test]
fn logger_no_index() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[NoIndex])
        .i("info")
        .w("warning")
        .e("error")
        .s("success")
        .c("critical");
}

#[test]
fn logger_no_symbol() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[NoSymbol])
        .i("info")
        .w("warning")
        .e("error")
        .s("success")
        .c("critical");
}

#[test]
fn logger_ns_nc_nb() {
    println!();
    let mut l = Logger::new();
    l.cfg(&[NoSymbol, NoColor, NoBold])
        .i("info")
        .w("warning")
        .e("error")
        .s("success")
        .c("critical");
}
