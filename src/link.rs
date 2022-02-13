use serde::{ Serialize, Deserialize };
use std::{fs::File, io::{ self, Read }};
use crate::navigation_item::NavigationItem;

#[derive(Serialize, Deserialize)]
pub struct Link {
    pub name: String,
    pub url: String
}

impl NavigationItem for Link {
    fn from_file(f: &str) -> Result<Vec<Self>, String> {
        match file_contents_from(f) {
            Ok(content) => match serde_json::from_str::<Vec<Self>>(&content) {
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