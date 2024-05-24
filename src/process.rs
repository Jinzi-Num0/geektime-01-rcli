use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

//Name,Position,DOB,Nationality,Kit Number
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Record {
    pub name: String,
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit_number: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Record = result?;
        ret.push(record);
    }
    print!("{:?}", ret);
    let json = serde_json::to_string(&ret)?;
    fs::write(output, json)?;
    Ok(())
}
