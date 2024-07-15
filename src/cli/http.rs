use std::path::PathBuf;

use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::process_http_serve;

use super::CmdExcuter;

#[enum_dispatch(CmdExcuter)]
#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(name = "serve", about = "Start a HTTP server")]
    Server(HttpsServerOpts),
}

impl CmdExcuter for HttpsServerOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let response = process_http_serve(self.dir, self.port);
        println!("{:?}", response.await?);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct HttpsServerOpts {
    #[arg(short, long, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
