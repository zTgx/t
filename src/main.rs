extern crate getopts;
extern crate term;
use std::io::prelude::*;

use getopts::Options;
use std::env;
use templateme::new;

fn print_usage(program: &str, _opts: Options) {
    let mut term = term::stdout().unwrap();
    let brief = format!("Usage: {} --new [project name]", program);

    term.fg(term::color::GREEN).unwrap();
    write!(term, "{}\n", brief).unwrap();
    term.reset().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help menu");
    opts.optflag("n", "new", "new project name");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => {
            m
        },

        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if args.len() != 3 {
        print_usage(&program, opts);
        return;
    }

    let name = &args[2];
    if matches.opt_present("n") {
        new(name);
    } else {
        print_usage(&program, opts);
        return;
    }

}
