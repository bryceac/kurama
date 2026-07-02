use clap::Parser;
use std::{ fs::{ 
    create_dir_all,
}, path::{ Path },
    sync::LazyLock, };
use crate::{ Configuration,
     Store, BuildMode };
use tera::Tera;
use local_ip_address::local_ip;

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
#[clap(version = "0.2", about = "build the website", long_about = None)]
pub struct Generate {
    #[clap(value_enum, default_value_t=BuildMode::Dev)]
    pub build_mode: BuildMode
}

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
    
        let mut site_configuration = Configuration::from_file("config.yaml").expect("Could not load configuration");

        /* 
         * change url to IP address and port used by preview
         * server in dev mode. */
        if let BuildMode::Dev = self.build_mode {
            if let Ok(ip_address) = local_ip() {
                site_configuration.url.set_ip_host(ip_address).unwrap();
                site_configuration.url.set_port(Some(8080)).unwrap();

                if site_configuration.url.scheme() != "http" {
                    site_configuration.url.set_scheme("http").unwrap();
                }
                
            }
        }
    
        store.generate_pages(&site_configuration, &TEMPLATES, "output");

        if !store.posts().is_empty() {
            let blog_path = if site_configuration.blog_path.is_empty() {
                "output/posts"
            } else {
                &format!("output/{}/posts", site_configuration.blog_path)
            };

            store.generate_posts(&site_configuration, &TEMPLATES, blog_path);
            store.generate_archive(&site_configuration, &TEMPLATES, "output");
            store.generate_feed(&site_configuration, "output")
        }
    }
}

