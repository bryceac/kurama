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
}