use serde::{Serialize, Deserialize };
use serde_json;
use std::{fs::File, io::{ self, Read, Write }};

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

    pub fn save(&self, p: &str) -> Result<(), io::Error> {
        let mut output = File::create(p);
        let json_string = serde_json::to_string_pretty(&self)?;

        match write!(output, "{}", format!("{}", json_string)) {
            Ok(()) => Ok(()),
            Err(error) => Err(error)
        }
    }
}

fn file_contents_from(f: &str) -> Result<String, io::Error> {
    let mut file_content = String::new();
    File::open(f)?.read_to_string(&mut file_content)?;

    Ok(file_content)
}