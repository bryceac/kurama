mod configuration;
mod link;
mod navigation_item;
mod metadata;
mod page;
mod section;


use crate::{ 
    configuration::Configuration, 
    link::Link, 
    section::Section, 
    navigation_item::NavigationItem,
    page::Page
};

#[macro_use] extern crate lazy_static;
use tera::{ Context, Tera  };
use pulldown_cmark::{ html, Parser};
use std::{fs::File, io::{ Write, Error }};


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

    match render_page(&site_configuration, "hello.html") {
        Ok(page) => if let Err(error) = save_html(&page, "~/Desktop/test.html") {
            println!("{}", error);
        },
        Err(error) => println!("{}", error)
    }
}

fn menu_from<T: NavigationItem>(f: &str) -> Option<Vec<T>> {
    match T::from_file(f) {
        Ok(items) => Some(items),
        _ => None
    }
}

fn parse_string(text: &str) -> String {
    let parser = Parser::new(text);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

fn render_page(config: &Configuration, p: &str) -> Result<String, String> {
    let mut page = Page::from_file(p)?;
    let mut context = Context::new();
    context.insert("site", &config);
    page.content = parse_string(&page.content);
    context.insert("page", &page);

    if let Some(sections) = menu_from::<Section>("links.json") {
        context.insert("sections", &sections);
    } else if let Some(links) = menu_from::<Link>("links.json") {
        context.insert("links", &links);
    }

    match TEMPLATES.render("page.html", &context) {
        Ok(output) => Ok(format!("{:#?}", output)),
        Err(errors) => Err(format!("{}", errors))
    }
}

fn save_html(text: &str, path: &str) -> Result<(), String> {
    let mut output = File::create(path);

    match write!(output, "{}", String::from(text)) {
        Ok(()) => Ok(()),
        Err(error) => Err(format!("{}", error))
    }
}