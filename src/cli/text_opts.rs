use crate::{process_decrypt, process_encrypt, process_generate_key, process_sign, process_verify};

use super::{verify_file, verify_file_path, CmdExcuter};
use clap::Parser;
use std::{fmt, path::PathBuf, str::FromStr};

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(name = "sign", about = "Sign a text")]
    Sign(TextSignOpts),
    #[command(name = "verify", about = "Verify a text")]
    Verify(TextVerifyOpts),
    #[command(name = "generate-key", about = "Generate a key")]
    GenerateKey(TextGenerateKeyOpts),
    #[command(name = "encrypt", about = "Encrypt a text")]
    Encrypt(TextEncryptOpts),
    #[command(name = "decrypt", about = "Decrypt a text")]
    Decrypt(TextDecryptOpts),
}

impl CmdExcuter for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let signed = process_sign(&self.input, &self.key, self.format)?;
        println!("{}", signed);
        Ok(())
    }
}

impl CmdExcuter for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let valid = process_verify(&self.input, &self.key, &self.sig, self.format)?;
        println!("{}", valid);
        Ok(())
    }
}

impl CmdExcuter for TextGenerateKeyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = process_generate_key(self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.text");
                std::fs::write(name, &key[0])?;
            }
            TextSignFormat::Ed25519 => {
                let name = self.output;
                std::fs::write(name.join("ed25519.sk"), &key[0])?;
                std::fs::write(name.join("ed25519.pk"), &key[1])?;
            }
        }
        Ok(())
    }
}

impl CmdExcuter for TextEncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encrypted = process_encrypt(&self.input, &self.key)?;
        println!("{}", encrypted);
        Ok(())
    }
}

impl CmdExcuter for TextDecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decrypted = process_decrypt(&self.input, &self.key)?;
        println!("{}", decrypted);
        Ok(())
    }
}

impl CmdExcuter for TextSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            TextSubCommand::Sign(opt) => opt.execute().await,
            TextSubCommand::Verify(opt) => opt.execute().await,
            TextSubCommand::GenerateKey(opt) => opt.execute().await,
            TextSubCommand::Encrypt(opt) => opt.execute().await,
            TextSubCommand::Decrypt(opt) => opt.execute().await,
        }
    }
}

#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    #[arg(short, long, value_parser = verify_file,default_value="-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    #[arg(short, long, value_parser = verify_file,default_value="-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file,default_value="-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long,default_value = "blake3",value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file,default_value="-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long)]
    pub sig: String,
    #[arg(long,default_value = "blake3",value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextGenerateKeyOpts {
    #[arg(short, long, default_value = "blake3",value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_file_path)]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
