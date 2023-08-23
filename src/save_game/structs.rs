use serde::*;

#[derive(Serialize, Deserialize)]
pub struct SaveGameInfo {
    pub name: String,
}
