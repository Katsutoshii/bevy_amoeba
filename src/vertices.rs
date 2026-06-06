use bevy::{
    app::Plugin,
    asset::{DirectAssetAccessExt, Handle},
    ecs::{
        resource::Resource,
        world::{FromWorld, World},
    },
    math::Vec2,
    render::{
        extract_resource::ExtractResource, render_resource::ShaderType,
        storage::ShaderStorageBuffer,
    },
    shader::load_shader_library,
};

pub struct SoftBodyVerticesPlugin;
impl Plugin for SoftBodyVerticesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        load_shader_library!(app, "rand.wgsl");
        load_shader_library!(app, "vertices.wgsl");
        app.init_resource::<SoftBodyVertex2dBuffer>();
    }
}
#[derive(Default, ShaderType, Copy, Clone, Debug)]
pub struct SoftBodyVertex2d {
    pub position: Vec2,
}

#[derive(Resource, ExtractResource, Clone)]
pub struct SoftBodyVertex2dBuffer(pub Handle<ShaderStorageBuffer>);
impl SoftBodyVertex2dBuffer {
    pub const NUM_VERTICES: u32 = 128;
}

impl FromWorld for SoftBodyVertex2dBuffer {
    fn from_world(world: &mut World) -> Self {
        let particles = [SoftBodyVertex2d::default(); Self::NUM_VERTICES as usize];
        Self(world.add_asset(ShaderStorageBuffer::from(particles)))
    }
}
