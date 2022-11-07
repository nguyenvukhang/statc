extern crate termion;

use std::io::{stdin, stdout, Write};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

type RawOut = termion::raw::RawTerminal<std::io::Stdout>;

fn make_cmd(buffer: &Vec<char>) -> Command {
    let mut command = Command::new("statc");
    let mut iter = buffer.iter();
    let mut word: Vec<char> = Vec::new();
    while let Some(c) = iter.next() {
        match c {
            ' ' => {
                command.arg(String::from_iter(&word));
                word.clear();
            }
            c => word.push(*c),
        }
    }
    if !word.is_empty() {
        command.arg(String::from_iter(word));
    }
    command
}

fn print_lines<R: std::io::Read>(b: BufReader<R>, stdout: &mut RawOut) {
    b.lines().filter_map(|v| v.ok()).for_each(|v| {
        write!(stdout, "{}\n\r", v).ok();
    });
}

fn print_stdout(cmd: &mut Command, stdout: &mut RawOut) -> Option<()> {
    cmd.stderr(Stdio::piped());
    cmd.stdout(Stdio::piped());
    let proc = cmd.spawn().ok()?;
    if let Some(v) = proc.stderr {
        print_lines(BufReader::new(v), stdout);
    }
    if let Some(v) = proc.stdout {
        print_lines(BufReader::new(v), stdout);
    }
    Some(())
}

fn prompt() -> String {
    format!(
        "{green}> {blue}statc{reset}",
        green = color::Fg(color::Green),
        blue = color::Fg(color::Blue),
        reset = color::Fg(color::Reset)
    )
}

fn main() {
    // Get the standard input stream.
    let stdin = stdin();
    // Get the standard output stream and go to raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();

    // key jump points
    let help_loc = termion::cursor::Goto(1, 1);
    let prompt_loc = termion::cursor::Goto(1, 2);
    let statc_loc = termion::cursor::Goto(1, 3);

    // pretty prompt
    let prompt = prompt();

    write!(
        stdout,
        "{}{}q to exit.{}",
        termion::clear::All,
        help_loc,
        termion::cursor::Hide
    )
    .unwrap();

    write!(stdout, "{}{prompt} _", prompt_loc,).unwrap();
    write!(stdout, "{}", statc_loc).unwrap();
    let mut buffer: Vec<char> = Vec::new();
    let mut command = make_cmd(&buffer);
    print_stdout(&mut command, &mut stdout);

    stdout.flush().ok();

    for c in stdin.keys() {
        // clear required lines
        write!(stdout, "{}{}", prompt_loc, termion::clear::AfterCursor).ok();
        let c = c.unwrap_or(Key::Esc);
        match c {
            Key::Char('\n') | Key::Char('q') | Key::Esc => break,
            Key::Char(c) => buffer.push(c),
            Key::Backspace => {
                buffer.pop();
            }
            _ => (),
        }
        write!(
            stdout,
            "{}{prompt} {}_",
            prompt_loc,
            String::from_iter(&buffer)
        )
        .ok();
        write!(stdout, "{}", statc_loc).ok();

        let mut command = make_cmd(&buffer);
        print_stdout(&mut command, &mut stdout);
        stdout.flush().unwrap();
    }

    // Show the cursor again before we exit.
    write!(stdout, "{}{}", help_loc, termion::cursor::Show).unwrap();
}
