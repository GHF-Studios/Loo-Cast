struct ShaderParams {
    chunk_pos: vec2<i32>,
    chunk_size: u32,
    chunk_scale: i32,
    current_view_scale: i32,
    _padding: vec3<u32>,
};

@group(0) @binding(0) var output_texture: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(1) var<storage, read> params: ShaderParams;

fn positive_mod(value: i32, modulo: i32) -> i32 {
    let r = value % modulo;
    return select(r + modulo, r, r >= 0);
}

const CH_SPACE: i32 = 0;
const CH_S: i32 = 1;
const CH_C: i32 = 2;
const CH_A: i32 = 3;
const CH_L: i32 = 4;
const CH_E: i32 = 5;
const CH_M: i32 = 6;
const CH_CARET: i32 = 7;
const CH_PLUS: i32 = 8;
const CH_MINUS: i32 = 9;
const CH_COLON: i32 = 10;
const CH_0: i32 = 11;
const CH_1: i32 = 12;
const CH_2: i32 = 13;
const CH_3: i32 = 14;
const CH_4: i32 = 15;
const CH_5: i32 = 16;
const CH_6: i32 = 17;
const CH_7: i32 = 18;
const CH_8: i32 = 19;
const CH_9: i32 = 20;

const LABEL_LEN: u32 = 14u; // "SCALE: 10^+00M"
const GLYPH_W: u32 = 5u;
const GLYPH_H: u32 = 7u;
const FONT_SCALE: u32 = 4u;
const GLYPH_ADVANCE: u32 = (GLYPH_W + 1u) * FONT_SCALE;
const LABEL_W: u32 = LABEL_LEN * GLYPH_ADVANCE - FONT_SCALE;
const LABEL_H: u32 = GLYPH_H * FONT_SCALE;

fn pick_row(row: u32, r0: u32, r1: u32, r2: u32, r3: u32, r4: u32, r5: u32, r6: u32) -> u32 {
    switch (row) {
        case 0u: {
            return r0;
        }
        case 1u: {
            return r1;
        }
        case 2u: {
            return r2;
        }
        case 3u: {
            return r3;
        }
        case 4u: {
            return r4;
        }
        case 5u: {
            return r5;
        }
        case 6u: {
            return r6;
        }
        default: {
            return 0u;
        }
    }
}

fn glyph_row_bits(ch: i32, row: u32) -> u32 {
    switch (ch) {
        case CH_SPACE: {
            return 0u;
        }
        case CH_S: {
            return pick_row(row, 0x1Fu, 0x10u, 0x10u, 0x1Fu, 0x01u, 0x01u, 0x1Fu);
        }
        case CH_C: {
            return pick_row(row, 0x0Fu, 0x10u, 0x10u, 0x10u, 0x10u, 0x10u, 0x0Fu);
        }
        case CH_A: {
            return pick_row(row, 0x0Eu, 0x11u, 0x11u, 0x1Fu, 0x11u, 0x11u, 0x11u);
        }
        case CH_L: {
            return pick_row(row, 0x10u, 0x10u, 0x10u, 0x10u, 0x10u, 0x10u, 0x1Fu);
        }
        case CH_E: {
            return pick_row(row, 0x1Fu, 0x10u, 0x10u, 0x1Eu, 0x10u, 0x10u, 0x1Fu);
        }
        case CH_M: {
            return pick_row(row, 0x11u, 0x1Bu, 0x15u, 0x15u, 0x11u, 0x11u, 0x11u);
        }
        case CH_CARET: {
            return pick_row(row, 0x04u, 0x0Au, 0x11u, 0x00u, 0x00u, 0x00u, 0x00u);
        }
        case CH_PLUS: {
            return pick_row(row, 0x00u, 0x04u, 0x04u, 0x1Fu, 0x04u, 0x04u, 0x00u);
        }
        case CH_MINUS: {
            return pick_row(row, 0x00u, 0x00u, 0x00u, 0x1Fu, 0x00u, 0x00u, 0x00u);
        }
        case CH_COLON: {
            return pick_row(row, 0x00u, 0x04u, 0x04u, 0x00u, 0x04u, 0x04u, 0x00u);
        }
        case CH_0: {
            return pick_row(row, 0x0Eu, 0x11u, 0x13u, 0x15u, 0x19u, 0x11u, 0x0Eu);
        }
        case CH_1: {
            return pick_row(row, 0x04u, 0x0Cu, 0x04u, 0x04u, 0x04u, 0x04u, 0x0Eu);
        }
        case CH_2: {
            return pick_row(row, 0x0Eu, 0x11u, 0x01u, 0x02u, 0x04u, 0x08u, 0x1Fu);
        }
        case CH_3: {
            return pick_row(row, 0x1Eu, 0x01u, 0x01u, 0x0Eu, 0x01u, 0x01u, 0x1Eu);
        }
        case CH_4: {
            return pick_row(row, 0x02u, 0x06u, 0x0Au, 0x12u, 0x1Fu, 0x02u, 0x02u);
        }
        case CH_5: {
            return pick_row(row, 0x1Fu, 0x10u, 0x10u, 0x1Eu, 0x01u, 0x01u, 0x1Eu);
        }
        case CH_6: {
            return pick_row(row, 0x0Eu, 0x10u, 0x10u, 0x1Eu, 0x11u, 0x11u, 0x0Eu);
        }
        case CH_7: {
            return pick_row(row, 0x1Fu, 0x01u, 0x02u, 0x04u, 0x08u, 0x08u, 0x08u);
        }
        case CH_8: {
            return pick_row(row, 0x0Eu, 0x11u, 0x11u, 0x0Eu, 0x11u, 0x11u, 0x0Eu);
        }
        case CH_9: {
            return pick_row(row, 0x0Eu, 0x11u, 0x11u, 0x0Fu, 0x01u, 0x01u, 0x0Eu);
        }
        default: {
            return 0u;
        }
    }
}

