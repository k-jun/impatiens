#[macro_use]
extern crate clap;

use std::sync::Arc;
use async_std::net::{TcpStream, ToSocketAddrs};
use async_std::task;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::time::{Duration, Instant};
use url::Url;
use std::sync::mpsc::{channel, Sender, Receiver};
type ImpatiensResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> ImpatiensResult<()> {
    let app = App::new("Impatiens")
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

    let mut app_copy = app.clone();
    match app.get_matches().subcommand() {
        ("dispersal", Some(matches)) => dispersal(matches)?,
        ("help", Some(_)) | _ => app_copy.print_help()?,
    }
    Ok(())
}

fn dispersal(matches: &ArgMatches) -> ImpatiensResult<()> {
    let concurrency: usize = matches
        .value_of("concurrency")
        .unwrap_or_default()
        .parse()?;

    println!("{}", concurrency);

    let requests: i64 = matches.value_of("requests").unwrap_or_default().parse()?;
    let url = Url::parse(matches.value_of("url").unwrap_or_default())?;
    //
    let host = match url.host_str() {
        Some(host) => host,
        None => {
            println!("url parse error");
            return Ok(());
        }
    };
    //
    let port = url.port().unwrap_or(80);

    let url_str = format!("{}:{}", host, port);

    let url_req = format!("GET / HTTP/1.1\nHost: {}\nUser-Agent: goku/0.0.1\n\n", host);

    let now = Instant::now();
    let (s, r) = channel(concurrency);

    let send_handler = task::spawn(async move {
        let host = Arc::<str>::from(host);
        let request = Arc::<str>::from(request);

        for _ in 0..requests {
            let host = host.to_string();
            let request = request.to_string();

            let handler = task::spawn(async move { send_request(&host, &request).await });
            s.send(handler).await;
        }
    });
    Ok(())
}

pub async fn seed(
    host: impl ToSocketAddrs,
    request: &str,
) -> Result<(Duration, ByteSize), async_std::io::Error> {
    let now = Instant::now();

    let mut stream = TcpStream::connect(host).await?;

    stream.write(&request.as_bytes()).await?;

    let mut buffer = vec![0u8; 1024];
    let n = stream.read(&mut buffer).await?;
    // let res = buffer.iter().filter(|s| **s != 0).map(|&s| s as char).collect::<String>();
    // println!("{}", n);

    Ok((now.elapsed(), n))
}
