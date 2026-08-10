use tera::{ Error, Filter, Kwargs, State, TeraResult };
use rslug::Slugifier;

pub struct Slugify {
    text: String
}

impl Slugify {
    pub fn new<S: Into<String>>()
}