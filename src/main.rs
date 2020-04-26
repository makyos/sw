use std::io::{stdin, stdout, Write};
use std::string::String;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

extern crate chrono;
use chrono::prelude::*;

extern crate termion;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;


fn dur(start: &chrono::DateTime<chrono::Local>) -> String {
    let dur = Local::now() - *start;
    return format!("+{:>02}:{:>02}:{:>02}({})",
                   dur.num_hours()   % 24,
                   dur.num_minutes() % 60,
                   dur.num_seconds() % 60,
                   dur.num_seconds());
}

fn main() {

    //// KEYBORD event from STDIO at new thread
    let stdin = stdin();
    let (tx, rx) = channel();
    thread::spawn(move || {
        for e in stdin.events() {
            if let Ok(evt) = e {
                tx.send(evt).unwrap();
            }
        }
    });

    let mut stdout = stdout().into_raw_mode().unwrap();
    let start: chrono::DateTime<chrono::Local> = Local::now();
    let _ = write!(stdout, "{} {}\r", Local::now().to_string(), dur(&start));
    let _ = write!(stdout, "\n");

    loop {

        stdout.flush().unwrap();
        let _ = write!(stdout, "{} {}\r", Local::now().to_string(), dur(&start));

        //// KEYBORD event hundle
        if let Ok(evt) = rx.recv_timeout(Duration::from_millis(10)) {
            match evt {
                Event::Key(Key::Char('q')) | Event::Key(Key::Ctrl('c')) => {
                    let _ = write!(stdout, "\n");
                    return;
                }
                _ => {
                    let _ = write!(stdout, "\n");
                }
            }
        }

    }

}

