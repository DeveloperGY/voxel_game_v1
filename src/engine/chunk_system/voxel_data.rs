#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum BlockType {
    Air,
    Solid,
}

#[derive(Clone)]
pub struct VoxelData {
    chunk_pos: (i32, i32),
    voxels: Box<[[[BlockType; 16]; 256]; 16]>,
}

impl VoxelData {
    pub fn new(voxels: Box<[[[BlockType; 16]; 256]; 16]>, chunk_pos: (i32, i32)) -> Self {
        Self { voxels, chunk_pos }
    }

    pub fn data(&self) -> &[[[BlockType; 16]; 256]; 16] {
        &self.voxels
    }

    pub fn pos(&self) -> (i32, i32) {
        self.chunk_pos
    }
}
