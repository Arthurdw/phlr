extern crate termion;

use termion::{color, style};

pub fn error(message: &str) {
    println!("{red}{bold}error:{reset} {message}", red=color::Fg(color::LightRed), bold=style::Bold, reset=style::Reset, message=message);
}

pub fn info(message: &str) {
    println!("{blue}{bold}info:{reset} {message}", blue=color::Fg(color::LightBlue), bold=style::Bold, reset=style::Reset, message=message);
}
