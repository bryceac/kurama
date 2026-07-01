use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Link {
    pub name: String,
    pub url: String
}