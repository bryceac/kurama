use clap::Parser;
use std::{ fs::create_dir_all, path::Path };
use crate::Configuration;
use url_path::UrlPath;

#[derive(Parser)]
#[clap(version = "0.2", about = "create a project directory at the given path", long_about = None)]
pub struct Create {
    pub path: String
}

impl Create {
    pub fn from(p: &str) -> Self {
        Self {
            path: String::from(p)
        }
    }

    pub async fn run(&self) {
        let input = if self.path.starts_with("~") {
            shellexpand::tilde(&self.path).into_owned()
        } else {
            UrlPath::new(&self.path).normalize()
        };

        let site_path = Path::new(&input);
    
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
    
        if let Err(error) = create_config(&self.path) {
            println!("{}", error)
        }
    }
}

fn create_config(path: &str) -> Result<(), String> {
    let site_path = Path::new(path);

    let config = Configuration::from("Hello, World!", "A Grand adventure");

    match config.save(site_path.join("config.yaml").to_str().unwrap()) {
        Ok(()) => Ok(()),
        Err(error) => Err(format!("{}", error))
    }
}