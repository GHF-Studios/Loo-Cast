use std::{
    collections::HashSet,
    fmt,
    sync::{RwLock, RwLockReadGuard},
};

use bevy::prelude::*;

use super::{ColorRamp, ScalarRange};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrawId(pub &'static str);

impl fmt::Display for DrawId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawDepth {
    World,
    Overlay,
}

#[derive(Debug, Clone)]
pub enum WorldPrimitive {
    Line {
        start: Vec3,
        end: Vec3,
        color: Color,
        depth: DrawDepth,
    },
    Arrow {
        start: Vec3,
        end: Vec3,
        color: Color,
        depth: DrawDepth,
    },
    Axes {
        transform: Transform,
        length: f32,
        depth: DrawDepth,
    },
    Rect {
        isometry: Isometry3d,
        size: Vec2,
        color: Color,
        depth: DrawDepth,
    },
    Cross {
        isometry: Isometry3d,
        size: f32,
        color: Color,
        depth: DrawDepth,
    },
    Sphere {
        isometry: Isometry3d,
        radius: f32,
        color: Color,
        resolution: u32,
        depth: DrawDepth,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarFieldMode {
    Heatmap,
    HeightField,
}

#[derive(Debug, Clone)]
pub struct WorldScalarField {
    pub id: DrawId,
    pub transform: Transform,
    pub size: Vec2,
    pub resolution: UVec2,
    pub values: Vec<f32>,
    pub range: ScalarRange,
    pub ramp: ColorRamp,
    pub opacity: f32,
    pub mode: ScalarFieldMode,
    pub height_scale: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorSpace {
    World,
    Local,
}

#[derive(Debug, Clone)]
pub struct WorldVectorField {
    pub id: DrawId,
    pub transform: Transform,
    pub size: Vec2,
    pub resolution: UVec2,
    pub vectors: Vec<Vec3>,
    pub vector_space: VectorSpace,
    pub vector_scale: f32,
    pub maximum_arrow_length: f32,
    pub color: Color,
    pub magnitude_range: Option<ScalarRange>,
    pub magnitude_ramp: ColorRamp,
}

#[derive(Debug, Default)]
pub struct WorldDrawBatch {
    pub primitives: Vec<WorldPrimitive>,
    pub scalar_fields: Vec<WorldScalarField>,
    pub vector_fields: Vec<WorldVectorField>,
}

impl WorldDrawBatch {
    pub fn line(&mut self, start: Vec3, end: Vec3, color: Color, depth: DrawDepth) {
        self.primitives.push(WorldPrimitive::Line {
            start,
            end,
            color,
            depth,
        });
    }

    pub fn arrow(&mut self, start: Vec3, end: Vec3, color: Color, depth: DrawDepth) {
        self.primitives.push(WorldPrimitive::Arrow {
            start,
            end,
            color,
            depth,
        });
    }

    pub fn axes(&mut self, transform: Transform, length: f32, depth: DrawDepth) {
        self.primitives.push(WorldPrimitive::Axes {
            transform,
            length,
            depth,
        });
    }

    pub fn rect(
        &mut self,
        isometry: Isometry3d,
        size: Vec2,
        color: Color,
        depth: DrawDepth,
    ) {
        self.primitives.push(WorldPrimitive::Rect {
            isometry,
            size,
            color,
            depth,
        });
    }

    pub fn cross(
        &mut self,
        isometry: Isometry3d,
        size: f32,
        color: Color,
        depth: DrawDepth,
    ) {
        self.primitives.push(WorldPrimitive::Cross {
            isometry,
            size,
            color,
            depth,
        });
    }

    pub fn sphere(
        &mut self,
        isometry: Isometry3d,
        radius: f32,
        color: Color,
        resolution: u32,
        depth: DrawDepth,
    ) {
        self.primitives.push(WorldPrimitive::Sphere {
            isometry,
            radius,
            color,
            resolution,
            depth,
        });
    }

    pub fn scalar_field(&mut self, field: WorldScalarField) {
        self.scalar_fields.push(field);
    }

    pub fn vector_field(&mut self, field: WorldVectorField) {
        self.vector_fields.push(field);
    }
}

#[derive(Resource, Debug, Default)]
pub struct WorldDrawFrame {
    inner: RwLock<WorldDrawBatch>,
}

impl WorldDrawFrame {
    pub fn submit(&self, mut batch: WorldDrawBatch) {
        let mut frame = self.inner.write().expect("world draw frame lock poisoned");

        let mut ids = frame
            .scalar_fields
            .iter()
            .map(|field| field.id)
            .chain(frame.vector_fields.iter().map(|field| field.id))
            .collect::<HashSet<_>>();
        for id in batch
            .scalar_fields
            .iter()
            .map(|field| field.id)
            .chain(batch.vector_fields.iter().map(|field| field.id))
        {
            assert!(ids.insert(id), "duplicate world draw field id {id}");
        }

        frame.primitives.append(&mut batch.primitives);
        frame.scalar_fields.append(&mut batch.scalar_fields);
        frame.vector_fields.append(&mut batch.vector_fields);
    }

    fn clear(&self) {
        let mut frame = self.inner.write().expect("world draw frame lock poisoned");
        frame.primitives.clear();
        frame.scalar_fields.clear();
        frame.vector_fields.clear();
    }

    pub(super) fn read(&self) -> RwLockReadGuard<'_, WorldDrawBatch> {
        self.inner.read().expect("world draw frame lock poisoned")
    }
}

pub(super) fn clear_world_draw_frame(frame: Res<WorldDrawFrame>) {
    frame.clear();
}
