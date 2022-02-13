use serde::{Serialize, Deserialize };
use std::{fs::File, io::{ self, Read }};

#[derive(Serialize, Deserialize)]
pub struct Site {
    pub name: String,
    #[serde(default = "String::default")]
    pub tagline: String,
    #[serde(default = "String::default")]
    pub dev_url: String,
    pub url: String
}

impl Site {
    pub fn new() -> Self {
        Site {
            name: String::new(),
            tagline: String::new(),
            dev_url: String::new(),
            url: String::new()
        }
    }

    pub fn from_file(f: &str) -> Result<Self, String> {
        match file_contents_from("config.json") {
            Ok(content) => match serde_json::from_str::<Site>(&content) {
                Ok(decoded_site) => Ok(decoded_site),
                Err(error) => Err(format!("{}", error))
            },
            Err(error) => Err(format!("{}", error))
        }
    }
}

fn file_contents_from(f: &str) -> Result<String, io::Error> {
    let mut file_content = String::new();
    File::open(f)?.read_to_string(&mut file_content)?;

    Ok(file_content)
}