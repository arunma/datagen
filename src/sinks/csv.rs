use crate::errors::*;
use crate::schema::Schema;
use crate::sinks::Sink;
use std::io;
use std::io::{Error, Write};
use crate::DValue;
use csv::Writer;
use crate::errors::DataGenError::WeirdCase;
use crate::errors::DataGenError::*;


pub struct CSVSink<W: Write> (Schema, csv::Writer<W>);

pub fn sink<W: Write>(schema: Schema, w: W) -> DataGenResult<CSVSink<W>> {
    Ok(CSVSink(schema, Writer::from_writer(w)))
}

impl<W: Write> Sink for CSVSink<W> {
    fn write(&mut self, value: DValue) -> Result<(), DataGenError> {
        match value {
            DValue::Record(vec) => {
                let rec: Vec<String> = vec.into_iter().map(|(key, value)| dvalue_to_csv(value)).collect();
                println!("Record from CSV Sink {:?}", &rec);
                self.1.write_record(rec)?;

                Ok(())
            }
            value => Err(WeirdCase { message: format!("The 'value' parameters received at the CSVSink is not a Record. Value found was : {:?}", value) })
        }
    }
}


fn dvalue_to_csv(value: DValue) -> String {
    use DValue::*;
    match value {
        Boolean(val) => format!("{}", val),
        Int(val) => format!("{}", val),
        Long(val) => format!("{}", val),
        Float(val) => format!("{}", val),
        Double(val) => format!("{}", val),
        Bytes(val) => format!("{:?}", val),
        Str(val) => format!("{}", val),
        x => format!("{:?}", x)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{Column, DataSet};
    use crate::DType;
    use std::fs::File;
    use std::io::BufWriter;

    #[test]
    fn generate_record_from_schema() {
        let record = DValue::Record(vec![
            ("id".to_string(), DValue::Int(1)),
            ("name".to_string(), DValue::Str("Jason".to_string())),
            ("age".to_string(), DValue::Int(90)),
            ("adult".to_string(), DValue::Boolean(true)),
            ("gender".to_string(), DValue::Str("Male".to_string()))
        ]);

        let schema = Schema {
            name: "person_schema".to_string(),
            dataset: DataSet {
                name: "person_table".to_string(),
                columns: vec![
                    Column { name: "id".to_string(), not_null: Some(false), dtype: DType::Int },
                    Column { name: "name".to_string(), not_null: None, dtype: DType::String },
                    Column { name: "age".to_string(), not_null: None, dtype: DType::Int },
                    Column { name: "adult".to_string(), not_null: None, dtype: DType::Boolean },
                    Column { name: "gender".to_string(), not_null: None, dtype: DType::String }
                ],
            },
        };

        println!("Record {:?}", record);

        let mut string = String::new();
        {
            let buffer = unsafe { BufWriter::new(string.as_mut_vec()) };
            let mut sink = sink(schema, buffer).unwrap();
            sink.write(record).unwrap()
        }

        assert_eq!("1,Jason,90,true,Male\n", string);
        //println!("Result Vector {:?}", &string);
    }
}

