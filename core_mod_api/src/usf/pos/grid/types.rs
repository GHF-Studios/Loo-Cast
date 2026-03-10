use crate::bevy::prelude::*;
use crate::bevy::reflect::Reflect;
use bevy_inspector_egui::inspector_egui_impls::InspectorPrimitive;
use bevy_inspector_egui::reflect_inspector::InspectorUi;
use egui::{Id, Ui};
use std::any::Any;
use std::sync::Arc;

use crate::usf::pos::unit::types::UnitVec;
use crate::usf::scale::Scale;

#[derive(Default)]
pub struct GridVecBuilder {
    chain: Vec<IVec3>,
}

impl GridVecBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(mut self, next: (i32, i32, i32)) -> Self {
        let next = IVec3::new(next.0, next.1, next.2);
        self.chain.push(next);
        self
    }

    pub fn push_many<I: IntoIterator<Item = (i32, i32, i32)>>(mut self, items: I) -> Self {
        self.chain
            .extend(items.into_iter().map(|xyz| IVec3::new(xyz.0, xyz.1, xyz.2)));
        self
    }

    pub fn repeat(mut self, xyz: (i32, i32, i32), count: usize) -> Self {
        self.chain
            .extend(std::iter::repeat_n(IVec3::new(xyz.0, xyz.1, xyz.2), count));
        self
    }

    // Takes in a closure for modifying the Builder to repeat something (like push or default_xy for example)
    pub fn repeat_n<F>(mut self, count: usize, mut f: F) -> Self
    where
        F: FnMut(Self) -> Self,
    {
        for _ in 0..count {
            self = f(self);
        }
        self
    }

    pub fn reverse(mut self) -> Self {
        self.chain.reverse();
        self
    }

    pub fn default_xyz(mut self) -> Self {
        self.chain.push(IVec3::ZERO);
        self
    }

    #[track_caller]
    pub fn finish(self) -> GridVec {
        GridVec::try_from(self.chain).unwrap()
    }
}

#[derive(Default, Clone, Reflect)]
pub struct GridVec {
    pub(crate) parent: Option<Arc<GridVec>>,
    pub(crate) scale: Scale,
    pub(crate) xy: IVec2,
    pub(crate) z: i32,
}
impl GridVec {
    pub const MAX_DEPTH: usize = 71;

    pub fn build() -> GridVecBuilder {
        GridVecBuilder::new()
    }

    /// Create a GridVec with all ancestors up to the root at Scale::MAX, pre-filled with IVec2::ZERO.
    pub fn default_n(count: usize) -> Self {
        Self::build().repeat_n(count, |b| b.default_xyz()).finish()
    }

    #[track_caller]
    fn validate_xy(xy: &IVec2) {
        if xy.x < -5 {
            panic!("X coordinate {} is too small. Range is (-5..5)", xy.x);
        }
        if xy.x >= 5 {
            panic!("X coordinate {} is too large. Range is (-5..5)", xy.x);
        }
        if xy.y < -5 {
            panic!("Y coordinate {} is too small. Range is (-5..5)", xy.y);
        }
        if xy.y >= 5 {
            panic!("Y coordinate {} is too large. Range is (-5..5)", xy.y);
        }
    }

    #[track_caller]
    fn validate_z(z: i32) {
        if z < -5 {
            panic!("Z coordinate {} is too small. Range is (-5..5)", z);
        }
        if z >= 5 {
            panic!("Z coordinate {} is too large. Range is (-5..5)", z);
        }
    }

    #[track_caller]
    fn try_validate_xy(xy: &IVec2) -> Result<(), String> {
        if xy.x < -5 {
            return Err(format!("X coordinate {} is too small. Range is (-5..5)", xy.x));
        }
        if xy.x >= 5 {
            return Err(format!("X coordinate {} is too large. Range is (-5..5)", xy.x));
        }
        if xy.y < -5 {
            return Err(format!("Y coordinate {} is too small. Range is (-5..5)", xy.y));
        }
        if xy.y >= 5 {
            return Err(format!("Y coordinate {} is too large. Range is (-5..5)", xy.y));
        }

        Ok(())
    }

    #[track_caller]
    fn try_validate_z(z: i32) -> Result<(), String> {
        if z < -5 {
            return Err(format!("Z coordinate {} is too small. Range is (-5..5)", z));
        }
        if z >= 5 {
            return Err(format!("Z coordinate {} is too large. Range is (-5..5)", z));
        }
        Ok(())
    }

    /// Create a GridVec with random (yet valid) coordinates, from the root, down to and including the specified scale, with the same random coords at each scale.
    #[track_caller]
    pub fn new_random_homo(_scale: Scale) -> Self {
        todo!()
    }

