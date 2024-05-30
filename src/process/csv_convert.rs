use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use crate::cliopts::OutputFormat;

#[derive(Debug, Serialize, Deserialize)]
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

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut rdr = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = rdr.headers()?.clone();
    for result in rdr.records() {
        let record = result?;
        // header.iter(): use the headers iterator to get the column names
        // record.iter(): use the record iterator to get the column values
        // zip(): combine the two iterators into a single iterator of tuples
        //        [(header1, record1), (header2, record2), ...]
        // collect::<Value>(): collect the tuples into a JSON value
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    fs::write(output, content)?;
    Ok(())
}
