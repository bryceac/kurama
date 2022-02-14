use std::{fs::File, io::{ Write, Error } };

pub trait Save {
    fn save(&self, path: &str) -> Result<(), Error>;
}

impl Save for String {
    fn save(&self, path: &str) -> Result<(), Error> {
        let mut output = File::create(path)?;

        match write!(output, "{}", self) {
            Ok(()) => Ok(()),
            Err(error) => Err(error)
        }
    }
}