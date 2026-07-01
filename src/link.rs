use serde::{ Serialize, Deserialize };
use crate::navigation_item::NavigationItem;

#[derive(Serialize, Deserialize)]
pub struct Link {
    pub name: String,
    pub url: String
}