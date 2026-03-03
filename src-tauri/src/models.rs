use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub name: String,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reminder {
    pub name: String,
    pub due_date: Option<String>,
    pub notes: Option<String>,
}
