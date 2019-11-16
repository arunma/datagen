use crate::errors::DataGenError::WeirdCase;
use crate::errors::*;
use crate::schema::Schema;
use crate::sinks::Sink;
use crate::DValue;
use std::io::Write;

pub struct CSVSink<W: Write>(Schema, csv::Writer<W>);

pub fn sink<W: Write>(schema: Schema, w: W, delimiter: u8) -> DataGenResult<CSVSink<W>> {
    Ok(CSVSink(
        schema,
        csv::WriterBuilder::new()
            .delimiter(delimiter)
            .quote_style(csv::QuoteStyle::NonNumeric)
            .from_writer(w),
    ))
}

impl<W: Write> Sink for CSVSink<W> {
    fn write(&mut self, value: DValue) -> Result<(), DataGenError> {
        match value {
            DValue::Record(vec) => {
                let rec: Vec<String> = vec.into_iter().map(|(_key, value)| dvalue_to_csv(value)).collect();
                self.1.write_record(rec)?;

                Ok(())
            }
            _ => Err(WeirdCase { message: format!("The 'value' parameters received at the CSVSink is not a Record. Value found was : {:?}", value) })
        }
    }
}

#[rustfmt::skip]
fn dvalue_to_csv(value: DValue) -> String {
    use DValue::*;
    match value {
        Boolean(val)   => val.to_string(),
        Int(val)        => val.to_string(),
        Long(val)       => val.to_string(),
        Float(val)      => val.to_string(),
        Double(val)     => val.to_string(),
        Bytes(val)  => format!("{:?}", val),
        Str(val)      => val.to_string(),
        Date(val)     => val.to_string(),
        DateTime(val) => val.to_string(),
        x            => format!("{:?}", x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_csv_record_from_schema() {
        let record = DValue::Record(vec![
            ("id".to_string(), DValue::Int(1)),
            ("name".to_string(), DValue::Str("Jason".to_string())),
            ("age".to_string(), DValue::Int(90)),
            ("adult".to_string(), DValue::Boolean(true)),
            ("gender".to_string(), DValue::Str("Male".to_string())),
        ]);

        let schema = Schema::from_path("./test_data/schema_simple.yaml".to_string()).unwrap();

        let mut vec: Vec<u8> = Vec::new();
        {
            let mut sink = sink(schema, &mut vec, ',' as u8).unwrap();
            sink.write(record).unwrap();
        }
        pretty_assertions::assert_eq!(
            "1,\"Jason\",90,\"true\",\"Male\"\n".as_bytes(),
            vec.as_slice()
        );
    }
}
