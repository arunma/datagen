use crate::errors::DataGenError::WeirdCase;
use crate::errors::DataGenResult;
use crate::sinks::Sink;
use crate::DValue;
use avro_rs;
use avro_rs::types::Record;
use avro_rs::{Codec, Writer};
use std::io;
use std::io::Write;

pub struct AvroSink<'a, W: io::Write>(avro_rs::Writer<'a, W>);

pub fn sink<W: Write>(
    avro_schema: &avro_rs::Schema,
    w: W,
    codec: Codec,
) -> Result<AvroSink<W>, failure::Error> {
    let writer = Writer::with_codec(&avro_schema, w, codec);
    Ok(AvroSink(writer))
}

impl<'a, W: Write> Sink for AvroSink<'a, W> {
    fn write(&mut self, value: DValue) -> DataGenResult<()> {
        match value {
            DValue::Record(vec) => {
                let key_value: Vec<(String, avro_rs::types::Value)> = vec.into_iter().map(|(key, value)| (key, dvalue_to_avro(value))).collect();
                let mut record = Record::new(self.0.schema()).unwrap();
                key_value.into_iter().for_each(|(key, value)| { record.put(&key, value) });
                self.0.append(record)?;
                self.0.flush().unwrap();
                Ok(())
            }
            _ => Err(WeirdCase { message: format!("The 'value' parameters received at the AvroSink is not a Record. Value found was : {:?}", value) })
        }
    }
}

#[rustfmt::skip]
fn dvalue_to_avro(value: DValue) -> avro_rs::types::Value {
    use avro_rs::types::*;
    match value {
        DValue::Boolean(val) => Value::Boolean(val),
        DValue::Int(val) => Value::Int(val),
        DValue::Long(val) => Value::Long(val),
        DValue::Float(val) => Value::Float(val),
        DValue::Double(val) => Value::Double(val),
        DValue::Bytes(val) => Value::Bytes(val),
        DValue::Str(val) => Value::String(val),
        DValue::Null => Value::Null,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::Schema;
    use crate::sinks::avro_schema_utils::to_avro_schema;
    use crate::sinks::avro_sink::sink;
    use crate::DValue;
    use avro_rs::Codec;

    #[test]
    fn generate_avro_record_from_schema() {
        let record = DValue::Record(vec![
            ("id".to_string(), DValue::Int(1)),
            ("name".to_string(), DValue::Str("Jason".to_string())),
            ("age".to_string(), DValue::Int(90)),
            ("adult".to_string(), DValue::Boolean(true)),
            ("gender".to_string(), DValue::Str("Male".to_string())),
        ]);

        let schema = Schema::from_path("./test_data/schema_simple.yaml".to_string()).unwrap();

        let avro_schema: avro_rs::Schema = to_avro_schema(schema.clone()).unwrap();
        let mut sink = sink(&avro_schema, Vec::new(), Codec::Deflate).unwrap();
        sink.write(record).unwrap();

        let encoded = sink.0.into_inner();
        let reader = avro_rs::Reader::with_schema(&avro_schema, &encoded[..]).unwrap();

        let mut data_count: i32 = 0;
        for _ in reader {
            data_count += 1;
        }

        assert_eq!(data_count, 1)
    }
}
