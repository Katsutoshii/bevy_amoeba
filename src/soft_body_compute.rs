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

use crate::{
    ComputeShader, ComputeShaderPlugin, SoftBodyVertex2dBuffer, instances::SoftBodyInstanceBuffer,
    nodes::SoftBodyNode2dBuffer,
};

pub struct SoftBodyComputePlugin;
impl Plugin for SoftBodyComputePlugin {
    fn build(&self, app: &mut App) {
        // embedded_asset!(app, "soft_body_compute.wgsl");
        app.add_plugins((ComputeShaderPlugin::<SoftBodyCompute>::default(),));
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Resource, ExtractResource)]
pub struct SoftBodyCompute {
    #[storage(0, visibility(compute))]
    pub vertices: Handle<ShaderStorageBuffer>,
    #[storage(1, read_only, visibility(compute))]
    pub nodes: Handle<ShaderStorageBuffer>,
    #[storage(2, read_only, visibility(compute))]
    pub instances: Handle<ShaderStorageBuffer>,

    #[uniform(3)]
    num_vertices_per_instance: u32,
}
impl FromWorld for SoftBodyCompute {
    fn from_world(world: &mut World) -> Self {
        Self {
            num_vertices_per_instance: SoftBodyVertex2dBuffer::NUM_VERTICES,
            vertices: world.resource::<SoftBodyVertex2dBuffer>().0.clone(),
            instances: world.resource::<SoftBodyInstanceBuffer>().0.clone(),
            nodes: world.resource::<SoftBodyNode2dBuffer>().0.clone(),
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
        UVec3::new(128, 1, 1)
    }
    fn workgroup_count() -> UVec3 {
        UVec3::new(
            SoftBodyVertex2dBuffer::NUM_VERTICES / Self::workgroup_size().x,
            SoftBodyInstanceBuffer::NUM_INSTANCES,
            1,
        )
    }
}
