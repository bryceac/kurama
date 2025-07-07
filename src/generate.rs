use clap::Parser;
use std::{ fs::{ 
    create_dir_all,
    read_dir 
}, path::{ Path, 
    PathBuf },
    sync::LazyLock, };
use crate::{ Configuration,
    Page,
    Save };
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
    
        if !Path::exists(output_path) {
            if let Err(error) = create_dir_all(output_path) {
                println!("{}", error)
            }
        }
    
        let site_configuration = Configuration::from_file("config.json").expect("Could not load configuration");
    
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
                       match Page::from_file(file_path) {
                           Ok(page) => match page.render(&site_configuration, &TEMPLATES) {
                               Ok(html) => {
                                let output_file_name = format!("{}.html", page.metadata.slug);
    
                                let file_path = output_path.join(output_file_name);
    
                                if let Err(error) = html.save(&file_path.to_str().unwrap()) {
                                    println!("{}", error)
                                }
                               },
                               Err(error) => println!("{}", error)
                           },
                           Err(error) => println!("{}", error)
                       }
                   }
                }
            }
        }
    }
}

