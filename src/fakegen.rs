#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};

use crate::{DType, DValue};
use crate::fakegen::DValue::Record;
use crate::schema::Schema;

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

mod tests {
    use crate::schema::Schema;
    use crate::fakegen::gen_record_for_schema;

    #[test]
    fn generate_record_from_schema() {
        let schema = Schema::from_path("./test_data/schema_simple.yaml".to_string()).unwrap();
        pretty_assertions::assert_eq!(format!("{:?}", schema), r#"Schema { name: "person_schema", dataset: DataSet { name: "person_table", columns: [Column { name: "id", not_null: Some(false), dtype: Int }, Column { name: "name", not_null: None, dtype: String }, Column { name: "age", not_null: None, dtype: Int }, Column { name: "adult", not_null: None, dtype: Boolean }, Column { name: "gender", not_null: None, dtype: String }] } }"#);
        let record = gen_record_for_schema(schema);
        assert!(bincode::serialize(&record).unwrap().len() > 0);
    }
}
