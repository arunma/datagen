use std::collections::HashMap;

use fake::faker::*;
use fake::faker::Lorem;
use serde::Serialize;
use unindent::unindent;

use crate::{DValue, DType};
use crate::fakegen::DValue::{Bytes, Double, Float, Int, Long, Record};
use crate::schema::{Column, Schema};

pub fn gen_record_for_schema(schema: Schema) -> DValue {
    Record(
        schema
            .dataset
            .columns
            .into_iter()
            .map(|col| (col.name, generate_fake_data(col.dtype)))
            .collect(),
    )
}

pub fn generate_fake_data(dtype: DType) -> DValue {
    use DValue::*;
    match dtype {
        DType::Boolean => Boolean(fake!(Boolean.boolean)),
        DType::Int => Int(dummy!(i32)),
        DType::Long => Long(dummy!(i64)),
        DType::Float => Float(dummy!(f32)),
        DType::Double => Double(dummy!(f64)),
        DType::Bytes => Bytes((fake!(Lorem.sentence(4, 10))).as_bytes().to_vec()),
        DType::String => DValue::Str((fake!(Lorem.word)).to_string()),

        //Special types
        DType::Age => Int(fake!(Number.between(1, 100))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_record_from_schema() {
        let yaml = unindent(
            "schema:
    name: person_schema
    dataset:
        name: person_table
        columns:
            - {name: id, not_null: false, dtype: int}
            - {name: name, dtype: string}
            - {name: age, dtype: int}
            - {name: adult, default: 'false', dtype: boolean}
            - {name: gender, dtype: string}
",
        );

        let schema = Schema::from(&yaml);
        assert_eq!(format!("{:?}", schema), r#"Schema { name: "person_schema", dataset: DataSet { name: "person_table", columns: [Column { name: "id", not_null: Some(false), dtype: Int }, Column { name: "name", not_null: None, dtype: String }, Column { name: "age", not_null: None, dtype: Int }, Column { name: "adult", not_null: None, dtype: Boolean }, Column { name: "gender", not_null: None, dtype: String }] } }"#);
        let record = gen_record_for_schema(schema);
        assert!(bincode::serialize(&record).unwrap().len() > 0);
    }
}
