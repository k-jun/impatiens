#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};

// subcommandはdispersal
// 引数が

fn main() {
    let mut app = build_app();
    match app.clone().get_matches().subcommand() {
        ("dispersal", Some(_)) => app.print_help(),
        ("help", Some(_)) | _ => app.print_help(),
    };
}

fn build_app() -> App<'static, 'static> {
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
                        .short("r")
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
                ),
        );
    matches
}
