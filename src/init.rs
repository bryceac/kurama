use crate::Create;
use clap::Parser;
use std::fs;

#[derive(Default, Parser)]
#[clap(about = "create project directory structure in current directory", long_about = None)]
pub struct Init {}

impl Init {
    pub async fn run(&self) {
        let site_path = ".";
    
        match fs::canonicalize(site_path) {
            Ok(real_path) => if let Some(real_path) = real_path.to_str() {
                Create::from(real_path).run().await
            },
            Err(error) => println!("{}", error)
        }
    }
}