use std::io::{stdin, stdout, Write};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

extern crate chrono;
use chrono::prelude::*;

extern crate termion;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn split(stdout: &mut termion::raw::RawTerminal<std::io::Stdout>, start: &chrono::DateTime<chrono::Local>) {
    let now = Local::now();
    let dur = now - *start;
    let h   = dur.num_hours()   % 24;
    let m   = dur.num_minutes() % 60;
    let s   = dur.num_seconds() % 60;
    let _ = write!(stdout, "{} +{:>02}:{:>02}:{:>02}({}sec) \r\n", now.to_string(), h, m, s, dur.num_seconds());
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
    split(&mut stdout, &start);

    loop {

        stdout.flush().unwrap();
        let _ = write!(stdout, "{}\r", Local::now().to_string());

        //// KEYBORD event hundle
        if let Ok(evt) = rx.recv_timeout(Duration::from_millis(100)) {
            match evt {
                Event::Key(Key::Char('q')) | Event::Key(Key::Ctrl('c')) => {
                    split(&mut stdout, &start);
                    return;
                }
                _ => {
                    split(&mut stdout, &start);
                }
            }
        }

    }

}

