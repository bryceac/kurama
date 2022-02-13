use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Link {
    pub name: String,
    pub url: String
}