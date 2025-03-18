use crate::engine::chunk_system::chunk_loader::ChunkLoader;
use crate::engine::chunk_system::chunk_vertex::ChunkVertex;
use crate::engine::chunk_system::voxel_data::{BlockType, VoxelData};
use crate::engine::gpu::{CpuMesh, GpuCtx, GpuMesh};
use crate::engine::utils::ThreadPool;
use std::collections::{HashMap, HashSet};
use std::num::NonZero;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, channel};

pub struct ThreadedChunkLoader {
    thread_pool: Option<ThreadPool>,
    voxels: HashMap<(i32, i32), VoxelData>,
    meshes: HashMap<(i32, i32), GpuMesh>,
    mesh_priorities: HashMap<(i32, i32), u8>,
    voxels_to_load: HashSet<(i32, i32)>,
    meshes_to_load: HashSet<(i32, i32)>,

    voxel_job_tx: Sender<VoxelData>,
    voxel_job_recv: Receiver<VoxelData>,
    mesh_job_tx: Sender<((i32, i32), GpuMesh, u8)>,
    mesh_job_recv: Receiver<((i32, i32), GpuMesh, u8)>,

    gpu_ctx: Arc<GpuCtx>,
}

impl ThreadedChunkLoader {
    pub fn new(gpu_ctx: Arc<GpuCtx>) -> Self {
        let thread_pool = Some(ThreadPool::new(
            std::thread::available_parallelism()
                .unwrap_or(NonZero::new(4).unwrap())
                .get(),
        ));

        let (voxel_job_tx, voxel_job_recv) = channel();
        let (mesh_job_tx, mesh_job_recv) = channel();

        Self {
            thread_pool,
            voxels: HashMap::new(),
            meshes: HashMap::new(),
            mesh_priorities: HashMap::new(),
            voxels_to_load: HashSet::new(),
            meshes_to_load: HashSet::new(),
            voxel_job_tx,
            voxel_job_recv,
            mesh_job_tx,
            mesh_job_recv,
            gpu_ctx,
        }
    }
}

impl ChunkLoader for ThreadedChunkLoader {
    fn queue_load_chunk(&mut self, pos: (i32, i32)) {
        if !self.voxels.contains_key(&pos) {
            self.voxels_to_load.insert(pos);
        }
    }

    fn queue_unload_chunk(&mut self, pos: (i32, i32)) {
        self.voxels.remove(&pos);
        self.meshes.remove(&pos);
        self.mesh_priorities.remove(&pos);
    }

    fn process_chunks(&mut self) {
        let pool = self.thread_pool.as_ref().unwrap();

        // Queue voxel generation
        const QUEUE_SIZE: usize = 4;
        let mut loaded_voxels = Vec::with_capacity(QUEUE_SIZE);

        for pos in self.voxels_to_load.iter().take(QUEUE_SIZE) {
            if !self.voxels.contains_key(pos) {
                // queue voxel gen
                let pos = *pos;
                let rx = Sender::clone(&self.voxel_job_tx);

                pool.run(move || {
                    let voxels = generate_voxels(pos);
                    let _ = rx.send(voxels);
                })
            }

            loaded_voxels.push(*pos);
        }

        for pos in loaded_voxels {
            self.voxels_to_load.remove(&pos);
        };

        // Receive voxel data
        while let Ok(voxels) = self.voxel_job_recv.try_recv() {
            let (c_x, c_z) = voxels.pos();
            self.voxels.insert(voxels.pos(), voxels);

            // doesnt work due to race condition
            self.meshes_to_load.insert((c_x, c_z));

            let adj_chunks = [
                (c_x + 1, c_z),
                (c_x - 1, c_z),
                (c_x, c_z + 1),
                (c_x, c_z - 1),
            ];

            for pos in adj_chunks {
                self.meshes_to_load.insert(pos);
            }
        }

        // Queue mesh generation
        for pos in &self.meshes_to_load {
            if self.voxels.contains_key(pos) {
                // queue voxel gen
                let local = self.voxels.get(pos).unwrap().clone();
                let pos_x = self.voxels.get(&(pos.0 + 1, pos.1)).cloned();
                let neg_x = self.voxels.get(&(pos.0 - 1, pos.1)).cloned();
                let pos_z = self.voxels.get(&(pos.0, pos.1 + 1)).cloned();
                let neg_z = self.voxels.get(&(pos.0, pos.1 - 1)).cloned();

                let input = MeshGenInput {
                    local,
                    pos_x,
                    neg_x,
                    pos_z,
                    neg_z,
                };

                let rx = Sender::clone(&self.mesh_job_tx);
                let gpu_ctx = Arc::clone(&self.gpu_ctx);

                pool.run(move || {
                    let mesh = generate_mesh(input, gpu_ctx);
                    let _ = rx.send(mesh);
                })
            }
        }

        self.meshes_to_load.clear();

        // Receive mesh data
        while let Ok((pos, mesh, priority)) = self.mesh_job_recv.try_recv() {
            if self.mesh_priorities.get(&pos).map(|prev| *prev <= priority).unwrap_or(true) && self.voxels.contains_key(&pos) {
                self.mesh_priorities.insert(pos, priority);
                self.meshes.insert(pos, mesh);
            }
        }
    }

