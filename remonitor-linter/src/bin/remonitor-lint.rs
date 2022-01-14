use clap::{ArgEnum, Parser};
use remonitor_linter::lint;
use std::path::PathBuf;

#[derive(Debug, Clone, ArgEnum)]
enum Format {
    Json,
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    /// Output format
    #[clap(arg_enum, short, long, default_value = "json")]
    format: Format,

    /// Repository root path
    #[clap(short, long, parse(from_os_str), default_value = ".")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    match lint(&args.path) {
        Ok(report) => match args.format {
            Format::Json => match serde_json::to_string(&report) {
                Ok(output) => println!("{output}"),
                Err(err) => panic!("{err:?}"),
            },
        },
        Err(err) => panic!("{err:?}"),
    }
}
