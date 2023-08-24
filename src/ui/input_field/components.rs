use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct InputField {
    pub value: String,
}

impl Default for InputField {
    fn default() -> Self {
        Self {
            value: String::new(),
        }
    }
}