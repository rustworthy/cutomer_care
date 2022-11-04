use serde::{Deserialize, Serialize};
use std::hash::Hash;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: QuestId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct QuestId(String);
impl QuestId {
    fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl std::str::FromStr for QuestId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            true => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "No id provided",
            )),
            false => Ok(QuestId(id.to_string())),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QuestInput {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}
impl QuestInput {
    pub fn prepare_for_storage(self) -> Question {
        Question {
            id: QuestId::new(),
            title: self.title,
            content: self.content,
            tags: self.tags,
        }
    }
}
