use defs::*;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};

mod defs;
mod interpreter;
mod parser;
mod scanner;
mod token;

use crate::interpreter::interpret;
use crate::parser::{parser_run, Parser};
use defs::ERR_MSGS;

const VER: &str = "0.1.0";

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

fn usage() {
    println!("Usage: pl0r srcfile");
}

fn main() {
    println!("PL0R {VER}: PL/0 in Rust (c) Jari Korhonen, 2023");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
        std::process::exit(EX_USAGE);
    }
    run_file(&args[1]);
}

fn run_file(srcfile: &str) {
    run(srcfile);
    if true == HAD_ERROR.load(Ordering::Relaxed) {
        std::process::exit(EX_DATAERR);
    }
}

fn run(src: &str) {
    let mut parser = Parser::new(src);
    parser = parser_run(parser);
    interpret(parser)
}

fn report(line: i32, _where: &str, message: &str) {
    eprintln!("[line {line} Error {_where} : {message}");
    HAD_ERROR.store(true, Ordering::Relaxed)
}

pub fn scan_error(line: i32, message: &str) {
    report(line, "", message);
}

pub fn parse_error(line: i32, n: usize) {
    report(line, "", ERR_MSGS[n]);
    std::process::exit(EX_DATAERR);
}
