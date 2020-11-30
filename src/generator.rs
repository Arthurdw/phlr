// ©Arthurdw
use super::printer::Logger;
use std::fs;
use std::io::{Write, BufRead, BufReader};
use std::path::Path;
use std::process::exit;

pub fn generate(logger: Logger, input_path: &str, output_path: Option<&str>) {
    if !Path::new(input_path).exists() {
        return logger.error(&format!(
            "File with path \"{}\" could not be found!",
            input_path
        ));
    }

    let out: String;

    if output_path.is_none() {
        logger.debug("No file path provided, using filename as directory name.");

        let path_splitted_file_name: Vec<&str> = input_path.split("/").collect();
        let full_file_name = path_splitted_file_name.last().unwrap().to_string();
        logger.debug(&format!("Full filename: \"{}\"", &full_file_name));
        let extension_splitted_file_name: Vec<&str> = full_file_name.split(".").collect();
        out = extension_splitted_file_name.first().unwrap().to_string();
        logger.debug(&format!("Filename: \"{}\"", &out));
    } else {
        out = output_path.unwrap().to_string();
    }

    logger.debug(&format!("Destination folder: \"{}\"", out));

    if Path::new(&format!("{}/", out)).exists() {
        logger.error(&format!(
            "Folder with name \"{}\" already exists in current directory.",
            out
        ));
        return logger.error("Omit a custom output folder name using --out <name>.");
    }

    logger.debug(&format!("Creating directory: \"{}\"", out));
    match fs::create_dir(&format!("{}/", out)) {
        Ok(_) => logger.info(&format!("Creating destination directory: \"{}\"", out)),
        Err(e) => {
            logger.error("An error occured while trying to create the destination directory.");
            logger.fatal(&e.to_string());
            exit(1)
        }
    }

    logger.debug("Started reading file...");
    match fs::File::open(input_path) {
        Ok(file) => {
            match fs::File::create(&format!("{dir}/{dir}-out.txt", dir=out)) {
                Ok(_) => {
                    logger.debug(&format!("Successfully created file: \"{}-out.txt\"", out))
                },
                Err(e) => {
                    logger.error("An error occured while trying to open the output file.");
                    logger.fatal(&e.to_string());
                    exit(1)
                }
            }

            let file = BufReader::new(file);
            for line in file.lines().filter_map(|result| result.ok()) {
                if line.is_empty() {
                    continue;
                }
                logger.debug(&format!("Reading \"{}\"", line));

                let mut file = fs::OpenOptions::new().write(true).append(true).open(&format!("{dir}/{dir}-out.txt", dir=out)).unwrap();

                if let Err(e) = writeln!(file, "{}", line) {
                    logger.error(&format!("Could not write \"{}\" to file!", line));
                    logger.error(&e.to_string());
                }
            }
        }
        Err(e) => {
            logger.error("An error occured while trying to read the file.");
            logger.fatal(&e.to_string());
            exit(1)
        }
    }
}
