use structopt::StructOpt;
use std::num::ParseIntError;

#[derive(Debug, StructOpt)]
#[structopt(
name = "docgen",
about = "An easy to use tool to generate fake data in bulk and export it as Avro, Parquet or directly into your database as tables"
)]
pub struct Args {
    #[structopt(subcommand)]
    pub command: Command,
}

fn convert_to_u8(src: &str) -> Result<u8, ParseIntError> {
    Ok(src.as_bytes()[0])
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "csv", alias = "c")]
    GenerateCSV {
        #[structopt(name = "output", alias = "o")]
        output_path: String,

        #[structopt(name = "schema", alias = "s")]
        schema_path: String,

        #[structopt(name = "numrecs", alias = "n")]
        num_records: usize,

        #[structopt(name = "delim", alias = "d", default_value = ",", parse(try_from_str = "convert_to_u8"))]
        delimiter: u8,
    },

    #[structopt(name = "avro", alias = "a")]
    GenerateAvro {
        #[structopt(name = "output", alias = "o")]
        output_path: String,

        #[structopt(name = "schema", alias = "s")]
        schema_path: String,

        #[structopt(name = "numrecs", alias = "n")]
        num_records: usize,
    },
}
