#import bevy_render::view::View

struct UniformBuffer {
    color: vec4<f32>,
    width: u32,
    height: u32,
    advance: u32,
    line_spacing: u32,
}


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

struct Model {
    model: mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> uniform_buffer: UniformBuffer;
@group(0) @binding(1) var<uniform> view: View;
@group(0) @binding(2) var<uniform> model: Model;
@vertex 
fn vertex(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.position = view.view_proj * model.model * vec4<f32>(input.pos.x, input.pos.y, 0.0, 1.0);
    out.uv = input.uv;
    return out;
}

@group(0) @binding(3) var atlas_texture: texture_storage_2d<rgba8unorm, read>;

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let coords = vec2<u32>(vec2<f32>(0.375) + vec2<f32>(textureDimensions(atlas_texture).xy) * input.uv);

    let sample_color = textureLoad(atlas_texture, coords);

    let max_component = max(max(sample_color.r, sample_color.g), sample_color.b);

    return vec4<f32>(sample_color.rgb / max_component, max_component * sample_color.a) * uniform_buffer.color ;
}