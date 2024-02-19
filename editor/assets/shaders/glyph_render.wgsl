#import bevy_render::view::View

struct UniformBuffer {
    position: vec2<i32>,
    size: vec2<u32>,
    target_size: vec2<u32>,
    depth: f32,
    padding: f32

}


struct VertexOutput {
    @builtin(position)
    position: vec4<f32>,
    @location(0)
    uv: vec2<f32>,
}


struct InstanceInput {
    @builtin(vertex_index) vertex_index: u32,
    @builtin(instance_index) instance_index: u32,
}



@group(0) @binding(0) var<uniform> uniform_buffer: UniformBuffer;
@group(0) @binding(1) var glyph_buffer: texture_2d<u32>;


var<private> verticies: array<vec2<f32>,6> = array(
    vec2<f32>(0.0, 0.0),
    vec2<f32>(1.0, 0.0),
    vec2<f32>(1.0, 1.0),
    vec2<f32>(0.0, 0.0),
    vec2<f32>(1.0, 1.0),
    vec2<f32>(0.0, 1.0),
);

@vertex 
fn vertex(input: InstanceInput) -> VertexOutput {
    let corner = verticies[input.vertex_index];
    let target_size = vec2<f32>(uniform_buffer.target_size) ;

    let start = vec2<f32>(uniform_buffer.position) / target_size;
    let end = vec2<f32>(uniform_buffer.position + vec2<i32>(uniform_buffer.size)) / target_size;
    let pos = start * (vec2<f32>(1.0) - corner) + end * corner;

    var out: VertexOutput;
    out.position = vec4<f32>(2.0 * pos.x - 1.0, 1.0 - 2.0 * pos.y, 0.5 + uniform_buffer.depth / 2048.0, 1.0);
    out.uv = corner;
    return out;
}


@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<u32> {
    let uv = vec2<f32>(textureDimensions(glyph_buffer).xy);
    let sample = textureLoad(glyph_buffer, vec2<u32>(uv * input.uv), 0);
    let glyph_id = sample.r;
    if glyph_id == 65535u {
        discard;
    }

    return vec4<u32>(glyph_id + 1u, sample.gba);
}
