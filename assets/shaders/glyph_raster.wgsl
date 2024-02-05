#import bevy_render::view::View

struct UniformBuffer {
    color: vec4<f32>,
    width: u32,
    height: u32,
    advance: u32,
    line_spacing: u32,
}

struct Model {
    model: mat4x4<f32>,
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
    @location(0)
    start: vec2<u32>, 
    @location(1)
    size: vec2<u32>, 
    @location(2)
    offset: vec2<i32>, 
    @location(3)
    padding: vec2<f32>,
}



@group(0) @binding(0) var<uniform> uniform_buffer: UniformBuffer;
@group(0) @binding(1) var<uniform> view: View;
@group(0) @binding(2) var<uniform> model: Model;
@group(0) @binding(3) var atlas_texture: texture_2d<f32>;


var<private> verticies: array<vec2<i32>,6> = array(
    vec2<i32>(0, 0),
    vec2<i32>(1, 0),
    vec2<i32>(1, 1),
    vec2<i32>(0, 0),
    vec2<i32>(1, 1),
    vec2<i32>(0, 1),
);



@vertex 
fn vertex(input: InstanceInput) -> VertexOutput {
    var grid_size: vec2<i32> = vec2<i32>(i32(uniform_buffer.advance), i32(uniform_buffer.line_spacing));

    let location = vec2<i32>(i32(input.instance_index % uniform_buffer.width), i32(input.instance_index / uniform_buffer.width));
    let corner = verticies[input.vertex_index];

    let pos: vec2<f32> = vec2<f32>(location * grid_size + input.offset + corner * vec2<i32>(i32(input.size.x), -i32(input.size.y)));

    var out: VertexOutput;
    out.position = view.view_proj * model.model * vec4<f32>(f32(pos.x), f32(pos.y), 0.0, 1.0);
    out.uv = vec2<f32>(input.start + input.size * vec2<u32>(u32(corner.x), u32(corner.y))) / vec2<f32>(textureDimensions(atlas_texture).xy);
    return out;
}


@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let coords = vec2<u32>(vec2<f32>(0.375) + vec2<f32>(textureDimensions(atlas_texture).xy) * input.uv);
    let sample_color = textureLoad(atlas_texture, coords, 0);
    let max_component = max(max(sample_color.r, sample_color.g), sample_color.b);
    return vec4<f32>(sample_color.rgb / max_component, max_component * sample_color.a) * uniform_buffer.color ;
    // return vec4<f32>(input.uv, 1.0, 1.0);
}