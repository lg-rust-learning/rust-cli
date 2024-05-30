use clap::Parser;
use std::{fmt, str::FromStr};

use super::verify_file_exists;

#[derive(Parser, Debug)]
pub struct CsvOpts {
    // valure_parser: use the verify_file_exists function to check input data
    #[arg(short, long, value_parser = verify_file_exists)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    // default_value: convert value to OutputFormat
    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
    // default_value_t: ',' is char, we don't need to convert the type
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format: {}", value)),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
