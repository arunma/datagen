extern crate serde;
extern crate serde_yaml;
extern crate unindent;
#[macro_use]
extern crate fake;
#[macro_use]
extern crate failure;
extern crate csv;

pub mod errors;
pub mod fakegen;
pub mod options;
pub mod schema;
pub mod sinks;

use serde::{Deserialize, Serialize};
use crate::sinks::{Sink, csv_sink};
use crate::schema::Schema;
use crate::errors::DataGenResult;
use std::io;

///
/// This program just delegates all the fake data generation work to the wonderful fake-rs library
///
//TODO Need to consider Enum, Union, Fixed, Date, Timestamp and other logical types of Avro too.
#[derive(Debug, PartialEq, Serialize)]
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
    Long,
    Float,
    Double,
    Bytes,
    String,
    Age,
    //TODO - For now, let's stick to basic types
    //    Date, Array, Map, Nullable (union/null), Record,
}

pub fn write_fake_records <W:io::Write> (writer: W, schema_path: String, num_records: i64) -> DataGenResult<()> {
    let schema = Schema::from_path(schema_path)?;
    let mut sink = csv_sink::sink(schema.clone(), writer)?;
    let record = fakegen::gen_record_for_schema(schema.clone());
    sink.write(record)
}