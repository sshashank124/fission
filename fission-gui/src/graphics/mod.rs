mod texture;

use wgpu::util::DeviceExt;
use winit::{dpi::PhysicalSize, window::Window};

use fission::{color::{Color, RGB}, graphite::{A3, Conv, ConvFrom, F},
              image::{Image, pixel::Pixel}};

use texture::Texture;

pub struct Pipeline {
    surface:         wgpu::Surface,
    device:          wgpu::Device,
    queue:           wgpu::Queue,
    sc_desc:         wgpu::SwapChainDescriptor,
    swap_chain:      wgpu::SwapChain,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer:   wgpu::Buffer,
    bind_group:      wgpu::BindGroup,
    image:           Texture,
    dims:            PhysicalSize<u32>,
}

impl Pipeline {
    pub async fn new(window: &Window, dims: PhysicalSize<u32>) -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            .. Default::default()
        }).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor::default(), None).await.unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: dims.width, height: dims.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let image = Texture::new(&device, dims);

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
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
                ],
                label: Some("texture_bind_group_layout"),
            });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&image.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&image.sampler),
                },
            ],
            label: Some("bind_group"),
        });

        let vs_module = device.create_shader_module(
            wgpu::include_spirv!("shader.vert.spv"));
        let fs_module = device.create_shader_module(
            wgpu::include_spirv!("shader.frag.spv"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
                clamp_depth: false,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
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
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsage::VERTEX,
        });

        Self { surface, device, queue, sc_desc, swap_chain, render_pipeline,
               vertex_buffer, bind_group, image, dims }
    }

    pub fn update(&mut self, image: &Image) {
        let srgb = |f| if f <= 0.0031308 { 12.92 * f }
                       else { 1.055 * F::powf(f, 1.0 / 2.4) - 0.055 };

        let data = image.pixels().map(Pixel::eval).map(Color::to_rgb)
                                 .map(|RGB(c)| <A3<f32>>::of(c))
                                 .map(|c| c.map(srgb).swizzle(2, 1, 0))
                                 .flat_map(|c| vec![c[0], c[1], c[2], 1.])
                                 .map(|f| f32::clamp(255. * f, 0., 255.).conv())
                                 .collect::<Vec<_>>();

        self.queue.write_texture(
            wgpu::TextureCopyView
            { texture: &self.image.texture, mip_level: 0, origin: wgpu::Origin3d::ZERO },
            &data,
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * self.dims.width,
                rows_per_image: self.dims.height
            },
            wgpu::Extent3d { width: self.dims.width, height: self.dims.height, depth: 1 },
        );
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        let res = self.try_render();
        if let Err(wgpu::SwapChainError::Lost) = res
        { self.recreate_sc(); return Ok(()) }
        Ok(res?)
    }

    fn try_render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                    label: Some("Render Encoder") });

        {
            let mut render_pass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1, g: 0.2, b: 0.3, a: 1.0
                        }), store: true,
                    },
                }], depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..VERTICES.len() as u32, 0..1);
        }
        self.queue.submit(Some(encoder.finish()));

        Ok(())
    }

    fn recreate_sc(&mut self) {
        self.swap_chain = self.device.create_swap_chain(&self.surface,
                                                        &self.sc_desc);
    }
}

const VERTICES: &[[f32; 2]] = &[[0., 0.], [1., 1.], [0., 1.],
                                [0., 0.], [1., 0.], [1., 1.]];