    /// Create a GridVec with random (yet valid) coordinates, from the root, down to and including the specified scale, with different random coords at each scale.
    #[track_caller]
    pub fn new_random_hetero(_scale: Scale) -> Self {
        todo!()
    }

    /// Create a GridVec at the absolute root (Scale::MAX) with no parent.
    #[track_caller]
    pub fn new_root(xy: IVec2) -> Self {
        Self::validate_xy(&xy);
        let mut my_self = Self {
            parent: None,
            scale: Scale::MAX,
            xy,
            z: 0,
        };
        my_self.normalize();
        my_self
    }

    /// Create a GridVec at the absolute root (Scale::MAX) with no parent.
    #[track_caller]
    pub fn new_root_unchecked(xy: IVec2) -> Self {
        let mut my_self = Self {
            parent: None,
            scale: Scale::MAX,
            xy,
            z: 0,
        };
        my_self.normalize();
        my_self
    }

    /// Create a GridVec with the specified parent and xy. The parent can be thought of as a stack onto which we push another level.
    #[track_caller]
    pub fn new(parent: GridVec, xy: IVec2) -> Self {
        Self::validate_xy(&xy);
        if parent.scale == Scale::MIN {
            panic!("Cannot create a child GridVec from a parent at Scale::MIN, as there is no smaller scale.");
        }
        let scale = parent.scale.zoomed_in();
        let parent = Some(Arc::new(parent));

        let mut my_self = Self { parent, scale, xy, z: 0 };
        my_self.normalize();
        my_self
    }

    /// Create a GridVec with the specified parent and xy. The parent can be thought of as a stack onto which we push another level.
    #[track_caller]
    pub fn new_unchecked(parent: GridVec, xy: IVec2) -> Self {
        let scale = parent.scale.zoomed_in();
        let parent = Some(Arc::new(parent));

        let mut my_self = Self { parent, scale, xy, z: 0 };
        my_self.normalize();
        my_self
    }

    /// Create a GridVec with all ancestors up, from the specified scale to the root at Scale::MAX, pre-filled with IVec2::ZERO, except for the leaf at the specified scale, which is set to the specified xy.
    #[track_caller]
    pub fn new_at_scale(scale: Scale, xy: IVec2) -> Self {
        Self::validate_xy(&xy);
        if scale == Scale::MAX {
            return Self::new_root(xy);
        }

        let mut current = Self::new_root(IVec2::ZERO);
        let mut current_scale = Scale::MAX;

        while current_scale > scale {
            current_scale = current_scale.zoomed_in();
            current = Self::new(current, IVec2::ZERO);
        }

        let mut my_self = Self {
            parent: current.parent,
            scale,
            xy,
            z: 0,
        };
        my_self.normalize();
        my_self
    }

    /// Create a GridVec with all ancestors, from the specified scale up to the root at Scale::MAX, pre-filled with the specified xy.
    #[track_caller]
    pub fn new_splat(scale: Scale, xy: IVec2) -> Self {
        Self::validate_xy(&xy);
        if scale == Scale::MAX {
            return Self::new_root(xy);
        }

        let mut current = Self::new_root(xy);
        let mut current_scale = Scale::MAX;

        while current_scale > scale {
            current_scale = current_scale.zoomed_in();
            current = Self::new(current, xy);
        }

        let mut my_self = Self {
            parent: current.parent,
            scale,
            xy,
            z: 0,
        };
        my_self.normalize();
        my_self
    }

    #[track_caller]
    pub fn is_zero(&self) -> bool {
        let mut cursor = self;

        while cursor.scale < Scale::MAX {
            if cursor.xy != IVec2::ZERO || cursor.z != 0 {
                return false;
            }

            cursor = cursor.parent.as_ref().unwrap().as_ref();
        }

        if cursor.xy != IVec2::ZERO || cursor.z != 0 {
            return false;
        }

        true
    }

    /// Converts an `[IVec3; N]` from root to leaf into a GridVec
    pub fn from_raw<const N: usize>(raw: [IVec3; N]) -> Result<Self, &'static str> {
        if N == 0 {
            return Err("Cannot build a GridVec from an empty array");
        }
        if N > Self::MAX_DEPTH {
            return Err("Too many levels in GridVec::from_raw");
        }

        let mut current = GridVec::new_root(raw[0].xy());
        current.z = raw[0].z;
        current.normalize();
        for &xyz in &raw[1..] {
            let mut next = GridVec::new(current, xyz.xy());
            next.z = xyz.z;
            next.normalize();
            current = next;
        }

