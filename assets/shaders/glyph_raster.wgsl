#import bevy_render::view::View

struct VertexInput {
    @location(0)
    pos: vec2<f32>,
    @location(1)
    uv: vec2<f32>
}

struct VertexOutput {
    @builtin(position)
    position: vec4<f32>,
    @location(0)
    uv: vec2<f32>,
    
}

@group(0) @binding(0) var<uniform> view: View;
@vertex 
fn vertex(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.position = view.view_proj * vec4<f32>(input.pos.x, -input.pos.y, 0.0, 1.0);
    out.uv = input.uv;
    return out;
}

@group(0) @binding(1) var atlas_texture: texture_storage_2d<rgba8unorm, read>;

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let coords = vec2<u32>(vec2<f32>(0.5) + vec2<f32>(textureDimensions(atlas_texture).xy) * input.uv);

    return textureLoad(atlas_texture, coords);
}