mod cli;
mod process;

pub use cli::{Base64SubCommand, Opts, SubCommand};

pub use {
    process::process_csv, process::process_decode, process::process_encode,
    process::process_genpass,
};
