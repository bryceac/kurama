use serde::{Serialize, Deserialize };
use serde_json;
use std::{fs::File, io::{ Read, Error }};
use crate::save_string::Save;

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub name: String,
    #[serde(default = "String::default", skip_serializing_if = "String::is_empty")]
    pub tagline: String,
    #[serde(default = "String::default", skip_serializing_if = "String::is_empty")]
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

    pub fn from(n: &str, t: &str) -> Self {
        Self {
            name: String::from(n),
            tagline: String::from(t),
            url: String::default()
        }
    }

    pub fn save(&self, p: &str) -> Result<(), Error> {
        let json_string = serde_json::to_string_pretty(&self)?;

        match json_string.save(p) {
            Ok(()) => Ok(()),
            Err(error) => Err(error)
        }
    }
}

fn file_contents_from(f: &str) -> Result<String, Error> {
    let mut file_content = String::new();
    File::open(f)?.read_to_string(&mut file_content)?;

    Ok(file_content)
}