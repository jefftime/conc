// mod buffer;
mod command_buffer;
// mod framebuffer;
// mod pipeline;
mod shader;

use bytemuck::cast_slice;
// pub use buffer::Buffer;
pub use command_buffer::CommandBuffer;
// pub use framebuffer::Framebuffer;
// pub use pipeline::Pipeline;
pub use shader::Shader;

use crate::window::Window;
use std::borrow::Cow;
use wgpu::{
    Adapter, Backends, ColorTargetState, Device, DeviceDescriptor, Features,
    FragmentState, Instance, Limits, MultisampleState,
    PipelineLayoutDescriptor, PowerPreference, PresentMode as WgpuPresentMode,
    PrimitiveState, Queue, RenderPipeline, RenderPipelineDescriptor,
    RequestAdapterOptions, ShaderModuleDescriptor, ShaderSource, Surface,
    SurfaceConfiguration, TextureFormat, TextureUsages, VertexState,
};

#[allow(dead_code)]
pub enum PresentMode {
    Immediate,
    Fifo,
    Mailbox,
}

impl PresentMode {
    fn to_wgpu(&self) -> WgpuPresentMode {
        match self {
            PresentMode::Immediate => WgpuPresentMode::Immediate,
            PresentMode::Fifo => WgpuPresentMode::Fifo,
            PresentMode::Mailbox => WgpuPresentMode::Mailbox,
        }
    }
}

pub struct Render {
    _instance: Instance,
    _adapter: Adapter,
    surface: Surface,
    swapchain_format: TextureFormat,
    device: Device,
    queue: Queue,
}

impl Render {
    pub async fn new(window: &Window, present_mode: PresentMode) -> Render {
        #[cfg(target_os = "linux")]
        let instance = Instance::new(Backends::VULKAN);

        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to create WebGPU adapter");

        let (device, queue): (Device, Queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    features: Features::default(),
                    limits: Limits::downlevel_webgl2_defaults(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let swapchain_format = surface
            .get_preferred_format(&adapter)
            .expect("Failed to get swapchain format");

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: window.width as u32,
            height: window.height as u32,
            present_mode: present_mode.to_wgpu(),
        };

        surface.configure(&device, &config);

        Render {
            _instance: instance,
            _adapter: adapter,
            surface,
            swapchain_format,
            device,
            queue,
        }
    }

    pub fn create_shader(
        &self,
        vertex_shader: &[u8],
        fragment_shader: Option<&[u8]>,
    ) -> Shader {
        let vertex_module =
            self.device.create_shader_module(&ShaderModuleDescriptor {
                label: None,
                source: ShaderSource::SpirV(Cow::Borrowed(cast_slice(
                    vertex_shader,
                ))),
            });

        let fragment_module = if let Some(fshader) = fragment_shader {
            Some(self.device.create_shader_module(&ShaderModuleDescriptor {
                label: None,
                source: ShaderSource::SpirV(Cow::Borrowed(cast_slice(fshader))),
            }))
        } else {
            None
        };

        Shader::new(vertex_module, fragment_module)
    }

    pub fn create_pipeline(&self, shader: &Shader) -> RenderPipeline {
        let pipeline_layout =
            self.device
                .create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        let frag_state: &[ColorTargetState] = &[self.swapchain_format.into()];
        let frag_info = if let Some(ref f) = shader.frag {
            Some(FragmentState {
                module: f,
                entry_point: "main",
                targets: frag_state,
            })
        } else {
            None
        };

        self.device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &shader.vert,
                    entry_point: "main",
                    buffers: &[],
                },
                fragment: frag_info,
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
            })
    }

    pub fn start_commands(&self) -> CommandBuffer {
        CommandBuffer::begin(&self.device)
    }

    pub fn reconfigure(
        &mut self,
        width: i32,
        height: i32,
        present_mode: PresentMode,
    ) {
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: self.swapchain_format,
            width: width as u32,
            height: height as u32,
            present_mode: present_mode.to_wgpu(),
        };

        self.surface.configure(&self.device, &config);
    }
}