        Ok(current)
    }

    /// Converts a GridVec into an `[IVec3; N]` array
    /// If the GridVec's actual depth does not match N, this will return `None`
    pub fn to_raw<const N: usize>(&self) -> Option<[IVec3; N]> {
        if N > Self::MAX_DEPTH {
            return None;
        }

        let mut stack = [IVec3::ZERO; N];
        let mut cursor = self;
        let mut len = 0;

        loop {
            if len >= N {
                return None; // Mismatch: too deep
            }
            stack[N - 1 - len] = IVec3::new(cursor.xy.x, cursor.xy.y, cursor.z);
            len += 1;
            match &cursor.parent {
                Some(parent) => cursor = parent,
                None => break,
            }
        }

        if len != N {
            return None;
        }

        Some(stack)
    }

    /// Converts a GridVec into a `Vec<IVec3>`
    pub fn to_raw_vec(&self) -> Option<Vec<IVec3>> {
        let mut stack = Vec::new();
        let mut cursor = self;

        loop {
            stack.push(IVec3::new(cursor.xy.x, cursor.xy.y, cursor.z));
            match &cursor.parent {
                Some(parent) => cursor = parent,
                None => break,
            }
        }

        stack.reverse();
        Some(stack)
    }

    fn accumulate_grid_units(diff_grid: &GridVec) -> (f64, f64, f64) {
        let mut acc_x = 0.0_f64;
        let mut acc_y = 0.0_f64;
        let mut acc_z = 0.0_f64;
        let mut factor = 1.0_f64;
        let mut depth = 0_usize;

        let mut cursor = diff_grid;
        loop {
            let term_x = cursor.xy.x as f64 * factor;
            let term_y = cursor.xy.y as f64 * factor;
            let term_z = cursor.z as f64 * factor;
            acc_x += term_x;
            acc_y += term_y;
            acc_z += term_z;

            if let Some(parent) = &cursor.parent {
                factor *= 10.0;
                if !factor.is_finite() {
                    panic!("USF grid accumulation panic: factor became non-finite at depth {}, factor={}", depth, factor);
                }
                cursor = parent;
                depth += 1;
            } else {
                break;
            }
        }

        if !acc_x.is_finite() || !acc_y.is_finite() || !acc_z.is_finite() {
            panic!(
                "USF grid accumulation panic: non-finite accumulated coordinates, acc=({acc_x}, {acc_y}, {acc_z}), root_scale={:?}",
                diff_grid.scale
            );
        }

        (acc_x, acc_y, acc_z)
    }

    /// - Assumes that the given `scale` is greater than or equal to that of `self`.
    /// - Assumes that the parent of `grid_diff` is the same as `self`'s parent.
    #[track_caller]
    pub fn from_native_logical(origin: Self, (native_logical_pos, scale): (Vec2, Scale)) -> Self {
        assert!(scale == origin.scale);
        let unit_offset_x = native_logical_pos.x / 1000.0;
        let unit_offset_y = native_logical_pos.y / 1000.0;

        let diff_x = unit_offset_x.round() as i32;
        let diff_y = unit_offset_y.round() as i32;

        let diff = GridVec {
            parent: origin.parent.clone(),
            scale,
            xy: IVec2::new(diff_x, diff_y),
            z: 0,
        };

        origin + diff
    }

    /// 3D variant of native logical conversion.
    #[track_caller]
    pub fn from_native_logical_3d(origin: Self, (native_logical_pos, scale): (Vec3, Scale)) -> Self {
        assert!(scale == origin.scale);
        let unit_offset_x = native_logical_pos.x / 1000.0;
        let unit_offset_y = native_logical_pos.y / 1000.0;
        let unit_offset_z = native_logical_pos.z / 1000.0;

        let diff = GridVec {
            parent: origin.parent.clone(),
            scale,
            xy: IVec2::new(unit_offset_x.round() as i32, unit_offset_y.round() as i32),
            z: unit_offset_z.round() as i32,
        };

        origin + diff
    }

    /// - Assumes that the given `scale` is greater than or equal to that of `self`.
    /// - Assumes that the parent of `grid_diff` is the same as `self`'s parent.
    #[track_caller]
    pub fn from_native_visual(origin: Self, native_visual_pos: Vec2, scale: Scale) -> Self {
        assert!(scale == origin.scale);
        let scale_diff = scale as i8 - origin.scale as i8;
        let scale_factor = 10.0_f32.powi(scale_diff as i32);
        let native_unit = 1000.0 * scale_factor;

        let unit_offset_x = native_visual_pos.x / native_unit;
        let unit_offset_y = native_visual_pos.y / native_unit;

        let diff_x = unit_offset_x.round() as i32;
        let diff_y = unit_offset_y.round() as i32;

        let diff = GridVec {
            parent: origin.parent.clone(),
            scale,
            xy: IVec2::new(diff_x, diff_y),
            z: 0,
        };

        origin + diff
    }

    /// 3D variant of native visual conversion.
    #[track_caller]
    pub fn from_native_visual_3d(origin: Self, native_visual_pos: Vec3, scale: Scale) -> Self {
        assert!(scale == origin.scale);
        let scale_diff = scale as i8 - origin.scale as i8;
        let scale_factor = 10.0_f32.powi(scale_diff as i32);
        let native_unit = 1000.0 * scale_factor;

        let diff = GridVec {
            parent: origin.parent.clone(),
            scale,
            xy: IVec2::new(
                (native_visual_pos.x / native_unit).round() as i32,
                (native_visual_pos.y / native_unit).round() as i32,
            ),
            z: (native_visual_pos.z / native_unit).round() as i32,
        };

        origin + diff
    }

    /// - Assumes that `self`'s scale is greater than or equal to that of `origin`.
    /// - Assumes that the parent of `self` is the same as `origin`'s parent.
    #[track_caller]
    pub fn to_native_logical(self, origin: Self) -> Vec2 {
        assert!(self.scale >= origin.scale);
        let scale_diff = self.scale as i8 - origin.scale as i8;
        let self_unit = UnitVec {
            grid_offset: self.clone(),
            unit_offset: Vec3::ZERO,
        };
        let origin_unit = UnitVec {
            grid_offset: origin.clone(),
            unit_offset: Vec3::ZERO,
        };
        let diff_unit = self_unit - origin_unit;
        // Keep native spacing invariant (1000.0) and derive cross-scale distance from the
        // grid stack significance itself (leaf=10^0, parent=10^1, ...).
        let native_unit = 1000.0_f64;
        let acc = Self::accumulate_grid_units(&diff_unit.grid_offset);
        // Grid digits are centered in a [-5, 4] domain; when comparing a coarser
        // chunk against a finer origin, missing lower digits imply a half-cell bias.
        // Sum_{k=0..d-1}(10^k) * 0.5 converts that bias into origin-scale units.
        let center_bias_units = if scale_diff > 0 {
            0.5 * ((10.0_f64.powi(scale_diff as i32) - 1.0) / 9.0)
        } else {
            0.0
        };
        let native_x = ((acc.0 - center_bias_units) * native_unit) as f32;
        let native_y = ((acc.1 - center_bias_units) * native_unit) as f32;
        if !native_x.is_finite() || !native_y.is_finite() {
            panic!(
                "USF native logical conversion panic: non-finite viewport coords, native=({native_x}, {native_y}), acc=({:.3e}, {:.3e}), self_scale={:?}, origin_scale={:?}",
                acc.0, acc.1, self.scale, origin.scale
            );
        }

        Vec2::new(native_x, native_y)
    }

    /// 3D variant of native logical conversion.
    #[track_caller]
    pub fn to_native_logical_3d(self, origin: Self) -> Vec3 {
        assert!(self.scale >= origin.scale);
        let scale_diff = self.scale as i8 - origin.scale as i8;
        let self_unit = UnitVec {
            grid_offset: self.clone(),
            unit_offset: Vec3::ZERO,
        };
        let origin_unit = UnitVec {
            grid_offset: origin.clone(),
            unit_offset: Vec3::ZERO,
        };
        let diff_unit = self_unit - origin_unit;
        let native_unit = 1000.0_f64;
        let acc = Self::accumulate_grid_units(&diff_unit.grid_offset);
        let center_bias_units = if scale_diff > 0 {
            0.5 * ((10.0_f64.powi(scale_diff as i32) - 1.0) / 9.0)
        } else {
            0.0
        };
        let native_x = ((acc.0 - center_bias_units) * native_unit) as f32;
        let native_y = ((acc.1 - center_bias_units) * native_unit) as f32;
        let native_z = ((acc.2 - center_bias_units) * native_unit) as f32;
        if !native_x.is_finite() || !native_y.is_finite() || !native_z.is_finite() {
            panic!(
                "USF native logical conversion panic: non-finite viewport coords, native=({native_x}, {native_y}, {native_z}), acc=({:.3e}, {:.3e}, {:.3e}), self_scale={:?}, origin_scale={:?}",
                acc.0, acc.1, acc.2, self.scale, origin.scale
            );
        }

        Vec3::new(native_x, native_y, native_z)
    }

    /// - Assumes that `self`'s scale is greater than or equal to that of `origin`.
    /// - Assumes that the parent of `self` is the same as `origin`'s parent.
    #[track_caller]
    pub fn to_native_visual(self, origin: Self) -> (Vec2, f32) {
        assert!(self.scale >= origin.scale);
        let scale_diff = self.scale as i8 - origin.scale as i8;
        let self_unit = UnitVec {
            grid_offset: self.clone(),
            unit_offset: Vec3::ZERO,
        };
        let origin_unit = UnitVec {
            grid_offset: origin.clone(),
            unit_offset: Vec3::ZERO,
        };
        let diff_unit = self_unit - origin_unit;

        let scale = 10.0_f32.powi(scale_diff as i32);
        // Translation spacing remains in native 1000.0 units; visual size scaling is returned
        // separately via `scale`.
        let native_unit = 1000.0_f64;
        let acc = Self::accumulate_grid_units(&diff_unit.grid_offset);
        // Grid digits are centered in a [-5, 4] domain; when comparing a coarser
        // chunk against a finer origin, missing lower digits imply a half-cell bias.
        // Sum_{k=0..d-1}(10^k) * 0.5 converts that bias into origin-scale units.
        let center_bias_units = if scale_diff > 0 {
            0.5 * ((10.0_f64.powi(scale_diff as i32) - 1.0) / 9.0)
        } else {
            0.0
        };
        let native_x = ((acc.0 - center_bias_units) * native_unit) as f32;
        let native_y = ((acc.1 - center_bias_units) * native_unit) as f32;
        if !native_x.is_finite() || !native_y.is_finite() {
            panic!(
                "USF native visual conversion panic: non-finite viewport coords, native=({native_x}, {native_y}), acc=({:.3e}, {:.3e}), self_scale={:?}, origin_scale={:?}",
                acc.0, acc.1, self.scale, origin.scale
            );
        }

        (Vec2::new(native_x, native_y), scale)
    }

    /// 3D variant of native visual conversion.
    #[track_caller]
    pub fn to_native_visual_3d(self, origin: Self) -> (Vec3, f32) {
        assert!(self.scale >= origin.scale);
        let scale_diff = self.scale as i8 - origin.scale as i8;
        let self_unit = UnitVec {
            grid_offset: self.clone(),
            unit_offset: Vec3::ZERO,
        };
        let origin_unit = UnitVec {
            grid_offset: origin.clone(),
            unit_offset: Vec3::ZERO,
        };
        let diff_unit = self_unit - origin_unit;

        let scale = 10.0_f32.powi(scale_diff as i32);
        let native_unit = 1000.0_f64;
        let acc = Self::accumulate_grid_units(&diff_unit.grid_offset);
        let center_bias_units = if scale_diff > 0 {
            0.5 * ((10.0_f64.powi(scale_diff as i32) - 1.0) / 9.0)
        } else {
            0.0
        };
        let native_x = ((acc.0 - center_bias_units) * native_unit) as f32;
        let native_y = ((acc.1 - center_bias_units) * native_unit) as f32;
        let native_z = ((acc.2 - center_bias_units) * native_unit) as f32;
        if !native_x.is_finite() || !native_y.is_finite() || !native_z.is_finite() {
            panic!(
                "USF native visual conversion panic: non-finite viewport coords, native=({native_x}, {native_y}, {native_z}), acc=({:.3e}, {:.3e}, {:.3e}), self_scale={:?}, origin_scale={:?}",
                acc.0, acc.1, acc.2, self.scale, origin.scale
            );
        }

        (Vec3::new(native_x, native_y, native_z), scale)
    }

    // TODO: REFACTOR: PERF: This is much less performant than it could be;
    //      it just abuses the fact that Add(and Sub) internally perform a wrap, by just doing +/- "zero", or rather a default that equates in a no-op, akin to the concept of "zero".
    //      In short: It's a fast, dirty, and lazy solution
    /// Recursively normalize the GridVec to ensure all coordinates are within valid ranges.
    #[track_caller]
    pub fn normalize(&mut self) {
        let zero = GridVec::default();
        let normalized = self.clone() + zero;
        self.parent = normalized.parent;
        self.scale = normalized.scale;
        self.xy = normalized.xy;
        self.z = normalized.z;
    }

    #[track_caller]
    pub fn normalized(&mut self) -> &mut Self {
        self.normalize();
        self
    }

    #[track_caller]
    pub fn zoom_in(&mut self, logical_world_pos: Vec3) -> Vec3 {
        let mut unit_pos = UnitVec::new(self.clone(), logical_world_pos);
        unit_pos.zoom_in();
        self.parent = unit_pos.grid_offset.parent;
        self.scale = unit_pos.grid_offset.scale;
        self.xy = unit_pos.grid_offset.xy;
        self.z = unit_pos.grid_offset.z;
        unit_pos.unit_offset
    }

    #[track_caller]
    pub fn zoom_out(&mut self) {
        let mut unit_extent = UnitVec {
            grid_offset: self.clone(),
            unit_offset: Vec3::ZERO,
        };
        unit_extent.zoom_out();
        self.parent = unit_extent.grid_offset.parent;
        self.scale = unit_extent.grid_offset.scale;
        self.xy = unit_extent.grid_offset.xy;
        self.z = unit_extent.grid_offset.z;
    }

    #[track_caller]
    pub fn query_grid_radius(&self, radius: u32) -> Vec<GridVec> {
        if radius == 0 {
            return vec![self.clone()];
        }

        let radius = radius as i32;
        let mut raw_offsets = Vec::with_capacity(((radius * 2 + 1).pow(3)) as usize);

        for dz in -radius..=radius {
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    raw_offsets.push(self.clone() + IVec3::new(dx, dy, dz));
                }
            }
        }

        raw_offsets
    }

    /// Returns a root-to-leaf stack that preserves Z components.
    pub fn to_raw_vec_3d(&self) -> Vec<IVec3> {
        let mut stack = Vec::new();
        let mut cursor = self;

        loop {
            stack.push(IVec3::new(cursor.xy.x, cursor.xy.y, cursor.z));
            match &cursor.parent {
                Some(parent) => cursor = parent,
                None => break,
            }
        }

        stack.reverse();
        stack
    }
}
impl std::fmt::Debug for GridVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        let mut cursor = self;
        loop {
            let suffix = if msg.is_empty() { String::new() } else { format!(", {}", msg) };
            msg = format!("({}, {}, {}){}", cursor.xy.x, cursor.xy.y, cursor.z, suffix);
            if let Some(p) = &cursor.parent {
                cursor = p;
            } else {
                break;
            }
        }

        write!(f, "[{msg}]")
    }
}
impl PartialEq for GridVec {
    fn eq(&self, other: &Self) -> bool {
        self.clone().normalized().to_raw_vec_3d() == other.clone().normalized().to_raw_vec_3d()
    }
}
impl Eq for GridVec {}
impl std::hash::Hash for GridVec {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.clone().normalized().to_raw_vec_3d().hash(state);
    }
}
impl std::ops::Add<IVec2> for GridVec {
    type Output = Self;

