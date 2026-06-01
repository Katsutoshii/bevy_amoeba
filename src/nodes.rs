use bevy::{
    app::{Plugin, Update},
    asset::{Assets, DirectAssetAccessExt, Handle},
    ecs::{
        change_detection::DetectChangesMut,
        component::Component,
        hierarchy::Children,
        query::With,
        resource::Resource,
        schedule::IntoScheduleConfigs,
        system::{Query, Res, ResMut},
        world::{FromWorld, World},
    },
    math::{Vec2, Vec3Swizzles},
    pbr::MeshMaterial3d,
    reflect::Reflect,
    render::{extract_resource::ExtractResource, storage::ShaderStorageBuffer},
    time::Time,
    transform::components::Transform,
};

use crate::{SoftBodyMaterial, soft_body_compute::SoftBodyCompute};

pub struct NodesPlugin;
impl Plugin for NodesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<SoftBodyNodesBuffer>().add_systems(
            Update,
            (SoftBodyNode::update, SoftBodyNodesBuffer::update).chain(),
        );
    }
}

#[derive(Resource, Clone, ExtractResource)]
pub struct SoftBodyNodesBuffer(pub Handle<ShaderStorageBuffer>);
impl FromWorld for SoftBodyNodesBuffer {
    fn from_world(world: &mut World) -> Self {
        Self(world.add_asset(ShaderStorageBuffer::from(vec![
            Vec2::ONE;
            Self::MAX_NODES as usize
        ])))
    }
}
impl SoftBodyNodesBuffer {
    pub const MAX_NODES: u32 = 16;
    pub fn update(
        mut compute: ResMut<SoftBodyCompute>,
        mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
        query: Query<&Children, With<MeshMaterial3d<SoftBodyMaterial>>>,
        child_transforms: Query<&Transform, With<SoftBodyNode>>,
    ) {
        let buffer = buffers.get_mut(&compute.nodes).unwrap();
        let mut nodes = Vec::with_capacity(Self::MAX_NODES as usize);
        for children in query.iter() {
            for child in children {
                if let Ok(transform) = child_transforms.get(*child) {
                    nodes.push(transform.translation.xy())
                }
            }
        }
        compute.set_changed();
        buffer.set_data(nodes);
    }
}

#[derive(Component, Reflect)]
pub struct SoftBodyNode;
impl SoftBodyNode {
    pub fn update(mut query: Query<&mut Transform, With<Self>>, time: Res<Time>) {
        let alpha = 0.002;
        let omega = 2.0;
        for (i, mut transform) in query.iter_mut().enumerate() {
            let phi = i as f32;
            transform.translation.x += alpha * (time.elapsed_secs() * omega + phi).cos();
            transform.translation.y += alpha * (time.elapsed_secs() * omega + phi).sin();
        }
    }
}
