use serde::{Deserialize, Serialize};

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
pub struct QuestIn {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
    pub status: Option<QuestStatus>,
}

impl QuestIn {
    pub fn authored_by(self, user_id: String) -> QuestByUser {
        QuestByUser {
            title: self.title,
            content: self.content,
            tags: self.tags,
            status: self.status,
            user_id,
        }
    }
}

pub struct QuestByUser {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
    pub status: Option<QuestStatus>,
    pub user_id: String,
}

impl QuestByUser {
    pub fn parse_status(&self) -> String {
        if self.status.is_none() {
            return QuestStatus::Pending.to_str();
        }
        self.status.as_ref().unwrap().to_str()
    }
}

#[derive(Serialize)]
pub struct QuestOut {
    pub _id: String,
    pub created_at: String,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
    pub status: QuestStatus,
    pub author: String,
}
