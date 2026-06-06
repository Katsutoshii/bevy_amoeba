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
    pub const NUM_VERTICES_PER_INSTANCE: u32 = 64;
}
impl SoftBodyVertex2dBuffer {
    /// Resizes the vertex buffer to support the given number of instances.
    pub fn resize_buffer(num_instances: u32, buffer: &mut ShaderStorageBuffer) {
        let element_size = SoftBodyVertex2d::min_size().get() as usize;
        let new_buffer_size = element_size
            * num_instances as usize
            * SoftBodyVertex2dBuffer::NUM_VERTICES_PER_INSTANCE as usize;
        if let Some(data) = buffer.data.as_mut() {
            data.resize(new_buffer_size, 0u8);
        }
    }
}
impl FromWorld for SoftBodyVertex2dBuffer {
    fn from_world(world: &mut World) -> Self {
        Self(world.add_asset(ShaderStorageBuffer::from(Vec::<SoftBodyVertex2d>::new())))
    }
}
