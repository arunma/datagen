use crate::errors::DataGenResult;
use crate::DValue;

pub mod avro_schema_utils;
pub mod csv_sink;
pub mod avro_sink;
pub mod json_sink;

pub trait Sink {
    fn write(&mut self, value: DValue) -> DataGenResult<()>;
}
