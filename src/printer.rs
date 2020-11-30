extern crate termion;

use super::verbose::Verbose;
use termion::{color, style};

pub struct Logger {
    pub intensity: Verbose,
}

impl Logger {
    fn inner_formatter(&self, prefix: &str, message: &str, color: &str) -> String {
        return format!(
            "{clr}{bold}{pre}:{reset} {msg}",
            clr = color,
            bold = style::Bold,
            pre = prefix,
            reset = style::Reset,
            msg = message
        );
    }

    fn inner_printer(&self, message: &str, intensity: u64) {
        if self.intensity.intensity_value >= intensity {
            return;
        }
        println!("{}", message);
    }

    pub fn debug(&self, message: &str) {
        let formatted_message = self.inner_formatter(
            "debug",
            message,
            &format!("{}", color::Fg(color::LightBlue)),
        );
        self.inner_printer(&formatted_message, 5)
    }

    pub fn info(&self, message: &str) {
        let formatted_message = self.inner_formatter(
            "info",
            message,
            &format!("{}", color::Fg(color::LightBlue)),
        );

        // let formatted_message = &format!(
        //     "{color}{bold}info:{reset} {message}",
        //     color = color::Fg(color::LightBlue),
        //     bold = style::Bold,
        //     reset = style::Reset,
        //     message = message
        // );

        self.inner_printer(&formatted_message, 5)
    }

    pub fn error(&self, message: &str) {
        let formatted_message = self.inner_formatter(
            "error",
            message,
            &format!("{}", color::Fg(color::LightRed)),
        );

        // let formatted_message = &format!(
        //     "{color}{bold}error:{reset} {message}",
        //     color = color::Fg(color::LightRed),
        //     bold = style::Bold,
        //     reset = style::Reset,
        //     message = message
        // );

        self.inner_printer(&formatted_message, 3)
    }
}