    fn get_meshes(&self) -> Vec<&GpuMesh> {
        self.meshes.values().collect()
    }
}

struct MeshGenInput {
    pub local: VoxelData,
    pub pos_x: Option<VoxelData>,
    pub neg_x: Option<VoxelData>,
    pub pos_z: Option<VoxelData>,
    pub neg_z: Option<VoxelData>,
}

fn generate_voxels((c_x, c_z): (i32, i32)) -> VoxelData {
    // Generate chunk on heap to avoid stack overflow
    let mut uninit_chunk = Box::<[[[BlockType; 16]; 256]; 16]>::new_uninit();

    let ptr = uninit_chunk.as_mut_ptr();

    for z in 0..16 {
        for y in 0..256 {
            for x in 0..16 {
                let v_x = x as i32 + 16 * c_x;
                let v_z = z as i32 + 16 * c_z;

                let math_x = v_x as f32 / 16.0;
                let math_z = v_z as f32 / 16.0;

                let y_max = 240+ ((math_x.sin() * math_z.sin() + 1.0) * 8.0).trunc() as i32;

                unsafe {
                    (*ptr)[z][y][x] = if y <= y_max as usize {
                        BlockType::Solid
                    } else {
                        BlockType::Air
                    }
                };
            }
        }
    }

    let voxel_data = unsafe { uninit_chunk.assume_init() };
    VoxelData::new(voxel_data, (c_x, c_z))
}

