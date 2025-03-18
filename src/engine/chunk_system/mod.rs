use std::collections::HashSet;
use crate::engine::chunk_system::chunk_vertex::ChunkVertex;
use crate::engine::gpu::{GpuCtx, GpuMesh, Vertex};
use crate::engine::render_system::Renderable;
use std::sync::Arc;
use wgpu::{
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BlendState, BufferBindingType,
    ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState, Face,
    FragmentState, FrontFace, IndexFormat, MultisampleState, PipelineCompilationOptions,
    PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPass,
    RenderPipeline, RenderPipelineDescriptor, ShaderStages, StencilState, TextureFormat,
    VertexState,
};

pub use chunk_loader::ChunkLoader;
pub use threaded_chunk_loader::ThreadedChunkLoader;

mod chunk_loader;
mod chunk_vertex;
mod threaded_chunk_loader;
mod voxel_data;

pub struct ChunkSystem<L: ChunkLoader> {
    chunk_loading_center: (i32, i32),
    chunk_loading_radius: i32,
    loader: L,

    chunk_render_pipeline: RenderPipeline,
}

impl<L: ChunkLoader> ChunkSystem<L> {
    pub fn new(gpu_ctx: Arc<GpuCtx>, loader: L) -> Self {
        let chunk_render_pipeline = create_chunk_render_pipeline(&gpu_ctx);

        let mut system = Self {
            chunk_loading_center: (0, 0),
            chunk_loading_radius: 16,

            loader,
            chunk_render_pipeline,
        };

        system.load_chunks(system.get_chunks_to_load(system.chunk_loading_center));
        system
    }

    pub fn player_moved(&mut self, p_x: i32, p_z: i32) {
        let new_c_x = p_x / 16;
        let new_c_z = p_z / 16;
        let new_chunk_loading_center = (new_c_x, new_c_z);

        if new_chunk_loading_center != self.chunk_loading_center {
            let old_chunks = self.get_chunks_to_load(self.chunk_loading_center);
            self.chunk_loading_center = new_chunk_loading_center;
            let new_chunks = self.get_chunks_to_load(self.chunk_loading_center);
            let chunks_to_remove = old_chunks.difference(&new_chunks).copied();
            let chunks_to_load = new_chunks.difference(&old_chunks).copied();
            self.unload_chunks(chunks_to_remove);
            self.load_chunks(chunks_to_load);
        }
    }

    pub fn unload_chunks(&mut self, chunks_to_remove: impl IntoIterator<Item = (i32, i32)>) {
        for pos in chunks_to_remove {
            self.loader.queue_unload_chunk(pos);
        }
    }

    pub fn load_chunks(&mut self, chunks_to_load: impl IntoIterator<Item = (i32, i32)>) {
        for pos in chunks_to_load {
            self.loader.queue_load_chunk(pos);
        }
    }

    pub fn get_chunk_meshes(&self) -> Vec<&GpuMesh> {
        self.loader.get_meshes()
    }

    pub fn handle_chunk_jobs(&mut self) {
        self.loader.process_chunks();
    }

    fn get_chunks_to_load(&self, (center_x, center_z): (i32, i32)) -> HashSet<(i32, i32)> {
        (-self.chunk_loading_radius..self.chunk_loading_radius)
            .flat_map(|y| {
                (-self.chunk_loading_radius..self.chunk_loading_radius)
                    .map(move |x| (x + center_x, y + center_z))
            }).collect()
    }
}

impl<L: ChunkLoader> Renderable for ChunkSystem<L> {
    fn render(&self, pass: &mut RenderPass) {
        pass.set_pipeline(&self.chunk_render_pipeline);
        for mesh in self.get_chunk_meshes() {
            let vertex = mesh.get_vertices();
            let index = mesh.get_indices();

            pass.set_vertex_buffer(0, vertex.slice(..));
            pass.set_index_buffer(index.slice(..), IndexFormat::Uint32);
            pass.draw_indexed(0..mesh.get_index_count(), 0, 0..1);
        }
    }
}

fn create_chunk_render_pipeline(gpu_ctx: &GpuCtx) -> RenderPipeline {
    let camera_bind_group_layout =
        gpu_ctx
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

    let layout = gpu_ctx
        .device
        .create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&camera_bind_group_layout],
            push_constant_ranges: &[],
        });

    let shader = gpu_ctx
        .device
        .create_shader_module(wgpu::include_wgsl!("chunk_shader.wgsl"));

    gpu_ctx
        .device
        .create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: VertexState {
                module: &shader,
                entry_point: None,
                buffers: &[ChunkVertex::layout()],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: None,
                targets: &[Some(ColorTargetState {
                    format: gpu_ctx.surface_format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::LessEqual,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multiview: None,
            cache: None,
        })
}
