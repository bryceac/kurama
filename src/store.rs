use std::{ fs, path::{ Path, PathBuf } };
use fs_extra::dir;
use crate::{Page, Post, Configuration, Save};
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
        let output_path = Path::new(p);

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
}