use serde::{ Deserialize, Serialize };
use std::{fs::File, io::{ self, Read }};
use crate::metadata::Metadata;

#[derive(Deserialize, Serialize)]
pub struct Page {
    pub metadata: Metadata,
    pub content: String
}