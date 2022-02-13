use serde::{ Serialize};
use std::{fs::File, io::{ self, Read }};

#[derive(Serialize)]
pub struct Page {
    pub title: String,
    pub content: String
}