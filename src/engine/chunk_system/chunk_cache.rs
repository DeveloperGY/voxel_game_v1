use crate::engine::chunk_system::chunk_generator::ChunkGenerator;
use crate::engine::chunk_system::chunk_mesher::ChunkMesher;
use crate::engine::chunk_system::voxel_data::VoxelData;
use crate::engine::gpu::{GpuCtx, GpuMesh};
use std::collections::{HashMap, VecDeque};

pub struct ChunkCache {
    meshes: Vec<GpuMesh>,
    voxels: Vec<VoxelData>,
    queue: VecDeque<(i32, i32)>,
    pos_to_data_ix: HashMap<(i32, i32), usize>,
    len: usize,
    max_size: usize,
}

impl ChunkCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            max_size: capacity,
            voxels: Vec::with_capacity(capacity),
            meshes: Vec::with_capacity(capacity),
            queue: VecDeque::with_capacity(capacity),
            pos_to_data_ix: HashMap::new(),
            len: 0,
        }
    }

    pub fn load_chunk(
        &mut self,
        pos: (i32, i32),
        generator: &ChunkGenerator,
        mesher: &ChunkMesher,
        gpu_ctx: &GpuCtx
    ) {
        match (self.pos_to_data_ix.get(&pos), self.len < self.max_size) {
            (Some(data_ix), _) => {
                let mesh = &self.meshes[*data_ix];
                let queue_ix = self.queue.iter().position(|e| *e == pos).unwrap();
                self.queue.remove(queue_ix);
                self.queue.push_back(pos);
            }
            (None, true) => {
                let voxel_data = generator.generate_chunk(pos.0, pos.1);
                let mesh = mesher.mesh_chunk(&voxel_data).to_gpu_mesh(gpu_ctx).unwrap();
                self.pos_to_data_ix.insert(pos, self.len);

                self.voxels.push(voxel_data);
                self.meshes.push(mesh);
                self.queue.push_back(pos);
                self.len += 1;
            }
            (None, false) => {
                let lru_pos = self.queue.pop_front().unwrap();
                let lru_index = *self.pos_to_data_ix.get(&lru_pos).unwrap();

                let voxel_data = generator.generate_chunk(pos.0, pos.1);
                let mesh = mesher.mesh_chunk(&voxel_data).to_gpu_mesh(gpu_ctx).unwrap();
                self.voxels[lru_index] = voxel_data;
                self.meshes[lru_index] = mesh;
                self.pos_to_data_ix.insert(pos, lru_index);
                self.pos_to_data_ix.remove(&lru_pos);
                self.queue.push_back(pos);
            }
        }
    }

    pub fn meshes(&self) -> &[GpuMesh] {
        &self.meshes
    }
}