    #[track_caller]
    fn add(mut self, rhs: IVec2) -> Self::Output {
        self.xy += rhs;
        if Self::try_validate_xy(&self.xy).is_err() {
            self.normalize();
        }
        self
    }
}
impl std::ops::Add<IVec3> for GridVec {
    type Output = Self;

    #[track_caller]
    fn add(mut self, rhs: IVec3) -> Self::Output {
        self.xy += rhs.xy();
        self.z += rhs.z;
        if Self::try_validate_xy(&self.xy).is_err() || Self::try_validate_z(self.z).is_err() {
            self.normalize();
        }
        self
    }
}
impl std::ops::AddAssign<IVec2> for GridVec {
    #[track_caller]
    fn add_assign(&mut self, rhs: IVec2) {
        self.xy += rhs;
        if Self::try_validate_xy(&self.xy).is_err() {
            self.normalize();
        }
    }
}
impl std::ops::AddAssign<IVec3> for GridVec {
    #[track_caller]
    fn add_assign(&mut self, rhs: IVec3) {
        self.xy += rhs.xy();
        self.z += rhs.z;
        if Self::try_validate_xy(&self.xy).is_err() || Self::try_validate_z(self.z).is_err() {
            self.normalize();
        }
    }
}
impl std::ops::Sub<IVec2> for GridVec {
    type Output = Self;

