extern crate clap;
extern crate termion;
use clap::{App, Arg};
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{event, screen};

type FileBuffer = Vec<String>;

fn read_file<P>(path: Option<P>) -> io::Result<FileBuffer>
where
    P: AsRef<Path>,
{
    if let Some(path) = path {
        let file = File::open(path)?;
        io::BufReader::new(file).lines().collect()
    } else {
        let file = io::stdin();
        io::BufReader::new(file.lock()).lines().collect()
    }
}

fn write_file(fb: FileBuffer) -> io::Result<()> {
    let tty = termion::get_tty()?;
    let mut screen = screen::AlternateScreen::from(tty.try_clone()?.into_raw_mode()?);

    write!(
        screen,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )?;
    for (i, line) in fb.iter().enumerate() {
        write!(
            screen,
            "{}{}",
            line,
            termion::cursor::Goto(1, (i + 2).try_into().unwrap()),
        )?;
    }
    screen.flush()?;

    for c in tty.keys() {
        match c? {
            event::Key::Char('q') => break,
            _ => {}
        }
        screen.flush()?;
    }
    Ok(())
}

fn main() {
    let matches = App::new("pageq")
        .arg(Arg::with_name("input").help("The input file to read"))
        .get_matches();

    let input = matches.value_of("input");
    println!("Input file: {:?}", input);
    let fb = read_file(input).unwrap();
    println!("Data: {:?}", fb);
    println!("Terminal size: {:?}", termion::terminal_size().unwrap());
    write_file(fb).unwrap();
}
