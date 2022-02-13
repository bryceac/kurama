mod configuration;

use crate::configuration::Configuration;
#[macro_use] extern crate lazy_static;
use tera::{ Context, Tera };


lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(error) => {
                println!("Parsing error(s): {}", error);
            }
        };
        tera
    };
}

fn main() {
    let site_configuration = Configuration::from_file("config.json").expect("Could not load configuration");
    let mut context = Context::new();
    context.insert("site", &site_configuration);
}


