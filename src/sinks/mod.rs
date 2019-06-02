use std::io::Error;
use crate::DValue;

pub mod avro;
pub mod csv;

pub trait Sink {
    fn write(&mut self, value: DValue) -> Result<(), Error>;
}
