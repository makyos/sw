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

fn main() {

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
    let last_time = Local::now();
	let _ = write!(stdout, "{}\r\n", last_time);

    loop {
        let current_time = Local::now();
        let _ = write!(stdout, "{}\r", current_time.to_string());
        if let Ok(evt) = rx.recv_timeout(Duration::from_millis(100)) {
            match evt {
                Event::Key(Key::Ctrl('c')) => {
					let dur = current_time - last_time;
                    let _ = write!(stdout, "{} duration {} sec \r\n", current_time.to_string(), dur.num_seconds());
                    return;
                }
                Event::Key(Key::Char(c)) => {
                    if c == ' ' {
						let dur = current_time - last_time;
                        let _ = write!(stdout, "{} duration {} sec \r\n", current_time.to_string(), dur.num_seconds());
                    }
                }
                _ => {}
            }
        }
        stdout.flush().unwrap();
    }

}

