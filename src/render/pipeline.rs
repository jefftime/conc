use wgpu::RenderPipeline;

pub struct Pipeline {
    pipeline: RenderPipeline,
}

impl<'a> Pipeline {
    pub fn new(pipeline: RenderPipeline) -> Pipeline {
        Pipeline { pipeline }
    }

    pub fn pipeline(&'a self) -> &'a RenderPipeline {
        &self.pipeline
    }
}
