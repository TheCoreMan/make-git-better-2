use yew::callback::Callback;
use anyhow::Error;
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

#[derive(Debug)]
pub struct SingleFlagStatus {
    pub level_name: String,
    pub is_correct: bool
}

#[derive(Debug)]
pub enum MainPageMsg {
    CheckSingleFlag(SingleFlagStatus),
    // Fetch-related messages
    GetFlagsResponse,
    FlagsResponseReady(Result<LevelsInfo, Error>),
}

pub type CheckFlagCallback = Callback<SingleFlagStatus>;