    #[track_caller]
    fn sub(mut self, rhs: IVec2) -> Self::Output {
        self.xy -= rhs;
        if Self::try_validate_xy(&self.xy).is_err() {
            self.normalize();
        }
        self
    }
}
impl std::ops::Sub<IVec3> for GridVec {
    type Output = Self;

    #[track_caller]
    fn sub(mut self, rhs: IVec3) -> Self::Output {
        self.xy -= rhs.xy();
        self.z -= rhs.z;
        if Self::try_validate_xy(&self.xy).is_err() || Self::try_validate_z(self.z).is_err() {
            self.normalize();
        }
        self
    }
}
impl std::ops::SubAssign<IVec2> for GridVec {
    #[track_caller]
    fn sub_assign(&mut self, rhs: IVec2) {
        self.xy -= rhs;
        if Self::try_validate_xy(&self.xy).is_err() {
            self.normalize();
        }
    }
}
impl std::ops::SubAssign<IVec3> for GridVec {
    #[track_caller]
    fn sub_assign(&mut self, rhs: IVec3) {
        self.xy -= rhs.xy();
        self.z -= rhs.z;
        if Self::try_validate_xy(&self.xy).is_err() || Self::try_validate_z(self.z).is_err() {
            self.normalize();
        }
    }
}
impl std::ops::Add<GridVec> for GridVec {
    type Output = GridVec;

