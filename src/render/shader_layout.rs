use std::{cell::Cell, mem::size_of, rc::Rc};

use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, VertexAttribute, VertexFormat,
};

use crate::util::IntoArray;

use super::Render;

// macro_rules! shader_attributes {
//     (_ $offset:expr => $attr:expr, $attrs:expr,*) => {
//         let attr: ShaderAttribute = $attr;
//         let offset = $offset + attr.kind.size();
//         shader_attributes(_ offset => )
//     };

//     ($attr:expr, $attrs:expr*) => {
//         shader_attributes!(_ 0 => $attr, $attrs)
//     };
// }

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
}

impl ShaderAttribute {
    pub fn new(kind: ShaderAttributeType, location: usize) -> ShaderAttribute {
        ShaderAttribute { kind, location }
    }
}

pub struct ShaderLayout<const T: usize> {
    // bind_group_layout: BindGroupLayout,
    attrs: [ShaderAttribute; T],
}

impl<'a, const T: usize> ShaderLayout<T> {
    pub fn new(
        render: &Render,
        attrs: [ShaderAttribute; T],
    ) -> ShaderLayout<T> {
        // let bind_group_layout = render.device.create_bind_group_layout(
        //     &BindGroupLayoutDescriptor {
        //         label: None,
        //         entries: &[],
        //     },
        // );

        ShaderLayout {
            // bind_group_layout,
            attrs,
        }
    }

    // pub fn get_bind_group_layout(&'a self) -> &'a BindGroupLayout {
    //     &self.bind_group_layout
    // }

    pub fn wgpu_attributes(&self) -> (usize, [VertexAttribute; T]) {
        let offset = Rc::new(Cell::new(0_usize));

        (
            self.attrs.iter().fold(0, |acc, x| acc + x.kind.size()),
            self.attrs
                .iter()
                .map(move |attr| {
                    let result = VertexAttribute {
                        format: attr.kind.to_wgpu(),
                        offset: offset.get() as u64,
                        // offset: attr.offset as u64,
                        shader_location: attr.location as u32,
                    };
                    offset.set(offset.get() + attr.kind.size());

                    result
                })
                .collect::<IntoArray<VertexAttribute, T>>()
                .array,
        )
    }
}
