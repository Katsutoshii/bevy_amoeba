use bevy::app::{App, Plugin};

mod assets;
mod compute;
mod instances;
mod mesh;
mod nodes;
mod soft_body_compute;
mod soft_body_material;
mod vertices;

pub use crate::{
    assets::SoftBodyAssets,
    compute::{ComputeShader, ComputeShaderPlugin},
    mesh::CircleNGon,
    nodes::{SoftBody, SoftBodyNode, SoftBodyNode2dBuffer},
    soft_body_material::SoftBodyMaterial,
    vertices::{SoftBodyVertex2d, SoftBodyVertex2dBuffer},
};

pub struct SoftBodyPlugin;
impl Plugin for SoftBodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            vertices::SoftBodyVerticesPlugin,
            instances::SoftBodyInstancesPlugin,
            nodes::SoftBodyNodesPlugin,
            soft_body_material::SoftBodyMaterial2dPlugin,
            soft_body_compute::SoftBodyComputePlugin,
        ))
        .init_resource::<assets::SoftBodyAssets>();
    }
}
