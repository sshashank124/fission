mod blit;

use winit::window::Window;

use fission::image::{Image, pixel::Pixel};

use blit::Blit;

const DISPLAY_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;

pub struct Context {
    surface:         wgpu::Surface,
    device:          wgpu::Device,
    queue:           wgpu::Queue,
    sc_desc:         wgpu::SwapChainDescriptor,
    swap_chain:      wgpu::SwapChain,
    blit:            Blit,
}

impl Context {
    pub async fn new(window: &Window) -> Self {
        let dims = window.inner_size();

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
            format: DISPLAY_FORMAT,
            width: dims.width, height: dims.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let blit = Blit::new(&device, dims);

        Self { surface, device, queue, sc_desc, swap_chain, blit }
    }

    pub fn update(&mut self, image: &Image) {
        let data = image.pixels().map(Pixel::to_rgbw).collect::<Vec<_>>();
        self.blit.update(bytemuck::cast_slice(&data), &self.queue);
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        let res = self.try_render();
        if let Err(wgpu::SwapChainError::Lost) = res
        { self.recreate_sc(); } else { res? }
        Ok(())
    }

    fn try_render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("render_encoder")
            });

        let render_pass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations::default(),
                }],
                depth_stencil_attachment: None,
        });
        self.blit.render_pass(render_pass);

        self.queue.submit(Some(encoder.finish()));

        Ok(())
    }

    fn recreate_sc(&mut self) {
        self.swap_chain = self.device.create_swap_chain(&self.surface,
                                                        &self.sc_desc);
    }
}
