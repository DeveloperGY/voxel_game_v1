use crate::engine::chunk_system::chunk_vertex::ChunkVertex;
use crate::engine::chunk_system::voxel_data::{BlockType, VoxelData};
use crate::engine::gpu::CpuMesh;

pub struct ChunkMesher {}

impl ChunkMesher {
    pub fn new() -> Self {
        Self {}
    }

    pub fn mesh_chunk(&self, voxel_data: &VoxelData) -> CpuMesh<ChunkVertex> {
        let mut c_vertices = vec![];
        let mut c_indicies = vec![];

        let (c_x, c_z) = voxel_data.pos();
        println!("Meshing chunk ({}, {})", c_x, c_z);

        for z in 0..16 {
            for y in 0..16 {
                for x in 0..16 {
                    let v_x = c_x * 16 + x;
                    let v_z = c_z * 16 + z;

                    let index_x_off = x;
                    let index_y_off = y * 16;
                    let index_z_off = z * 16 * 16;
                    let index_off = index_x_off + index_y_off + index_z_off;

                    let (vertices, indices) = Self::generate_voxel(v_x, y, v_z, index_off as u32);
                    c_vertices.extend_from_slice(&vertices);
                    c_indicies.extend_from_slice(&indices);
                }
            }
        }

        CpuMesh::new(c_vertices, c_indicies)
    }

    fn generate_voxel(x: i32, y: i32, z: i32, index_offset: u32) -> (Vec<ChunkVertex>, Vec<u32>) {
        let x = x as f32;
        let y = y as f32;
        let z = z as f32;

        // Face normals
        let pos_z = [0.0, 0.0, 1.0];
        let neg_z = [0.0, 0.0, -1.0];
        let pos_y = [0.0, 1.0, 0.0];
        let neg_y = [0.0, -1.0, 0.0];
        let pos_x = [1.0, 0.0, 0.0];
        let neg_x = [-1.0, 0.0, 0.0];

        let mut vertices = Vec::with_capacity(24);
        let mut indices = Vec::with_capacity(36);

        // Front Face
        let tl = ChunkVertex {
            pos: [x, y + 1.0, z + 1.0],
            normal: pos_z
        };
        let bl = ChunkVertex {
            pos: [x, y, z + 1.0],
            normal: pos_z
        };
        let tr = ChunkVertex {
            pos: [x + 1.0, y + 1.0, z + 1.0],
            normal: pos_z
        };
        let br = ChunkVertex {
            pos: [x + 1.0, y, z + 1.0],
            normal: pos_z
        };

        vertices.push(tl);
        vertices.push(bl);
        vertices.push(tr);
        vertices.push(br);
        indices.push(0);
        indices.push(1);
        indices.push(2);
        indices.push(1);
        indices.push(3);
        indices.push(2);

        // Right Face
        let tl = ChunkVertex {
            pos: [x + 1.0, y + 1.0, z + 1.0],
            normal: pos_x
        };
        let bl = ChunkVertex {
            pos: [x + 1.0, y, z + 1.0],
            normal: pos_x
        };
        let tr = ChunkVertex {
            pos: [x + 1.0, y + 1.0, z],
            normal: pos_x
        };
        let br = ChunkVertex {
            pos: [x + 1.0, y, z],
            normal: pos_x
        };

        vertices.push(tl);
        vertices.push(bl);
        vertices.push(tr);
        vertices.push(br);
        indices.push(4);
        indices.push(5);
        indices.push(6);
        indices.push(5);
        indices.push(7);
        indices.push(6);

        // Back Face
        let tl = ChunkVertex {
            pos: [x + 1.0, y + 1.0, z],
            normal: neg_z
        };
        let bl = ChunkVertex {
            pos: [x + 1.0, y, z],
            normal: neg_z
        };
        let tr = ChunkVertex {
            pos: [x, y + 1.0, z],
            normal: neg_z
        };
        let br = ChunkVertex {
            pos: [x, y, z],
            normal: neg_z
        };

        vertices.push(tl);
        vertices.push(bl);
        vertices.push(tr);
        vertices.push(br);
        indices.push(8);
        indices.push(9);
        indices.push(10);
        indices.push(9);
        indices.push(11);
        indices.push(10);

        // Left Face
        let tl = ChunkVertex {
            pos: [x, y + 1.0, z],
            normal: neg_x
        };
        let bl = ChunkVertex {
            pos: [x, y, z],
            normal: neg_x
        };
        let tr = ChunkVertex {
            pos: [x, y + 1.0, z + 1.0],
            normal: neg_x
        };
        let br = ChunkVertex {
            pos: [x, y, z + 1.0],
            normal: neg_x
        };

        vertices.push(tl);
        vertices.push(bl);
        vertices.push(tr);
        vertices.push(br);
        indices.push(12);
        indices.push(13);
        indices.push(14);
        indices.push(13);
        indices.push(15);
        indices.push(14);

        // Top Face
        let tl = ChunkVertex {
            pos: [x, y + 1.0, z],
            normal: pos_y
        };
        let bl = ChunkVertex {
            pos: [x, y + 1.0, z + 1.0],
            normal: pos_y
        };
        let tr = ChunkVertex {
            pos: [x + 1.0, y + 1.0, z],
            normal: pos_y
        };
        let br = ChunkVertex {
            pos: [x + 1.0, y + 1.0, z + 1.0],
            normal: pos_y
        };

        vertices.push(tl);
        vertices.push(bl);
        vertices.push(tr);
        vertices.push(br);
        indices.push(16);
        indices.push(17);
        indices.push(18);
        indices.push(17);
        indices.push(19);
        indices.push(18);

        // Bottom Face
        let tl = ChunkVertex {
            pos: [x, y, z + 1.0],
            normal: neg_y
        };
        let bl = ChunkVertex {
            pos: [x, y, z],
            normal: neg_y
        };
        let tr = ChunkVertex {
            pos: [x + 1.0, y, z + 1.0],
            normal: neg_y
        };
        let br = ChunkVertex {
            pos: [x + 1.0, y, z],
            normal: neg_y
        };

        vertices.push(tl);
        vertices.push(bl);
        vertices.push(tr);
        vertices.push(br);
        indices.push(20);
        indices.push(21);
        indices.push(22);
        indices.push(21);
        indices.push(23);
        indices.push(22);

        (vertices, indices.into_iter().map(|e| e + index_offset * 24).collect())
    }
}
