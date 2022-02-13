pub trait NavigationItem {
    pub fn from_file(f: &str) -> Result<Vec<Self>, String>
}