use clap::Parser;
use rcli::{CmdExcuter, Opts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    println!("{:?}", opts);
    opts.cmd.execute().await?;
    Ok(())
}
