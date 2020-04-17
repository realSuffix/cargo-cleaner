use std::{error::Error, fmt};
use std::io::Error as IOError;
use cargo::util::errors::CargoResult;

#[derive(Debug)]
pub enum CleanError {
    IOError(std::io::Error),
    CargoError(cargo::util::errors::CargoResult<()>),
}

impl Error for CleanError {}

impl fmt::Display for CleanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

impl From<IOError> for CleanError {
    fn from(error: IOError) -> Self {
        Self::IOError(error)
    }
}

impl From<CargoResult<()>> for CleanError {
    fn from(error: CargoResult<()>) -> Self {
        Self::CargoError(error)
    }
}
