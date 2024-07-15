mod base64_opts;
mod csv_opts;
mod genpass_opts;
mod http;
mod text_opts;

use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::{Path, PathBuf};

pub use base64_opts::*;
pub use csv_opts::*;
pub use genpass_opts::*;
pub use http::*;
pub use text_opts::*;

//rcli csv -i input.csv -o output.json --herder -d ','
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExcuter)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV,or Convert CSV to Other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    Genpass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode or decode")]
    Base64(Base64SubCommand),
    #[command(
        subcommand,
        about = "Text sign, verify, encrypt, decrypt, or generate key"
    )]
    Text(TextSubCommand),
    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubCommand),
}

#[enum_dispatch]
#[allow(async_fn_in_trait)]
pub trait CmdExcuter {
    async fn execute(self) -> anyhow::Result<()>;
}

pub fn verify_file(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File not found".to_string())
    }
}

pub fn verify_file_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".to_string()));
        assert_eq!(verify_file("*"), Err("File not found".to_string()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".to_string()));
        assert_eq!(verify_file("not_found"), Err("File not found".to_string()));
    }
}
