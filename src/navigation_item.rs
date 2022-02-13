pub trait NavigationItem: Sized {
    fn from_file(f: &str) -> Result<Vec<Self>, String>;
}