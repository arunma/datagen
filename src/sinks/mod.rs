use std::io::Error;
use crate::DValue;
use crate::errors::DataGenResult;

pub mod avro;
pub mod csv;

pub trait Sink {
    fn write(&mut self, value: DValue) -> DataGenResult<()>;
}
