use clap::Parser;
use std::{ fs::{ 
    create_dir_all,
}, path::{ Path },
    sync::LazyLock, };
use crate::{ Configuration,
     Store, 
     archive::ArchiveType };
use tera::{Tera, Kwargs, State};
use tera_contrib::dates;
use rslug::Slugifier;

static TEMPLATES: LazyLock<Tera> = LazyLock::new(|| {
    let mut tera = Tera::default();

    tera.load_from_glob("templates/*.html").expect("Something went wrong");
    tera.autoescape_on(Vec::<&str>::new());
    tera.register_filter("date", dates::date);
    tera.register_filter("slugify", | s: &str, _: Kwargs, _: &State | {
        let slugify_with_underscore = Slugifier::new()
        .separator("_");

        slugify_with_underscore.slugify(s)
    });
    tera
});

#[derive(Default, Parser)]
#[clap(version = "0.2.1", about = "build the website", long_about = None)]
pub struct Generate {}

impl Generate {
    pub async fn run(&self) {
        let output_path = Path::new("output");

        let store = Store::from("assets", "content");
    
        if !Path::exists(output_path) {
            if let Err(error) = create_dir_all(output_path) {
                println!("{}", error)
            }
        }

        store.copy_assets("output");
    
        let site_configuration = Configuration::from_file("config.json").expect("Could not load configuration");
    
        store.generate_pages(&site_configuration, &TEMPLATES, "output");

        if !store.posts().is_empty() {
            let blog_path = if site_configuration.blog_path.is_empty() {
                "output/posts"
            } else {
                &format!("output/{}/posts", site_configuration.blog_path)
            };

            store.generate_posts(&site_configuration, &TEMPLATES, blog_path);
            store.generate_archive(&site_configuration, &TEMPLATES, "", ArchiveType::Blog, "output");
            store.generate_tag_archives(&site_configuration, &TEMPLATES, "output");

            store.generate_feed(&site_configuration, "", "output");
            store.geberate_tag_feeds(&site_configuration, "output");
        }

        store.generate_sitemap(&site_configuration, "output");

        println!("Your site is now ready.")
    }
}

