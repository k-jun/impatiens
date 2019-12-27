#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::time::{Duration, Instant};
use url::Url;

type ImpatiensResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> ImpatiensResult<()> {
    let mut app = build_app();
    match app.clone().get_matches().subcommand() {
        ("dispersal", Some(matches)) => dispersal(matches)?,
        ("help", Some(_)) | _ => app.print_help()?,
    }
    Ok(())
}

fn dispersal(matches: &ArgMatches) -> ImpatiensResult<()> {
    let concurrency: String = matches
        .value_of("concurrency")
        .unwrap_or_default()
        .parse()?;

    println!("{}", concurrency);

    let requests: String = matches.value_of("requests").unwrap_or_default().parse()?;
    let url = Url::parse(matches.value_of("url").unwrap_or_default())?;
    //
    let host = match url.host_str() {
        Some(host) => host,
        None => {
            // TODO スキーマがなくてもパースできるようにしたい
            println!("url parse error");
            return Ok(());
        }
    };
    //
    let port = url.port().unwrap_or(80);
    let report = seed()?;
    //
    println!("report {}", "here impatiens report comes");
    Ok(())
}

fn seed() -> ImpatiensResult<()> {
    Ok(())
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
