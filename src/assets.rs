use bevy::{
    asset::{DirectAssetAccessExt, Handle},
    color::Color,
    ecs::{
        resource::Resource,
        world::{FromWorld, World},
    },
    mesh::Mesh,
    reflect::Reflect,
    render::alpha::AlphaMode,
    utils::default,
};

use crate::{CircleNGon, SoftBodyMaterial, SoftBodyVertex2dBuffer};

#[derive(Resource, Reflect)]
pub struct SoftBodyAssets {
    pub material: Handle<SoftBodyMaterial>,
    pub mesh: Handle<Mesh>,
}
impl FromWorld for SoftBodyAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            material: world.add_asset(SoftBodyMaterial {
                color: Color::WHITE.to_linear(),
                color_texture: Some(world.load_asset("textures/bubble_7.png")),
                num_vertices_per_instance: SoftBodyVertex2dBuffer::NUM_VERTICES,
                vertices: world.resource::<SoftBodyVertex2dBuffer>().0.clone(),
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            mesh: world.add_asset(CircleNGon {
                n: (SoftBodyVertex2dBuffer::NUM_VERTICES - 1) as usize,
                r: 1.0,
            }),
        }
    }
}
