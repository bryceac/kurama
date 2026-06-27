use crate::Post;

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

    pub fn total_items(&self) -> usize {
        self.posts.len()
    }

    pub fn page_count(&self) -> usize {
        self.total_items()/self.items_per_page
    }

    pub fn page(&self, page: usize) -> Vec<Post> {
        if self.items_per_page > 0 {
            let start_index = if page == 1 {
                page-1
            } else {
                self.items_per_page*(page-1)
            };
    
            let increment = self.items_per_page-1;
    
            let end_index = if start_index+increment >= self.total_items() {
                self.total_items()-1
            } else {
                start_index+increment
            };
    
            self.posts[start_index..=end_index].to_vec()
        } else {
            self.posts.clone()
        } 
    }
}