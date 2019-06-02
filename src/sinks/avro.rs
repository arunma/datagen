use crate::errors::DataGenResult;
use crate::schema::Schema;
use crate::sinks::Sink;
use std::io;
use std::io::Error;

pub struct AvroSink<W: io::Write>(Schema, W);

pub fn sink<W: io::Write>(schema: Schema, w: W) -> DataGenResult<AvroSink<W>> {
    Ok(AvroSink(schema, w))
}

/*
impl<W> Sink for AvroSink<W> where W: io::Write {
    fn write(&mut self) -> Result<(), Error> {
        //unimplemented!()
        Ok(())
    }
}*/
