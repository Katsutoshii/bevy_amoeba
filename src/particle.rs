use bevy::{
    app::Plugin,
    asset::{DirectAssetAccessExt, Handle},
    color::LinearRgba,
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

pub struct Particle2dPlugin;
impl Plugin for Particle2dPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        load_shader_library!(app, "particle.wgsl");
        app.init_resource::<Particle2dBuffer>();
    }
}
#[derive(Default, ShaderType, Copy, Clone, Debug)]
pub struct Particle2d {
    pub position: Vec2,
    pub velocity: Vec2,
    pub color: LinearRgba,
}

#[derive(Resource, ExtractResource, Clone)]
pub struct Particle2dBuffer(pub Handle<ShaderStorageBuffer>);
impl Particle2dBuffer {
    pub const MAX_PARTICLES: u32 = 64;
}

impl FromWorld for Particle2dBuffer {
    fn from_world(world: &mut World) -> Self {
        let particles = [Particle2d::default(); Self::MAX_PARTICLES as usize];
        Self(world.add_asset(ShaderStorageBuffer::from(particles)))
    }
}
