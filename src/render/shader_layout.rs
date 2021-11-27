use wgpu::{BindGroupLayout, BindGroupLayoutDescriptor};

use super::Render;

pub struct ShaderLayout {
    bind_group_layout: BindGroupLayout,
}

impl<'a> ShaderLayout {
    pub fn new(render: &Render) -> ShaderLayout {
        let bind_group_layout = render.device.create_bind_group_layout(
            &BindGroupLayoutDescriptor {
                label: None,
                entries: &[]
                
                // entries: &[BindGroupLayoutEntry {
                //     binding: 0,
                //     visibility: wgpu::ShaderStages::VERTEX,
                //     ty: wgpu::BindingType::Buffer {
                //         ty: wgpu::BufferBindingType::Storage {
                //             read_only: true,
                //         },
                //         has_dynamic_offset: false,
                //         min_binding_size: wgpu::BufferSize::new(24),
                //     },
                //     count: None,
                // }],
            },
        );

        ShaderLayout {
            bind_group_layout: bind_group_layout,
        }
    }

    pub fn get_bind_group_layout(&'a self) -> &'a BindGroupLayout {
        &self.bind_group_layout
    }
}
