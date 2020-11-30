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

<<<<<<< HEAD
    let intensity = match matches.occurrences_of("v") {
        5 => verbose::VerboseIntensity::DEBUG,
        4 => verbose::VerboseIntensity::CRITICAL,
        3 => verbose::VerboseIntensity::ERROR,
        2 => verbose::VerboseIntensity::WARNING,
        1 => verbose::VerboseIntensity::INFO,
        0 | _ => verbose::VerboseIntensity::CRITICAL,
    };

    match intensity {
        verbose::VerboseIntensity::DEBUG => {
            printer::info("Currently in mode: DEBUG")
        }
        verbose::VerboseIntensity::CRITICAL => {
            printer::info("Currently in mode: CRITICAL")
        }
        verbose::VerboseIntensity::ERROR => {
            printer::info("Currently in mode: ERROR")
        }
        verbose::VerboseIntensity::WARNING => {
            printer::info("Currently in mode: WARNING")
        }
        verbose::VerboseIntensity::INFO => {
            printer::info("Currently in mode: INFO")
        }
    }

    if let Some(matches) = matches.subcommand_matches("generate") {
        if matches.is_present("source") {
            generator::generate(matches.value_of("source").expect("output.txt"));
        } else {
            printer::error("No source param found.")
        }
    } else {
        printer::error("Please provide a valid subcommand or argument.");
        printer::error(&format!(
=======
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
>>>>>>> f423c26 (Improved code.)
            "See `phlr {green}--help{reset}` for further information.",
            green = color::Fg(color::Green),
            reset = style::Reset
        ));
    }
}
