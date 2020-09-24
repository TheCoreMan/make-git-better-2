
use serde::{Serialize, Deserialize};
// todo move to same lib as scripts
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LevelInfo {
    pub name: String, 
    pub flag: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LevelsInfo {
    pub levels: Vec<LevelInfo>,
}
