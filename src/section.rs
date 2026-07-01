use serde::{ Serialize, Deserialize };
use crate::{ link::Link, navigation_item::NavigationItem };

#[derive(Serialize, Deserialize)]
pub struct Section {
    pub name: String,
    pub links: Vec<Link>
}