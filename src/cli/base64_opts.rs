use std::{fmt, str::FromStr};

use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{process_decode, process_encode};

use super::{verify_file, CmdExcuter};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExcuter)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to Base64")]
    Encode(BaseEncodeOpts),
    #[command(name = "decode", about = "Decode a Base64 string")]
    Decode(BaseDecodeOpts),
}

impl CmdExcuter for BaseDecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decoded = process_decode(&self.input, self.format)?;
        println!("{}", String::from_utf8_lossy(&decoded));
        Ok(())
    }
}

impl CmdExcuter for BaseEncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encoded = process_encode(&self.input, self.format)?;
        println!("{}", encoded);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct BaseEncodeOpts {
    #[arg(short, long, value_parser = verify_file,default_value="-")]
    pub input: String,
    #[arg(long,value_parser = parse_base64_format,default_value="standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct BaseDecodeOpts {
    #[arg(short, long, value_parser = verify_file,default_value="-")]
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
