use serde::{Serialize, Deserialize };
use std::{fs::File, io::{ self, Read }};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub name: String,
    #[serde(default = "String::default")]
    pub tagline: String,
    #[serde(default = "String::default")]
    pub url: String
}

impl Configuration {
    pub fn from_file(f: &str) -> Result<Self, String> {
        match file_contents_from(f) {
            Ok(content) => match serde_json::from_str::<Configuration>(&content) {
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