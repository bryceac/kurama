mod configuration;
mod link;
mod section;
mod navigation_item;

use crate::{ configuration::Configuration, link::Link, section::Section, navigation_item::NavigationItem };
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

    if let Some(sections) = menu_from::<Section>("links.json") {
        context.insert("sections", &sections);
    } else if let Some(links) = menu_from::<Link>("links.json") {
        context.insert("links", &links);
    }

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

fn menu_from<T: NavigationItem>(f: &str) -> Option<Vec<T>> {
    match T::from_file(f) {
        Ok(items) => Some(items),
        _ => None
    }
}

fn render_page(configuration: &Configuration, page: &str) -> String {
    let mut context = Context::new();
    context.insert("site", configuration);
}