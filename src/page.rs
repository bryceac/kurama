use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Page {
    pub name: String,
    pub url: String
}