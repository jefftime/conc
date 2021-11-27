use wgpu::ShaderModule;

pub struct Shader {
    pub vert: ShaderModule,
    pub frag: Option<ShaderModule>,
}

impl<'a> Shader {
    pub fn new(vert: ShaderModule, frag: Option<ShaderModule>) -> Shader {
        Shader { vert, frag }
    }
}
