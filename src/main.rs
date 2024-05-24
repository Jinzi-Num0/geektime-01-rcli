use anyhow::Result;
use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

//Name,Position,DOB,Nationality,Kit Number
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_number: u8,
}

//rcli csv -i input.csv -o output.json --herder -d ','
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Show CSV,or Convert CSV to Other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,
    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    output: String,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
    #[arg(long, default_value_t = true)]
    header: bool,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opt) => {
            let mut reader = Reader::from_path(opt.input)?;
            let mut ret = Vec::with_capacity(128);
            for result in reader.deserialize() {
                let record: Record = result?;
                ret.push(record);
            }
            print!("{:?}", ret);
            let json = serde_json::to_string(&ret)?;
            fs::write(opt.output, json)?;
        }
    }
    Ok(())
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File not found".to_string())
    }
}
