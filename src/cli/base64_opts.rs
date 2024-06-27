use std::{fmt, str::FromStr};

use clap::Parser;

use super::verify_input_file;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to Base64")]
    Encode(BaseEncodeOpts),
    #[command(name = "decode", about = "Decode a Base64 string")]
    Decode(BaseDecodeOpts),
}

#[derive(Debug, Parser)]
pub struct BaseEncodeOpts {
    #[arg(short, long, value_parser = verify_input_file,default_value="-")]
    pub input: String,
    #[arg(long,value_parser = parse_base64_format,default_value="standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct BaseDecodeOpts {
    #[arg(short, long, value_parser = verify_input_file,default_value="-")]
    pub input: String,
    #[arg(long,value_parser = parse_base64_format,default_value="standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
