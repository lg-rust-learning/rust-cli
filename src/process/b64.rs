use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

use crate::cliopts::Base64Format;
use std::{fs::File, io::Read};

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    // Box<dyn Read> is a trait object that can hold any type that implements the Read trait
    // Box::new(std::io::stdin()) creates a Box<dyn Read> that reads from stdin
    // Box::new(File::open(input)?) creates a Box<dyn Read> that reads from a file
    let mut reader = get_reader(input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
    };

    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    println!("{}", String::from_utf8(decoded)?);
    Ok(())
}

fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    Ok(if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        let result = process_encode(input, format);
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::UrlSafe;
        let result = process_decode(input, format);
        assert!(result.is_ok());
    }
}
