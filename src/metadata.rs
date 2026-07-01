use chrono::{ DateTime, Local };
use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize, Eq )]
pub struct Metadata {
    #[serde(default="String::default")]
    pub title: String,
    pub date: Option<DateTime<Local>>,
    pub slug: String
}

impl PartialEq for Metadata {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title &&
        self.date == other.date &&
        self.slug == other.slug
    }
}