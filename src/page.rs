use serde::{ Deserialize, Serialize };
use std::{fs::File, io::{ self, Read }};
use crate::metadata::Metadata;
use yaml_front_matter::YamlFrontMatter;

#[derive(Deserialize, Serialize)]
pub struct Page {
    pub metadata: Metadata,
    pub content: String
}

impl Page {
    pub fn from_file(f: &str) -> Result<Self, String> {
        match file_contents_from(f) {
            Ok(content) => {
                match YamlFrontMatter::parse::<Metadata>(&content) {
                    Ok(document) => {
                        Ok(Self {
                            metadata: document.metadata,
                            content: document.content
                        })
                    },
                    Err(error) => format!("{}", error)
                }
            },
            _ => Err(String::from("Could not parse file"))
        }
    }
}

fn file_contents_from(f: &str) -> Result<String, io::Error> {
    let mut file_content = String::new();
    File::open(f)?.read_to_string(&mut file_content)?;

    Ok(file_content)
}