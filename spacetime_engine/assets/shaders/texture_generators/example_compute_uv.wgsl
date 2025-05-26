struct ShaderParams {
    chunk_pos: vec2<i32>,
    chunk_size: f32,
    _padding: u32, // 16-byte alignment
};






fn mod289(x: vec2<f32>) -> vec2<f32> {
    return x - floor(x * (1. / 289.)) * 289.;
}

fn mod289_3(x: vec3<f32>) -> vec3<f32> {
    return x - floor(x * (1. / 289.)) * 289.;
}

fn permute3(x: vec3<f32>) -> vec3<f32> {
    return mod289_3(((x * 34.) + 1.) * x);
}

//  MIT License. © Ian McEwan, Stefan Gustavson, Munrocket
fn simplexNoise2(v: vec2<f32>) -> f32 {
    let C = vec4(
        0.211324865405187, // (3.0-sqrt(3.0))/6.0
        0.366025403784439, // 0.5*(sqrt(3.0)-1.0)
        -0.577350269189626, // -1.0 + 2.0 * C.x
        0.024390243902439 // 1.0 / 41.0
    );

    // First corner
    var i = floor(v + dot(v, C.yy));
    let x0 = v - i + dot(i, C.xx);

    // Other corners
    var i1 = select(vec2(0., 1.), vec2(1., 0.), x0.x > x0.y);

    // x0 = x0 - 0.0 + 0.0 * C.xx ;
    // x1 = x0 - i1 + 1.0 * C.xx ;
    // x2 = x0 - 1.0 + 2.0 * C.xx ;
    var x12 = x0.xyxy + C.xxzz;
    x12.x = x12.x - i1.x;
    x12.y = x12.y - i1.y;

    // Permutations
    i = mod289(i); // Avoid truncation effects in permutation

    var p = permute3(permute3(i.y + vec3(0., i1.y, 1.)) + i.x + vec3(0., i1.x, 1.));
    var m = max(0.5 - vec3(dot(x0, x0), dot(x12.xy, x12.xy), dot(x12.zw, x12.zw)), vec3(0.));
    m *= m;
    m *= m;

    // Gradients: 41 points uniformly over a line, mapped onto a diamond.
    // The ring size 17*17 = 289 is close to a multiple of 41 (41*7 = 287)
    let x = 2. * fract(p * C.www) - 1.;
    let h = abs(x) - 0.5;
    let ox = floor(x + 0.5);
    let a0 = x - ox;

    // Normalize gradients implicitly by scaling m
    // Approximation of: m *= inversesqrt( a0*a0 + h*h );
    m *= 1.79284291400159 - 0.85373472095314 * (a0 * a0 + h * h);

    // Compute final noise value at P
    let g = vec3(a0.x * x0.x + h.x * x0.y, a0.yz * x12.xz + h.yz * x12.yw);
    return 130. * dot(m, g);
}

fn fbm(p: vec2<f32>, octaves: i32, lacunarity: f32, gain: f32) -> f32 {
    var frequency = 1.0;
    var amplitude = 1.0;
    var sum = 0.0;
    var total_amplitude = 0.0;

    for (var i = 0; i < octaves; i++) {
        sum += simplexNoise2(p * frequency) * amplitude;
        total_amplitude += amplitude;
        frequency *= lacunarity;
        amplitude *= gain;
    }

    return sum / total_amplitude;
}










@group(0) @binding(0) var output_texture: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(1) var<storage, read> params: ShaderParams;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let tex_size = vec2<f32>(textureDimensions(output_texture));
    let uv = vec2<f32>(global_id.xy) / tex_size;
    let flipped_uv = vec2<f32>(uv.x, 1.0 - uv.y);

    // Compute world position
    let chunk_pos_f32 = vec2<f32>(params.chunk_pos);
    let chunk_size = params.chunk_size;
    let world_pos = chunk_pos_f32 * chunk_size + flipped_uv * chunk_size;

    // Define the visible world range
    let world_min = -8.0 * chunk_size;
    let world_max =  8.0 * chunk_size;

    // Normalize world position to [0.0, 1.0]
    let norm = (world_pos - vec2<f32>(world_min)) / (world_max - world_min);
    let clamped = clamp(norm, vec2<f32>(0.0), vec2<f32>(1.0));

    // Map X to red, Y to green
    let noise_val = fbm(world_pos * 0.01, 5, 2.0, 0.5);
    let color = vec4<f32>(noise_val, noise_val, noise_val, 1.0);

    textureStore(output_texture, global_id.xy, color);
}
