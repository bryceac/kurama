use std::{ fs, path::{ Path, PathBuf }, sync::LazyLock };
use fs_extra::dir;
use jfeed::{Item, Dates, Author, Content, Feed, FeedVersion };
use crate::{ Archive, Page, Post, Configuration, Save, Paginator, PaginationMethod};
use tera::Tera;
use http::uri::Uri;
use unicode_segmentation::UnicodeSegmentation;

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
        println!("copying assets...");
        let output_path = Path::new(p);
        if let Ok(assets) = fs::read_dir(self.assets.clone()) {
            for item in assets {
                if let Ok(entry) = item {
                    let path = PathBuf::from(entry.path());
    
                    let mut directory_copy_options = dir::CopyOptions::new();
                    directory_copy_options.copy_inside = true;
                    directory_copy_options.overwrite = true;
    
                    if path.is_dir() {
                        if let Err(error) = dir::copy(&path, output_path.join(&path.as_path().file_stem().unwrap()), &directory_copy_options) {
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
        let feed = feed_url(config, 1);
        for (index, page) in self.pages().iter().enumerate() {
            println!("Attempting to create page {} of {}", index+1, self.pages().len());
            if config.blog_path.is_empty() && !self.posts().is_empty() && page.metadata.slug == "index" {
                println!("Skipping this file because index.html is not allowed here.");
                continue;
            }
            
            match page.render(config, templates, &feed) {
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
        let feed = feed_url(config, 1);

        for (index, post) in self.posts().iter().enumerate() {
            println!("Attempting to create post {} of {}", index+1, self.posts().len());
            match post.render(config, templates, &feed) {
                Ok(html) => {
                    let output_file = format!("{}.html", post.slug);
    
                    let post_dir = output_path
                    .join(post.date_components()[0].clone())
                    .join(post.date_components()[1].clone())
                    .join(post.date_components()[2].clone());

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

    pub fn generate_archive(&self, config: &Configuration, templates: &LazyLock<Tera>, p: &str) {
        if !config.blog_path.is_empty() && config.blog_name.is_empty() {
            println!("Blog name must be provided if a path is specified.");
            return;
        }

        let paginator = Paginator::from(&self.posts(), config.items_per_page);

        let output_path = Path::new(p);

        let mut archive = Archive::default();

        for page in 1..=paginator.page_count() {
            println!("attempting to create page {} of the {}-page archive.", page, paginator.page_count());
            archive.page = page;

            let feed = feed_url(config, page);

            match archive.render(config, templates, &paginator, &feed) {
                Ok(html) => write_archive(&html, config, page, output_path),
                Err(error) => println!("{}", error),
            }
        }
    }

    pub fn generate_feed(&self, config: &Configuration, p: &str) {
        let paginator = Paginator::from(&self.posts(), config.items_per_page);
        let mut feed_builder = Feed::builder();
        feed_builder.set_version(&FeedVersion::JSONFeed1_1);
        feed_builder.set_home_page(&format!("{}", config.url.clone()));

        let output_dir = Path::new(p);

        for page in 1..=paginator.page_count() {
            println!("attempting to create page {} of the {}-page feed.", page, paginator.page_count());
            let url = feed_url(config, page);
            let next_url = if page == paginator.page_count() {
                None
            } else {
                Some(feed_url(config, page+1))
            };
            
            feed_builder.set_title(&feed_title(config, page));
            
            feed_builder.set_url(&url);

            if let Some(next_url) = next_url {
                feed_builder.set_next_url(&next_url);
            }

            for post in paginator.page(page) {
                feed_builder.add_item(&post_to_item(&post, config));
            }

            let file_path = output_dir.join(feed_output_path(&url));

            let _ = fs::create_dir_all(file_path.clone()).unwrap();

            match feed_builder.build() {
                Ok(feed) => {
                    if let Err(_) = feed.to_string().expect("Unable to create feed").save(file_path.to_str().unwrap()) {
                        println!("{} could not be created", file_path.to_str().unwrap());
                    }
                },
                Err(error) => println!("{}", error)
            }
        }
    }
}

fn write_archive(content: &str, config: &Configuration, page: usize, output_dir: &Path) {
    match config.pagination_method {
        PaginationMethod::File => if !config.blog_path.is_empty() {
            let archive_dir = output_dir.join(&config.blog_path);
            let output_file = if page > 1 {
                format!("index{}.html", page)
            } else {
                "index.html".to_owned()
            };

            let file_path = archive_dir.join(output_file);

            if let Err(error) = content.save(file_path.to_str().unwrap()) {
                println!("{}", error);
            }
        } else {
            let output_file = if page > 1 {
                format!("index{}.html", page)
            } else {
                "index.html".to_owned()
            };

            let file_path = output_dir.join(output_file);

            if let Err(error) = content.save(file_path.to_str().unwrap()) {
                println!("{}", error);
            }
        },
        PaginationMethod::Dir => if !config.blog_path.is_empty() {
            let archive_dir = output_dir.join(&config.blog_path);
            let output_file = "index.html".to_owned();

            let file_path = if page > 1 {
                archive_dir.join(format!("{}", page)).join(output_file)
            } else {
                archive_dir.join(output_file)
            };
            let _ = fs::create_dir_all(file_path.clone()).unwrap();

            if let Err(error) = content.save(file_path.to_str().unwrap()) {
                println!("{}", error);
            }
        } else {
            let output_file = "index.html".to_owned();

            let file_path = if page > 1 {
                output_dir.join(format!("{}", page)).join(output_file)
            } else {
                output_dir.join(output_file)
            };
            let _ = fs::create_dir_all(file_path.clone()).unwrap();

            if let Err(error) = content.save(file_path.to_str().unwrap()) {
                println!("{}", error);
            }
        }
    }
}

fn permalink_for_post(post: &Post, config: &Configuration) -> String {

    let path = if !config.blog_path.is_empty() {
        format!("{}/posts/{}/{}/{}/{}", config.blog_path, 
        post.date_components()[0], 
        post.date_components()[1], 
        post.date_components()[2], 
        post.slug)
    } else {
        format!("posts/{}/{}/{}/{}", post.date_components()[0], 
        post.date_components()[1], 
        post.date_components()[2], 
        post.slug)
    };

    let mut site_url = format!("{}", config.url);
    site_url.push_str(&format!("/{}", path));
    site_url
}

fn feed_title(config: &Configuration, page: usize) -> String {
    let title = if !config.blog_name.is_empty() {
        config.blog_name.clone()
    } else {
        config.name.clone()
    };

    if page > 1 {
        format!("{} ({})", title, page)
    } else {
        format!("{}", title)
    }
}

fn feed_url(config: &Configuration, page: usize) -> String {
    let mut path = if !config.blog_path.is_empty() {
        config.blog_path.clone()
    } else {
        String::default()
    };

    let feed = match config.pagination_method {
        PaginationMethod::File => if page > 1 {
            format!("feed{}.json", page)
        } else {
            "feed.json".to_owned()
        },
        PaginationMethod::Dir => if page > 1 {
            format!("{}/feed.json", page)
        } else {
            "feed.json".to_owned()
        }
    };

    path.push_str("/");
    path.push_str(&feed);

    let mut site_url = format!("{}", config.url);
    site_url.push_str(&format!("/{}", path));
    site_url
}

fn post_to_item(post: &Post, config: &Configuration) -> Item {
    let permalink = permalink_for_post(post, config);

    let dates = Dates::builder()
    .set_published(&post.date.to_rfc3339())
    .build().unwrap();

    let author = Author::builder()
    .set_name(&config.author)
    .build().unwrap();

    let content = Content::builder()
    .set_html(&post.content_html())
    .build().unwrap();


    Item::builder()
    .set_id(&permalink)
    .set_url(&permalink)
    .set_title(&post.title)
    .set_dates(&dates)
    .add_author(&author)
    .set_content(&content)
    .build().unwrap()
}

fn drop_first_character_from(s: &str) -> String {
    let characters: Vec<String> = s.graphemes(true).map(|s| s.to_owned()).collect();

    let content: String = characters[1..].iter().map(|s| s.to_owned()).collect();

    content
}

fn feed_output_path(path: &str) -> String {
    if let Ok(url) = path.parse::<Uri>() {
        drop_first_character_from(url.path())
    } else {
        drop_first_character_from(path)
    }
}