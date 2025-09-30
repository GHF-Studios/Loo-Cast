use bevy::ecs::entity::Entity;
use bevy::prelude::*;
use tokio::task::JoinHandle;

use crate::chunk::traits::{Vec2Ext, IVec2Ext};
use crate::usf::scale::Scale;
use crate::workflow::composite_workflow_context::ScopedCompositeWorkflowContext;

#[derive(Reflect)]
pub struct ChunkActionWorkflowHandles {
    #[reflect(ignore)]
    pub spawn: Option<JoinHandle<ScopedCompositeWorkflowContext>>,
    #[reflect(ignore)]
    pub despawn: Option<JoinHandle<ScopedCompositeWorkflowContext>>,
    #[reflect(ignore)]
    pub transfer: Option<JoinHandle<ScopedCompositeWorkflowContext>>,
}

#[derive(Debug, Clone, Reflect)]
pub struct ChunkOwnerId {
    id: String,
    entity: Entity,
    scale: Scale,
}
impl ChunkOwnerId {
    pub fn new(id: String, entity: Entity, scale: Scale) -> Self {
        Self {
            id,
            entity,
            scale,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn entity(&self) -> &Entity {
        &self.entity
    }

    pub fn scale(&self) -> &Scale {
        &self.scale
    }

    pub fn scale_mut(&mut self) -> &mut Scale {
        &mut self.scale
    }
}
impl Default for ChunkOwnerId {
    fn default() -> Self {
        Self {
            id: "PLACEHOLDER".to_string(),
            entity: Entity::from_raw(0),
            scale: Scale::default(),
        }
    }
}
impl std::hash::Hash for ChunkOwnerId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl PartialEq for ChunkOwnerId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for ChunkOwnerId {}
impl PartialOrd for ChunkOwnerId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ChunkOwnerId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialEq)]
pub struct WorldCoord {
    pub xy: Vec2,
    pub scale: Scale,
}
impl std::fmt::Debug for WorldCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WorldCoord {{ x: {}, y: {}, scale: {} }}",
            self.xy.x, self.xy.y, self.scale
        )
    }
}
impl From<ChunkCoord> for WorldCoord {
    fn from(value: ChunkCoord) -> Self {
        let chunk_size = 1000.0;
        let chunk_x = value.xy.x as f32 * chunk_size;
        let chunk_y = value.xy.y as f32 * chunk_size;
        Vec2::new(chunk_x, chunk_y).scaled(value.scale)
    }
}
impl WorldCoord {
    pub fn new(x: f32, y: f32, scale: Scale) -> Self {
        Self {
            xy: Vec2::new(x, y),
            scale
        }
    }

    pub fn unscaled(&self) -> Vec2 {
        Vec2::new(self.xy.x, self.xy.y)
    }

    pub fn distance_squared(&self, rhs: &Self) -> f32 {
        self.xy.distance_squared(rhs.xy)
    }

    pub fn scale_distance(&self, rhs: &Self) -> i8 {
        self.scale as i8 - rhs.scale as i8
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub xy: IVec2,
    pub scale: Scale,
}
impl std::fmt::Debug for ChunkCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ChunkCoord {{ x: {}, y: {}, scale: {} }}",
            self.xy.x, self.xy.y, self.scale
        )
    }
}
impl From<WorldCoord> for ChunkCoord {
    fn from(value: WorldCoord) -> Self {
        let chunk_size = 1000.0;
        let chunk_x = ((value.xy.x + chunk_size / 2.0) / chunk_size).floor() as i32;
        let chunk_y = ((value.xy.y + chunk_size / 2.0) / chunk_size).floor() as i32;
        IVec2::new(chunk_x, chunk_y).scaled(value.scale)
    }
}
impl ChunkCoord {
    pub fn new(x: i32, y: i32, scale: Scale) -> Self {
        Self {
            xy: IVec2::new(x, y),
            scale
        }
    }

    pub fn unscaled(&self) -> IVec2 {
        IVec2::new(self.xy.x, self.xy.y)
    }

    pub fn distance_squared(&self, rhs: &Self) -> i32 {
        self.xy.distance_squared(rhs.xy)
    }

    pub fn scale_distance(&self, rhs: &Self) -> i8 {
        self.scale as i8 - rhs.scale as i8
    }
    
    pub fn coords_in_radius(&self, radius: u32) -> Vec<ChunkCoord> {
        let mut chunks = Vec::new();

        let radius = radius as i32;

        let mut x = 0;
        let mut y = radius;
        let mut d = 1 - radius; // Decision parameter

        while x <= y {
            // Add filled lines between symmetrical points
            for dx in -x..=x {
                chunks.push(IVec2::new(self.xy.x + dx, self.xy.y + y).scaled(self.scale));
                chunks.push(IVec2::new(self.xy.x + dx, self.xy.y - y).scaled(self.scale));
            }
            for dx in -y..=y {
                chunks.push(IVec2::new(self.xy.x + dx, self.xy.y + x).scaled(self.scale));
                chunks.push(IVec2::new(self.xy.x + dx, self.xy.y - x).scaled(self.scale));
            }

            if d < 0 {
                // Midpoint is inside the circle
                d += 2 * x + 3;
            } else {
                // Midpoint is outside the circle
                d += 2 * (x - y) + 5;
                y -= 1;
            }
            x += 1;
        }

        chunks
    }
}