use clap::Parser;
use std::{ fs::{ 
    create_dir_all,
    read_dir 
}, path::{ Path, 
    PathBuf },
    sync::LazyLock, };
use crate::{ Configuration,
    Page,
    Save, Store };
use fs_extra::dir;
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
    
        
    }
}

