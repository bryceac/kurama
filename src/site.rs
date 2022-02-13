use serde::{Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
struct Site {
    name: String,
    tagline: String,
    dev_url: String,
    url: String
}