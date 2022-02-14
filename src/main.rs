mod configuration;
mod link;
mod navigation_item;
mod metadata;
mod page;
mod section;
mod save_string;


use crate::{ 
    configuration::Configuration, 
    link::Link, 
    section::Section, 
    navigation_item::NavigationItem,
    page::Page,
    save_string::Save
};

#[macro_use] extern crate lazy_static;
use tera::{ Context, Tera  };
use std::{ fs::{ read_dir, 
    create_dir_all,
 },
 path::Path
 };
 use warp::Filter;



lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(error) => {
                println!("Parsing error(s): {}", error);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![]);
        tera
    };
}

#[tokio::main]
async fn main() {
    let site_configuration = Configuration::from_file("config.json").expect("Could not load configuration");

    serve(&site_configuration).await;
}

fn menu_from<T: NavigationItem>(f: &str) -> Option<Vec<T>> {
    match T::from_file(f) {
        Ok(items) => Some(items),
        _ => None
    }
}

fn page_from_file(p: &str) -> Result<Page, String> {
    Page::from_file(p)
}

fn render_page(config: &Configuration, p: &Page) -> Result<String, String> {
    let page = p;

    let mut context = Context::new();
    context.insert("site", &config);
    context.insert("page", &page);
    context.insert("content", &page.content_html());

    if let Some(sections) = menu_from::<Section>("links.json") {
        context.insert("sections", &sections);
    } else if let Some(links) = menu_from::<Link>("links.json") {
        context.insert("links", &links);
    }

    match TEMPLATES.render("page.html", &context) {
        Ok(output) => Ok(format!("{:#}", output)),
        Err(errors) => Err(format!("{}", errors))
    }
}

fn generate(config: &Configuration) {
    let output_path = Path::new("output");

    if !Path::exists(output_path) {
        if let Err(error) = create_dir_all(output_path) {
            println!("{}", error)
        }
    }

    if let Ok(files) = read_dir("content") {
        for item in files {
            if let Ok(entry) = item {
               if let Some(file_path) = entry.path().to_str() {
                   if let Ok(page) = page_from_file(file_path) {
                       if let Ok(html) = render_page(&config, &page) {
                           let output_file_name = format!("{}.html", page.metadata.slug);

                           let file_path = output_path.join(output_file_name);

                           if let Err(error) = html.save(&file_path.to_str().unwrap()) {
                               println!("{}", error)
                           }
                       }
                   }
               }
            }
        }
    }
}

async fn serve(config: &Configuration) {
    let server_root = Path::new("output");

    generate(config);

    let site = warp::path::end().and(warp::fs::dir(server_root));

    let routes = warp::get().and(
        site
    );

    println!("website viewable at 127.0.0.1:8080");

    warp::serve(routes)
    .run(([127, 0, 0, 1], 8080))
    .await;
}