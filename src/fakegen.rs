use crate::schema::Schema;
use crate::{DValue, DType};

pub fn gen_record_for_schema(schema: Schema) -> DValue {
    DValue::Record(
        schema
            .dataset
            .columns
            .into_iter()
            .map(|col| (col.name, generate_fake_data(col.dtype)))
            .collect(),
    )
}

fn random_gender(rnd: i8) -> String {
    //TODO - Consider full form
    match rnd {
        0 => "M",
        1 => "F",
        _ => "O",
    }
    .to_string()
}

pub fn generate_fake_data(dtype: DType) -> DValue {
    match dtype {
        DType::Boolean => DValue::Boolean(fake!(Boolean.boolean)),
        DType::Int => DValue::Int(dummy!(i32)),
        DType::Long => DValue::Long(dummy!(i64)),
        DType::Float => DValue::Float(dummy!(f32)),
        DType::Double => DValue::Double(dummy!(f64)),
        DType::Bytes => DValue::Bytes((fake!(Lorem.sentence(4, 10))).as_bytes().to_vec()),
        DType::String => DValue::Str((fake!(Lorem.word)).to_string()),

        //Special types
        DType::Age => DValue::Int(fake!(Number.between(1, 100))),
        DType::Gender => DValue::Str(random_gender(fake!(Number.between(0, 2)))),
    }
}

#[cfg(test)]
mod tests {
    use crate::fakegen::gen_record_for_schema;
    use crate::schema::Schema;

    #[test]
    fn generate_record_from_schema() {
        let schema = Schema::from_path("./test_data/schema_simple.yaml".to_string()).unwrap();
        pretty_assertions::assert_eq!(format!("{:?}", schema), r#"Schema { name: "person_schema", dataset: DataSet { name: "person_table", columns: [Column { name: "id", not_null: Some(false), dtype: Int }, Column { name: "name", not_null: None, dtype: String }, Column { name: "age", not_null: None, dtype: Age }, Column { name: "adult", not_null: None, dtype: Boolean }, Column { name: "gender", not_null: None, dtype: Gender }] } }"#);
        let record = gen_record_for_schema(schema);
        assert!(bincode::serialize(&record).unwrap().len() > 0);
    }
}
