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
        .version("0.1.1")
        .author("Arthurdw <mail.arthurdw@gmail.com>")
        .about("Generate a hashed dataset from a dataset")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("The intensity of the verbose."),
        )
        .arg(
            Arg::with_name("force")
                .short("F")
                .long("force")
                .takes_value(false)
                .help("Force the current action. (overrides or appends to any existing files etc)"),
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
                        .value_name("filename")
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
                            "The delimiter that is between the password and the hash. DELAULT: \';\'",
                        )
                )
                .arg(
                    Arg::with_name("method")
                        .short("M")
                        .long("method")
                        .takes_value(true)
                        .help(
                            "The method that should be used to hash. Default is \'sha256\'. Options: sha224, sha256, sha384, sha512, sha512/224, sha512/256",
                        ),
                ),
        )
        .get_matches();

    let occurences = matches.occurrences_of("v");

    let logger = printer::Logger {
        intensity: verbose::Verbose::new(occurences),
    };

    logger.debug("Successfully created logger object.");

    if let Some(sub) = matches.subcommand_matches("generate") {
        logger.print_log_mode();
        logger.debug("Sucommand match found with \"generate\".");
        generator::generate(
            logger,
            sub.value_of("source").expect("input.txt"),
            sub.value_of("output"),
            sub.value_of("delimiter"),
            sub.value_of("method"),
            matches.is_present("force"),
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
