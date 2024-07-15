use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(name = "serve", about = "Start a HTTP server")]
    Server(HttpsServerOpts),
}

#[derive(Debug, Parser)]
pub struct HttpsServerOpts {
    #[arg(short, long, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
