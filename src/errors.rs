use std::{io, result};

pub type DataGenResult<T> = result::Result<T, DataGenError>;

#[derive(Fail, Debug)]
pub enum DataGenError {
    #[fail(display = "File IO Error")]
    FileIO(#[cause] io::Error),
    #[fail(display = "CSV error")]
    Csv(#[cause] csv::Error),
    #[fail(display = "Avro error")]
    Avro(#[cause] failure::Error),
    #[fail(display = "SerDe error")]
    SerDe(#[cause] serde_yaml::Error),
    #[fail(display = "{}", message)]
    WeirdCase { message: String },
}

impl From<csv::Error> for DataGenError {
    fn from(error: csv::Error) -> Self {
        DataGenError::Csv(error)
    }
}

impl From<serde_yaml::Error> for DataGenError {
    fn from(error: serde_yaml::Error) -> Self {
        DataGenError::SerDe(error)
    }
}

impl From<failure::Error> for DataGenError {
    fn from(error: failure::Error) -> Self {
        DataGenError::Avro(error)
    }
}
