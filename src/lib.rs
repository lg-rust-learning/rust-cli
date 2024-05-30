mod cliopts;
mod process;

pub use cliopts::{Base64SubCommand, Opts, SubCommand};
pub use process::{process_csv, process_decode, process_encode, process_genpass};
