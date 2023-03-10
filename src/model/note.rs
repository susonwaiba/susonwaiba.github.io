use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub date: String,
    pub content: String,
    pub path: String,
    pub sort_order: i32,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct NoteItem {
    pub id: String,
    pub title: String,
    pub date: String,
    pub path: String,
    pub sort_order: i32,
}
