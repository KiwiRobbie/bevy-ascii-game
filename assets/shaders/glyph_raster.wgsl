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
    @location(1)
    color: vec4<f32>,
    
}


struct InstanceInput {
    @builtin(vertex_index) vertex_index: u32,
    @builtin(instance_index) instance_index: u32,
    // @location(0)
    // start: vec2<u32>, 
    // @location(1)
    // size: vec2<u32>, 
    // @location(2)
    // offset: vec2<i32>, 
    // @location(3)
    // padding: vec2<f32>,
    // @location(4)
    // color: vec4<f32>,
}



@group(0) @binding(0) var<uniform> uniform_buffer: UniformBuffer;
@group(0) @binding(1) var<uniform> view: View;
@group(0) @binding(2) var<uniform> model: Model;
@group(0) @binding(3) var atlas_texture: texture_2d<f32>;
@group(0) @binding(4) var atlas_uvs: texture_2d<u32>;
@group(0) @binding(5) var glyph_buffer: texture_2d<u32>;


var<private> verticies: array<vec2<i32>,6> = array(
    vec2<i32>(0, 0),
    vec2<i32>(1, 0),
    vec2<i32>(1, 1),
    vec2<i32>(0, 0),
    vec2<i32>(1, 1),
    vec2<i32>(0, 1),
);



fn index_to_pos_2(index: u32, size: vec2<u32>) -> vec2<u32> {
    return vec2<u32>(index % size.x, index / size.x);
} 

@vertex 
fn vertex(input: InstanceInput) -> VertexOutput {
    let location = vec2<i32>(i32(input.instance_index % uniform_buffer.width), i32(input.instance_index / uniform_buffer.width));
    let grid_size: vec2<i32> = vec2<i32>(i32(uniform_buffer.advance), i32(uniform_buffer.line_spacing));
    let corner = verticies[input.vertex_index];

    let glyph_data = textureLoad(glyph_buffer, location, 0);
    let glyph_id = glyph_data.r - 1u;
    let glpyh_color = bitcast<vec3<f32>>(glyph_data.gba);





    let atlas_uv_dim = textureDimensions(atlas_uvs);
    let glyph_atlas_pos = vec2<u32>(glyph_id % atlas_uv_dim.x, glyph_id / atlas_uv_dim.x);

    let start: vec2<u32> = textureLoad(atlas_uvs, index_to_pos_2(glyph_id * 3u, atlas_uv_dim.xy), 0).rg ;
    let size: vec2<u32> = textureLoad(atlas_uvs, index_to_pos_2(glyph_id * 3u + 1u, atlas_uv_dim.xy), 0).rg ;
    let offset: vec2<i32> = vec2<i32>(textureLoad(atlas_uvs, index_to_pos_2(glyph_id * 3u + 2u, atlas_uv_dim.xy), 0).rg) ;

    let pos: vec2<f32> = vec2<f32>(location * grid_size + offset + corner * vec2<i32>(i32(size.x), -i32(size.y)));

    var out: VertexOutput;
    out.position = view.clip_from_world * model.model * vec4<f32>(f32(pos.x), f32(pos.y), 0.0, 1.0);
    out.uv = vec2<f32>(start + size * vec2<u32>(u32(corner.x), u32(corner.y))) / vec2<f32>(textureDimensions(atlas_texture).xy);
    out.color = vec4<f32>(glpyh_color, 1.0);
    return out;
}


@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let coords = vec2<u32>(vec2<f32>(0.375) + vec2<f32>(textureDimensions(atlas_texture).xy) * input.uv);
    let sample_color = textureLoad(atlas_texture, coords, 0);

    let max_component = max(max(sample_color.r, sample_color.g), sample_color.b);
    let color: vec3<f32> = clamp(sample_color.rgb / max_component, vec3<f32>(0.0), vec3<f32>(1.0));
    let alpha: f32 = clamp(max_component * sample_color.a, 0.0, 1.0);
    return vec4<f32>(color, alpha) * input.color ;
}
