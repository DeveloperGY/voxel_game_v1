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
                    if matches!(voxel_data.data()[z as usize][y as usize][x as usize], BlockType::Air) {
                        continue;
                    }

                    // Ensure face needs to be generated

                    // Front Face
                    let gen_front = if z < 15 {
                        matches!(voxel_data.data()[(z + 1) as usize][y as usize][x as usize], BlockType::Air)
                    } else {
                        true
                    };
                    // Right Face
                    let gen_right = if x < 15 {
                        matches!(voxel_data.data()[z as usize][y as usize][(x + 1) as usize], BlockType::Air)
                    } else {
                        true
                    };
                    // Back Face
                    let gen_back = if z > 0 {
                        matches!(voxel_data.data()[(z - 1) as usize][y as usize][x as usize], BlockType::Air)
                    } else {
                        true
                    };
                    // Left Face
                    let gen_left = if x > 0 {
                        matches!(voxel_data.data()[z as usize][y as usize][(x - 1) as usize], BlockType::Air)
                    } else {
                        true
                    };
                    // Top Face
                    let gen_top = if y < 15 {
                        matches!(voxel_data.data()[z as usize][(y + 1) as usize][x as usize], BlockType::Air)
                    } else {
                        true
                    };
                    // Bottom Face
                    let gen_bottom = if y > 0 {
                        matches!(voxel_data.data()[z as usize][(y - 1) as usize][x as usize], BlockType::Air)
                    } else {
                        true
                    };

                    // generate faces
                    let v_x = c_x * 16 + x;
                    let v_z = c_z * 16 + z;

                    if gen_front {
                        let v = Self::gen_front(v_x, y, v_z);
                        let i = Self::gen_face_indices(c_vertices.len() as u32);
                        c_vertices.extend_from_slice(&v);
                        c_indicies.extend_from_slice(&i);
                    }

                    if gen_right {
                        let v = Self::gen_right(v_x, y, v_z);
                        let i = Self::gen_face_indices(c_vertices.len() as u32);
                        c_vertices.extend_from_slice(&v);
                        c_indicies.extend_from_slice(&i);
                    }

                    if gen_back {
                        let v = Self::gen_back(v_x, y, v_z);
                        let i = Self::gen_face_indices(c_vertices.len() as u32);
                        c_vertices.extend_from_slice(&v);
                        c_indicies.extend_from_slice(&i);
                    }

                    if gen_left {
                        let v = Self::gen_left(v_x, y, v_z);
                        let i = Self::gen_face_indices(c_vertices.len() as u32);
                        c_vertices.extend_from_slice(&v);
                        c_indicies.extend_from_slice(&i);
                    }

                    if gen_top {
                        let v = Self::gen_top(v_x, y, v_z);
                        let i = Self::gen_face_indices(c_vertices.len() as u32);
                        c_vertices.extend_from_slice(&v);
                        c_indicies.extend_from_slice(&i);
                    }

                    if gen_bottom {
                        let v = Self::gen_bottom(v_x, y, v_z);
                        let i = Self::gen_face_indices(c_vertices.len() as u32);
                        c_vertices.extend_from_slice(&v);
                        c_indicies.extend_from_slice(&i);
                    }
                }
            }
        }

        CpuMesh::new(c_vertices, c_indicies)
    }

    fn gen_face_indices(starting_index: u32) -> [u32; 6] {
        [
            starting_index,
            starting_index + 1,
            starting_index + 2,
            starting_index + 1,
            starting_index + 3,
            starting_index + 2
        ]
    }

    fn gen_front(x: i32, y: i32, z: i32) -> [ChunkVertex; 4] {
        let x = x as f32;
        let y = y as f32;
        let z = z as f32;
        let pos_z = [0.0, 0.0, 1.0];

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
        [tl, bl, tr, br]
    }

    fn gen_right(x: i32, y: i32, z: i32) -> [ChunkVertex; 4] {
        let x = x as f32;
        let y = y as f32;
        let z = z as f32;
        let pos_x = [1.0, 0.0, 0.0];

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
        [tl, bl, tr, br]
    }

    fn gen_back(x: i32, y: i32, z: i32) -> [ChunkVertex; 4] {
        let x = x as f32;
        let y = y as f32;
        let z = z as f32;
        let neg_z = [0.0, 0.0, -1.0];

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
        [tl, bl, tr, br]
    }

    fn gen_left(x: i32, y: i32, z: i32) -> [ChunkVertex; 4] {
        let x = x as f32;
        let y = y as f32;
        let z = z as f32;
        let neg_x = [-1.0, 0.0, 0.0];

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
        [tl, bl, tr, br]
    }

    fn gen_top(x: i32, y: i32, z: i32) -> [ChunkVertex; 4] {
        let x = x as f32;
        let y = y as f32;
        let z = z as f32;
        let pos_y = [0.0, 1.0, 0.0];

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
        [tl, bl, tr, br]
    }

    fn gen_bottom(x: i32, y: i32, z: i32) -> [ChunkVertex; 4] {
        let x = x as f32;
        let y = y as f32;
        let z = z as f32;
        let neg_y = [0.0, -1.0, 0.0];

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
        [tl, bl, tr, br]
    }
}
