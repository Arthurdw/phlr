// ©Arthurdw
use super::printer::Logger;
use std::fs;
use std::path::Path;

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


    if Path::new(&format!("/{}/", out)).exists() {
        logger.error(&format!("Folder with name \"{}\" already exists in current directory.", out));
        return logger.error("Omit a custom output folder name using --out <name>.")
    }

    logger.info(&format!("Processing file: \"{}\"", input_path));
}
