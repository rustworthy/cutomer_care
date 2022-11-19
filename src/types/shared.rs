use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Id(String);
impl Id {
    pub fn as_dict(&self) -> std::collections::HashMap<String, Self> {
        let mut dict = std::collections::HashMap::new();
        dict.insert(String::from("_id"), self.clone());
        dict
    }
    pub fn to_str(&self) -> String {
        String::from(&self.0)
    }
}

impl std::str::FromStr for Id {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            true => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No id provided")),
            false => Ok(Id(id.to_string())),
        }
    }
}
