extern crate structopt;

use std::fmt::Error;
use std::fs;
use std::fs::File;

use failure_tools::ok_or_exit;
use structopt::StructOpt;

use datagen::{write_csv, write_avro};
use std::path::Path;

mod options;

fn run() -> Result<(), Error> {
    use options::Command::*;

    let opt: options::Args = options::Args::from_args();

    match opt.command {
        GenerateCSV {
            output_path,
            schema_path,
            num_records,
            delimiter,
        } => {
            println!(
                "Output Path {}, Schema Path {}, Num Records {}",
                &output_path, schema_path, num_records
            );
            fs::create_dir_all(Path::new(&output_path).parent().unwrap());
            let writer = File::create(&output_path).expect("Output File Path not found");
            write_csv(writer, schema_path, num_records as i64, delimiter); //TODO - Throws warning
        }
        GenerateAvro {
            output_path,
            schema_path,
            num_records,
        } => {
            println!(
                "Output Path {}, Schema Path {}, Num Records {}",
                output_path, schema_path, num_records
            );
            fs::create_dir_all(Path::new(&output_path).parent().unwrap());
            let writer = File::create(&output_path).expect("Output File Path not found");
            write_avro(writer, schema_path, num_records as i64); //TODO - Throws warning
        },
    }

    Ok(())
}

fn main() {
    ok_or_exit(run())
    /*let string = String::from_str("hello");
    let fmt_string = format!("{:?}", string.to_string);
    println!(fmt_string);*/
}
