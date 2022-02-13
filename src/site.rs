use serde::{Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Site {
    pub name: String,
    #[serde(default = "String::default")]
    pub tagline: String,
    #[serde(default = "String::default")]
    pub dev_url: String,
    pub url: String
}

impl Site {
    pub fn new() -> Self {
        Site {
            name: String::new(),
            tagline: String::new(),
            dev_url: String::new(),
            url: String::new()
        }
    }
}