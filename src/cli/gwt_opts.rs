use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{
    process::{process_gwt_sign, process_gwt_verify},
    CmdExcuter,
};

#[enum_dispatch(CmdExcuter)]
#[derive(Debug, Parser)]
pub enum GWTSubCommand {
    #[command(name = "sign", about = "Sign a GWT token")]
    Sign(GWTSignOpts),
    #[command(name = "verify", about = "Verify a GWT token")]
    Verify(GWTVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct GWTSignOpts {
    #[arg(short, long)]
    pub sub: String,
    #[arg(short, long)]
    pub aud: String,
    #[arg(short, long, value_parser= vsrify_time)]
    pub exp: String,
}

#[derive(Debug, Parser)]
pub struct GWTVerifyOpts {
    #[arg(short, long)]
    pub token: String,
}

impl CmdExcuter for GWTSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token = process_gwt_sign(self.sub, self.aud, self.exp)?;
        println!("{}", token);
        Ok(())
    }
}

impl CmdExcuter for GWTVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token = process_gwt_verify(self.token)?;
        println!("{:?}", token);
        Ok(())
    }
}

//time parser: 13d 12h 11m 10s
fn vsrify_time(time: &str) -> Result<String, anyhow::Error> {
    Ok(time.to_string())
}
