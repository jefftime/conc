mod buffer;
mod command_buffer;
mod framebuffer;
mod pipeline;
mod shader;
mod shader_layout;

pub use buffer::Buffer;
use bytemuck::cast_slice;
pub use command_buffer::CommandBuffer;
pub use framebuffer::Framebuffer;
pub use pipeline::Pipeline;
pub use shader::Shader;
pub use shader_layout::{ShaderAttribute, ShaderAttributeType, ShaderLayout};

use crate::window::Window;
use std::{
    borrow::Cow,
    mem::{replace, size_of},
};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Adapter, Backends, BindGroup, BindGroupDescriptor, BindGroupEntry,
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, BufferBindingType, ColorTargetState, Device, DeviceDescriptor,
    Features, FragmentState, Instance, Limits, MultisampleState,
    PipelineLayoutDescriptor, PowerPreference, PresentMode as WgpuPresentMode,
    PrimitiveState, Queue, RenderPipelineDescriptor, RequestAdapterOptions,
    ShaderModuleDescriptor, ShaderSource, ShaderStages, Surface,
    SurfaceConfiguration, SurfaceTexture, TextureFormat, TextureUsages,
    TextureViewDescriptor, VertexBufferLayout, VertexState,
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
    pub surface: Surface,
    swapchain_format: TextureFormat,
    device: Device,
    pub queue: Queue,
    active_frame: Option<SurfaceTexture>,
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
            active_frame: None,
        }
    }

    pub fn create_shader_layout<const T: usize>(
        &self,
        attrs: [ShaderAttribute; T],
    ) -> ShaderLayout<T> {
        ShaderLayout::new(&self, attrs)
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

    pub fn create_pipeline<const T: usize>(
        &self,
        layout: &ShaderLayout<T>,
        shader: &Shader,
        bind_group_layout: &BindGroupLayout,
    ) -> Pipeline {
        let pipeline_layout =
            self.device
                .create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&bind_group_layout],
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

        let (stride, attrs) = layout.wgpu_attributes();
        let pipeline =
            self.device
                .create_render_pipeline(&RenderPipelineDescriptor {
                    label: None,
                    layout: Some(&pipeline_layout),
                    vertex: VertexState {
                        module: &shader.vert,
                        entry_point: "main",
                        buffers: &[VertexBufferLayout {
                            array_stride: stride as u64,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &attrs,
                        }],
                    },
                    fragment: frag_info,
                    primitive: PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: MultisampleState::default(),
                });

        Pipeline::new(pipeline)
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

    pub fn get_presentation_framebuffer(&mut self) -> Framebuffer {
        let frame = self.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&TextureViewDescriptor {
            ..Default::default()
        });
        self.active_frame = Some(frame);

        Framebuffer::new(view)
    }

    pub fn present(&mut self) {
        let frame = replace(&mut self.active_frame, None);
        self.active_frame = None;

        if let Some(frame) = frame {
            frame.present();
        }
    }

    fn create_buffer(&self, data: &[u8], usage: wgpu::BufferUsages) -> Buffer {
        let buf = self.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: data,
            usage,
        });

        Buffer::new(buf, data)
    }

    pub fn create_vertex_buffer(&self, data: &[u8]) -> Buffer {
        self.create_buffer(data, wgpu::BufferUsages::VERTEX)
    }

    pub fn create_index_buffer(&self, data: &[u8]) -> Buffer {
        self.create_buffer(data, wgpu::BufferUsages::INDEX)
    }

    pub fn create_uniforms<T>(&self, data: &T) -> Buffer {
        let data_bytes = unsafe {
            std::slice::from_raw_parts(
                data as *const T as *const u8,
                std::mem::size_of::<T>(),
            )
        };
        let buf = self.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: data_bytes,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Buffer::new(buf, data_bytes)
    }

    pub fn create_bind_group_layout<T>(&self) -> BindGroupLayout {
        self.device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            size_of::<T>() as u64
                        ),
                    },
                    count: None,
                }],
            })
    }

    pub fn create_bind_group(
        &self,
        layout: &BindGroupLayout,
        buffer: &Buffer,
    ) -> BindGroup {
        self.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.binding_resource(),
            }],
        })
    }

    pub fn write_buffer(&self, buffer: &Buffer, data: &[u8]) {
        self.queue.write_buffer(buffer.get_buf(), 0, data)
    }
}
