use fake::Faker;
use fake::faker::address::en::*;
use fake::faker::chrono::en::*;
use fake::faker::lorem::en::*;
use fake::faker::name::en::*;
use fake::faker::phone_number::en::*;
use fake::locales::EN;
use rand::Rng;
use rand_distr::{Distribution, Normal, Uniform};
use chrono::prelude::*;

use crate::{DType, DValue};
use crate::fake::Fake;
use crate::schema::{Column, Schema};

// DateTime specifiers https://docs.rs/chrono/0.4.9/chrono/format/strftime/index.html#specifiers

pub fn gen_record_for_schema(schema: Schema) -> DValue {
    DValue::Record(
        schema
            .dataset
            .columns
            .into_iter()
            .map(|col| (col.name.clone(), generate_fake_data(col)))
            .collect(),
    )
}

pub fn generate_fake_data(column: Column) -> DValue {
    match column.dtype {
        DType::Boolean => generate_value(column),
        DType::Int => generate_value(column),
        DType::Long => generate_value(column),
        DType::Float => generate_value(column),
        DType::Double => generate_value(column),
        DType::String => generate_value(column),
        DType::Date => generate_value(column),
        DType::DateTime => generate_value(column),

        //Special types
        DType::Age => DValue::Int((1..100).fake()),
        DType::Name => DValue::Str(Name().fake()),
        DType::City => DValue::Str(CityName().fake()),
        DType::Phone => DValue::Str(PhoneNumber().fake()),

        DType::Latitude => DValue::Str(Latitude().fake()),
        DType::Longitude => DValue::Str(Longitude().fake()),
    }
}


fn generate_value(column: Column) -> DValue {
    let mut rng = rand::thread_rng();
    //println!("{:?} column : ", column);
    if column.one_of.is_some() {
        let str_val = {
            let one_off = column.one_of.unwrap();
            let count = one_off.len();
            one_off[rng.gen_range(0, count)].clone()
        };

        //println!("Attempting to cast one_of value {} to datatype {:?}", str_val, column.dtype);

        //TODO The following ought to be closed in a Result
        match column.dtype {
            DType::Boolean => DValue::Boolean(str_val.parse::<bool>().unwrap()),
            DType::Int => DValue::Int(str_val.parse::<i32>().unwrap()),
            DType::Long => DValue::Long(str_val.parse::<i64>().unwrap()),
            DType::Float => DValue::Float(str_val.parse::<f32>().unwrap()),
            DType::Double => DValue::Double(str_val.parse::<f64>().unwrap()),
            DType::DateTime => DValue::Date(str_val),
            DType::String => DValue::Str(str_val),
            _ => panic!("ERROR: Unable to parse provided one_off value {} to the expected data type {:?}", str_val, column.dtype)
        }
    } else if column.min.is_some() && column.max.is_some() &&
        (column.dtype == DType::Int || column.dtype == DType::Long || column.dtype == DType::Float || column.dtype == DType::Double) {
        let from: f64 = column.min.unwrap().parse().unwrap();
        let to: f64 = column.max.unwrap().parse().unwrap();
        //println!("{:?} column : ", column);
        match column.dtype {
            DType::Int => DValue::Int(rng.gen_range(from, to) as i32),
            DType::Long => DValue::Long(rng.gen_range(from, to) as i64),
            DType::Float => DValue::Float(rng.gen_range(from, to) as f32),
            DType::Double => DValue::Double(rng.gen_range(from, to)),
            _ => panic!("ERROR: Unable to apply range_from and range_to option to the Datatype {:?}", column.dtype)
        }
    } else if column.mean.is_some() && column.std.is_some() {
        let distribution = Normal::new(column.mean.unwrap(), column.std.unwrap()).unwrap();
        match column.dtype {
            DType::Int => DValue::Int(distribution.sample(&mut rng) as i32),
            DType::Long => DValue::Long(distribution.sample(&mut rng) as i64),
            DType::Float => DValue::Float(distribution.sample(&mut rng) as f32),
            DType::Double => DValue::Double(distribution.sample(&mut rng)),
            _ => panic!("ERROR: Unable to apply mean and std options to the Datatype {:?}", column.dtype)
        }
    } else {
        match column.dtype {
            DType::Boolean => DValue::Boolean(Faker.fake::<bool>()),
            DType::Int => DValue::Int(Faker.fake::<i32>()),
            DType::Long => DValue::Long(Faker.fake::<i64>()),
            DType::Float => DValue::Float(Faker.fake::<f32>()),
            DType::Double => DValue::Double(Faker.fake::<f64>()),
            DType::Date => {
                if column.format.is_none() {
                    panic!("Format is a mandatory parameter for date datatype");
                }
                let date_fmt = column.format.unwrap();
                let rnd_date: chrono::DateTime<Utc> = {
                    if column.min.is_some() && column.max.is_some() {
                        let start_dt_naive = NaiveDate::parse_from_str(column.min.unwrap().as_str(), date_fmt.as_str()).unwrap();
                        let start_dt: chrono::DateTime<Utc> = DateTime::from_utc(NaiveDateTime::new(start_dt_naive, chrono::NaiveTime::from_hms(0, 0, 0)), Utc);
                        let end_dt_naive = NaiveDate::parse_from_str(column.max.unwrap().as_str(), date_fmt.as_str()).unwrap();
                        let end_dt: chrono::DateTime<Utc> = DateTime::from_utc(NaiveDateTime::new(end_dt_naive, chrono::NaiveTime::from_hms(0, 0, 0)), Utc);
                        DateTimeBetween(start_dt, end_dt).fake()
                    } else {
                        DateTime().fake()
                    }
                };
                DValue::Date(rnd_date.format(date_fmt.as_str()).to_string())
            }
            DType::DateTime => {
                if column.format.is_none() {
                    panic!("Format is a mandatory parameter for date datatype");
                }
                let date_fmt = column.format.unwrap();
                let rnd_date: chrono::DateTime<Utc> = {
                    if column.min.is_some() && column.max.is_some() {
                        let start_dt = Utc.datetime_from_str(column.min.unwrap().as_str(), date_fmt.as_str()).unwrap();
                        let end_dt = Utc.datetime_from_str(column.max.unwrap().as_str(), date_fmt.as_str()).unwrap();
                        DateTimeBetween(start_dt, end_dt).fake()
                    } else {
                        DateTime().fake()
                    }
                };
                DValue::DateTime(rnd_date.format(date_fmt.as_str()).to_string())
            }
            DType::String => DValue::Str(Word().fake()),
            _ => panic!("Error : The current version does not support one_of for this datatype {:?}", column.dtype)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fakegen::gen_record_for_schema;
    use crate::schema::Schema;

    #[test]
    fn generate_record_from_schema() {
        let schema = Schema::from_path("./test_data/schema_simple.yaml".to_string()).unwrap();
        //pretty_assertions::assert_eq!(format!("{:?}", schema), r#"Schema { name: "person_schema", dataset: DataSet { name: "person_table", columns: [Column { name: "id", not_null: Some(false), dtype: Int }, Column { name: "name", not_null: None, dtype: Name }, Column { name: "age", not_null: None, dtype: Age }, Column { name: "adult", not_null: None, dtype: Boolean }, Column { name: "gender", not_null: None, dtype: Gender }] } }"#);
        let record = gen_record_for_schema(schema);
        println!("Record {:?}", record.clone());
        assert!(bincode::serialize(&record).unwrap().len() > 0);
    }
}
