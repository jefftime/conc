use wgpu::{
    BindGroup, CommandEncoder, CommandEncoderDescriptor, Device, LoadOp,
    Operations, Queue, RenderPass, RenderPassColorAttachment,
    RenderPassDescriptor,
};

use super::{Buffer, Framebuffer, Pipeline};

pub struct CommandBuffer<'a> {
    encoder: Box<CommandEncoder>,
    render_pass: *mut RenderPass<'a>,
}

impl<'a> CommandBuffer<'a> {
    fn new(
        encoder: Box<CommandEncoder>,
        render_pass: *mut RenderPass<'a>,
    ) -> CommandBuffer {
        CommandBuffer {
            encoder,
            render_pass,
        }
    }

    pub fn begin(device: &Device) -> CommandBuffer {
        let encoder = device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        CommandBuffer::new(Box::new(encoder), std::ptr::null_mut())
    }

    pub fn configure_draw(
        self,
        pipeline: &'a Pipeline,
        framebuffer: &'a Framebuffer,
    ) -> Self {
        let encoder = Box::into_raw(self.encoder);
        let mut render_pass = unsafe {
            (*encoder).begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[RenderPassColorAttachment {
                    view: framebuffer.get_target(),
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            })
        };

        render_pass.set_pipeline(pipeline.pipeline());

        CommandBuffer::new(
            unsafe { Box::from_raw(encoder) },
            Box::into_raw(Box::new(render_pass)),
        )
    }

    pub fn set_vertices(self, buffer: &'a Buffer) -> Self {
        let pass = unsafe { &mut *self.render_pass };
        pass.set_vertex_buffer(0, buffer.buf.slice(..));

        CommandBuffer::new(self.encoder, self.render_pass)
    }

    pub fn bind_resources(self, bind_info: &'a BindGroup) -> Self {
        let pass = unsafe { &mut *self.render_pass };
        pass.set_bind_group(0, bind_info, &[]);

        CommandBuffer::new(self.encoder, self.render_pass)
    }

    pub fn draw(self) -> Self {
        let pass = unsafe { &mut *self.render_pass };
        pass.draw(0..3, 0..1);

        CommandBuffer::new(self.encoder, self.render_pass)
    }

    pub fn submit(self, queue: &Queue) {
        {
            let _render_pass = unsafe { Box::from_raw(self.render_pass) };
        }
        queue.submit(Some(self.encoder.finish()));
    }
}
