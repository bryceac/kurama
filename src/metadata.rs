use chrono::{ DateTime, Local };
use serde::{ Deserialize, Serialize };
use crate::Tag;

#[derive(Deserialize, Serialize, Eq, Clone )]
pub struct Metadata {
    #[serde(default="String::default")]
    pub title: String,
    pub date: Option<DateTime<Local>>,
    pub slug: String,
    pub tags: Vec<Tag>
}

impl Metadata {
    pub fn date_components(&self) -> Vec<String> {
        if let Some(date) = self.date {
            let components: Vec<String> = date.to_rfc3339()
                .split("T")
                .map(|c| c.to_owned())
                .collect();

            components[0].split("-")
            .map(|c| c.to_owned())
            .collect()
        } else {
            vec![]
        }
        
    }
}

impl PartialEq for Metadata {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title &&
        self.date == other.date &&
        self.slug == other.slug &&
        self.tags == other.tags
    }
}