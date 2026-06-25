use std::{ fs, path::{ Path, PathBuf } };
use fs_extra::dir;
use crate::{Page, Configuration, Save};
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

    pub fn pages(&self, config: &Configuration, p: &str) -> Vec<Page> {
        let mut pages: Vec<Page> = vec![];
        let output_path = Path::new(p);

        if let Ok(files) = fs::read_dir("content") {
            for item in files {
                if let Ok(entry) = item {
                   if let Some(file_path) = entry.path().to_str() {
                       match Page::from_file(file_path) {
                           Ok(page) => match page.render(&site_configuration, &TEMPLATES) {
                               Ok(html) => {
                                let output_file_name = format!("{}.html", page.metadata.slug);

                                if let Some(date) = page.metadata.date {
                                    let date_components: Vec<String> = date.to_string().split("-").map(|s| s.to_owned()).collect();

                                    let post_output_dir = output_path.join(date_components[0].clone()).join(date_components[1].clone()).join(date_components[2].clone());

                                    let _ = fs::create_dir_all(post_output_dir.as_path()).unwrap();

                                    let file_path = post_output_dir.as_path().join(output_file_name);
    
                                    if let Err(error) = html.save(&file_path.to_str().unwrap()) {
                                        println!("{}", error)
                                    }
                                } else {
                                    let file_path = output_path.join(output_file_name);
    
                                    if let Err(error) = html.save(&file_path.to_str().unwrap()) {
                                        println!("{}", error)
                                    }
                                }
                               },
                               Err(error) => println!("{}", error)
                           },
                           Err(error) => println!("{}", error)
                       }
                   }
                }
            }
        }

        return pages;
    }
}