use crate::engine::chunk_system::chunk_vertex::ChunkVertex;
use crate::engine::gpu::{GpuCtx, GpuMesh, Vertex};
use crate::engine::render_system::Renderable;
use std::rc::Rc;
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

        system.load_chunks();
        system
    }

    pub fn load_chunks(&mut self) {
        let center_x = self.chunk_loading_center.0;
        let center_y = self.chunk_loading_center.1;

        (-self.chunk_loading_radius..self.chunk_loading_radius)
            .flat_map(|y| {
                (-self.chunk_loading_radius..self.chunk_loading_radius)
                    .map(move |x| (x + center_x, y + center_y))
            })
            .for_each(|pos| {
                self.loader.queue_load_chunk(pos);
            });
    }

    pub fn get_chunk_meshes(&self) -> Vec<&GpuMesh> {
        self.loader.get_meshes()
    }

    pub fn handle_chunk_jobs(&mut self) {
        self.loader.process_chunks();
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
