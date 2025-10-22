use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;

    let records = reader
        .deserialize()
        .map(|record| record.unwrap())
        .collect::<Vec<Player>>();

    // let mut ret = Vec::with_capacity(128);
    // for result in reader.deserialize() {
    //     println!("Processing record: {:?}", result.unwrap());
    //     let record = result?;
    //     ret.push(record);
    // }
    // println!("{:?}", ret);

    let json = serde_json::to_string_pretty(&records)?;
    fs::write(output, json)?;

    Ok(())
}
