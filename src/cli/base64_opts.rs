use clap::Parser;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to Base64")]
    Encode(BaseEncodeOpts),
    #[command(name = "decode", about = "Decode a Base64 string")]
    Decode(BaseDecodeOpts),
}

#[derive(Debug, Parser)]
pub struct BaseEncodeOpts {
    #[arg(short, long)]
    pub input: String,
}

#[derive(Debug, Parser)]
pub struct BaseDecodeOpts {
    #[arg(short, long)]
    pub input: String,
}
