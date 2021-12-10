use std::{convert::From, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Bincode(bincode::Error),
    Protocol(&'static str),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<bincode::Error> for Error {
    fn from(e: bincode::Error) -> Self {
        Error::Bincode(e)
    }
}
