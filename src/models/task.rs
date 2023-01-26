use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct NewTask {
    pub name: String,
}
