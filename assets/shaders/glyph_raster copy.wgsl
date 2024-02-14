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
    @location(0)
    size: vec2<f32>, 
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
    var grid_size: vec2<i32> = vec2<i32>(i32(uniform_buffer.advance), i32(uniform_buffer.line_spacing));
    var size = vec2<i32>(i32(uniform_buffer.height), i32(uniform_buffer.width));
 



    // let location = vec2<i32>(i32(input.instance_index % uniform_buffer.width), i32(input.instance_index / uniform_buffer.width));
    let corner = verticies[input.vertex_index];

    // let pos: vec2<f32> = vec2<f32>(location * grid_size + input.offset + corner * vec2<i32>(i32(input.size.x), -i32(input.size.y)));
    let pos = input.size * corner;

    var out: VertexOutput;
    out.position = view.view_proj * model.model * vec4<f32>(f32(pos.x), f32(pos.y), 0.0, 1.0);
    out.uv = corner;
    // out.uv = vec2<f32>(input.start + input.size * vec2<u32>(u32(corner.x), u32(corner.y))) / vec2<f32>(textureDimensions(atlas_texture).xy);
    // out.color = input.color;
    return out;
}
fn atlas_index_to_uv(index: u32, size: vec2<u32>) -> vec2<u32> {
    return vec2<u32>(index % size.x, index / size.x);
}


@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let grid_size: vec2<i32> = vec2<i32>(i32(uniform_buffer.advance), i32(uniform_buffer.line_spacing));
    let size = vec2<i32>(i32(uniform_buffer.height), i32(uniform_buffer.width));
    let grid_pos: vec2<f32> = input.uv * vec2<f32>(f32(uniform_buffer.width), f32(uniform_buffer.height));
    let glyph_pos = vec2<u32>(u32(grid_pos.x), u32(grid_pos.y));
    let sub_glyph_pos: vec2<f32> = fract(grid_pos) ;
    let sub_glyph_pos_pixel: vec2<u32> = vec2<u32>(sub_glyph_pos * vec2<f32>(grid_size));

    let glyph_id: u32 = textureLoad(glyph_buffer, glyph_pos, 0).r;

    let atlas_uv_index = 24u;
    let atlas_uv_size: vec2<u32> = textureDimensions(atlas_uvs).xy;


    let atlas_start: vec2<u32> = textureLoad(atlas_uvs, vec2<u32>(3u + 0u, 3u), 0).rg; //atlas_index_to_uv(3u * atlas_uv_index, atlas_uv_size), 0).rg;
    let atlas_size: vec2<u32> = textureLoad(atlas_uvs, vec2<u32>(3u + 1u, 3u), 0).rg; //atlas_index_to_uv(3u * atlas_uv_index + 1u, atlas_uv_size), 0).rg;
    let atlas_offset: vec2<u32> = textureLoad(atlas_uvs, vec2<u32>(3u + 2u, 3u), 0).rg; //atlas_index_to_uv(3u * atlas_uv_index + 2u, atlas_uv_size), 0).rg;

    let atlas_pixel = sub_glyph_pos_pixel - atlas_offset;

    var result = vec4<f32>(1.0);
    if atlas_pixel.x > atlas_size.x || atlas_pixel.y > atlas_size.y {

        let glyph_pixel: f32 = textureLoad(atlas_texture, atlas_pixel + atlas_start, 0).r;
        result = vec4<f32>(sub_glyph_pos * 0.0, glyph_pixel, 1.0);
    }

    // let coords = vec2<u32>(vec2<f32>(0.375) + vec2<f32>(textureDimensions(atlas_texture).xy) * input.uv);
    // let sample_color = textureLoad(atlas_texture, coords, 0);

    // let max_component = max(max(sample_color.r, sample_color.g), sample_color.b);
    // let color: vec3<f32> = clamp(sample_color.rgb / max_component, vec3<f32>(0.0), vec3<f32>(1.0));
    // let alpha: f32 = clamp(max_component * sample_color.a, 0.0, 1.0);

    // return vec4<f32>(color, alpha) * input.color ;
    return result;
}
