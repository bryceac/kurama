mod site;

use serde_json;
use crate::site::Site;
use std::{fs::File, io::{ self, Read }};

fn main() {
    let site_configuration = match file_contents_from("config.json") {
        Ok(content) => match serde_json::from_str::<Site>(&content) {
            Ok(decoded_site) => decoded_site,
            _ => Site::new()
        },
        _ => Site::new()
    };

    println!("{}", site_configuration.name)
}

fn file_contents_from(f: &str) -> Result<String, io::Error> {
    let mut file_content = String::new();
    File::open(f)?.read_to_string(&mut file_content)?;

    Ok(file_content)
}
