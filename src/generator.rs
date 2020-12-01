// ©Arthurdw
use super::printer::Logger;
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512, Sha512Trunc224, Sha512Trunc256};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::exit;

pub fn generate(
    logger: Logger,
    input_path: &str,
    output_path: Option<&str>,
    delimiter: Option<&str>,
    method: Option<&str>,
    force: bool,
) {
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

    if Path::new(&format!("{}/", out)).exists() && !force {
        logger.error(&format!(
            "Folder with name \"{}\" already exists in current directory.",
            out
        ));
        return logger.error("Omit a custom output folder name using --out <name>.");
    }

    if !Path::new(&format!("{}/", out)).exists() {
        logger.debug(&format!("Creating directory: \"{}\"", out));
        match fs::create_dir(&format!("{}/", out)) {
            Ok(_) => logger.info(&format!("Creating destination directory: \"{}\"", out)),
            Err(e) => {
                logger.error("An error occured while trying to create the destination directory.");
                logger.fatal(&e.to_string());
                exit(1)
            }
        }
    }

    logger.debug("Started reading file...");
    match fs::File::open(input_path) {
        Ok(file) => {
            if !Path::new(&format!("{dir}/{dir}-out.txt", dir = out)).exists() {
                match fs::File::create(&format!("{dir}/{dir}-out.txt", dir = out)) {
                    Ok(_) => {
                        logger.debug(&format!("Successfully created file: \"{}-out.txt\"", out))
                    }
                    Err(e) => {
                        logger.error("An error occured while trying to open the output file.");
                        logger.fatal(&e.to_string());
                        exit(1)
                    }
                }
            }

            let del: String;

            if delimiter.is_none() {
                del = ";".to_string();
                logger.debug("No delimiter found, using standard delimiter \';\'");
            } else {
                del = delimiter.unwrap().to_string();
                logger.debug(&format!("Using provided delimiter: \'{}\'", del));
            }

            let meth: String;

            if method.is_none() {
                meth = "sha256".to_string();
                logger.debug("Hashing with default method: \'sha256\'");
            } else {
                meth = method.unwrap().to_string();
            }

            let file = BufReader::new(file);
            println!("");
            logger.debug("Started writing data to file!");
            for line in file.lines().filter_map(|result| result.ok()) {
                if line.is_empty() {
                    continue;
                }

                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&format!("{dir}/{dir}-out.txt", dir = out))
                    .unwrap();

                let hashed_result = match &meth.to_lowercase()[..] {
                    "sha224" => format!("{:x}", Sha224::digest(line.as_bytes())),
                    "sha256" => format!("{:x}", Sha256::digest(line.as_bytes())),
                    "sha384" => format!("{:x}", Sha384::digest(line.as_bytes())),
                    "sha512" => format!("{:x}", Sha512::digest(line.as_bytes())),
                    "sha512/224" => format!("{:x}", Sha512Trunc224::digest(line.as_bytes())),
                    "sha512/256" => format!("{:x}", Sha512Trunc256::digest(line.as_bytes())),
                    _ => {
                        logger.fatal(&format!(
                            "Provided hash type \'{}\' is not a valid hash type.",
                            meth
                        ));
                        exit(1);
                    }
                };

                match writeln!(file, "{}{}{}", line, del, hashed_result) {
                    Ok(_) => {
                        logger.debug(&format!("{}-out.txt | Hashed: {}", out, line));
                    }
                    Err(e) => {
                        logger.error(&format!("Could not write \"{}\" to file!", line));
                        logger.error(&e.to_string());
                    }
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
