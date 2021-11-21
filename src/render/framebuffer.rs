use wgpu::TextureView;

pub struct Framebuffer {
    pub target: TextureView,
}

impl Framebuffer {
    pub fn new(target: TextureView) -> Framebuffer {
        Framebuffer { target }
    }
}
