use winit::dpi::PhysicalSize;

pub struct Texture {
    pub view:    wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub texture: wgpu::Texture,
}

impl Texture {
    pub fn new(device: &wgpu::Device, dims: PhysicalSize<u32>) -> Self {
        let size = wgpu::Extent3d { width: dims.width, height: dims.height,
                                    depth: 1 };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None, size, mip_level_count: 1, sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self { view, sampler, texture }
    }
}