fn generate_mesh(
    MeshGenInput {
        local,
        pos_x,
        neg_x,
        pos_z,
        neg_z,
    }: MeshGenInput,
    gpu_ctx: Arc<GpuCtx>,
) -> ((i32, i32), GpuMesh, u8) {
    let mut c_vertices = vec![];
    let mut c_indicies = vec![];

    let (c_x, c_z) = local.pos();

    for z in 0..16 {
        for y in 0..256 {
            for x in 0..16 {
                if matches!(
                    local.data()[z as usize][y as usize][x as usize],
                    BlockType::Air
                ) {
                    continue;
                }

                // Ensure face needs to be generated
                const SHOULD_GENERATE_IF_ADJACENT_CHUNK_IS_UNKNOWN: bool = true;

                // Front Face
                let gen_front = match (z, pos_z.as_ref()) {
                    (0..15, _) => matches!(
                        local.data()[(z + 1) as usize][y as usize][x as usize],
                        BlockType::Air
                    ),
                    (15, Some(pos_z)) => {
                        matches!(pos_z.data()[0][y as usize][x as usize], BlockType::Air)
                    }
                    (15, None) => SHOULD_GENERATE_IF_ADJACENT_CHUNK_IS_UNKNOWN,
                    _ => unreachable!(),
                };

                // Right Face
                let gen_right = match (x, pos_x.as_ref()) {
                    (0..15, _) => matches!(
                        local.data()[z as usize][y as usize][(x + 1) as usize],
                        BlockType::Air
                    ),
                    (15, Some(pos_x)) => {
                        matches!(pos_x.data()[z as usize][y as usize][0], BlockType::Air)
                    }
                    (15, None) => SHOULD_GENERATE_IF_ADJACENT_CHUNK_IS_UNKNOWN,
                    _ => unreachable!(),
                };

                // Back Face
                let gen_back = match (z, neg_z.as_ref()) {
                    (1..16, _) => matches!(
                        local.data()[(z - 1) as usize][y as usize][x as usize],
                        BlockType::Air
                    ),
                    (0, Some(neg_z)) => {
                        matches!(neg_z.data()[15][y as usize][x as usize], BlockType::Air)
                    }
                    (0, None) => SHOULD_GENERATE_IF_ADJACENT_CHUNK_IS_UNKNOWN,
                    _ => unreachable!(),
                };

                // Left Face
                let gen_left = match (x, neg_x.as_ref()) {
                    (1..16, _) => matches!(
                        local.data()[z as usize][y as usize][(x - 1) as usize],
                        BlockType::Air
                    ),
                    (0, Some(neg_x)) => {
                        matches!(neg_x.data()[z as usize][y as usize][15], BlockType::Air)
                    }
                    (0, None) => SHOULD_GENERATE_IF_ADJACENT_CHUNK_IS_UNKNOWN,
                    _ => unreachable!(),
                };

                // Top Face
                let gen_top = if y < 255 {
                    matches!(
                        local.data()[z as usize][(y + 1) as usize][x as usize],
                        BlockType::Air
                    )
                } else {
                    true
                };
                // Bottom Face
                let gen_bottom = if y > 0 {
                    matches!(
                        local.data()[z as usize][(y - 1) as usize][x as usize],
                        BlockType::Air
                    )
                } else {
                    true
                };

                // generate faces
                let v_x = c_x * 16 + x;
                let v_z = c_z * 16 + z;

                if gen_front {
                    let v = gen_front_face(v_x, y, v_z);
                    let i = gen_face_indices(c_vertices.len() as u32);
                    c_vertices.extend_from_slice(&v);
                    c_indicies.extend_from_slice(&i);
                }

                if gen_right {
                    let v = gen_right_face(v_x, y, v_z);
                    let i = gen_face_indices(c_vertices.len() as u32);
                    c_vertices.extend_from_slice(&v);
                    c_indicies.extend_from_slice(&i);
                }

                if gen_back {
                    let v = gen_back_face(v_x, y, v_z);
                    let i = gen_face_indices(c_vertices.len() as u32);
                    c_vertices.extend_from_slice(&v);
                    c_indicies.extend_from_slice(&i);
                }

                if gen_left {
                    let v = gen_left_face(v_x, y, v_z);
                    let i = gen_face_indices(c_vertices.len() as u32);
                    c_vertices.extend_from_slice(&v);
                    c_indicies.extend_from_slice(&i);
                }

                if gen_top {
                    let v = gen_top_face(v_x, y, v_z);
                    let i = gen_face_indices(c_vertices.len() as u32);
                    c_vertices.extend_from_slice(&v);
                    c_indicies.extend_from_slice(&i);
                }

                if gen_bottom {
                    let v = gen_bottom_face(v_x, y, v_z);
                    let i = gen_face_indices(c_vertices.len() as u32);
                    c_vertices.extend_from_slice(&v);
                    c_indicies.extend_from_slice(&i);
                }
            }
        }
    }

    let priority = pos_x.map(|_| 1).unwrap_or(0)
        + neg_x.map(|_| 1).unwrap_or(0)
        + pos_z.map(|_| 1).unwrap_or(0)
        + neg_z.map(|_| 1).unwrap_or(0);

    (
        (c_x, c_z),
        CpuMesh::new(c_vertices, c_indicies)
            .to_gpu_mesh(&gpu_ctx)
            .unwrap(),
        priority,
    )
}

fn gen_face_indices(starting_index: u32) -> [u32; 6] {
    [
        starting_index,
        starting_index + 1,
        starting_index + 2,
        starting_index + 1,
        starting_index + 3,
        starting_index + 2,
    ]
}

