use std::io::{Read, Write};
use std::path::PathBuf;

use clap::Parser;
use thiserror::Error;
use tracing::{info, trace, Level};
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use uwu::UwuError;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input file to uwuify.
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    /// The content to uwuify.
    #[arg(trailing_var_arg = true)]
    text: Option<Vec<String>>,

    /// Output file. If not provided, will print to stdout.
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Output in JSON format.
    #[arg(long)]
    json: bool,

    /// Show debugging output.
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Error, Debug)]
pub enum UwuCliError {
    #[error("uwu error: {0}")]
    Uwu(#[from] UwuError),
    #[error("log error: {0}")]
    Log(#[from] tracing_subscriber::util::TryInitError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Unknown(#[from] Box<dyn std::error::Error + Send>),
}

fn main() -> Result<(), UwuCliError> {
    let args = Cli::parse();
    init_log(&args)?;

    trace!("Arguments: {args:?}");

    let input = read_input(&args)?;
    let uwuified = uwu::Uwu::new().uwuify(input)?;
    write_output(uwuified, &args)?;

    Ok(())
}

fn init_log(args: &Cli) -> Result<(), UwuCliError> {
    let stderr_layer = fmt::layer().with_writer(std::io::stderr);
    let log_level = match args.verbose {
        0 => Level::WARN,
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE,
    };

    tracing_subscriber::registry()
        .with(stderr_layer)
        .with(
            Targets::default()
                .with_target("uwu", log_level)
                .with_target("uwu_cli", log_level)
                .with_default(Level::WARN),
        )
        .try_init()
        .map_err(UwuCliError::from)?;
    Ok(())
}

fn read_input(args: &Cli) -> Result<String, UwuCliError> {
    if let Some(file) = &args.file {
        info!("Reading from file: {}", file.display());
        let mut f = std::fs::File::open(file)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        Ok(buf)
    } else if let Some(text) = &args.text {
        info!("Reading from command args");
        Ok(text.join(" "))
    } else {
        info!("Reading from stdin");
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        Ok(buf)
    }
}

fn write_output(content: String, args: &Cli) -> Result<(), UwuCliError> {
    if let Some(output) = &args.output {
        info!("Writing to file: {}", output.display());
        let mut f = std::fs::File::create(output)?;
        f.write_all(content.as_bytes())?;
    } else {
        info!("Writing to stdout");
        print!("{content}");
    }

    Ok(())
}
