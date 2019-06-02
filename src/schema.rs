use serde::{Deserialize, Serialize};
use serde_yaml::Error;
use unindent::unindent;
use crate::DType;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    pub name: String,
    pub dataset: DataSet,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSet {
    pub name: String,
    pub columns: Vec<Column>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub not_null: Option<bool>,
    pub dtype: DType,
}

impl Schema {
    pub fn from(s: &str) -> Schema {
        return serde_yaml::from_str(s)
            .expect("Unable to parse the input YAML. Please check the syntax of the file");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_struct_from_yaml() {
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
    }
}
