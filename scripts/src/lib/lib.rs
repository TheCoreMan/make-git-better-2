use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Level {
    pub title: String,
    pub branch: String,
    pub solution_checker: String,
    pub flags: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GameConfig {
    pub levels: Vec<Level>,
}
