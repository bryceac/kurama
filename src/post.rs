use chrono::{ DateTime, Local };
use crate::{ Configuration, Page, page::menu_from, Link, Section };
use pulldown_cmark::{html, Parser};
use std::sync::LazyLock;
use tera::{ Context, Tera };

use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    pub title: String,
    pub date: DateTime<Local>,
    pub content: String,
    pub slug: String
}

impl Post {
    pub fn from_page(p: Page) -> Self {
        Self {
            title: p.metadata.title,
            date: p.metadata.date.unwrap(),
            content: p.content,
            slug: p.metadata.slug
        }
    }

    pub fn content_html(&self) -> String {
        let parser = Parser::new(&self.content);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output
    }

    pub fn date_components(&self) -> Vec<String> {
        let components: Vec<String> = self.date.to_rfc3339()
        .split("T")
        .map(|c| c.to_owned())
        .collect();

        components[0].split("-")
        .map(|c| c.to_owned())
        .collect()
    }

    pub fn render(&self, config: &Configuration, templates: &LazyLock<Tera>) -> Result<String, String> {
        let output_url = format!("{}.html", self.slug);
    
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

        match templates.render("entry.html", &context) {
            Ok(output) => Ok(format!("{:#}", output)),
            Err(errors) => Err(format!("{}", errors))
        }
    }
}