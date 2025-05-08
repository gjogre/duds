// Bindings
@group(2) @binding(0) var tile_texture: texture_2d<f32>;
@group(2) @binding(1) var tile_sampler: sampler;
@group(2) @binding(2) var<uniform> tile_index: vec2<u32>;
@group(2) @binding(3) var<uniform> tile_count: vec2<u32>;

// Vertex output structure
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

// Fragment shader
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let tile_size = vec2(1.0 / f32(tile_count.x), 1.0 / f32(tile_count.y));

    // Keep in.uv as-is (0..1 range per face)
    let uv_in_tile = in.uv;

    // Calculate tile origin based on tile_index
    let tile_origin = vec2(
        f32(tile_index.x) * tile_size.x,
        f32(tile_index.y) * tile_size.y,
    );

    let final_uv = tile_origin + uv_in_tile * tile_size;

    return textureSample(tile_texture, tile_sampler, final_uv);
}
