use serde::{ Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Archive {
    pub page: usize
}