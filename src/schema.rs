use std::fs::File;
use serde_yaml::Error;
use crate::DType;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Schema {
    pub name: String,
    pub dataset: DataSet,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DataSet {
    pub name: String,
    pub columns: Vec<Column>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Column {
    pub name: String,
    pub not_null: Option<bool>,
    pub dtype: DType,
    pub one_of: Option<Vec<String>>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub mean: Option<f64>,
    pub std: Option<f64>,
    pub format: Option<String>

}

impl Schema {
    pub fn from(s: &str) -> Result<Schema, Error> {
        serde_yaml::from_str(s)
    }

    pub fn from_path(path: String) -> Result<Schema, Error> {
        let file = File::open(path).expect("Unable to open the file");
        serde_yaml::from_reader(file)
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::Schema;

    #[test]
    fn derive_struct_from_yaml() {
        let yaml = r#"name: person_schema
dataset:
    name: person_table
    columns:
        -
            name: id
            not_null: false
            dtype: int
        -
            name: name
            dtype: string
        -
            name: age
            dtype: int
        -
            name: adult
            default: 'false'
            dtype: boolean
        -
            name: gender
            dtype: string
"#;

        let schema = Schema::from(&yaml);
        pretty_assertions::assert_eq!(format ! ("{:?}", schema.unwrap()), r#"Schema { name: "person_schema", dataset: DataSet { name: "person_table", columns: [Column { name: "id", not_null: Some(false), dtype: Int, one_of: None, min: None, max: None, mean: None, std: None, format: None }, Column { name: "name", not_null: None, dtype: String, one_of: None, min: None, max: None, mean: None, std: None, format: None }, Column { name: "age", not_null: None, dtype: Int, one_of: None, min: None, max: None, mean: None, std: None, format: None }, Column { name: "adult", not_null: None, dtype: Boolean, one_of: None, min: None, max: None, mean: None, std: None, format: None }, Column { name: "gender", not_null: None, dtype: String, one_of: None, min: None, max: None, mean: None, std: None, format: None }] } }"#);
    }

    #[test]
    fn derive_struct_from_file() {
        let file_path = "./test_data/schema_simple.yaml".to_string();
        let schema = Schema::from_path(file_path);
        pretty_assertions::assert_eq!(format!("{:?}", schema.unwrap()), r#"Schema { name: "person_schema", dataset: DataSet { name: "person_table", columns: [Column { name: "id", not_null: Some(false), dtype: Int, one_of: None, min: None, max: None, mean: None, std: None, format: None }, Column { name: "name", not_null: None, dtype: Name, one_of: None, min: None, max: None, mean: None, std: None, format: None }, Column { name: "age", not_null: None, dtype: Age, one_of: None, min: None, max: None, mean: None, std: None, format: None }, Column { name: "adult", not_null: None, dtype: Boolean, one_of: None, min: None, max: None, mean: None, std: None, format: None }, Column { name: "gender", not_null: None, dtype: String, one_of: Some(["M", "F"]), min: None, max: None, mean: None, std: None, format: None }, Column { name: "date", not_null: None, dtype: Date, one_of: None, min: Some("01/01/2014"), max: Some("03/01/2014"), mean: None, std: None, format: Some("%d/%m/%Y") }] } }"#);
    }
}
