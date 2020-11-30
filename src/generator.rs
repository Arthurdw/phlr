// ©Arthurdw
use super::printer::{Logger};

pub fn generate(logger: Logger, input_path: &str) {
    // let file = input_path;
    logger.info(&format!("Processing file: {}", input_path));
}
