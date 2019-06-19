extern crate serde;
extern crate serde_yaml;
#[macro_use]
extern crate fake;
#[macro_use]
extern crate failure;
extern crate avro_rs;
extern crate csv;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod errors;
pub mod fakegen;
pub mod options;
pub mod schema;
pub mod sinks;

use crate::errors::DataGenResult;
use crate::schema::Schema;
use crate::sinks::avro_schema_utils::to_avro_schema;
use crate::sinks::{avro_sink, csv_sink, json_sink, Sink};
use avro_rs::Codec;
use std::io;
use std::fs;

///
/// This program just delegates all the fake data generation work to the wonderful fake-rs library
///
//TODO Need to consider Enum, Union, Fixed, Date, Timestamp and other logical types of Avro too.
#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum DValue {
    Null,
    Boolean(bool),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Bytes(Vec<u8>),
    Str(String),
    Record(Vec<(String, DValue)>),
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DType {
    Boolean,
    Int,
    Float,
    Long,
    Double,
    Bytes,
    String,
    Age,
    Gender,
    //TODO - For now, let's stick to basic types
    //    Date, Array, Map, Nullable (union/null), Record,
}

pub fn write_csv<W: io::Write>(
    writer: W,
    schema_path: String,
    num_records: i64,
    delimiter: u8,
) -> DataGenResult<()> {
    let schema = Schema::from_path(schema_path)?;
    let mut sink = csv_sink::sink(schema.clone(), writer, delimiter)?;
    for _num in 0..num_records {
        let record = fakegen::gen_record_for_schema(schema.clone());
        sink.write(record)?;
    }
    Ok(())
}

pub fn write_avro<W: io::Write>(
    writer: W,
    schema_path: String,
    num_records: i64,
) -> DataGenResult<()> {
    let schema = Schema::from_path(schema_path)?;
    let avro_schema: avro_rs::Schema = to_avro_schema(schema.clone()).unwrap();
    let mut sink = avro_sink::sink(&avro_schema, writer, Codec::Deflate).unwrap();

    for _num in 0..num_records {
        let record = fakegen::gen_record_for_schema(schema.clone());
        sink.write(record)?;
    }
    Ok(())
}

pub fn write_json<W: io::Write>(
    writer: W,
    schema_path: String,
    num_records: i64,
) -> DataGenResult<()> {
    let schema = Schema::from_path(schema_path)?;
    let mut sink = json_sink::sink(schema.clone(), writer, true)?;
    sink.start_array();
    for _num in 0..num_records {
        let record = fakegen::gen_record_for_schema(schema.clone());
        sink.write(record)?;
    }
    sink.end_array();
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;
    use crate::DValue;
    use crate::schema::Schema;
    use serde_json::Value;


    fn get_file_writer(path: &str) -> File {
        fs::create_dir_all(Path::new(path).parent().unwrap()).unwrap();
        let writer = File::create(path).expect("Output File Path not found");
        writer
    }

    #[test]
    fn test_write_csv() {
        let output_path = "./output_data/output.csv";
        let writer = get_file_writer(output_path);
        super::write_csv(
            writer,
            "./test_data/schema_simple.yaml".to_string(),
            100,
            ',' as u8,
        ).unwrap();

        let read_file = File::open(output_path).expect("Output File Path not found");
        let reader = BufReader::new(read_file);
        assert_eq!(reader.lines().count(), 100);
    }

    #[test]
    fn test_write_avro() {
        let output_path = "./output_data/output.avro";
        let writer = get_file_writer(output_path);
        super::write_avro(writer, "./test_data/schema_simple.yaml".to_string(), 100).unwrap();

        let read_file = File::open(output_path).expect("Output File Path not found");
        let mut reader = BufReader::new(read_file);

        let write_avro_schema = r#"{"name":"person_schema","type":"record","fields":[{"name":"id","type":"int"},{"name":"name","type":"string"},{"name":"age","type":"int"},{"name":"adult","type":"boolean"},{"name":"gender","type":"string"}]}"#;

        let schema = avro_rs::Schema::parse_str(write_avro_schema).unwrap();
        let reader = avro_rs::Reader::with_schema(&schema, &mut reader).unwrap();
        assert_eq!(reader.count(), 100);
    }

    #[test]
    fn test_write_json() {
        let output_path = "./output_data/output.json";
        let writer = get_file_writer(output_path);

        super::write_json(writer, "./test_data/schema_simple.yaml".to_string(), 100).unwrap();

        let read_file = File::open(output_path).expect("Output File Path not found");
        let reader = BufReader::new(read_file);

        let value: Value = serde_json::from_reader(reader).unwrap();
        match value {
            Value::Array(vec) => assert_eq!(vec.len(), 100),
            _ => panic!("Unable to parse JSON as an array")
        }
    }
}
