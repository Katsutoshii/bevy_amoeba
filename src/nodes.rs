use bevy::{
    app::{Plugin, Update},
    asset::{Assets, DirectAssetAccessExt, Handle},
    ecs::{
        change_detection::DetectChangesMut,
        component::Component,
        entity::Entity,
        query::With,
        resource::Resource,
        schedule::IntoScheduleConfigs,
        system::{Query, Res, ResMut},
        world::{FromWorld, World},
    },
    math::{Vec2, Vec3Swizzles},
    pbr::MeshMaterial3d,
    reflect::Reflect,
    render::{
        extract_resource::ExtractResource, render_resource::ShaderType,
        storage::ShaderStorageBuffer,
    },
    shader::load_shader_library,
    time::Time,
    transform::{
        TransformSystems,
        components::{GlobalTransform, Transform},
    },
};

use crate::{SoftBodyMaterial, soft_body_compute::SoftBodyCompute};

pub struct SoftBodyNodesPlugin;
impl Plugin for SoftBodyNodesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        load_shader_library!(app, "nodes.wgsl");
        app.init_resource::<SoftBodyNode2dBuffer>().add_systems(
            Update,
            (
                SoftBodyNodes::update.after(TransformSystems::Propagate),
                SoftBodyNode::update,
                SoftBodyNode2dBuffer::update,
            )
                .chain(),
        );
    }
}

#[derive(Default, ShaderType, Copy, Clone, Debug)]
pub struct SoftBodyNode2d {
    pub position: Vec2,
    pub radius: f32,
}

#[derive(Resource, Clone, ExtractResource)]
pub struct SoftBodyNode2dBuffer(pub Handle<ShaderStorageBuffer>);
impl FromWorld for SoftBodyNode2dBuffer {
    fn from_world(world: &mut World) -> Self {
        Self(world.add_asset(ShaderStorageBuffer::from(vec![
            SoftBodyNode2d::default();
            Self::MAX_NODES as usize
        ])))
    }
}
impl SoftBodyNode2dBuffer {
    pub const MAX_NODES: u32 = 16;

    /// Copy relative positions into the nodes buffer.
    pub fn update(
        mut compute: ResMut<SoftBodyCompute>,
        mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
        query: Query<(&GlobalTransform, &SoftBodyNodes), With<MeshMaterial3d<SoftBodyMaterial>>>,
        node_transforms: Query<(&GlobalTransform, &SoftBodyNode)>,
    ) {
        let buffer = buffers.get_mut(&compute.nodes).unwrap();
        let mut all_nodes = Vec::with_capacity(Self::MAX_NODES as usize);
        for (transform, nodes) in query.iter() {
            for entity in &nodes.0 {
                if let Ok((node_transform, node)) = node_transforms.get(*entity) {
                    let rel_transform = node_transform.reparented_to(transform);
                    all_nodes.push(SoftBodyNode2d {
                        position: rel_transform.translation.xy(),
                        radius: node.radius,
                    })
                }
            }
        }
        compute.set_changed();
        buffer.set_data(all_nodes);
    }
}

#[derive(Component, Reflect)]
pub struct SoftBodyNode {
    pub radius: f32,
}
impl SoftBodyNode {
    /// Make the nodes move around.
    pub fn update(mut query: Query<&mut Transform, With<Self>>, time: Res<Time>) {
        let alpha = 0.0015;
        let omega = 2.0;
        for (i, mut transform) in query.iter_mut().enumerate() {
            let phi = i as f32;
            transform.translation.x += alpha * (time.elapsed_secs() * omega + phi).cos();
            transform.translation.y += alpha * (time.elapsed_secs() * omega + phi).sin();
        }
    }
}

#[derive(Component, Reflect)]
pub struct SoftBodyNodes(pub Vec<Entity>);
impl SoftBodyNodes {
    /// Update to the center of mass of all nodes.
    pub fn update(
        mut query: Query<(&mut Transform, &Self)>,
        node_transforms: Query<&GlobalTransform, With<SoftBodyNode>>,
    ) {
        for (mut transform, nodes) in query.iter_mut() {
            let mut sum_pos = Vec2::ZERO;
            for entity in &nodes.0 {
                if let Ok(node_transform) = node_transforms.get(*entity) {
                    sum_pos += node_transform.translation().xy();
                }
            }
            let centroid = sum_pos / (nodes.0.len() as f32);
            transform.translation.x = centroid.x;
            transform.translation.y = centroid.y;
        }
    }
}
