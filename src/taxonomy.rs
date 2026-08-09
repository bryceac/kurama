use std::fmt::Display;
use serde::{ Serialize, Deserialize };
use rslug::Slugifier;

pub trait Taxonomy {
    fn slug(&self) -> String;
}

#[derive(Clone, Eq)]
pub struct Tag {
    pub name: String
}

impl Tag {
    pub fn from(s: &str) -> Self {
        Self { name: s.to_owned() }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Taxonomy for Tag {
    fn slug(&self) -> String {
        let slugify_with_underscore = Slugifier::new()
        .separator("_");

        slugify_with_underscore.slugify(&self.name)
    }
}

impl Serialize for Tag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        
        serializer.serialize_str(&self.name)
    }
}

impl<'de> Deserialize<'de> for Tag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let value = String::deserialize(deserializer)?;
        Ok(Self::from(&value))
    }
}