use crate::errors::*;
use crate::schema::Schema;
use crate::sinks::Sink;
use std::io;
use std::io::{Error, Write};
use crate::DValue;

pub struct CSVSink<W: io::Write>(Schema, W);

pub fn sink<W: Write>(schema: Schema, w: W) -> DataGenResult<CSVSink<W>> {
    Ok(CSVSink(schema, w))
}

impl<W: Write> Sink for CSVSink<W> {
    fn write(&mut self, value: DValue) -> Result<(), Error> {
        //self.0 = schema
        //self.1 = writer
        //Iterate through the columns in the schema, generate values and use the writer to append/write to an output source
        Ok(()) //ERROR : Does not have a size known at compile-time
    }
}
