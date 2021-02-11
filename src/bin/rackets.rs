use std::fmt;
use std::io::{stdin, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Square {
    Racket,
}

impl fmt::Display for Square {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Square::Racket => fmt.write_str("■■■■■■■■"),
        }
    }
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut row = 1;
    let col = 20;

    write!(
        stdout,
        "{}{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(row, col),
        termion::cursor::Hide,
        Square::Racket
    )
    .unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Left => {
                if row > 1 {
                    row -= 1;
                }
            }
            Key::Right => {
                if row < 50 {
                    row += 1;
                }
            }
            _ => (),
        }

        write!(
            stdout,
            "{}{}{}",
            termion::cursor::Goto(row, col),
            termion::clear::All,
            Square::Racket
        )
        .unwrap();

        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
