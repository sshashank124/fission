use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;

use fission::color::{Color, RGB};
use fission::graphite::{A3, Conv, ConvFrom, F};
use fission::image::{Image, pixel::Pixel};

pub const VERTICES: &[[f32; 2]] = &[[0., 0.], [1., 0.], [0., 1.], [1., 1.]];

pub struct Bitmap {
    texture:       wgpu::Texture,
    size:          wgpu::Extent3d,

    render_bundle: wgpu::RenderBundle,
}

impl Bitmap {
    pub fn new(device: &wgpu::Device, dims: PhysicalSize<u32>) -> Self {
        let size = wgpu::Extent3d { width: dims.width, height: dims.height,
                                    depth: 1 };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("bitmap_texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("bitmap_sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bgle = &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::SampledTexture {
                    multisampled: false,
                    dimension: wgpu::TextureViewDimension::D2,
                    component_type: wgpu::TextureComponentType::Float,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Sampler { comparison: false },
                count: None,
            },
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
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
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
                rasterization_state:
                    Some(wgpu::RasterizationStateDescriptor::default()),
                primitive_topology: wgpu::PrimitiveTopology::TriangleStrip,
                color_states: &[wgpu::ColorStateDescriptor {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    alpha_blend: wgpu::BlendDescriptor::default(),
                    color_blend: wgpu::BlendDescriptor::default(),
                    write_mask: wgpu::ColorWrite::default(),
                }],
                depth_stencil_state: None,
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[wgpu::VertexBufferDescriptor {
                        stride: 8,
                        step_mode: wgpu::InputStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float2],
                    }],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
        });

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("bitmap_vertex_buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsage::VERTEX,
        });

        let mut render_bundle_encoder = device.create_render_bundle_encoder(
            &wgpu::RenderBundleEncoderDescriptor {
                label: Some("bitmap_render_bundle_encoder"),
                color_formats: &[wgpu::TextureFormat::Bgra8UnormSrgb],
                depth_stencil_format: None,
                sample_count: 1,
        });

        render_bundle_encoder.set_pipeline(&render_pipeline);
        render_bundle_encoder.set_bind_group(0, &bind_group, &[]);
        render_bundle_encoder.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_bundle_encoder.draw(0..VERTICES.len() as u32, 0..1);
        let render_bundle = render_bundle_encoder.finish(
            &wgpu::RenderBundleDescriptor {
                label: Some("bitmap_render_bundle"),
        });

        Self { texture, size, render_bundle }
    }

    pub fn update(&mut self, image: &Image, queue: &wgpu::Queue) {
        let srgb = |f| if f <= 0.0031308 { 12.92 * f }
                       else { 1.055 * F::powf(f, 1.0 / 2.4) - 0.055 };

        let clamp_255 = |f: f32| f32::clamp(255. * f, 0., 255.);

        let data = image.pixels().map(Pixel::eval).map(Color::to_rgb)
                                 .map(|RGB(c)| <A3<f32>>::of(c))
                                 .map(|c| c.map(srgb).swizzle(2, 1, 0))
                                 .flat_map(|c| vec![c[0], c[1], c[2], 1.])
                                 .map(clamp_255)
                                 .map(Conv::conv)
                                 .collect::<Vec<_>>();

        queue.write_texture(wgpu::TextureCopyView {
            texture: &self.texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO 
        }, &data, wgpu::TextureDataLayout {
            offset: 0,
            bytes_per_row: 4 * self.size.width,
            rows_per_image: self.size.height,
        }, self.size);
    }

    pub fn render_pass<'a>(&'a self, mut render_pass: wgpu::RenderPass<'a>)
    { render_pass.execute_bundles(std::iter::once(&self.render_bundle)); }
}