fn digit_char(digit: u32) -> i32 {
    return CH_0 + i32(min(digit, 9u));
}

fn label_char(index: u32, exponent: i32) -> i32 {
    let abs_exp = abs(exponent);
    let tens = u32(abs_exp / 10);
    let ones = u32(abs_exp % 10);
    let sign = select(CH_PLUS, CH_MINUS, exponent < 0);

    switch (index) {
        case 0u: {
            return CH_S;
        }
        case 1u: {
            return CH_C;
        }
        case 2u: {
            return CH_A;
        }
        case 3u: {
            return CH_L;
        }
        case 4u: {
            return CH_E;
        }
        case 5u: {
            return CH_COLON;
        }
        case 6u: {
            return CH_SPACE;
        }
        case 7u: {
            return digit_char(1u);
        }
        case 8u: {
            return digit_char(0u);
        }
        case 9u: {
            return CH_CARET;
        }
        case 10u: {
            return sign;
        }
        case 11u: {
            return digit_char(tens);
        }
        case 12u: {
            return digit_char(ones);
        }
        case 13u: {
            return CH_M;
        }
        default: {
            return CH_SPACE;
        }
    }
}

fn glyph_pixel(ch: i32, x: u32, y: u32) -> bool {
    if (x >= GLYPH_W || y >= GLYPH_H) {
        return false;
    }

    let row_bits = glyph_row_bits(ch, y);
    let shift = (GLYPH_W - 1u) - x;
    return ((row_bits >> shift) & 1u) == 1u;
}

fn is_label_pixel(pixel: vec2<u32>, label_origin: vec2<u32>, exponent: i32) -> bool {
    if (pixel.x < label_origin.x || pixel.y < label_origin.y) {
        return false;
    }

    let rel = pixel - label_origin;
    if (rel.x >= LABEL_W || rel.y >= LABEL_H) {
        return false;
    }

    let char_slot = rel.x / GLYPH_ADVANCE;
    if (char_slot >= LABEL_LEN) {
        return false;
    }

    let x_in_slot = rel.x % GLYPH_ADVANCE;
    let glyph_w_scaled = GLYPH_W * FONT_SCALE;
    if (x_in_slot >= glyph_w_scaled) {
        return false;
    }

    let glyph_x = x_in_slot / FONT_SCALE;
    let glyph_y = rel.y / FONT_SCALE;
    let ch = label_char(char_slot, exponent);
    return glyph_pixel(ch, glyph_x, glyph_y);
}

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let chunk_size = params.chunk_size;
    if (global_id.x >= chunk_size || global_id.y >= chunk_size) {
        return;
    }

    let subgrid_size = max(1u, chunk_size / 10u);
    let subgrid_size_i = i32(subgrid_size);
    let chunk_size_i = i32(chunk_size);

    let world_x = params.chunk_pos.x * chunk_size_i + i32(global_id.x);
    let world_y = params.chunk_pos.y * chunk_size_i + i32(global_id.y);
    // GridVec digits represent centered chunk positions. Shift subgrid phase by
    // half a sub-cell so parent grid boundaries align with child chunk borders.
    let half_subgrid_i = subgrid_size_i / 2;
    let sub_x = positive_mod(world_x + half_subgrid_i, subgrid_size_i);
    let sub_y = positive_mod(world_y + half_subgrid_i, subgrid_size_i);

    let major_thickness = 6u;
    let minor_thickness = 2;

    let is_chunk_border = global_id.x < major_thickness
        || global_id.y < major_thickness
        || global_id.x >= (chunk_size - major_thickness)
        || global_id.y >= (chunk_size - major_thickness);

    let is_subgrid_line = sub_x < minor_thickness
        || sub_y < minor_thickness
        || sub_x >= (subgrid_size_i - minor_thickness)
        || sub_y >= (subgrid_size_i - minor_thickness);

    let parity = (params.chunk_pos.x + params.chunk_pos.y + params.chunk_scale) & 1;
    let is_current_scale = abs(params.chunk_scale - params.current_view_scale) == 0;

    let dark_base = vec3<f32>(0.08, 0.09, 0.10);
    let light_base = vec3<f32>(0.11, 0.12, 0.14);
    var base = select(dark_base, light_base, parity == 1);
    if (is_current_scale) {
        base *= 1.15;
    } else {
        base *= 0.90;
    }

    var color = vec3<f32>(base);
    if (is_subgrid_line) {
        color = vec3<f32>(0.40, 0.42, 0.45);
    }
    if (is_chunk_border) {
        color = vec3<f32>(0.96, 0.98, 1.00);
    }

    let label_origin = vec2<u32>(24u, 24u);
    let panel_pad_x = 10u;
    let panel_pad_y = 8u;
    let panel_min = vec2<u32>(label_origin.x - panel_pad_x, label_origin.y - panel_pad_y);
    let panel_max = vec2<u32>(
        label_origin.x + LABEL_W + panel_pad_x,
        label_origin.y + LABEL_H + panel_pad_y,
    );
    let in_label_panel = global_id.x >= panel_min.x
        && global_id.y >= panel_min.y
        && global_id.x < panel_max.x
        && global_id.y < panel_max.y;
    if (in_label_panel && !is_chunk_border) {
        color = mix(color, vec3<f32>(0.02, 0.03, 0.05), 0.72);
    }

    if (is_label_pixel(global_id.xy, label_origin, params.chunk_scale)) {
        color = vec3<f32>(0.98, 0.95, 0.72);
    }

    textureStore(output_texture, global_id.xy, vec4<f32>(color, 1.0));
}
