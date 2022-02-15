mod commands;
mod configuration;
mod kurama;
mod link;
mod navigation_item;
mod metadata;
mod page;
mod section;
mod save_string;


use crate::{ 
    commands::Commands,
    configuration::Configuration,
    kurama::Kurama,
    link::Link, 
    section::Section, 
    navigation_item::NavigationItem,
    page::Page,
    save_string::Save
};

#[macro_use] extern crate lazy_static;
use tera::{ Context, Tera  };
use std::{ fs::{ self, read_dir, 
    create_dir_all
 },
 path::{ Path,
    PathBuf
 }
 };
 use warp::Filter;

 use fs_extra::{ dir };

 use clap::Parser;



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
    let args = Kurama::parse();

    match args.command {
        Commands::New { path } => {
            if path.starts_with("~") {
                let expanded_path = shellexpand::tilde(&path);

                new(&expanded_path)
            } else {
                if let Ok(expanded_path) = fs::canonicalize(path.clone()) {
                    if let Some(real_path) = expanded_path.to_str() {
                        new(&real_path)
                    }
                } else {
                    new(&path)
                }
            }
        },
        Commands::Gen { } => {
            let site_configuration = Configuration::from_file("config.json").expect("Could not load configuration");
            generate(&site_configuration)
        },
        Commands::Serve { } => {
            let site_configuration = Configuration::from_file("config.json").expect("Could not load configuration");
            serve(&site_configuration).await;
        }
    }
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

    if let Ok(assets) = read_dir("assets") {
        for item in assets {
            if let Ok(entry) = item {
                let p = PathBuf::from(entry.path());

                let mut directory_copy_options = dir::CopyOptions::new();
                directory_copy_options.copy_inside = true;
                directory_copy_options.overwrite = true;

                if p.is_dir() {
                    if let Err(error) = dir::copy(p, output_path.join(entry.path().file_stem().unwrap()), &directory_copy_options) {
                        println!("{}", error)
                    }
                }
            }
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

fn new(path: &str) {
    let site_path = Path::new(path);

    if !Path::exists(site_path) {
       if let Err(error) = create_dir_all(site_path) {
           println!("{}", error)
       }
    }

    if let Err(error) = create_dir_all(site_path.join("assets/css")) {
        println!("{}", error)
    }

    if let Err(error) = create_dir_all(site_path.join("assets/js")) {
        println!("{}", error)
    }

    if let Err(error) = create_dir_all(site_path.join("assets/images")) {
        println!("{}", error)
    }

    if let Err(error) = create_dir_all(site_path.join("content")) {
        println!("{}", error)
    }

    if let Err(error) = create_dir_all(site_path.join("templates")) {
        println!("{}", error)
    }

    if let Err(error) = create_config(path) {
        println!("{}", error)
    }
}

fn create_config(path: &str) -> Result<(), String> {
    let site_path = Path::new(path);

    let config = Configuration::from("Hello, World!", "A Grand adventure");

    match config.save(site_path.join("config.json").to_str().unwrap()) {
        Ok(()) => Ok(()),
        Err(error) => Err(format!("{}", error))
    }
}


async fn serve(config: &Configuration) {
    let server_root = Path::new("output");

    generate(config);

    let site = warp::path::end().and(warp::fs::dir(server_root));
    let css = warp::path("css").and(warp::fs::dir(server_root.join("css")));
    let js = warp::path("js").and(warp::fs::dir(server_root.join("js")));
    let images = warp::path("images").and(warp::fs::dir(server_root.join("images")));

    let routes = warp::get().and(
        site
        .or(css)
        .or(js)
        .or(images)
    );

    println!("website viewable at 127.0.0.1:8080");

    warp::serve(routes)
    .run(([127, 0, 0, 1], 8080))
    .await;
}