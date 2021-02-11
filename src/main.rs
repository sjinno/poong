use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}q to exit. Type stuff, use alt, and so on.{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();

    let mut row = 1;
    let mut col = 1;

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(row, col),
            // termion::clear::CurrentLine
            termion::clear::All,
        )
        .unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => println!("ESC"),
            Key::Left => {
                if row > 1 {
                    row -= 1;
                }
                println!("←")
            }
            Key::Right => {
                row += 1;
                println!("→")
            }
            Key::Up => {
                if col > 1 {
                    col -= 1;
                }
                println!("↑")
            }
            Key::Down => {
                col += 1;
                println!("↓")
            }
            Key::Backspace => println!("×"),
            _ => {}
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
