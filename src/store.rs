use std::collections::HashMap;

use crate::types::question::{QuestId, Question};

#[derive(Clone)]
pub struct Store {
    pub questions: HashMap<QuestId, Question>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            questions: Self::init(),
        }
    }

    fn init() -> HashMap<QuestId, Question> {
        let file = include_str!("../questions.json");
        if file.is_empty() {
            return HashMap::new();
        }
        serde_json::from_str(file).expect("Failed to read quetions from storage")
    }
}
