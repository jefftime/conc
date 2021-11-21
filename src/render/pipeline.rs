use wgpu::RenderPipeline;

pub struct Pipeline {
    pipeline: RenderPipeline,
}

impl Pipeline {
    pub fn new(pipeline: RenderPipeline) -> Pipeline {
        Pipeline { pipeline }
    }
}
