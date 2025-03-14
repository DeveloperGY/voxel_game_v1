use crate::engine::chunk_system::voxel_data::{BlockType, VoxelData};

pub struct ChunkGenerator {}

impl ChunkGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_chunk(&self, chunk_x: i32, chunk_y: i32) -> VoxelData {
        println!("Generating chunk ({}, {})", chunk_x, chunk_y);
        
        // Generate chunk on heap to avoid stack overflow
        let mut uninit_chunk = Box::<[[[BlockType; 16]; 16]; 16]>::new_uninit();

        let ptr = uninit_chunk.as_mut_ptr();

        for z in 0..16 {
            for y in 0..16 {
                for x in 0..16 {
                    unsafe { (*ptr)[z][y][x] = BlockType::Solid };
                }
            }
        }

        let voxel_data = unsafe { uninit_chunk.assume_init() };
        VoxelData::new(voxel_data, (chunk_x, chunk_y))
    }
}
