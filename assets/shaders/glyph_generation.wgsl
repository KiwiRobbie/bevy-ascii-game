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
@group(0) @binding(2) var atlas_texture: texture_storage_2d<r16uint, read>;
@group(0) @binding(3) var<storage, read> atlas_uv: AtlasUvStorage;
@group(0) @binding(4) var<storage, write> vertex_buffer: VertexBuffer;


@compute @workgroup_size(8, 8, 1)
fn compute(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let index: u32 = u32(location.x + location.y) * uniform_buffer.width;

    let character: u32 = textureLoad(glyph_texture, location).r;
    let atlas_item: AtlasItem = atlas_uv.uvs[character];


    for (var i = 0; i < 4; i++) {
        let corner = vec2<i32>(min(min(i, 3 - i), 1), i / 2);

        var vertex: Vertex ;
        vertex.pos = vec2<f32>(location.xy + atlas_item.offset + corner * vec2<i32>(atlas_item.size));
        vertex.uv = vec2<f32>(atlas_item.start + atlas_item.size * vec2<u32>(corner)) / vec2<f32>(textureDimensions(atlas_texture).xy);
        vertex_buffer.verticies[4u * index + u32(i)] = vertex;
    }
}
