use std::collections::HashMap;
use crate::engine::chunk_system::voxel_data::BlockType;

pub struct FaceAtlas {
    pub front: [u8; 2],
    pub back: [u8; 2],
    pub top: [u8; 2],
    pub bottom: [u8; 2],
    pub left: [u8; 2],
    pub right: [u8; 2],
}

pub struct TextureAtlas {
    map: HashMap<BlockType, FaceAtlas>
}

impl TextureAtlas {
    const DIRT_ATLAS: FaceAtlas = FaceAtlas {
        front: [1, 0],
        back: [1, 0],
        top: [2, 0],
        bottom: [0, 0],
        left: [1, 0],
        right: [1, 0]
    };
    
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(BlockType::Solid, Self::DIRT_ATLAS);
        
        Self {
            map
        }
    }
    
    pub fn get(&self, ty: BlockType) -> Option<&FaceAtlas> {
        self.map.get(&ty)
    }
}