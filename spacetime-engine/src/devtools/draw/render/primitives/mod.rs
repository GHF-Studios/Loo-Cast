//! Gizmo realization of line/arrow/axes/rect/cross/sphere world-draw primitives.

use super::*;

pub(super) fn render_primitives(
    frame: Res<WorldDrawFrame>,
    mut world: Gizmos<DeveloperWorldGizmos>,
    mut overlay: Gizmos<DeveloperOverlayGizmos>,
) {
    let frame = frame.read();

    for primitive in &frame.primitives {
        match primitive_depth(primitive) {
            DrawDepth::World => draw_world_primitive(&mut world, primitive),
            DrawDepth::Overlay => draw_overlay_primitive(&mut overlay, primitive),
        }
    }
}

fn primitive_depth(primitive: &WorldPrimitive) -> DrawDepth {
    match primitive {
        WorldPrimitive::Line { depth, .. }
        | WorldPrimitive::Arrow { depth, .. }
        | WorldPrimitive::Axes { depth, .. }
        | WorldPrimitive::Rect { depth, .. }
        | WorldPrimitive::Cross { depth, .. }
        | WorldPrimitive::Sphere { depth, .. } => *depth,
    }
}

fn draw_world_primitive(gizmos: &mut Gizmos<DeveloperWorldGizmos>, primitive: &WorldPrimitive) {
    match primitive {
        WorldPrimitive::Line {
            start, end, color, ..
        } => gizmos.line(*start, *end, *color),
        WorldPrimitive::Arrow {
            start, end, color, ..
        } => {
            gizmos.arrow(*start, *end, *color);
        }
        WorldPrimitive::Axes {
            transform, length, ..
        } => gizmos.axes(*transform, *length),
        WorldPrimitive::Rect {
            isometry,
            size,
            color,
            ..
        } => {
            gizmos.rect(*isometry, *size, *color);
        }
        WorldPrimitive::Cross {
            isometry,
            size,
            color,
            ..
        } => {
            gizmos.cross(*isometry, *size, *color);
        }
        WorldPrimitive::Sphere {
            isometry,
            radius,
            color,
            resolution,
            ..
        } => {
            gizmos
                .sphere(*isometry, *radius, *color)
                .resolution(*resolution);
        }
    }
}

fn draw_overlay_primitive(gizmos: &mut Gizmos<DeveloperOverlayGizmos>, primitive: &WorldPrimitive) {
    match primitive {
        WorldPrimitive::Line {
            start, end, color, ..
        } => gizmos.line(*start, *end, *color),
        WorldPrimitive::Arrow {
            start, end, color, ..
        } => {
            gizmos.arrow(*start, *end, *color);
        }
        WorldPrimitive::Axes {
            transform, length, ..
        } => gizmos.axes(*transform, *length),
        WorldPrimitive::Rect {
            isometry,
            size,
            color,
            ..
        } => {
            gizmos.rect(*isometry, *size, *color);
        }
        WorldPrimitive::Cross {
            isometry,
            size,
            color,
            ..
        } => {
            gizmos.cross(*isometry, *size, *color);
        }
        WorldPrimitive::Sphere {
            isometry,
            radius,
            color,
            resolution,
            ..
        } => {
            gizmos
                .sphere(*isometry, *radius, *color)
                .resolution(*resolution);
        }
    }
}
