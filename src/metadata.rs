use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize )]
pub struct Metadata {
    pub title: String
}