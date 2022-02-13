pub trait NavigationItem {
    fn from_file(f: &str) -> Result<Vec<Self>, String>
}