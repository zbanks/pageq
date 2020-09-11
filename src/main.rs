extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("pageq")
        .arg(Arg::with_name("input").help("The input file to read"))
        .get_matches();

    let input = matches.value_of("input").unwrap_or("/dev/stdin");
    println!("Input file: {}", input);
}
