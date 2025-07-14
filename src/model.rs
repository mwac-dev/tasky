use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(clap::ValueEnum, Clone, Debug, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(clap::ValueEnum, Clone, Debug, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub tasks: HashMap<Uuid, Task>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub priority: Option<Priority>,
    pub difficulty: Option<Difficulty>,
    pub completed: bool,
    pub depends_on: Vec<Uuid>,
}
