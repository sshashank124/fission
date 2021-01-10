use std::mem;

use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;

const ELEM_SIZE: u32 = mem::size_of::<[f32; 4]>() as _;

pub struct Blit {
    img_buf:       wgpu::Buffer,
    render_bundle: wgpu::RenderBundle,
}

impl Blit {
    pub fn new(device: &wgpu::Device, dims: PhysicalSize<u32>) -> Self {
        let img_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("bitmap_image_buffer"),
            size: (dims.width * dims.height * ELEM_SIZE) as wgpu::BufferAddress,
            usage: wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        let width_buf = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("bitmap_uniform_width"),
                contents: bytemuck::bytes_of(&dims.width),
                usage: wgpu::BufferUsage::UNIFORM,
        });

        let bgle = &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::StorageBuffer {
                    dynamic: false,
                    min_binding_size: std::num::NonZeroU64::new(ELEM_SIZE as _),
                    readonly: true,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::UniformBuffer {
                    dynamic: false,
                    min_binding_size:
                        std::num::NonZeroU64::new(mem::size_of::<u32>() as _),
                },
                count: None,
            }
        ];

        let bgl = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("bitmap_bind_group_layout"),
                entries: bgle,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bitmap_bind_group"),
            layout: &bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(img_buf.slice(..)),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource:
                        wgpu::BindingResource::Buffer(width_buf.slice(..)),
                }
            ],
        });

        let vs_module = device.create_shader_module(
            wgpu::include_spirv!("shader.vert.spv"));
        let fs_module = device.create_shader_module(
            wgpu::include_spirv!("shader.frag.spv"));

        let rpl = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("bitmap_render_pipeline_layout"),
                bind_group_layouts: &[&bgl],
                push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("bitmap_render_pipeline"),
                layout: Some(&rpl),
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module: &vs_module,
                    entry_point: "main",
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                    module: &fs_module,
                    entry_point: "main",
                }),
                rasterization_state: None,
                primitive_topology: wgpu::PrimitiveTopology::TriangleStrip,
                color_states: &[wgpu::ColorStateDescriptor {
                    format: super::DISPLAY_FORMAT,
                    alpha_blend: wgpu::BlendDescriptor::default(),
                    color_blend: wgpu::BlendDescriptor::default(),
                    write_mask: wgpu::ColorWrite::default(),
                }],
                depth_stencil_state: None,
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
        });

        let mut render_bundle_encoder = device.create_render_bundle_encoder(
            &wgpu::RenderBundleEncoderDescriptor {
                label: Some("bitmap_render_bundle_encoder"),
                color_formats: &[super::DISPLAY_FORMAT],
                depth_stencil_format: None,
                sample_count: 1,
        });

        render_bundle_encoder.set_pipeline(&render_pipeline);
        render_bundle_encoder.set_bind_group(0, &bind_group, &[]);
        render_bundle_encoder.draw(0..4, 0..1);
        let render_bundle = render_bundle_encoder.finish(
            &wgpu::RenderBundleDescriptor {
                label: Some("bitmap_render_bundle"),
        });

        Self { img_buf, render_bundle }
    }

    pub fn update(&mut self, img_buf: &[u8], queue: &wgpu::Queue)
    { queue.write_buffer(&self.img_buf, 0, img_buf); }

    pub fn render_pass<'a>(&'a self, mut render_pass: wgpu::RenderPass<'a>)
    { render_pass.execute_bundles(std::iter::once(&self.render_bundle)); }
}
