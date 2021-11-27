use std::mem::size_of;

use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindingType, VertexAttribute,
    VertexFormat,
};

use crate::util::IntoArray;

use super::Render;

pub enum ShaderAttributeType {
    Float,
    Vec2,
    Vec3,
    Vec4,
}

impl ShaderAttributeType {
    pub fn to_wgpu(&self) -> VertexFormat {
        match self {
            ShaderAttributeType::Float => wgpu::VertexFormat::Float32,
            ShaderAttributeType::Vec2 => wgpu::VertexFormat::Float32x2,
            ShaderAttributeType::Vec3 => wgpu::VertexFormat::Float32x3,
            ShaderAttributeType::Vec4 => wgpu::VertexFormat::Float32x4,
            _ => panic!(),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            ShaderAttributeType::Float => size_of::<f32>(),
            ShaderAttributeType::Vec2 => size_of::<f32>() * 2,
            ShaderAttributeType::Vec3 => size_of::<f32>() * 3,
            ShaderAttributeType::Vec4 => size_of::<f32>() * 4,
        }
    }
}

pub struct ShaderAttribute {
    kind: ShaderAttributeType,
    location: usize,
    offset: usize,
}

impl ShaderAttribute {
    pub fn new(
        kind: ShaderAttributeType,
        location: usize,
        offset: usize,
    ) -> ShaderAttribute {
        ShaderAttribute {
            kind,
            location,
            offset,
        }
    }
}

pub struct ShaderLayout<const T: usize> {
    bind_group_layout: BindGroupLayout,
    attrs: [ShaderAttribute; T],
}

impl<'a, const T: usize> ShaderLayout<T> {
    pub fn new(
        render: &Render,
        attrs: [ShaderAttribute; T],
    ) -> ShaderLayout<T> {
        let bind_group_layout = render.device.create_bind_group_layout(
            &BindGroupLayoutDescriptor {
                label: None,
                entries: &[],
            },
        );

        ShaderLayout {
            bind_group_layout,
            attrs,
        }
    }

    pub fn get_bind_group_layout(&'a self) -> &'a BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn wgpu_attributes(&self) -> [VertexAttribute; T] {
        self.attrs
            .iter()
            .map(|attr| VertexAttribute {
                format: attr.kind.to_wgpu(),
                offset: attr.offset as u64,
                shader_location: attr.location as u32,
            })
            .collect::<IntoArray<VertexAttribute, T>>()
            .array
    }
}
