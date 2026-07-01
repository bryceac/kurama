use serde::{ Serialize, Deserialize };
use crate::{ Configuration, Paginator, Section, Link, page::menu_from };
use tera::{ Tera, Context };
use std::sync::LazyLock;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Archive {
    pub page: usize
}

impl Archive {
    pub fn render(&self, config: &Configuration, templates: &LazyLock<Tera>, paginator: &Paginator) -> Result<String, String> {
        let mut context = Context::new();
        context.insert("site", &config);
        if !config.blog_path.is_empty() {
            context.insert("current_dir", &format!("{}/", config.blog_path));
        } else {
            context.insert("current_dir", "/");
        }
        context.insert("archive", &self);
        context.insert("posts", &paginator.page(self.page));
        context.insert("pages", &paginator.page_count());
    
        if let Some(sections) = menu_from::<Section>("links.json") {
            context.insert("sections", &sections);
        } else if let Some(links) = menu_from::<Link>("links.json") {
            context.insert("links", &links);
        }

        match templates.render("archive.html", &context) {
            Ok(output) => Ok(format!("{:#}", output)),
            Err(errors) => Err(format!("{}", errors))
        }
    }
}

fn next_page_from(p: usize) -> Option<String>