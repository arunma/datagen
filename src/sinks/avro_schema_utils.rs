use serde_json;

use crate::DType;
use crate::schema::Schema;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
struct Record {
    #[serde(rename(serialize = "type"))]
    dtype: String,
    name: String,
    fields: Vec<Field>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
struct Field {
    name: String,
    #[serde(rename(serialize = "type"))]
    dtype: String,
    default: String,
}

pub fn to_avro_schema(schema: Schema) -> Result<avro_rs::Schema, failure::Error> {
    let record = Record {
        name: schema.name,
        dtype: "record".to_string(),
        fields: schema
            .dataset
            .columns
            .into_iter()
            .map(|col| {
                Field {
                    name: col.name,
                    dtype: dtype_to_avro_type(col.dtype),
                    default: "".to_string(),
                    //default:col.default,
                }
            })
            .collect(),
    };

    let schema_string = serde_json::to_string(&record)?;
    avro_rs::Schema::parse_str(&schema_string)
}

#[rustfmt::skip]
fn dtype_to_avro_type(dtype: DType) -> String {
    use DType::*;
    match dtype {
        Age         =>      "int".to_string(),
        Gender      =>      "string".to_string(),
        Id          =>      "string".to_string(),
        Name        =>      "string".to_string(),
        City        =>      "string".to_string(),
        Phone       =>      "string".to_string(),
        Date        =>      "string".to_string(),
        Latitude    =>      "string".to_string(),
        Longitude   =>      "string".to_string(),
        _           =>      format!("{:?}", dtype).to_lowercase(),
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::Schema;

    #[test]
    fn test_inner_schema_to_avro_schema() {
        let expected_avro_schema = r#"{"name":"person_schema","type":"record","fields":[{"name":"id","type":"int"},{"name":"name","type":"string"},{"name":"age","type":"int"},{"name":"adult","type":"boolean"},{"name":"gender","type":"string"}]}"#;
        let inner_schema = Schema::from_path("./test_data/schema_simple.yaml".to_string()).unwrap();

        let avro_schema = super::to_avro_schema(inner_schema).unwrap();

        pretty_assertions::assert_eq!(expected_avro_schema, avro_schema.canonical_form());
    }
}
