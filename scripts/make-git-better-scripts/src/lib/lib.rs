use serde::{Deserialize, Serialize};
use log::debug;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Level {
    pub title: String,
    pub branch: String,
    pub solution_checker: String,
    pub flags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameConfig {
    pub levels: Vec<Level>,
}
