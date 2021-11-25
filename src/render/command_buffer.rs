use wgpu::{
    CommandEncoder, CommandEncoderDescriptor, Device, LoadOp, Operations,
    Queue, RenderPass, RenderPassColorAttachment, RenderPassDescriptor,
};

use super::{Framebuffer, Pipeline};

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
                        load: LoadOp::Clear(wgpu::Color::GREEN),
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

    pub fn draw(self) -> Self {
        unsafe {
            (*self.render_pass).draw(0..3, 0..1);
        }

        self
    }

    pub fn submit(self, queue: &Queue) {
        {
            let _render_pass = unsafe { Box::from_raw(self.render_pass) };
        }
        queue.submit(Some(self.encoder.finish()));
    }
}
