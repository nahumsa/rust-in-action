use std::io::{self, prelude::*};
use std::{fs::File, io::BufReader};

use clap::{App, Arg};
use regex::Regex;

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for (i, line_) in reader.lines().enumerate() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}: {}", i, line),
            None => (),
        };
    }
}

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let search_term = args.value_of("pattern").unwrap();
    let re = Regex::new(search_term).unwrap();
    match args.value_of("input") {
        Some(v) => {
            let f = File::open(v).unwrap();
            let reader = BufReader::new(f);
            process_lines(reader, re);
        }
        None => {
            let reader = io::stdin().lock();
            process_lines(reader, re);
        }
    };
}