    #[track_caller]
    fn add(self, rhs: GridVec) -> Self::Output {
        // === Phase 1: Collect full stack from root to leaf ===
        fn stack_up(mut cursor: &GridVec) -> Vec<(Scale, IVec3)> {
            let mut stack = Vec::new();
            loop {
                stack.push((cursor.scale, IVec3::new(cursor.xy.x, cursor.xy.y, cursor.z)));
                if let Some(p) = &cursor.parent {
                    cursor = p;
                } else {
                    break;
                }
            }
            stack.reverse();
            stack
        }

        let mut a_stack = stack_up(&self);
        let mut b_stack = stack_up(&rhs);

        let max_depth = a_stack.len().max(b_stack.len());

        // Pad shorter stack with (scale, ZERO)
        while a_stack.len() < max_depth {
            let (s, _) = b_stack[a_stack.len()];
            a_stack.push((s, IVec3::ZERO));
        }
        while b_stack.len() < max_depth {
            let (s, _) = a_stack[b_stack.len()];
            b_stack.push((s, IVec3::ZERO));
        }

        // === Phase 2: Accumulate raw sums top-down ===
        let mut raw_stack = Vec::with_capacity(max_depth);
        for i in 0..max_depth {
            let scale = a_stack[i].0; // should match in both stacks
            let sum = a_stack[i].1 + b_stack[i].1;
            raw_stack.push((scale, sum));
        }

        // === Phase 3: Normalize bottom-up with wrapping + carry ===
        let mut carry = IVec3::ZERO;
        for i in (0..raw_stack.len()).rev() {
            let (_scale, sum) = raw_stack[i];
            let wrapped_x = ((sum.x + carry.x + 5).rem_euclid(10)) - 5;
            let wrapped_y = ((sum.y + carry.y + 5).rem_euclid(10)) - 5;
            let wrapped_z = ((sum.z + carry.z + 5).rem_euclid(10)) - 5;
            let carry_x = (sum.x + carry.x - wrapped_x).div_euclid(10);
            let carry_y = (sum.y + carry.y - wrapped_y).div_euclid(10);
            let carry_z = (sum.z + carry.z - wrapped_z).div_euclid(10);

            raw_stack[i].1 = IVec3::new(wrapped_x, wrapped_y, wrapped_z);
            carry = IVec3::new(carry_x, carry_y, carry_z);
        }

        // === Phase 4: Build final GridVec tree ===
        let mut result: Option<GridVec> = None;
        for (scale, xyz) in raw_stack {
            result = Some(GridVec {
                parent: result.map(Arc::new),
                scale,
                xy: xyz.xy(),
                z: xyz.z,
            })
        }

        result.expect("GridVec addition should yield a result")
    }
}
impl std::ops::AddAssign<GridVec> for GridVec {
    #[track_caller]
    fn add_assign(&mut self, rhs: GridVec) {
        *self = self.clone() + rhs;
    }
}
impl std::ops::Sub<GridVec> for GridVec {
    type Output = Self;

