use crate::engine::gpu::GpuMesh;

pub trait ChunkLoader {
    fn queue_load_chunk(&mut self, pos: (i32, i32));
    fn queue_unload_chunk(&mut self, pos: (i32, i32));
    fn process_chunks(&mut self);
    fn get_meshes(&self) -> Vec<&GpuMesh>;
}
