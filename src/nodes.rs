use bevy::{
    app::{Plugin, Update},
    asset::{Assets, DirectAssetAccessExt, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        lifecycle::HookContext,
        query::With,
        resource::Resource,
        schedule::IntoScheduleConfigs,
        system::{Query, Res, ResMut},
        world::{DeferredWorld, FromWorld, World},
    },
    math::{Vec2, Vec3Swizzles},
    mesh::Mesh3d,
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

use crate::{
    SoftBodyAssets, SoftBodyVertex2dBuffer, instances::SoftBodyInstanceData,
    soft_body_compute::SoftBodyCompute,
};

pub struct SoftBodyNodesPlugin;
impl Plugin for SoftBodyNodesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        load_shader_library!(app, "nodes.wgsl");
        app.init_resource::<SoftBodyNode2dBuffer>().add_systems(
            Update,
            (
                SoftBody::update.after(TransformSystems::Propagate),
                SoftBodyNode::update,
                SoftBody::update_buffers,
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
        Self(world.add_asset(ShaderStorageBuffer::from(Vec::<SoftBodyNode2d>::new())))
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
#[component(on_add = SoftBody::on_add, on_remove = SoftBody::on_remove)]
pub struct SoftBody(pub Vec<Entity>);
impl SoftBody {
    pub const MAX_NODES: usize = 1024;
    pub const MAX_INSTANCES: usize = 256;

    /// Initialize soft body on component add.
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let SoftBodyAssets { mesh, material } = world.resource::<SoftBodyAssets>().clone();
        world
            .commands()
            .entity(context.entity)
            .insert((Mesh3d(mesh), MeshMaterial3d(material)));

        let num_instances = world.resource_mut::<SoftBodyCompute>().add_instance();
        let buffer_handle = world.resource_mut::<SoftBodyVertex2dBuffer>().0.clone();
        if let Some(buffer) = world
            .resource_mut::<Assets<ShaderStorageBuffer>>()
            .get_mut(&buffer_handle)
        {
            SoftBodyVertex2dBuffer::resize_buffer(num_instances, buffer);
        }
    }

    /// Decrement compute counter on component remove.
    fn on_remove(mut world: DeferredWorld, _context: HookContext) {
        let num_instances = world.resource_mut::<SoftBodyCompute>().remove_instance();
        let buffer_handle = world.resource_mut::<SoftBodyVertex2dBuffer>().0.clone();
        if let Some(buffer) = world
            .resource_mut::<Assets<ShaderStorageBuffer>>()
            .get_mut(&buffer_handle)
        {
            SoftBodyVertex2dBuffer::resize_buffer(num_instances, buffer);
        }
    }

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

    /// Copy relative positions into the nodes buffer.
    pub fn update_buffers(
        mut compute: ResMut<SoftBodyCompute>,
        mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
        query: Query<(&GlobalTransform, &SoftBody)>,
        node_transforms: Query<(&GlobalTransform, &SoftBodyNode)>,
    ) {
        let mut all_nodes = [SoftBodyNode2d::default(); Self::MAX_NODES];
        let mut all_instances = [SoftBodyInstanceData::default(); Self::MAX_INSTANCES];

        let mut node_i = 0;
        let mut instance_i = 0;

        for (transform, nodes) in query.iter() {
            let node_offset = node_i;
            for entity in &nodes.0 {
                if let Ok((node_transform, node)) = node_transforms.get(*entity) {
                    let rel_transform = node_transform.reparented_to(transform);
                    all_nodes[node_i] = SoftBodyNode2d {
                        position: rel_transform.translation.xy(),
                        radius: node.radius,
                    };
                    node_i += 1;
                }
            }
            all_instances[instance_i] = SoftBodyInstanceData {
                node_offset: node_offset as u32,
                node_length: (node_i - node_offset) as u32,
            };
            instance_i += 1;
        }
        compute.num_instances = instance_i as u32;
        if let Some(node_buffer) = buffers.get_mut(&compute.nodes) {
            node_buffer.set_data(all_nodes);
        }
        if let Some(instance_buffer) = buffers.get_mut(&compute.instances) {
            instance_buffer.set_data(all_instances);
        }
    }
}
