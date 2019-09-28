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

	let _ = write!(stdout, "[spc]: split, [q]: stop & quit\r\n");
	let _ = write!(stdout, "{} START!\r\n", start);

    loop {

        stdout.flush().unwrap();
        let _ = write!(stdout, "{}\r", Local::now().to_string());

		//// KEYBORD event hundle
        if let Ok(evt) = rx.recv_timeout(Duration::from_millis(100)) {
            match evt {
                Event::Key(Key::Char('q')) | Event::Key(Key::Ctrl('c')) => {
                    let _ = write!(stdout, "{} GOAL! {} sec \r\n", Local::now().to_string(), (Local::now() - start).num_seconds());
                    return;
                }
                Event::Key(Key::Char(' ')) => {
                    let _ = write!(stdout, "{} duration {} sec \r\n", Local::now().to_string(), (Local::now() - start).num_seconds());
                }
				Event::Key(Key::Ctrl('n')) => {
				}
                _ => {}
            }
        }

    }

}

