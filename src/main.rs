mod configuration;
mod link;
mod section;

use crate::{ configuration::Configuration, link::Link, section::Section };
use std:: { error::Error };
#[macro_use] extern crate lazy_static;
use tera::{ Context, Tera  };


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

fn sections_from(f: &str) -> Option<Vec<Section>> {
    match Section::from_file(f) {
        Ok(sections) => Some(sections),
        _ => None
    }
}

fn links_from(f: &str) -> Option<Vec<Link>> {
    match Link::from_file(f) {
        Ok(links) => Some(links),
        _ => None
    }
}