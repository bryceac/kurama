use serde::{Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
struct Site {
    name: String,
    #[serde(default = "String::default")]
    tagline: String,
    #[serde(default = "String::default")]
    dev_url: String,
    url: String
}