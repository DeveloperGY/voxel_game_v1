use std::rc::Rc;
use wgpu::{BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BlendState, BufferBindingType, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState, Face, FragmentState, FrontFace, IndexFormat, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPass, RenderPipeline, RenderPipelineDescriptor, ShaderStages, StencilState, TextureFormat, VertexState};
use crate::engine::chunk_system::chunk_cache::ChunkCache;
use crate::engine::chunk_system::chunk_generator::ChunkGenerator;
use crate::engine::chunk_system::chunk_mesher::ChunkMesher;
use crate::engine::chunk_system::chunk_vertex::ChunkVertex;
use crate::engine::gpu::{GpuCtx, GpuMesh, Vertex};
use crate::engine::render_system::Renderable;

mod chunk_cache;
mod chunk_generator;
mod chunk_mesher;
mod chunk_vertex;
mod voxel_data;

pub struct ChunkSystem {
    chunk_loading_center: (i32, i32),
    chunk_loading_radius: i32,
    chunk_generator: ChunkGenerator,
    chunk_mesher: ChunkMesher,
    chunk_cache: ChunkCache,
    gpu_ctx: Rc<GpuCtx>,
    chunk_render_pipeline: RenderPipeline
}

impl ChunkSystem {
    pub fn new(gpu_ctx: Rc<GpuCtx>) -> Self {
        let chunk_render_pipeline = create_chunk_render_pipeline(&gpu_ctx);
        
        let mut system = Self {
            chunk_loading_center: (0, 0),
            chunk_loading_radius: 16,
            chunk_generator: ChunkGenerator::new(),
            chunk_mesher: ChunkMesher::new(),
            chunk_cache: ChunkCache::new(33 * 33),
            gpu_ctx,
            chunk_render_pipeline
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
                self.chunk_cache
                    .load_chunk(pos, &self.chunk_generator, &self.chunk_mesher, &self.gpu_ctx)
            });
    }

    pub fn get_chunk_meshes(&self) -> &[GpuMesh] {
        self.chunk_cache.meshes()
    }
}

impl Renderable for ChunkSystem {
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
    let camera_bind_group_layout = gpu_ctx.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            }
        ],
    });

    let layout = gpu_ctx.device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&camera_bind_group_layout],
        push_constant_ranges: &[]
    });
    
    let shader = gpu_ctx.device.create_shader_module(wgpu::include_wgsl!("chunk_shader.wgsl"));
    
    gpu_ctx.device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: Some(&layout),
        vertex: VertexState {
            module: &shader,
            entry_point: None,
            buffers: &[ChunkVertex::layout()],
            compilation_options: PipelineCompilationOptions::default()
        },
        fragment: Some(FragmentState {
            module: &shader,
            entry_point: None,
            targets: &[Some(ColorTargetState {
                format: gpu_ctx.surface_format,
                blend: Some(BlendState::REPLACE),
                write_mask: ColorWrites::ALL
            })],
            compilation_options: PipelineCompilationOptions::default()
        }),
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: Some(Face::Back),
            unclipped_depth: false,
            polygon_mode: PolygonMode::Line,
            conservative: false,
        },
        multisample: MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false
        },
        depth_stencil: Some(DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::Less,
            stencil: StencilState::default(),
            bias: DepthBiasState::default()
        }),
        multiview: None,
        cache: None
    })
}