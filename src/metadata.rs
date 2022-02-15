use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize, Eq )]
pub struct Metadata {
    #[serde(default="String::default")]
    pub title: String,
    pub slug: String
}

impl PartialEq for Metadata {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title &&
        self.slug == other.slug
    }
}