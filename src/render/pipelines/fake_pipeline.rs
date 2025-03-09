use crate::gpu::GpuContext;
use bytemuck::{Pod, Zeroable};
use wgpu::{
    BlendComponent, BlendState, BufferAddress, ColorTargetState, ColorWrites, FragmentState,
    FrontFace, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode,
    PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, ShaderModule,
    TextureFormat, VertexAttribute, VertexBufferLayout, VertexState, VertexStepMode,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct FakeVertex {
    pos: [f32; 3],
}

impl FakeVertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { pos: [x, y, z] }
    }
}

impl FakeVertex {
    const ATTRIBS: [VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];

    pub fn layout<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            attributes: &Self::ATTRIBS,
            array_stride: size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
        }
    }
}

pub struct FakePipeline {
    pub pipeline: RenderPipeline,
}

impl FakePipeline {
    pub fn new(
        gpu_ctx: &GpuContext,
        shader: &ShaderModule,
        render_target_format: TextureFormat,
    ) -> Self {
        let layout = gpu_ctx
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let pipeline = gpu_ctx
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: None,
                layout: Some(&layout),
                vertex: VertexState {
                    module: shader,
                    entry_point: None,
                    buffers: &[FakeVertex::layout()],
                    compilation_options: PipelineCompilationOptions::default(),
                },
                fragment: Some(FragmentState {
                    module: shader,
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
                    unclipped_depth: false,
                    polygon_mode: PolygonMode::Fill,
                    conservative: false,
                },
                multisample: MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                depth_stencil: None,
                multiview: None,
                cache: None,
            });

        Self { pipeline }
    }
}
