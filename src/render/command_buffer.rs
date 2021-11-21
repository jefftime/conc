use wgpu::{
    CommandEncoder, CommandEncoderDescriptor, Device, LoadOp, Operations,
    Queue, RenderPass, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, TextureView,
};

pub struct CommandBuffer<'a> {
    encoder: *mut CommandEncoder,
    pipeline: Option<*const RenderPipeline>,
    render_pass: Option<RenderPass<'a>>,
}

impl<'a> CommandBuffer<'a> {
    fn new(
        encoder: *mut CommandEncoder,
        pipeline: Option<*const RenderPipeline>,
        render_pass: Option<RenderPass>,
    ) -> CommandBuffer {
        CommandBuffer {
            encoder,
            pipeline,
            render_pass,
        }
    }

    pub fn begin(device: &Device) -> CommandBuffer {
        let encoder = device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        CommandBuffer::new(Box::into_raw(Box::new(encoder)), None, None)
    }

    pub fn with_pipeline(
        self,
        pipeline: *const RenderPipeline,
        view: &'a TextureView,
    ) -> Self {
        let pass = {
            let render_pass = unsafe {
                (*self.encoder).begin_render_pass(&RenderPassDescriptor {
                    label: None,
                    color_attachments: &[RenderPassColorAttachment {
                        view,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(wgpu::Color::GREEN),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                })
            };
            render_pass
        };

        CommandBuffer::new(self.encoder, Some(pipeline), Some(pass))
    }

    pub fn draw(mut self) -> Self {
        if let (Some(ref mut pass), Some(pipeline)) =
            (&mut self.render_pass, self.pipeline)
        {
            unsafe {
                pass.set_pipeline(&*pipeline);
            }
            pass.draw(0..3, 0..1);
        }

        self
    }

    pub fn submit(self, queue: &Queue) {
        let encoder = unsafe { Box::from_raw(self.encoder) };
        queue.submit(Some(encoder.finish()));
    }
}
