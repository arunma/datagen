#[macro_use]
extern crate structopt;

use failure_tools::ok_or_exit;
use std::fmt::Error;
use structopt::StructOpt;

mod options;

fn run() -> Result<(), Error> {
    use options::Command::*;

    let opt: options::Args = options::Args::from_args();

    match opt.command {
        GenerateCSV {
            output_path,
            schema_path,
            num_records,
        } => (
            println!("Output Path {}", output_path)
        ),
        GenerateAvro {
            output_path,
            schema_path,
            num_records,
        } => (
            println!("Output Path {}", output_path)
        ),
    }

    Ok(())
}


fn main() {
    println!("Hello, world!");
    ok_or_exit(run())
}
