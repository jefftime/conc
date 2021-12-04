use wgpu::{BindingResource, Buffer as WgpuBuffer};

pub struct Buffer {
    buf: WgpuBuffer,
    size: usize,
    offset: usize,
}

impl<'a> Buffer {
    pub fn new(buf: WgpuBuffer, data: &[u8]) -> Buffer {
        Buffer {
            buf,
            size: data.len(),
            offset: 0,
        }
    }

    pub fn get_buf(&'a self) -> &'a WgpuBuffer {
        &self.buf
    }

    pub fn binding_resource(&self) -> BindingResource {
        self.buf.as_entire_binding()
    }
}
