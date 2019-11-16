use std::io::Result;
use std::io::Write;

use serde_json;

use crate::DValue;
use crate::errors::{DataGenError, DataGenResult};
use crate::errors::DataGenError::*;
use crate::schema::Schema;
use crate::sinks::Sink;

pub struct JsonSink<W: Write>(Schema, W, bool);
static NEW_LINE: &str ="\n";

pub fn sink<W: Write>(schema: Schema, w: W, first_record: bool) -> DataGenResult<JsonSink<W>> {
    Ok(JsonSink(
        schema,
        w,
        first_record,
    ))
}

fn wrap_as_datagen_error<T>(result: Result<T>) -> DataGenResult<()> {
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(DataGenError::FileIO(err))
    }
}

impl<W: Write> JsonSink<W> {
    pub fn continue_object(&mut self) -> DataGenResult<()> {
        wrap_as_datagen_error(self.1.write(",".as_bytes()))
    }
    pub fn start_array(&mut self) -> DataGenResult<()> {
        wrap_as_datagen_error(self.1.write(format!("{}{}", "[", NEW_LINE) .as_bytes()))
    }
    pub fn end_array(&mut self) -> DataGenResult<()> {
        wrap_as_datagen_error(self.1.write("]".as_bytes()))
    }
}

impl<W: Write> Sink for JsonSink<W> {
    fn write(&mut self, value: DValue) -> DataGenResult<()> {
        match value {
            DValue::Record(vec) => {
                if !self.2 { self.continue_object(); } else { self.2 = false; }

                let json_rec = format!("{}{}", dvalue_to_json(vec)?, NEW_LINE);
                wrap_as_datagen_error(self.1.write(json_rec.as_bytes()));
                Ok(())
            }
            _ => Err(WeirdCase { message: format!("The 'value' parameters received at the JsonSink is not a Record. Value found was : {:?}", value) })
        }
    }
}

//TODO - Do a Result wrapper
fn dvalue_to_json(vec: Vec<(String, DValue)>) -> serde_json::Result<String> {
    use serde_json::*;

    let object: Map<String, Value> = vec.into_iter().map(|(key, value)| {
        match value {
            DValue::Boolean(val) => (key, Value::Bool(val)),
            DValue::Int(val) => (key, json!(val)),
            DValue::Long(val) => (key, json!(val)),
            DValue::Float(val) => (key, json!(val)),
            DValue::Double(val) => (key, json!(val)),
            DValue::Bytes(_val) => unimplemented!(),
            DValue::Date(val) => (key, Value::String(val)),
            DValue::DateTime(val) => (key, Value::String(val)),
            DValue::Str(val) => (key, Value::String(val)),
            DValue::Null => (key, Value::Null),
            _ => unimplemented!()
        }
    }).collect();
    serde_json::to_string(&object)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_json_record_from_schema() {
        let record1 = DValue::Record(vec![
            ("id".to_string(), DValue::Int(1)),
            ("name".to_string(), DValue::Str("Jason".to_string())),
            ("age".to_string(), DValue::Int(90)),
            ("adult".to_string(), DValue::Boolean(false)),
            ("gender".to_string(), DValue::Str("Male".to_string())),
        ]);

        let record2 = DValue::Record(vec![
            ("id".to_string(), DValue::Int(1)),
            ("name".to_string(), DValue::Str("Daisy".to_string())),
            ("age".to_string(), DValue::Int(100)),
            ("adult".to_string(), DValue::Boolean(true)),
            ("gender".to_string(), DValue::Str("Female".to_string())),
        ]);

        let schema = Schema::from_path("./test_data/schema_simple.yaml".to_string()).unwrap();

        let mut vec: Vec<u8> = Vec::new();
        {
            let mut sink = sink(schema, &mut vec, true).unwrap();
            sink.write(record1).unwrap();
            sink.write(record2).unwrap();
        }

        //println!("Json string    {:?}", str::from_utf8(&vec).unwrap());
        pretty_assertions::assert_eq!(
            "{\"adult\":false,\"age\":90,\"gender\":\"Male\",\"id\":1,\"name\":\"Jason\"}\n,{\"adult\":true,\"age\":100,\"gender\":\"Female\",\"id\":1,\"name\":\"Daisy\"}\n".as_bytes(),
            vec.as_slice()
        );
    }
}

