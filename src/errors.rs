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

    #[fail(display = "Json error")]
    Json(#[cause] serde_json::Error),

    #[fail(display = "SerDe error")]
    SerDe(#[cause] serde_yaml::Error),

    #[fail(display = "{}", message)]
    WeirdCase { message: String },
}

macro_rules! from_error {
    ($from:ty, $to:path) => {
        impl From<$from> for DataGenError {
            fn from(error: $from) -> Self {
                $to(error)
            }
        }
    };
}

from_error!(csv::Error, DataGenError::Csv);
from_error!(serde_yaml::Error, DataGenError::SerDe);
from_error!(failure::Error, DataGenError::Avro);
from_error!(serde_json::Error, DataGenError::Json);
from_error!(io::Error, DataGenError::FileIO);
