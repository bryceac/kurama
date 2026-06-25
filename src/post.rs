use chrono::NaiveDate;
use crate::Page;
use pulldown_cmark::{html, Parser};
pub struct Post {
    title: String,
    date: NaiveDate,
    content: String,
    slug: String
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
}