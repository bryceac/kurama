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