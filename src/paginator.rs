use crate::Page;
use serde::{ Deserialize, Serialize };

pub struct Paginator {
    posts: Vec<Page>,
    items_per_page: usize
}

impl Paginator {
    pub fn from(posts: &Vec<Page>, items_per_page: usize) -> Self {
        Self {
            posts: posts.clone(),
            items_per_page
        }
    }

    pub fn total_items(&self) -> usize {
        self.posts.len()
    }

    pub fn page_count(&self) -> usize {
        if self.total_items() >= self.items_per_page && self.items_per_page > 0 {
            self.total_items()/self.items_per_page
        } else {
            1
        }
    }

    pub fn page(&self, page: usize) -> Vec<Page> {
        if self.items_per_page > 0 {
            let start_index = if page == 1 {
                page-1
            } else {
                self.items_per_page*(page-1)
            };
    
            let incrementer = self.items_per_page-1;
    
            let end_index = if start_index+incrementer >= self.total_items() {
                self.total_items()-1
            } else {
                start_index+incrementer
            };
    
            self.posts[start_index..=end_index].to_vec()
        } else {
            self.posts.clone()
        } 
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaginationMethod {
    #[default]
    File,
    Dir
}

impl PaginationMethod {
    pub fn is_default(method: &PaginationMethod) -> bool {
        *method == PaginationMethod::default()
    }
}
