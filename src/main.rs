mod configuration;
mod link;

use crate::configuration::Configuration;
#[macro_use] extern crate lazy_static;
use tera::{ Context, Tera  };
use std::{ error::Error };


lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(error) => {
                println!("Parsing error(s): {}", error);
                ::std::process::exit(1);
            }
        };
        tera
    };
}

fn main() {
    let site_configuration = Configuration::from_file("config.json").expect("Could not load configuration");
    let mut context = Context::new();
    context.insert("site", &site_configuration);

    match TEMPLATES.render("test.html", &context) {
        Ok(page) => println!("{:?}", page),
        Err(errors) => {
            println!("{}", errors);

            let mut cause = errors.source();

            while let Some(error) = cause {
                println!("{}", error);
                cause = error.source();
            }
        }
    }
}


