// ©Arthurdw
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
                        .help("The input file.")
                        .index(1),
                )
                .arg(
                    Arg::with_name("output")
                        .short("O")
                        .long("out")
                        .takes_value(true)
                        .help("The output file."),
                )
                .arg(
                    Arg::with_name("delimiter")
                        .short("D")
                        .long("delimiter")
                        .takes_value(true)
                        .help(
                            "The delimiter that is between the password and the hash. DELAULT: ;",
                        ),
                ),
        )
        .get_matches();

    let occurences = matches.occurrences_of("v");

    let logger = printer::Logger {
        intensity: verbose::Verbose::new(occurences),
    };

    logger.debug("Successfully created logger object.");

    if let Some(matches) = matches.subcommand_matches("generate") {
        logger.print_log_mode();
        logger.debug("Sucommand match found with \"generate\".");
        generator::generate(
            logger,
            matches.value_of("source").expect("input.txt"),
            matches.value_of("output"),
            matches.value_of("delimiter"),
        );
    } else {
        logger.warn("Please provide a valid subcommand or argument.");
        logger.warn(&format!(
            "See `phlr {green}--help{reset}` for further information.",
            green = color::Fg(color::Green),
            reset = style::Reset
        ));
    }
}
