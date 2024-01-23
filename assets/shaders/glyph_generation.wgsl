struct UniformBuffer {
    color: vec4<f32>,
    width: u32,
    height: u32,
}

struct AtlasItem {
    start: vec2<u32>,
    size: vec2<u32>,
    offset: vec2<i32>,
}

struct AtlasUvStorage {
    uvs: array<AtlasItem>
}

struct Vertex {
    pos: vec2<f32>, 
    uv: vec2<f32>, 
}

struct VertexBuffer {
    verticies: array<Vertex>,
}



@group(0) @binding(0) var<uniform> uniform_buffer: UniformBuffer;
@group(0) @binding(1) var glyph_texture: texture_storage_2d<r16uint, read>;
@group(0) @binding(2) var atlas_texture: texture_storage_2d<rgba8unorm, read>;
@group(0) @binding(3) var<storage, read> atlas_uv: AtlasUvStorage;
@group(0) @binding(4) var<storage, write> vertex_buffer: VertexBuffer;

var<private> verticies: array<vec2<i32>,6> = array(
    vec2<i32>(0, 0),
    vec2<i32>(1, 0),
    vec2<i32>(1, 1),
    vec2<i32>(0, 0),
    vec2<i32>(1, 1),
    vec2<i32>(0, 1),
);

const grid_size = vec2<i32>(19, 32);

@compute @workgroup_size(8, 8, 1)
fn compute(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    if invocation_id.x >= num_workgroups.x || invocation_id.y >= num_workgroups.y || invocation_id.z >= num_workgroups.z {
        return;
    }

    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let index: u32 = u32(location.x) + u32(location.y) * uniform_buffer.width;

    let character: u32 = textureLoad(glyph_texture, location).r;
    let atlas_item: AtlasItem = atlas_uv.uvs[character];


    for (var i = 0; i < 6; i++) {
        let corner = verticies[i];

        var vertex: Vertex ;
        vertex.pos = vec2<f32>(location.xy * grid_size - atlas_item.offset * vec2<i32>(0, 0) + corner * vec2<i32>(atlas_item.size));
        vertex.uv = vec2<f32>(atlas_item.start + atlas_item.size * vec2<u32>(corner)) / vec2<f32>(textureDimensions(atlas_texture).xy);
        vertex_buffer.verticies[6u * index + u32(i)] = vertex;
    }
}
