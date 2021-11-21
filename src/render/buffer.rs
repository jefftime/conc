use wgpu::Buffer as WgpuBuffer;

pub struct Buffer {
    buf: WgpuBuffer,
    size: usize,
    offset: usize,
}

impl Buffer {
    pub fn new(buf: WgpuBuffer, data: &[u8]) -> Buffer {
        Buffer {
            buf,
            size: data.len(),
            offset: 0,
        }
    }
}
