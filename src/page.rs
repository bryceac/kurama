use serde::{ Serialize };
use std::{fs::File, io::{ self, Read }};
use crate::metadata::Metadata;

#[derive(Serialize)]
pub struct Page {
    pub metadata: Metadata,
    pub content: String
}