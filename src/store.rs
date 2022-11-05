use crate::types::question::{QuestId, QuestInput, Question};
use std::collections::HashMap;
use std::sync::Arc;

pub type ThreadSafeStore = Arc<parking_lot::RwLock<Store>>;

#[derive(Clone)]
pub struct Store {
    questions: HashMap<QuestId, Question>,
}

impl Store {
    pub fn new_arc() -> ThreadSafeStore {
        Arc::new(parking_lot::RwLock::new(Self {
            questions: Self::init(),
        }))
    }

    fn init() -> HashMap<QuestId, Question> {
        let file = include_str!("../questions.json");
        if file.is_empty() {
            return HashMap::new();
        }
        serde_json::from_str(file).expect("Failed to read quetions from storage")
    }

    pub fn clone(store: &ThreadSafeStore) -> ThreadSafeStore {
        Arc::clone(store)
    }

    pub fn all(&self) -> Vec<&Question> {
        self.questions.values().collect()
    }

    pub fn one(&self, id: QuestId) -> Option<&Question> {
        self.questions.get(&id)
    }

    pub fn save(&mut self, q: QuestInput) -> QuestId {
        let q = q.prepare_for_storage();
        let id = q.id.clone();
        self.questions.insert(q.id.clone(), q);
        id
    }

    pub fn update(&mut self, id: QuestId, quest_upd: QuestInput) -> Result<(), &'static str> {
        if let Some(quest) = self.questions.get_mut(&id) {
            quest.title = quest_upd.title;
            quest.content = quest_upd.content;
            quest.tags = quest_upd.tags;
            return Ok(());
        }
        Err("Entry does not exist.")
    }

    pub fn remove(&mut self, id: QuestId) -> Option<Question> {
        self.questions.remove(&id)
    }
}
