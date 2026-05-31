use bevy::{
    app::{App, Plugin},
    ecs::{component::Component, entity::Entity},
    reflect::Reflect,
    transform::components::Transform,
};

mod compute;
mod mesh;
mod particle;
mod soft_body_compute;
mod soft_body_material;

pub use crate::{
    compute::{ComputeShader, ComputeShaderPlugin},
    mesh::MeshBuilder,
    particle::{Particle2d, Particle2dBuffer},
    soft_body_material::SoftBodyMaterial,
};

pub struct AmoebaPlugin;
impl Plugin for AmoebaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            particle::Particle2dPlugin,
            soft_body_material::SoftBodyMaterial2dPlugin,
            soft_body_compute::SoftBodyComputePlugin,
        ));
    }
}

#[derive(Component, Reflect)]
#[require(Transform)]
pub struct SoftBodyNodes(pub Vec<Entity>);

#[derive(Component, Reflect)]
pub struct SoftBodyNode;
