extern crate getopts;
extern crate term;
use std::io::prelude::*;

use getopts::Options;
use std::env;

// static USAGE: &'static str = "Usage: t new project_name";

fn print_usage(program: &str, opts: Options) {
    let mut term = term::stdout().unwrap();
    let brief = format!("Usage: {} FILE [options]", program);

    term.fg(term::color::GREEN).unwrap();
    write!(term, "{}", brief).unwrap();
    term.reset().unwrap();
}

fn main() {
    // let mut t = term::stdout().unwrap();
    //
    // t.fg(term::color::GREEN).unwrap();
    // write!(t, "hello, ").unwrap();
    //
    // t.fg(term::color::RED).unwrap();
    // writeln!(t, "world!").unwrap();

    // t.reset().unwrap();

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("n", "new", "set project name", "NAME");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let output = matches.opt_str("n");
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

}
