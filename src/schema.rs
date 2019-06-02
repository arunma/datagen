use serde::{Deserialize, Serialize};
use serde_yaml::Error;
use unindent::unindent;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Schema {
    name: String,
    dataset: DataSet,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DataSet {
    name: String,
    columns: Vec<Column>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Column {
    name: String,
    not_null: Option<bool>,
    dtype: String,
}

impl Schema {
    fn from(s: &str) -> Schema {
        return serde_yaml::from_str(s).expect("Unable to parse the input YAML. Please check the syntax of the file");
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
            - {name: id, not_null: false, dtype: integer}
            - {name: name, dtype: string}
            - {name: age, dtype: smallint}
            - {name: adult, default: 'false', dtype: boolean}
            - {name: gender, dtype: string}
",
        );

        let schema = Schema::from(&yaml);
        assert_eq!(format!("{:?}", schema), r#"Schema { name: "person_schema", dataset: DataSet { name: "person_table", columns: [Column { name: "id", not_null: Some(false), dtype: "integer" }, Column { name: "name", not_null: None, dtype: "string" }, Column { name: "age", not_null: None, dtype: "smallint" }, Column { name: "adult", not_null: None, dtype: "boolean" }, Column { name: "gender", not_null: None, dtype: "string" }] } }"#);
    }
}