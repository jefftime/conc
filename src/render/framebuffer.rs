use wgpu::TextureView;

pub struct Framebuffer {
    pub target: TextureView,
}

impl<'a> Framebuffer {
    pub fn new(target: TextureView) -> Framebuffer {
        Framebuffer { target }
    }

    pub fn get_target(&'a self) -> &'a TextureView {
        &self.target
    }
}
