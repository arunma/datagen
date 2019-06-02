use std::{io, result};

pub type DataGenResult<T> = result::Result<T, DataGenError>;

#[derive(Fail, Debug)]
pub enum DataGenError {
    #[fail(display = "File IO Error")]
    FileIO(#[cause] io::Error),
    #[fail(display = "{}", message)]
    WeirdCase { message: String },
}