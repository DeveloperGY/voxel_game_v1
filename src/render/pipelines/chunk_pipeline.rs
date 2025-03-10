use crate::gpu::GpuContext;
use crate::render::vertex::Vertex;
use bytemuck::{Pod, Zeroable};
use wgpu::{
    BlendState, BufferAddress, ColorTargetState, ColorWrites, FragmentState, FrontFace,
    MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode,
    PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, TextureFormat,
    VertexAttribute, VertexBufferLayout, VertexState, VertexStepMode,
};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct ChunkVertex {
    pub pos: [f32; 3],
}

impl ChunkVertex {
    const ATTRIBS: [VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];
}

impl Vertex for ChunkVertex {
    fn layout<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            attributes: &Self::ATTRIBS,
            array_stride: size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
        }
    }
}

pub struct ChunkRenderPipeline {
    pipeline: RenderPipeline,
}

impl ChunkRenderPipeline {
    pub fn new(gpu_ctx: &GpuContext, render_target_format: TextureFormat) -> Self {
        let pipeline_layout = gpu_ctx
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let shader = gpu_ctx
            .device
            .create_shader_module(wgpu::include_wgsl!("chunk_render.wgsl"));

        let pipeline = gpu_ctx
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
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
                        format: render_target_format,
                        blend: Some(BlendState::REPLACE),
                        write_mask: ColorWrites::ALL,
                    })],
                    compilation_options: PipelineCompilationOptions::default(),
                }),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: None,
                    polygon_mode: PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                multisample: MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                depth_stencil: None,
                cache: None,
            });

        Self { pipeline }
    }
}
