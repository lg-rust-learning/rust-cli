mod base64;
mod csv;
mod gen_pass;

use std::path::Path;

use clap::Parser;

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
};
use self::{csv::CsvOpts, gen_pass::GenPassOpts};

#[derive(Parser, Debug)]
#[command(name = "rcli", version, author, about)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

// We also can return Result<String, &'static str>
// String literal is static
fn verify_file_exists(file_name: &str) -> Result<String, String> {
    if file_name == "-" || Path::new(file_name).exists() {
        Ok(file_name.to_string())
    } else {
        Err(format!("File not found: {}", file_name))
    }
}