    #[track_caller]
    fn sub(self, rhs: GridVec) -> Self::Output {
        // === Phase 1: Collect full stack from root to leaf ===
        fn stack_up(mut cursor: &GridVec) -> Vec<(Scale, IVec3)> {
            let mut stack = Vec::new();
            loop {
                stack.push((cursor.scale, IVec3::new(cursor.xy.x, cursor.xy.y, cursor.z)));
                if let Some(p) = &cursor.parent {
                    cursor = p;
                } else {
                    break;
                }
            }
            stack.reverse();
            stack
        }

        let mut a_stack = stack_up(&self);
        let mut b_stack = stack_up(&rhs);

        let max_depth = a_stack.len().max(b_stack.len());

        // Pad shorter stack with (scale, ZERO)
        while a_stack.len() < max_depth {
            let (s, _) = b_stack[a_stack.len()];
            a_stack.push((s, IVec3::ZERO));
        }
        while b_stack.len() < max_depth {
            let (s, _) = a_stack[b_stack.len()];
            b_stack.push((s, IVec3::ZERO));
        }

        // === Phase 2: Accumulate raw diffs top-down ===
        let mut raw_stack = Vec::with_capacity(max_depth);
        for i in 0..max_depth {
            let scale = a_stack[i].0; // should match in both stacks
            let diff = a_stack[i].1 - b_stack[i].1;
            raw_stack.push((scale, diff));
        }

        // === Phase 3: Normalize bottom-up with wrapping + carry ===
        let mut carry = IVec3::ZERO;
        for i in (0..raw_stack.len()).rev() {
            let (_scale, diff) = raw_stack[i];
            let wrapped_x = ((diff.x + carry.x + 5).rem_euclid(10)) - 5;
            let wrapped_y = ((diff.y + carry.y + 5).rem_euclid(10)) - 5;
            let wrapped_z = ((diff.z + carry.z + 5).rem_euclid(10)) - 5;
            let carry_x = (diff.x + carry.x - wrapped_x).div_euclid(10);
            let carry_y = (diff.y + carry.y - wrapped_y).div_euclid(10);
            let carry_z = (diff.z + carry.z - wrapped_z).div_euclid(10);

            raw_stack[i].1 = IVec3::new(wrapped_x, wrapped_y, wrapped_z);
            carry = IVec3::new(carry_x, carry_y, carry_z);
        }

        // === Phase 4: Build final GridVec tree ===
        let mut result: Option<GridVec> = None;
        for (scale, xyz) in raw_stack {
            result = Some(GridVec {
                parent: result.map(Arc::new),
                scale,
                xy: xyz.xy(),
                z: xyz.z,
            })
        }

        result.expect("GridVec subtraction should yield a result")
    }
}
impl std::ops::SubAssign<GridVec> for GridVec {
    #[track_caller]
    fn sub_assign(&mut self, rhs: GridVec) {
        *self = self.clone() - rhs;
    }
}
impl std::convert::TryFrom<Vec<IVec2>> for GridVec {
    type Error = &'static str;

