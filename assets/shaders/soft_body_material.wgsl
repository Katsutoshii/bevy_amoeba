

#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}
#import bevy_pbr::{mesh_view_bindings::globals};
#import bevy_amoeba::particle::Particle2d;

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> vertices_per_particle: u32;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var color_texture_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<storage, read> particles: array<Particle2d>;

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

fn get_position(vertex_index: u32) -> vec2<f32> {
    if (vertex_index > 0) {
        return particles[vertex_index - 1].position;
    } else {
        return vec2<f32>(0.0, 0.0);
    }
}

fn get_color(vertex_index: u32) -> vec4<f32> {
    if (vertex_index > 0) {
        return particles[vertex_index - 1].color;
    } else {
        return vec4<f32>(1.0, 1.0, 1.0, 1.0);
    }
}

fn get_clip_position(vertex: Vertex) -> vec4<f32> {
    return mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(get_position(vertex.index), 0.0, 1.0),
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
    return get_color(vertex_index) * color * textureSample(color_texture, color_texture_sampler, input.uv);
}
