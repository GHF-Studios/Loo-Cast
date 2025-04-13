pub mod stages;
pub mod user_imports {
    pub use bevy::prelude::*;
    pub use crate::camera::components::MainCamera;
    pub use crate::config::statics::CONFIG;
    pub use crate::follower::components::{FollowerComponent, FollowerTargetComponent};
}
pub mod user_items {}