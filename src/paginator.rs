use crate::Post;
use serde::{ Serialize, Deserialize };

pub struct Paginator {
    posts: Vec<Post>,
    items_per_page: usize
}

impl Paginator {
    pub fn from(posts: &Vec<Post>, items_per_page: usize) -> Self {
        Self {
            posts: posts.clone(),
            items_per_page
        }
    }
}