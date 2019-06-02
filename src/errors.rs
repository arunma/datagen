use std::{io, result};

pub type DataGenResult<T> = result::Result<T, DataGenError>;

#[derive(Fail, Debug)]
pub enum DataGenError {
    #[fail(display = "File IO Error")]
    FileIO(#[cause] io::Error),
    #[fail(display = "CSV error")]
    Csv(#[cause] csv::Error),
    #[fail(display = "{}", message)]
    WeirdCase { message: String },
}


impl From<csv::Error> for DataGenError {
    fn from(error: csv::Error) -> Self {
        DataGenError::Csv(error)
    }
}