use bevy::ecs::entity::Entity;
use bevy::prelude::*;
use tokio::task::JoinHandle;

use crate::chunk::traits::{Vec2Ext, I128Vec2Ext};
use crate::usf::scale::{Scale, DynScale};
use crate::utils::types::I128Vec2;
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
    pub grid_coord: GridCoord,
    pub local_offset: Vec2,
}
impl std::fmt::Debug for WorldCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WorldCoord {{ x: {}, y: {}, grid: {:?} }}",
            self.local_offset.x, self.local_offset.y, self.grid_coord
        )
    }
}
impl WorldCoord {
    pub fn new(scale: Scale, grid_x: i128, grid_y: i128, local_x: f32, local_y: f32) -> Self {
        Self {
            grid_coord: GridCoord::new(scale, grid_x, grid_y),
            local_offset: Vec2::new(local_x, local_y),
        }
    }

    pub fn distance_squared(&self, rhs: &Self) -> f32 {
        self.local_offset.distance_squared(rhs.local_offset)
    }

    pub fn scale_distance(&self, rhs: &Self) -> i8 {
        self.grid_coord.scale as i8 - rhs.grid_coord.scale as i8
    }

    pub fn to_grid_coord(&self) -> GridCoord {
        self.grid_coord
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialEq, Eq, Hash)]
pub struct GridCoord {
    pub scale: Scale,
    pub xy: I128Vec2,
}
impl std::fmt::Debug for GridCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GridCoord {{ x: {}, y: {}, scale: {} }}",
            self.xy.x, self.xy.y, self.scale
        )
    }
}
impl std::fmt::Display for GridCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GridCoord({}, {}, {})", self.scale, self.xy.x, self.xy.y)
    }
}
impl GridCoord {
    pub fn new(scale: Scale, x: i128, y: i128) -> Self {
        Self {
            xy: I128Vec2::new(x, y),
            scale
        }
    }

    pub fn distance_squared(&self, rhs: &Self) -> i128 {
        self.xy.distance_squared(&rhs.xy)
    }

    pub fn scale_distance(&self, rhs: &Self) -> i8 {
        self.scale as i8 - rhs.scale as i8
    }
    
    pub fn coords_in_radius(&self, radius: u32) -> Vec<GridCoord> {
        let mut chunks = Vec::new();

        let radius = radius as i128;

        let mut x = 0;
        let mut y = radius;
        let mut d = 1 - radius; // Decision parameter

        while x <= y {
            // Add filled lines between symmetrical points
            for dx in -x..=x {
                chunks.push(GridCoord::new(self.scale, self.xy.x + dx, self.xy.y + y));
                chunks.push(GridCoord::new(self.scale, self.xy.x + dx, self.xy.y - y));
            }
            for dx in -y..=y {
                chunks.push(GridCoord::new(self.scale, self.xy.x + dx, self.xy.y + x));
                chunks.push(GridCoord::new(self.scale, self.xy.x + dx, self.xy.y - x));
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

    pub fn to_world_coord(&self, grid_origin_offset: I128Vec2, local_offset: Vec2) -> WorldCoord {
        const GRID_SIZE: f32 = 1000.0;
        
        let scale_factor = self.scale.scale_factor() as f32;
        let chunk_diff = self.xy - grid_origin_offset;
        
        WorldCoord {
            grid_coord: *self,
            local_offset: Vec2::new(
                (chunk_diff.x as f32 * scale_factor * GRID_SIZE) + (local_offset.x * scale_factor),
                (chunk_diff.y as f32 * scale_factor * GRID_SIZE) + (local_offset.y * scale_factor),
            ),
        }
    }
}