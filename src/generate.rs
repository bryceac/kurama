use clap::Parser;
use std::{ fs::{ 
    create_dir_all,
}, path::{ Path },
    sync::LazyLock, };
use crate::{ Configuration,
     Store };
use tera::Tera;

static TEMPLATES: LazyLock<Tera> = LazyLock::new(|| {
    let mut tera = match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(error) => {
                println!("Parsing error(s): {}", error);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![]);
        tera
});

#[derive(Default, Parser)]
#[clap(about = "build the website", long_about = None)]
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

        let blog_path = if site_configuration.blog_path.is_empty() {
            "output/posts"
        } else {
            &format!("output/{}/posts", site_configuration.blog_path)
        };
        
        store.generate_posts(&site_configuration, &TEMPLATES, blog_path);
    }
}

