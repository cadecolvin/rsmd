extern crate termion;
use termion::{clear, cursor};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

use std::fs::File;
use std::io::{self, BufReader};
use std::io::{stdin, stdout};
use std::io::prelude::*;
use std::path::Path;
use std::fmt;

fn main() {
    let p = Path::new("main.rs");
    let l = read_markdown(p);

    for line in l {
        println!("{}", line);
    }

    // Lock the stdios
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let stdout = stdout.into_raw_mode().unwrap();
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut screen = Screen {
        stdin: stdin,
        stdout: stdout,
    };

    screen.init();

}

fn read_markdown(file_path: &Path) -> Vec<String> {
    let mut lines = Vec::new();
    let md_file = File::open(file_path).unwrap();
    let md_file = BufReader::new(md_file);
    
    for line in md_file.lines() {
        lines.push(line.unwrap());
    }

    return lines;
}

fn pretty_print(text: &str) {
}

struct Screen<R, W> {
    stdin: R,
    stdout: W,
}

impl<R: Read, W: Write> Screen<R, W> {
    pub fn init(&mut self) {
        write!(self.stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    }
}
