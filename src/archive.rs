use serde::{ Serialize, Deserialize };
use crate::{ Configuration, Paginator, Section, Link, page::menu_from, paginator::PaginationMethod };
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

fn next_page_from(page: usize, paginator: &Paginator, config: &Configuration) -> Option<String> {
    if page == paginator.page_count() {
        None
    } else {
        match config.pagination_method {
            PaginationMethod::File => if !config.blog_path.is_empty() {
                Some(format!("/{}/index{}.html", config.blog_path, page+1))
            } else {
                Some(format!("/index{}.html", page+1))
            },
            PaginationMethod::Dir => if !config.blog_path.is_empty() {
                Some(format!("/{}/{}", config.blog_path, page+1))
            } else {
                Some(format!("/{}", page+1))
            }
        }
    }
}

fn previous_page_from(page: usize, paginator: &Paginator, config: &Configuration) -> Option<String> {
    if page == 1 {
        None
    } else {
        match config.pagination_method {
            PaginationMethod::File => if !config.blog_path.is_empty() {
                Some(format!("/{}/index{}.html", config.blog_path, page-1))
            } else {
                Some(format!("/index{}.html", page-1))
            },
            PaginationMethod::Dir => if !config.blog_path.is_empty() {
                Some(format!("/{}/{}", config.blog_path, page-1))
            } else {
                Some(format!("/{}", page-1))
            }
        }
    }
}