fn gen_front_face(x: i32, y: i32, z: i32) -> [ChunkVertex; 4] {
    let x = x as f32;
    let y = y as f32;
    let z = z as f32;
    let pos_z = [0.0, 0.0, 1.0];

    let tl = ChunkVertex {
        pos: [x, y + 1.0, z + 1.0],
        normal: pos_z,
    };
    let bl = ChunkVertex {
        pos: [x, y, z + 1.0],
        normal: pos_z,
    };
    let tr = ChunkVertex {
        pos: [x + 1.0, y + 1.0, z + 1.0],
        normal: pos_z,
    };
    let br = ChunkVertex {
        pos: [x + 1.0, y, z + 1.0],
        normal: pos_z,
    };
    [tl, bl, tr, br]
}

fn gen_right_face(x: i32, y: i32, z: i32) -> [ChunkVertex; 4] {
    let x = x as f32;
    let y = y as f32;
    let z = z as f32;
    let pos_x = [1.0, 0.0, 0.0];

    let tl = ChunkVertex {
        pos: [x + 1.0, y + 1.0, z + 1.0],
        normal: pos_x,
    };
    let bl = ChunkVertex {
        pos: [x + 1.0, y, z + 1.0],
        normal: pos_x,
    };
    let tr = ChunkVertex {
        pos: [x + 1.0, y + 1.0, z],
        normal: pos_x,
    };
    let br = ChunkVertex {
        pos: [x + 1.0, y, z],
        normal: pos_x,
    };
    [tl, bl, tr, br]
}

fn gen_back_face(x: i32, y: i32, z: i32) -> [ChunkVertex; 4] {
    let x = x as f32;
    let y = y as f32;
    let z = z as f32;
    let neg_z = [0.0, 0.0, -1.0];

    let tl = ChunkVertex {
        pos: [x + 1.0, y + 1.0, z],
        normal: neg_z,
    };
    let bl = ChunkVertex {
        pos: [x + 1.0, y, z],
        normal: neg_z,
    };
    let tr = ChunkVertex {
        pos: [x, y + 1.0, z],
        normal: neg_z,
    };
    let br = ChunkVertex {
        pos: [x, y, z],
        normal: neg_z,
    };
    [tl, bl, tr, br]
}

fn gen_left_face(x: i32, y: i32, z: i32) -> [ChunkVertex; 4] {
    let x = x as f32;
    let y = y as f32;
    let z = z as f32;
    let neg_x = [-1.0, 0.0, 0.0];

    let tl = ChunkVertex {
        pos: [x, y + 1.0, z],
        normal: neg_x,
    };
    let bl = ChunkVertex {
        pos: [x, y, z],
        normal: neg_x,
    };
    let tr = ChunkVertex {
        pos: [x, y + 1.0, z + 1.0],
        normal: neg_x,
    };
    let br = ChunkVertex {
        pos: [x, y, z + 1.0],
        normal: neg_x,
    };
    [tl, bl, tr, br]
}

fn gen_top_face(x: i32, y: i32, z: i32) -> [ChunkVertex; 4] {
    let x = x as f32;
    let y = y as f32;
    let z = z as f32;
    let pos_y = [0.0, 1.0, 0.0];

    let tl = ChunkVertex {
        pos: [x, y + 1.0, z],
        normal: pos_y,
    };
    let bl = ChunkVertex {
        pos: [x, y + 1.0, z + 1.0],
        normal: pos_y,
    };
    let tr = ChunkVertex {
        pos: [x + 1.0, y + 1.0, z],
        normal: pos_y,
    };
    let br = ChunkVertex {
        pos: [x + 1.0, y + 1.0, z + 1.0],
        normal: pos_y,
    };
    [tl, bl, tr, br]
}

fn gen_bottom_face(x: i32, y: i32, z: i32) -> [ChunkVertex; 4] {
    let x = x as f32;
    let y = y as f32;
    let z = z as f32;
    let neg_y = [0.0, -1.0, 0.0];

    let tl = ChunkVertex {
        pos: [x, y, z + 1.0],
        normal: neg_y,
    };
    let bl = ChunkVertex {
        pos: [x, y, z],
        normal: neg_y,
    };
    let tr = ChunkVertex {
        pos: [x + 1.0, y, z + 1.0],
        normal: neg_y,
    };
    let br = ChunkVertex {
        pos: [x + 1.0, y, z],
        normal: neg_y,
    };
    [tl, bl, tr, br]
}
