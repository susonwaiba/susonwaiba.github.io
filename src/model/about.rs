use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct About {
    pub id: String,
    pub year: i32,
    pub title: String,
    pub content: String,
}
