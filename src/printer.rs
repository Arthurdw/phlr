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
            &color::Fg(color::LightMagenta).to_string(),
        );

        self.inner_printer(&formatted_message, 1)
    }

    pub fn critical(&self, message: &str) {
        let formatted_message =
            self.inner_formatter("critical", message, &color::Fg(color::Red).to_string());

        self.inner_printer(&formatted_message, 2)
    }

    pub fn error(&self, message: &str) {
        let formatted_message =
            self.inner_formatter("error", message, &color::Fg(color::LightRed).to_string());

        self.inner_printer(&formatted_message, 3)
    }

    pub fn warn(&self, message: &str) {
        let formatted_message =
            self.inner_formatter("warn", message, &color::Fg(color::LightYellow).to_string());

        self.inner_printer(&formatted_message, 4)
    }

    pub fn info(&self, message: &str) {
        let formatted_message =
            self.inner_formatter("info", message, &color::Fg(color::LightBlue).to_string());

        self.inner_printer(&formatted_message, 5)
    }
}
