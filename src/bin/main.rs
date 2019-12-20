#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};

// subcommandはdispersal
// 引数が

fn main() {
    let matches = App::new("Impatiens")
        .version(crate_version!())
        .about(crate_description!())
        // .setting(AppSettings::DeriveDisplayOrder)
        // .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("debug")
                .help("turn on debugging information")
                .short("d")
                .long("debug"),
        )
        .subcommand(
            SubCommand::with_name("help")
                .alias("h")
                .about("Show help")
                .setting(AppSettings::ColoredHelp),
        )
        .subcommand(
            SubCommand::with_name("dispersal")
                .setting(AppSettings::ColoredHelp)
                .about("run load testing")
                .arg(Arg::with_name("url").help("Target url").required(true))
                .arg(
                    Arg::with_name("requests")
                        .help("Number of requests to perform")
                        .short("n")
                        .long("requests")
                        .value_name("requests")
                        .required(true),
                )
                .arg(
                    Arg::with_name("concurrency")
                        .help("Number of multiple requests to make at a time")
                        .short("c")
                        .long("concurrency")
                        .value_name("concurrency")
                        .required(true),
                )
                .arg(
                    Arg::with_name("output")
                        .help("Output format.")
                        .short("o")
                        .long("output")
                        .possible_values(&["text", "json"])
                        .value_name("output"),
                )
                .arg(
                    Arg::with_name("verbose")
                        .help("Output all message")
                        .short("v")
                        .long("verbose"),
                ),
        )
        .get_matches();

    if matches.is_present("debug") {
        println!("Debugging is turned on");
    }
}
