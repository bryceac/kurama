use std::{fs::File, io::{ Write, Error } };

pub trait Save {
    fn save(&self, path &str) -> Result<(), Error>;
}

impl Save for Stringb{
    fn save(&self, path: &str) -> Res<(), Error> {

    }
}