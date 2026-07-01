use serde::{ Serialize, Deserialize };
use crate::Link;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Section {
    pub name: String,
    pub links: Vec<Link>
}