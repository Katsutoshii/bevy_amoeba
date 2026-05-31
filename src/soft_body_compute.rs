use bevy::{
    app::{App, Plugin},
    asset::{
        Asset,
        Handle,
        // AssetPath, embedded_asset, embedded_path
    },
    ecs::{
        resource::Resource,
        world::{FromWorld, World},
    },
    math::UVec3,
    reflect::TypePath,
    render::{
        extract_resource::ExtractResource, render_resource::AsBindGroup,
        storage::ShaderStorageBuffer,
    },
    shader::ShaderRef,
};

use crate::{ComputeShader, ComputeShaderPlugin, Particle2dBuffer};

pub struct SoftBodyComputePlugin;
impl Plugin for SoftBodyComputePlugin {
    fn build(&self, app: &mut App) {
        // embedded_asset!(app, "soft_body_compute.wgsl");
        app.add_plugins((ComputeShaderPlugin::<SoftBodyCompute>::default(),));
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Resource, ExtractResource)]
struct SoftBodyCompute {
    #[storage(0, visibility(compute))]
    particles: Handle<ShaderStorageBuffer>,
    #[uniform(1)]
    num_particles: u32,
    #[uniform(2)]
    dt: f32,
}
impl FromWorld for SoftBodyCompute {
    fn from_world(world: &mut World) -> Self {
        Self {
            dt: 0.005,
            num_particles: Particle2dBuffer::MAX_PARTICLES,
            particles: world.resource::<Particle2dBuffer>().0.clone(),
        }
    }
}
impl ComputeShader for SoftBodyCompute {
    fn compute_shader() -> ShaderRef {
        // ShaderRef::Path(
        //     AssetPath::from_path_buf(embedded_path!("soft_body_compute.wgsl"))
        //         .with_source("embedded"),
        // )
        "shaders/soft_body_compute.wgsl".into()
    }
    fn workgroup_size() -> UVec3 {
        UVec3::new(16, 1, 1)
    }
    fn workgroup_count() -> UVec3 {
        UVec3::new(
            Particle2dBuffer::MAX_PARTICLES / Self::workgroup_size().x,
            1,
            1,
        )
    }
}