    fn try_from(stack: Vec<IVec2>) -> Result<Self, Self::Error> {
        if stack.is_empty() {
            return Err("GridVec stack must contain at least one element");
        }

        let mut iter = stack.into_iter();
        let mut current = GridVec::new_root(iter.next().unwrap());

        for xy in iter {
            current = GridVec::new(current, xy);
        }

        Ok(current)
    }
}
impl std::convert::TryFrom<Vec<IVec3>> for GridVec {
    type Error = &'static str;

    fn try_from(stack: Vec<IVec3>) -> Result<Self, Self::Error> {
        if stack.is_empty() {
            return Err("GridVec stack must contain at least one element");
        }

        let mut iter = stack.into_iter();
        let first = iter.next().unwrap();
        let mut current = GridVec::new_root(first.xy());
        current.z = first.z;
        current.normalize();

        for xyz in iter {
            let mut next = GridVec::new(current, xyz.xy());
            next.z = xyz.z;
            next.normalize();
            current = next;
        }

        Ok(current)
    }
}
impl InspectorPrimitive for GridVec {
    fn ui(&mut self, ui: &mut Ui, _options: &dyn Any, _id: Id, _env: InspectorUi<'_, '_>) -> bool {
        let mut changed = false;

        // Step 1: Convert to Vec<IVec3>
        let mut coords = self.to_raw_vec_3d();

        // Step 2: Dynamic editable UI
        ui.horizontal_wrapped(|ui| {
            for (i, xyz) in coords.iter_mut().enumerate() {
                let scale = 35 - i as i8;
                let mut vec3 = [xyz.x as f32, xyz.y as f32, xyz.z as f32];

                let r = ui.add(egui::DragValue::new(&mut vec3[0]).speed(1.0));
                ui.add(egui::DragValue::new(&mut vec3[1]).speed(1.0));
                ui.add(egui::DragValue::new(&mut vec3[2]).speed(1.0));
                r.on_hover_text(format!("10^{scale}m"));

                let updated = IVec3::new(vec3[0] as i32, vec3[1] as i32, vec3[2] as i32);
                if *xyz != updated {
                    *xyz = updated;
                    changed = true;
                }
            }
        });

        // Step 3: Optional grow/shrink controls
        ui.horizontal(|ui| {
            if coords.len() < GridVec::MAX_DEPTH && ui.button("+").clicked() {
                coords.push(IVec3::ZERO);
                changed = true;
            }
            if coords.len() > 1 && ui.button("-").clicked() {
                coords.pop();
                changed = true;
            }
        });

        // Step 4: Rebuild GridVec if changed
        if changed {
            if let Ok(new_gv) = GridVec::try_from(coords) {
                *self = new_gv;
            } else {
                ui.label("Invalid GridVec conversion");
            }
        }

        changed
    }

    fn ui_readonly(&self, ui: &mut Ui, _options: &dyn Any, _id: Id, _env: InspectorUi<'_, '_>) {
        ui.label(format!("{:?}", self));
    }
}
