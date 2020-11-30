extern crate termion;

use super::verbose::{Verbose, VerboseIntensity};
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

    fn print_log_mode_helper(&self, mode: &str) {
        self.info(&format!(
            "Logging max level: {bold}{m}{reset}",
            m = mode,
            bold = style::Bold,
            reset = style::Reset
        ));
    }

    pub fn print_log_mode(&self) {
        match self.intensity.intensity {
            VerboseIntensity::DEBUG => self.print_log_mode_helper("DEBUG"),
            VerboseIntensity::CRITICAL => self.print_log_mode_helper("CRITICAL"),
            VerboseIntensity::ERROR => self.print_log_mode_helper("ERROR"),
            VerboseIntensity::WARNING => self.print_log_mode_helper("WARNING"),
            VerboseIntensity::INFO => self.print_log_mode_helper("INFO"),
        }
    }
}
