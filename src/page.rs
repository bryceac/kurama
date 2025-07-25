use serde::{ Deserialize, Serialize };
use std::{fs::File, 
    io::{ self, Read },
sync::LazyLock };
use crate::{ Configuration, 
    Link, 
    metadata::Metadata, 
    NavigationItem, 
    Section };
use yaml_front_matter::YamlFrontMatter;
use pulldown_cmark::{ html, Parser};
use tera::{ Context, Tera };

#[derive(Deserialize, Serialize, Eq)]
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
                    Err(error) => Err(format!("{}", error))
                }
            },
            Err(error) => Err(format!("{}", error))
        }
    }

    pub fn content_html(&self) -> String {
        let parser = Parser::new(&self.content);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output
    }

    pub fn render(&self, config: &Configuration, templates: &LazyLock<Tera>) -> Result<String, String> {
        let output_url = format!("{}.html", self.metadata.slug);
    
        let mut context = Context::new();
        context.insert("site", &config);
        context.insert("page", &self);
        context.insert("content", &self.content_html());
        context.insert("output_file", &output_url);
    
        if let Some(sections) = menu_from::<Section>("links.json") {
            context.insert("sections", &sections);
        } else if let Some(links) = menu_from::<Link>("links.json") {
            context.insert("links", &links);
        }
    
        match templates.render("page.html", &context) {
            Ok(output) => Ok(format!("{:#}", output)),
            Err(errors) => Err(format!("{}", errors))
        }
    }

    
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.metadata == other.metadata &&
        self.content == other.content
    }
}

fn file_contents_from(f: &str) -> Result<String, io::Error> {
    let mut file_content = String::new();
    File::open(f)?.read_to_string(&mut file_content)?;

    Ok(file_content)
}

fn menu_from<T: NavigationItem>(f: &str) -> Option<Vec<T>> {
    match T::from_file(f) {
        Ok(items) => Some(items),
        _ => None
    }
}