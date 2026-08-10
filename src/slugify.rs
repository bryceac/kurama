use tera::{ Filter, Kwargs, State, TeraResult };
use rslug::Slugifier;

pub struct Slugify {}

impl Filter<String, TeraResult<String>> for Slugify {
    fn call(&self, value: String, _kwargs: Kwargs, _state: &State) -> TeraResult<String> {
        let Slugify_with_underscore = Slugifier::new()
        .separator("_");

        Ok(Slugify_with_underscore.slugify(&value))
    }
}