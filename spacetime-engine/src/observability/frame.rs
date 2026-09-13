//! Ephemeral observation frame.
//!
//! Domain systems describe what is meaningful. They do not spawn debug entities,
//! own materials, know about cameras, or call renderer APIs directly.

use std::{collections::HashSet, sync::{RwLock, RwLockReadGuard}};

use bevy::prelude::*;

use super::{DebugColorRamp, DebugId, DebugScalarRange};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugDepth {
    World,
    Overlay,
}

#[derive(Debug, Clone)]
pub enum DebugPrimitive {
    Line {
        start: Vec3,
        end: Vec3,
        color: Color,
        depth: DebugDepth,
    },
    Arrow {
        start: Vec3,
        end: Vec3,
        color: Color,
        depth: DebugDepth,
    },
    Axes {
        transform: Transform,
        length: f32,
        depth: DebugDepth,
    },
    Rect {
        isometry: Isometry3d,
        size: Vec2,
        color: Color,
        depth: DebugDepth,
    },
    Cross {
        isometry: Isometry3d,
        size: f32,
        color: Color,
        depth: DebugDepth,
    },
    Sphere {
        isometry: Isometry3d,
        radius: f32,
        color: Color,
        resolution: u32,
        depth: DebugDepth,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum DebugTextFacing {
    Billboard,
    World(Quat),
}

#[derive(Debug, Clone)]
pub struct DebugLabel {
    pub position: Vec3,
    pub text: String,
    pub font_size: f32,
    pub anchor: Vec2,
    pub color: Color,
    pub subject: Option<Entity>,
    pub facing: DebugTextFacing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugScalarFieldMode {
    Heatmap,
    HeightField,
}

#[derive(Debug, Clone)]
/// Regular scalar samples on a local XY plane. `resolution` is the number of
/// sample points, not cells; values are row-major (`x + y * resolution.x`).
pub struct DebugScalarField {
    pub id: DebugId,
    pub transform: Transform,
    pub size: Vec2,
    pub resolution: UVec2,
    pub values: Vec<f32>,
    pub range: DebugScalarRange,
    pub ramp: DebugColorRamp,
    pub opacity: f32,
    pub mode: DebugScalarFieldMode,
    pub height_scale: f32,
    pub title: String,
    pub unit: &'static str,
    pub legend: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugVectorSpace {
    World,
    Local,
}

#[derive(Debug, Clone)]
/// Regular vector samples on a local XY plane. `resolution` is the number of
/// sample points, matching `vectors.len()`.
pub struct DebugVectorField {
    pub id: DebugId,
    pub transform: Transform,
    pub size: Vec2,
    pub resolution: UVec2,
    pub vectors: Vec<Vec3>,
    pub vector_space: DebugVectorSpace,
    pub vector_scale: f32,
    pub maximum_arrow_length: f32,
    pub color: Color,
    pub magnitude_range: Option<DebugScalarRange>,
    pub magnitude_ramp: DebugColorRamp,
    pub title: String,
    pub unit: &'static str,
    pub legend: bool,
}

#[derive(Debug, Default)]
pub struct DebugFrameBatch {
    pub primitives: Vec<DebugPrimitive>,
    pub labels: Vec<DebugLabel>,
    pub scalar_fields: Vec<DebugScalarField>,
    pub vector_fields: Vec<DebugVectorField>,
}

impl DebugFrameBatch {
    /// Default screen-space font size for focused object annotations.
    pub const DEFAULT_LABEL_FONT_SIZE: f32 = 12.0;

    /// Screen-space font size used by the remaining gizmo field legends.
    pub const DEFAULT_LEGEND_FONT_SIZE: f32 = 12.0;


    pub fn line(&mut self, start: Vec3, end: Vec3, color: Color, depth: DebugDepth) {
        self.primitives.push(DebugPrimitive::Line {
            start,
            end,
            color,
            depth,
        });
    }

    pub fn arrow(&mut self, start: Vec3, end: Vec3, color: Color, depth: DebugDepth) {
        self.primitives.push(DebugPrimitive::Arrow {
            start,
            end,
            color,
            depth,
        });
    }

    pub fn axes(&mut self, transform: Transform, length: f32, depth: DebugDepth) {
        self.primitives.push(DebugPrimitive::Axes {
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
        depth: DebugDepth,
    ) {
        self.primitives.push(DebugPrimitive::Rect {
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
        depth: DebugDepth,
    ) {
        self.primitives.push(DebugPrimitive::Cross {
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
        depth: DebugDepth,
    ) {
        self.primitives.push(DebugPrimitive::Sphere {
            isometry,
            radius,
            color,
            resolution,
            depth,
        });
    }

    pub fn label(
        &mut self,
        position: Vec3,
        text: impl Into<String>,
        font_size: f32,
        color: Color,
    ) {
        self.labels.push(DebugLabel {
            position,
            text: text.into(),
            font_size,
            // Object annotations use their world anchor as the center point.
            anchor: Vec2::ZERO,
            color,
            subject: None,
            facing: DebugTextFacing::Billboard,
        });
    }

    /// Object-scoped annotation. The renderer only presents labels whose
    /// subject matches the currently focused concrete or semantic entity.
    pub fn label_for(
        &mut self,
        subject: Entity,
        position: Vec3,
        text: impl Into<String>,
        font_size: f32,
        color: Color,
    ) {
        self.labels.push(DebugLabel {
            position,
            text: text.into(),
            font_size,
            anchor: Vec2::ZERO,
            color,
            subject: Some(subject),
            facing: DebugTextFacing::Billboard,
        });
    }

    pub fn scalar_field(&mut self, field: DebugScalarField) {
        self.scalar_fields.push(field);
    }

    pub fn vector_field(&mut self, field: DebugVectorField) {
        self.vector_fields.push(field);
    }
}

#[derive(Resource, Debug, Default)]
pub struct DebugFrame {
    inner: RwLock<DebugFrameBatch>,
}

impl DebugFrame {
    pub fn submit(&self, mut batch: DebugFrameBatch) {
        let mut frame = self.inner.write().expect("debug frame lock poisoned");

        // Field IDs are semantic identities used by retained renderer caches and
        // external tooling. Duplicate IDs would make output depend on parallel
        // collector completion order, so reject them instead of silently using
        // whichever producer happened to submit last.
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
            assert!(ids.insert(id), "duplicate debug field observation id {id}");
        }

        frame.primitives.append(&mut batch.primitives);
        frame.labels.append(&mut batch.labels);
        frame.scalar_fields.append(&mut batch.scalar_fields);
        frame.vector_fields.append(&mut batch.vector_fields);
    }

    fn clear(&self) {
        let mut frame = self.inner.write().expect("debug frame lock poisoned");
        frame.primitives.clear();
        frame.labels.clear();
        frame.scalar_fields.clear();
        frame.vector_fields.clear();
    }

    pub(super) fn read(&self) -> RwLockReadGuard<'_, DebugFrameBatch> {
        self.inner.read().expect("debug frame lock poisoned")
    }
}

pub(super) fn clear_debug_frame(frame: Res<DebugFrame>) {
    frame.clear();
}
