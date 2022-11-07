use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct QuestId(String);
impl QuestId {
    pub fn as_dict(&self) -> std::collections::HashMap<String, Self> {
        let mut dict = std::collections::HashMap::new();
        dict.insert(String::from("_id"), self.clone());
        dict
    }
    pub fn to_str(&self) -> String {
        String::from(&self.0)
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum QuestStatus {
    Resolved,
    Unresolved,
    Pending,
    Canceled,
}

impl std::str::FromStr for QuestStatus {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Resolved" => Ok(Self::Resolved),
            "Unresolved" => Ok(Self::Unresolved),
            "Pending" => Ok(Self::Pending),
            "Canceled" => Ok(Self::Canceled),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Status not supported",
            )),
        }
    }
}

impl QuestStatus {
    fn to_str(self) -> String {
        match self {
            Self::Resolved => "Resolved".to_string(),
            Self::Unresolved => "Unresolved".to_string(),
            Self::Pending => "Pending".to_string(),
            Self::Canceled => "Canceled".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct QuestInput {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
    pub status: Option<QuestStatus>,
}

impl QuestInput {
    pub fn parse_status(&self) -> String {
        if self.status.is_none() {
            return QuestStatus::Pending.to_str();
        }
        self.status.as_ref().unwrap().to_str()
    }
}

#[derive(Serialize)]
pub struct QuestOutput {
    pub _id: String,
    pub created_at: String,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
    pub status: QuestStatus,
    // pub author: String
}
