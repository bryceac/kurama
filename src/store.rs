use std::{ fs, path::{ Path, PathBuf }, sync::LazyLock };
use fs_extra::dir;
use crate::{ Archive, Page, Post, Configuration, Save, Paginator};
use tera::Tera;
pub struct Store {
    assets: String,
    content_dir: String,
}

impl Store {
    pub fn from(assets: &str, content: &str) -> Self {
        Self {
            assets: assets.to_owned(),
            content_dir: content.to_owned()
        }
    }

    pub fn copy_assets(&self, p: &str) {
        let output_path = Path::new(p);
        if let Ok(assets) = fs::read_dir(self.assets.clone()) {
            for item in assets {
                if let Ok(entry) = item {
                    let path = PathBuf::from(entry.path());
    
                    let mut directory_copy_options = dir::CopyOptions::new();
                    directory_copy_options.copy_inside = true;
                    directory_copy_options.overwrite = true;
    
                    if path.is_dir() {
                        if let Err(error) = dir::copy(p, output_path.join(entry.path().file_stem().unwrap()), &directory_copy_options) {
                            println!("{}", error)
                        }
                    }
                }
            }
        }
    }

    fn retrieve_pages(&self) -> Vec<Page> {
        let mut pages: Vec<Page> = vec![];

        if let Ok(files) = fs::read_dir(self.content_dir.clone()) {
            for item in files {
                if let Ok(entry) = item {
                   if let Some(file_path) = entry.path().to_str() {
                       if let Ok(page) = Page::from_file(file_path) {
                            pages.push(page);
                       }
                   }
                }
            }
        }

        pages
    }

    pub fn pages(&self) -> Vec<Page> {
        self.retrieve_pages()
        .into_iter()
        .filter(|p| p.metadata.date.is_none())
        .collect()
    }

    pub fn posts(&self) -> Vec<Post> {
        self.retrieve_pages()
        .into_iter()
        .filter(|p| p.metadata.date.is_some())
        .map(|p| Post::from_page(p))
        .collect()
    }

    pub fn generate_pages(&self, config: &Configuration, templates: &LazyLock<Tera>, p: &str) {
        let output_path = Path::new(p);
        for page in self.pages() {
            if config.blog_path.is_empty() && !self.posts().is_empty() && page.metadata.slug == "index" {
                println!("Skipping this file because index.html is not allowed here.");
                continue;
            }
            match page.render(config, templates) {
                Ok(html) => {
                    let output_file = format!("{}.html", page.metadata.slug);
    
                    let file_path = output_path.join(output_file);
    
                    if let Err(error) = html.save(file_path.to_str().unwrap()) {
                        println!("{}", error)
                    }
                },
                Err(error) => println!("{}", error)
            }
        }
    }
    
    pub fn generate_posts(&self, config: &Configuration, templates: &LazyLock<Tera>, p: &str) {
        let output_path = Path::new(p);
        for post in self.posts() {
            match post.render(config, templates) {
                Ok(html) => {
                    let output_file = format!("{}.html", post.slug);
                    let date_components: Vec<String> = post.date.to_string()
                    .split("-")
                    .map(|c| c.to_owned())
                    .collect();
    
                    let post_dir = output_path
                    .join(date_components[0].clone())
                    .join(date_components[1].clone())
                    .join(date_components[2].clone());

                    let _ = fs::create_dir_all(post_dir.clone()).unwrap();

                    let file_path = post_dir.clone().join(output_file);

                    if let Err(error) = html.save(file_path.to_str().unwrap()) {
                        println!("{}", error);
                    }
                },
                Err(error) => println!("{}", error)
            }
        }
    }

    pub fn generate_archive(&self, config: &Configuration, templates: &LazyLock<Tera>, paginator: &Paginator, p: &str) {
        if !config.blog_path.is_empty() && config.blog_name.is_empty() {
            println!("Blog name must be provided if a path is specified.");
            return;
        }

        let output_path = Path::new(p);

        let mut archive = Archive::default();

        for page in 1..=paginator.page_count() {
            archive.page = page;

            match archive.render(config, templates, paginator) {
                Ok(html) => if !config.blog_path.is_empty() {
                    let archive_dir = output_path.join(&config.blog_path);
                    let output_file = if archive.page > 1 {
                        format!("index{}.html", archive.page)
                    } else {
                        "index.html".to_owned()
                    };

                    let file_path = archive_dir.join(output_file);

                    if let Err(error) = html.save(file_path.to_str().unwrap()) {
                        println!("{}", error);
                    }
                } else {
                    let output_file = if archive.page > 1 {
                        format!("index{}.html", archive.page)
                    } else {
                        "index.html".to_owned()
                    };

                    let file_path = output_path.join(output_file);

                    if let Err(error) = html.save(file_path.to_str().unwrap()) {
                        println!("{}", error);
                    }
                },
                Err(error) => println!("{}", error),
            }
        }
    }
}