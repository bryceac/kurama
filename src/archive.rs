use serde::{ Serialize, Deserialize };
use crate::{ Configuration, Paginator, PaginationMethod, Taxonomy };
use tera::{ Tera, Context };
use std::sync::LazyLock;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ArchiveType {
    #[default]
    Blog,
    Tag
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Archive {
    pub name: String,
    pub archive_type: ArchiveType,
    pub page: usize
}

impl Archive {
    pub fn render(&self, config: &Configuration, templates: &LazyLock<Tera>, paginator: &Paginator, feed: &str) -> Result<String, String> {
        let mut context = Context::new();
        context.insert("site", &config);
        if !config.blog_path.is_empty() {
            if let ArchiveType::Group(dir) = self.archive_type.clone() {
                context.insert("current_dir", &format!("{}/{}/", config.blog_path, dir.slug()));
            } else {
                context.insert("current_dir", &format!("{}/", config.blog_path));
            }
            
        } else {
            if let ArchiveType::Group(dir) = self.archive_type.clone() {
                context.insert("current_dir", &format!("/{}/", dir.slug()));
            } else {
                context.insert("current_dir", "/");
            }
            
        }
        context.insert("archive", &self);
        context.insert("feed_url", feed);
        context.insert("posts", &paginator.page(self.page));
        context.insert("pages", &paginator.page_count());
        context.insert("prev_page", &previous_page_from(self.page, config, self.archive_type.group()));
        context.insert("next_page", &next_page_from(self.page, paginator, config, self.archive_type.group()));

        match templates.render("archive.html", &context) {
            Ok(output) => Ok(format!("{:#}", output)),
            Err(errors) => Err(format!("{:?}", errors))
        }
    }
}

fn next_page_from(page: usize, paginator: &Paginator, config: &Configuration, t: Option<T>) -> Option<String> {
    if page == paginator.page_count() {
        None
    } else {
        match config.pagination_method {
            PaginationMethod::File => if !config.blog_path.is_empty() {
                if let Some(dir) = t {
                    Some(format!("/{}/{}/index{}.html", config.blog_path, dir.slug(), page+1))
                } else {
                    Some(format!("/{}/index{}.html", config.blog_path, page+1))
                }
            } else {
                if let Some(dir) = t {
                    Some(format!("/{}/index{}.html", dir.slug(), page+1))
                } else {
                    Some(format!("/index{}.html", page+1))
                }
            },
            PaginationMethod::Dir => if !config.blog_path.is_empty() {
                if let Some(dir) = t {
                    Some(format!("/{}/{}/{}", config.blog_path, dir.slug(), page+1))
                } else {
                    Some(format!("/{}/{}", config.blog_path, page+1))
                }
            } else {
                if let Some(dir) = t {
                    Some(format!("/{}/{}", dir.slug(), page+1))
                } else {
                    Some(format!("/{}", page+1))
                }
            }
        }
    }
}

fn previous_page_from(page: usize, config: &Configuration, t: Option<T>) -> Option<String> {
    let prev_page = page -1;
    if page == 1 {
        None
    } else {
        match config.pagination_method {
            PaginationMethod::File => if !config.blog_path.is_empty() {
                if prev_page > 1 {
                    if let Some(dir) = t {
                        Some(format!("/{}/{}/index{}.html", config.blog_path, dir.slug(), prev_page))
                    } else {
                        Some(format!("/{}/index{}.html", config.blog_path, prev_page))
                    }
                } else {
                    if let Some(dir) = t {
                        Some(format!("/{}/{}/", config.blog_path, dir.slug()))
                    } else {
                        Some(format!("/{}/", config.blog_path))
                    }
                }
            } else {
                if prev_page > 1 {
                    if let Some(dir) = t {
                        Some(format!("/{}/index{}.html", dir.slug(), prev_page))
                    } else {
                        Some(format!("/index{}.html", prev_page))
                    }
                } else {
                    if let Some(dir) = t {
                        Some(format!("/{}/", dir.slug()))
                    } else {
                        Some(format!("/"))
                    }
                }
            },
            PaginationMethod::Dir => if !config.blog_path.is_empty() {
                if prev_page > 1 {
                    if let Some(dir) = t {
                        Some(format!("/{}/{}/{}", config.blog_path, dir.slug(), prev_page))
                    } else {
                        Some(format!("/{}/{}", config.blog_path, prev_page))
                    }
                } else {
                    if let Some(dir) = t {
                        Some(format!("/{}/{}/", config.blog_path, dir.slug()))
                    } else {
                        Some(format!("/{}/", config.blog_path))
                    }
                }
            } else {
                if prev_page > 1 {
                    if let Some(dir) = t {
                        Some(format!("/{}/{}/", dir.slug(), prev_page))
                    } else {
                        Some(format!("/{}/", prev_page))
                    }
                } else {
                    if let Some(dir) = t {
                        Some(format!("/{}/", dir.slug()))
                    } else {
                        Some(format!("/"))
                    }
                }
            }
        }
    }
}