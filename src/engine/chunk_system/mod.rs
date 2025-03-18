use std::collections::HashSet;
use crate::engine::chunk_system::chunk_vertex::ChunkVertex;
use crate::engine::gpu::{GpuCtx, GpuMesh, Vertex};
use crate::engine::render_system::Renderable;
use std::sync::Arc;
use wgpu::{AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, BlendState, BufferBindingType, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState, Extent3d, Face, FilterMode, FragmentState, FrontFace, IndexFormat, MultisampleState, Origin3d, PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPass, RenderPipeline, RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages, StencilState, TexelCopyBufferLayout, TexelCopyTextureInfo, Texture, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages, TextureView, TextureViewDescriptor, TextureViewDimension, VertexState};
pub use chunk_loader::ChunkLoader;
pub use threaded_chunk_loader::ThreadedChunkLoader;

mod chunk_loader;
mod chunk_vertex;
mod threaded_chunk_loader;
mod voxel_data;

const TEXTURE_ATLAS_BYTES: &[u8] = include_bytes!("texture_atlas.png");

pub struct ChunkSystem<L: ChunkLoader> {
    chunk_loading_center: (i32, i32),
    chunk_loading_radius: i32,
    loader: L,

    chunk_render_pipeline: RenderPipeline,
    texture_atlas: Texture,
    texture_atlas_view: TextureView,
    texture_atlas_sampler: Sampler,
    texture_atlas_bind_group: BindGroup
}

impl<L: ChunkLoader> ChunkSystem<L> {
    pub fn new(gpu_ctx: Arc<GpuCtx>, loader: L) -> Self {
        let chunk_render_pipeline = create_chunk_render_pipeline(&gpu_ctx);
        let (texture_atlas, texture_atlas_view, texture_atlas_sampler) = create_texture_atlas(&gpu_ctx);
        let texture_atlas_bind_group = gpu_ctx.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &gpu_ctx.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float {
                                filterable: true
                            },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false
                        },
                        count: None
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None
                    }
                ]
            }),
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&texture_atlas_view),
            }, BindGroupEntry {
                binding: 1,
                resource: BindingResource::Sampler(&texture_atlas_sampler),
            }
            ]
        });

        let mut system = Self {
            chunk_loading_center: (0, 0),
            chunk_loading_radius: 16,

            loader,
            chunk_render_pipeline,
            texture_atlas,
            texture_atlas_view,
            texture_atlas_sampler,
            texture_atlas_bind_group
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
        pass.set_bind_group(1, &self.texture_atlas_bind_group, &[]);
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

    let texture_atlas_bind_group_layout = gpu_ctx.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float {
                        filterable: true
                    },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false
                },
                count: None
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
                count: None
            }
        ]
    });

    let layout = gpu_ctx
        .device
        .create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&camera_bind_group_layout, &texture_atlas_bind_group_layout],
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

fn create_texture_atlas(gpu_ctx: &GpuCtx) -> (Texture, TextureView, Sampler) {
    let size = Extent3d {
        width: 512,
        height: 512,
        depth_or_array_layers: 1
    };

    let texture = gpu_ctx.device.create_texture(&TextureDescriptor {
        label: None,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba8UnormSrgb,
        usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        view_formats: &[]
    });

    let diffuse_image = image::load_from_memory(TEXTURE_ATLAS_BYTES).unwrap();
    let diffuse_rgba = diffuse_image.to_rgba8();

    gpu_ctx.queue.write_texture(TexelCopyTextureInfo {
        texture: &texture,
        mip_level: 0,
        origin: Origin3d::ZERO,
        aspect: TextureAspect::All
    }, &diffuse_rgba, TexelCopyBufferLayout {
        offset: 0,
        bytes_per_row: Some(4 * size.width),
        rows_per_image: Some(size.height)
    }, size);

    let view = texture.create_view(&TextureViewDescriptor::default());
    let sampler = gpu_ctx.device.create_sampler(&SamplerDescriptor {
        label: None,
        address_mode_u: AddressMode::ClampToEdge,
        address_mode_v: AddressMode::ClampToEdge,
        address_mode_w: AddressMode::ClampToEdge,
        mag_filter: FilterMode::Nearest,
        min_filter: FilterMode::Nearest,
        mipmap_filter: FilterMode::Nearest,
        ..Default::default()
    });

    (texture, view, sampler)
}