extern crate termion;

use std::io;

use termion::clear;
use termion::cursor;
use termion::raw;

use io::Write;
use raw::IntoRawMode;

pub struct Canvas {
    stdout: raw::RawTerminal<io::Stdout>,
    height: u16,
}

impl Canvas {
    pub fn new(height: u16) -> Canvas {
        // Take control of stdout.
        let mut stdout = io::stdout().into_raw_mode().unwrap();

        // Clear a space for us to write to.
        let space = std::iter::repeat("\n").take(height as usize).collect::<String>();
        write!(stdout, "{}{}", space, cursor::Up(height)).unwrap();

        Canvas{stdout, height}
    }

    pub fn write(&mut self, line: u16, message: &str) {
        assert!(line <= self.height);

        // The prompt is one-based, maybe?
        let line = line + 1;

        let size = message.len() as u16;

        write!(self.stdout, "{}{}{}", cursor::Down(line), clear::CurrentLine, message).unwrap();
        write!(self.stdout, "{}{}", cursor::Up(line), cursor::Left(size)).unwrap();
        self.stdout.flush().unwrap();
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        // Clear all of our canvas.
        for _ in 0..self.height {
            write!(self.stdout, "{}\n", clear::CurrentLine).unwrap();
        }

        // Move cursor back to the top.
        write!(self.stdout, "{}", cursor::Up(self.height)).unwrap();
    }
}

