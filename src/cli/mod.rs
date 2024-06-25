mod base64_opts;
mod csv_opts;
mod genpass_opts;

use clap::Parser;

pub use base64_opts::Base64SubCommand;
pub use csv_opts::CsvOpts;
pub use csv_opts::OutputFormat;
pub use genpass_opts::GenPassOpts;

//rcli csv -i input.csv -o output.json --herder -d ','
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV,or Convert CSV to Other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    Genpass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}
