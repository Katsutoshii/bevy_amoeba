use std::f32::consts::TAU;

use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, Mesh, MeshVertexAttribute, PrimitiveTopology, VertexFormat},
};

/// Utility struct for building a mesh.
#[derive(Default)]
pub struct MeshBuilder {
    pub positions: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

impl MeshBuilder {
    pub const ATTRIBUTE_SOFT_BODY: MeshVertexAttribute =
        MeshVertexAttribute::new("SoftBody", 0x7512b1e2bb023882, VertexFormat::Float32);

    pub fn circle_ngon(radius: f32, subdivisions: u32) -> Self {
        let mut builder = Self {
            positions: Vec::with_capacity((subdivisions + 1) as usize),
            uvs: Vec::with_capacity((subdivisions + 1) as usize),
            indices: Vec::with_capacity((subdivisions * 3) as usize),
        };

        // Center vertex (Index 0)
        builder.positions.push([0.0, 0.0, 0.0]);
        builder.uvs.push([0.5, 0.5]); // Center of UV texture space

        // Perimeter vertices (Indices 1 to N)
        for i in 0..subdivisions {
            let angle = (i as f32 / subdivisions as f32) * TAU;

            let x = radius * angle.cos();
            let y = radius * angle.sin();

            builder.positions.push([x, y, 0.0]);

            // Map [-radius, radius] coordinates to a normalized [0.0, 1.0] UV range
            let u = (x / radius) * 0.5 + 0.5;
            let v = (-y / radius) * 0.5 + 0.5;
            builder.uvs.push([u, v]);
        }

        // Build the N-gon fan by linking the center vertex (0) to outer pairs
        for i in 1..subdivisions {
            builder.indices.push(0); // Center
            builder.indices.push(i); // Current outer vertex
            builder.indices.push(i + 1); // Next outer vertex
        }

        // Final triangle: connects the last vertex back to the first perimeter vertex (1)
        builder.indices.push(0);
        builder.indices.push(subdivisions);
        builder.indices.push(1);

        builder
    }

    /// Produce a mesh from the accumulated attributes.
    pub fn build(self) -> Mesh {
        let soft_body = vec![0.0; self.positions.len()];
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs)
        .with_inserted_indices(Indices::U32(self.indices))
        .with_inserted_attribute(Self::ATTRIBUTE_SOFT_BODY, soft_body)
        .with_computed_smooth_normals()
    }
}
