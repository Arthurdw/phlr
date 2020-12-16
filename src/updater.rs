// ©Arthurdw
extern crate reqwest;

use super::printer::Logger;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use std::process::Command;
use std::str;

pub struct CommandExecutor {
    pub logger: Logger,
}

impl CommandExecutor {
    fn execute(&self, command: &str, args: &mut [&str]) -> String {
        let cmd = Command::new(command)
            .args(args)
            .output()
            .expect("Updating failed.");

        match cmd.status.code() {
            Some(code) => self
                .logger
                .debug(&format!("Command '{}' exited with code {}", command, code)),
            None => self.logger.fatal("Process terminated by signal."),
        };
        let output = match str::from_utf8(&cmd.stdout) {
            Ok(v) => v,
            Err(e) => {
                self.logger.fatal(&format!("Invalid UTF-8 sequence: {}", e));
                ""
            }
        };
        
        output.to_string()
    }

    pub fn update(&self) {
        self.logger.debug(&self.execute("sudo", &mut ["apt", "remove", "phlr", "-y"]));

        self.logger.debug("Fetching install commands...");
        let command = match reqwest::blocking::get("https://phlr.arthurdw.com") {
            Ok(v) => v.text().unwrap(),
            Err(e) => {
                self.logger.error("Could not fetch install commands!");
                self.logger.error(&e.to_string());
                exit(1);
            }
        };
        if Path::new("phlr_installer.sh").exists() {
            self.logger.debug("Existing updater found, removing...");
            self.logger.debug(&self.execute("sudo", &mut ["rm phlr_installer.sh"]));
        };

        self.logger.debug("Creating installer file...");
        match fs::File::create("phlr_installer.sh") {
            Ok(_) => {
                self.logger.debug("Successfully created installer file");
            }
            Err(e) => {
                self.logger.error("Could not create installer file!");
                self.logger.error(&e.to_string());
                exit(1);
            }
        }

        self.logger.debug("Opening installer file...");
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(false)
            .open("phlr_installer.sh")
            .unwrap();

        self.logger.debug("Started writing to installer file...");
        match writeln!(file, "{}", command) {
            Ok(_) => {
                self.logger.debug("Successfully wrote to installer file.");
            }
            Err(e) => {
                self.logger.error("Could not write to installer file!");
                self.logger.error(&e.to_string());
                exit(1);
            }
        }

        self.logger.debug("Started executing the installer file...");
        self.execute("sh", &mut ["phlr_installer.sh"]);

        self.logger.debug("Started removing the installer file...");
        self.execute("sudo", &mut ["rm", "phlr_installer.sh"]);

        self.logger.info("Update complete. (If the version appears under this line everything has installed corectly)");
        self.logger.info(&self.execute("phlr", &mut ["-V"]));
    }
}
