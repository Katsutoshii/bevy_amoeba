

#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip, get_tag}
#import bevy_pbr::{mesh_view_bindings::globals};
#import bevy_amoeba::vertices::SoftBodyVertex2d;

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> num_vertices_per_instance: u32;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var color_texture_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<storage, read> vertices: array<SoftBodyVertex2d>;

struct Vertex {
    @builtin(vertex_index) index: u32,
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) vertex_index: u32,
};

fn get_clip_position(vertex: Vertex) -> vec4<f32> {
    let n = num_vertices_per_instance;
    let vi = vertex.index;
    // let ii = vertex.instance_index;
    let ii = get_tag(vertex.instance_index);
    return mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(vertices[vi + ii * n].position, 0.0, 1.0),
    );
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = get_clip_position(vertex);
    out.uv = vertex.uv;
    out.vertex_index = vertex.index;
    return out;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let vertex_index = input.vertex_index;
    return color * textureSample(color_texture, color_texture_sampler, input.uv);
}
