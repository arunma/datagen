extern crate structopt;

use std::fmt::Error;
use std::fs::File;

use failure_tools::ok_or_exit;
use structopt::StructOpt;

use datagen::{write_fake_records};

mod options;

fn run() -> Result<(), Error> {
    use options::Command::*;

    let opt: options::Args = options::Args::from_args();

    match opt.command {
        GenerateCSV {
            output_path,
            schema_path,
            num_records,
        } => {
            println!("Output Path {}, Schema Path {}, Num Records {}", output_path, schema_path, num_records);
            let writer = File::create(output_path).expect("Output File Path not found");
            write_fake_records(writer, schema_path, num_records as i64);

        },
        GenerateAvro {
            output_path,
            schema_path,
            num_records,
        } => {
            println!("Output Path {}, Schema Path {}, Num Records {}", output_path, schema_path, num_records)
        },
    }

    Ok(())
}

fn main() {
    println!("Hello, world!");
    ok_or_exit(run())
}
