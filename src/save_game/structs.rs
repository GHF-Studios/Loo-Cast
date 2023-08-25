use serde::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveGameInfo {
    pub name: String,
}