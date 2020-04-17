use crate::error::CleanError;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct CargoDir {
    dir: PathBuf,
}

impl CargoDir {
    pub fn clean(&self) -> Result<(), CleanError> {
        todo!()
    }
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }
}
