use bevy::{
    app::Plugin,
    asset::{DirectAssetAccessExt, Handle},
    ecs::{
        resource::Resource,
        world::{FromWorld, World},
    },
    render::{
        extract_resource::ExtractResource, render_resource::ShaderType,
        storage::ShaderStorageBuffer,
    },
    shader::load_shader_library,
};

pub struct SoftBodyInstancesPlugin;
impl Plugin for SoftBodyInstancesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        load_shader_library!(app, "instances.wgsl");
        app.init_resource::<SoftBodyInstanceBuffer>();
    }
}

#[derive(Default, ShaderType, Copy, Clone, Debug)]
pub struct SoftBodyInstanceData {
    pub node_offset: u32,
    pub node_length: u32,
}

#[derive(Resource, ExtractResource, Clone)]
pub struct SoftBodyInstanceBuffer(pub Handle<ShaderStorageBuffer>);
impl FromWorld for SoftBodyInstanceBuffer {
    fn from_world(world: &mut World) -> Self {
        Self(world.add_asset(ShaderStorageBuffer::from(Vec::<SoftBodyInstanceData>::new())))
    }
}
