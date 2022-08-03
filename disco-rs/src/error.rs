use std::{
    error::Error as StdError,
    fmt::{self, Display, Error as FormatError},
    io::Error as IoError,
    num::ParseIntError,
};

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Format(FormatError),
    Io(IoError),
    ParseInt(ParseIntError),
    
}