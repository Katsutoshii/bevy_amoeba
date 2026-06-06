use bevy::{
    app::{App, Plugin},
    asset::{
        Asset,
        Handle,
        // AssetPath, embedded_asset, embedded_path
    },
    color::LinearRgba,
    ecs::resource::Resource,
    image::Image,
    mesh::{Mesh, MeshVertexBufferLayoutRef},
    pbr::{Material, MaterialPipeline, MaterialPipelineKey, MaterialPlugin},
    reflect::TypePath,
    render::{
        alpha::AlphaMode,
        extract_resource::ExtractResource,
        render_resource::{AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError},
        storage::ShaderStorageBuffer,
    },
    shader::ShaderRef,
};

#[derive(Default)]
pub struct SoftBodyMaterial2dPlugin;

impl Plugin for SoftBodyMaterial2dPlugin {
    fn build(&self, app: &mut App) {
        // embedded_asset!(app, "color_material.wgsl");

        app.add_plugins(MaterialPlugin::<SoftBodyMaterial>::default());
    }
}
// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Default, Debug, Clone, Resource, ExtractResource)]
pub struct SoftBodyMaterial {
    #[uniform(0)]
    pub color: LinearRgba,

    #[uniform(1)]
    pub vertices_per_particle: u32,

    #[texture(2)]
    #[sampler(3)]
    pub color_texture: Option<Handle<Image>>,

    #[storage(4, read_only)]
    pub vertices: Handle<ShaderStorageBuffer>,

    pub alpha_mode: AlphaMode,
}
impl SoftBodyMaterial {
    const SHADER_ASSET_PATH: &str = "shaders/soft_body_material.wgsl";
}
impl Material for SoftBodyMaterial {
    fn vertex_shader() -> ShaderRef {
        Self::SHADER_ASSET_PATH.into()
    }
    fn fragment_shader() -> ShaderRef {
        Self::SHADER_ASSET_PATH.into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}
