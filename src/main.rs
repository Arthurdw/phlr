extern crate clap;
extern crate termion;

use clap::{App, Arg, SubCommand};
use termion::{color, style};

mod generator;
mod printer;
mod verbose;

fn main() {
    let matches = App::new("Pre-Hash Loader/Renderer (phlr)")
        .version("0.0.1")
        .author("Arthurdw <mail.arthurdw@gmail.com>")
        .about("Generate a hashed dataset from a dataset")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("The intensity of the verbose."),
        )
        .subcommand(
            SubCommand::with_name("generate")
                .visible_alias("gen")
                .about("Generate a new dataset.")
                .arg(
                    Arg::with_name("source")
                        .short("S")
                        .long("source")
                        .takes_value(true)
                        .required(true)
                        .help("The output file.")
                        .index(1),
                ),
        )
        .get_matches();

    let occurences = matches.occurrences_of("v");

    let logger = printer::Logger {
        intensity: verbose::Verbose::from_intensity_value(occurences),
    };

    logger.debug("Successfully created logger object.");

    match logger.intensity.intensity {
        verbose::VerboseIntensity::DEBUG => logger.info("Currently in mode: DEBUG"),
        verbose::VerboseIntensity::CRITICAL => logger.info("Currently in mode: CRITICAL"),
        verbose::VerboseIntensity::ERROR => logger.info("Currently in mode: ERROR"),
        verbose::VerboseIntensity::WARNING => logger.info("Currently in mode: WARNING"),
        verbose::VerboseIntensity::INFO => logger.info("Currently in mode: INFO"),
    }

    if let Some(matches) = matches.subcommand_matches("generate") {
        logger.debug("Sucommand match found with \"generate\".");
        generator::generate(logger, matches.value_of("source").expect("output.txt"));
    } else {
        logger.error("Please provide a valid subcommand or argument.");
        logger.error(&format!(
            "See `phlr {green}--help{reset}` for further information.",
            green = color::Fg(color::Green),
            reset = style::Reset
        ));
    }
}
