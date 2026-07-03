use serde::{Serialize, Deserialize };
use std::{fs::File, io::{ Read, Error }};
use crate::{Save, PaginationMethod, Section, Link };
use http::uri::Uri;

#[derive(Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub name: String,
    #[serde(default = "String::default", skip_serializing_if = "String::is_empty")]
    pub tagline: String,
    #[serde(default = "String::default", skip_serializing_if= "String::is_empty")]
    pub author: String,
    #[serde(with = "http_serde_ext::uri")]
    pub url: Uri,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sections: Vec<Section>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    #[serde(default= "String::default", skip_serializing_if = "String::is_empty")]
    pub blog_path: String,
    #[serde(default= "String::default", skip_serializing_if = "String::is_empty")]
    pub blog_name: String,
    #[serde(rename = "items", default= "usize::default", skip_serializing_if = "number_is_default")]
    pub items_per_page: usize,
    #[serde(default, skip_serializing_if = "PaginationMethod::is_default")]
    pub pagination_method: PaginationMethod
}

impl Configuration {
    pub fn from_file(f: &str) -> Result<Self, String> {
        match file_contents_from(f) {
            Ok(content) => match yaml_serde::from_str::<Configuration>(&content) {
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
            author: String::new(),
            url: "//example.com".parse::<Uri>().unwrap(),
            sections: vec![],
            links: vec![],
            blog_path: String::default(),
            blog_name: String::default(),
            items_per_page: usize::default(),
            pagination_method: PaginationMethod::default()
        }
    }

    pub fn save(&self, p: &str) -> Result<(), String> {
        match yaml_serde::to_string(&self) {
            Ok(yaml_string) => match yaml_string.save(p) {
                Ok(()) => Ok(()),
                Err(error) => Err(format!("{}", error))
            },
            Err(error) => Err(format!("{}", error))
        }
    }
}

fn file_contents_from(f: &str) -> Result<String, Error> {
    let mut file_content = String::new();
    File::open(f)?.read_to_string(&mut file_content)?;

    Ok(file_content)
}

fn number_is_default(num: &usize) -> bool {
    *num == usize::default()
}