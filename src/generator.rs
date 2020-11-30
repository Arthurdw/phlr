<<<<<<< HEAD
pub fn generate(input: &str) {
    println!("Processing {}", input)
=======
use super::printer::{Logger};

pub fn generate(logger: Logger, input_path: &str) {
    // let file = input_path;
    logger.info(&format!("Processing file: {}", input_path));
>>>>>>> f423c26 (Improved code.)
}
