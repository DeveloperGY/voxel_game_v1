use crate::engine::chunk_system::chunk_vertex::ChunkVertex;
use crate::engine::chunk_system::voxel_data::{BlockType, VoxelData};
use crate::engine::gpu::CpuMesh;

pub struct ChunkMesher {}

impl ChunkMesher {
    pub fn new() -> Self {
        Self {}
    }

    pub fn mesh_chunk(&self, voxel_data: &VoxelData) -> CpuMesh<ChunkVertex> {
        let mut vertices = vec![];
        let mut indicies = vec![];

        let (x, z) = voxel_data.pos();
        println!("Meshing chunk ({}, {})", x, z);
        let (x, z) = (x as f32, z as f32);

        // Front face
        let normal = [0.0, 0.0, -1.0];
        let top_left = ChunkVertex {
            pos: [0.0 + x, 1.0, 0.0 + z],
            normal
        };
        let bottom_left = ChunkVertex {
            pos: [0.0 + x, 0.0, 0.0 + z],
            normal
        };
        let top_right = ChunkVertex {
            pos: [1.0 + x, 1.0, 0.0 + z],
            normal
        };
        let bottom_right = ChunkVertex {
            pos: [1.0 + x, 0.0, 0.0 + z],
            normal
        };
        vertices.push(top_left);
        vertices.push(bottom_left);
        vertices.push(top_right);
        vertices.push(bottom_right);
        indicies.extend_from_slice(&[0, 1, 2, 1, 3, 2]);
        // Right face
        // Back face
        // Left face
        // Top Face
        // Bottom Face

        CpuMesh::new(vertices, indicies)
    }
}
