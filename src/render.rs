use crate::window::Window;
use std::{borrow::Cow, fs::File, io::Read};
use wgpu::{
    Adapter, Device, Instance, Queue, RenderPipeline, Surface, TextureFormat,
};

pub enum PresentMode {
    Immediate,
    Fifo,
    Mailbox,
}

impl PresentMode {
    fn to_wgpu(&self) -> wgpu::PresentMode {
        match self {
            PresentMode::Immediate => wgpu::PresentMode::Immediate,
            PresentMode::Fifo => wgpu::PresentMode::Fifo,
            PresentMode::Mailbox => wgpu::PresentMode::Mailbox,
        }
    }
}

pub struct Render {
    instance: Instance,
    adapter: Adapter,
    surface: Surface,
    swapchain_format: TextureFormat,
    device: Device,
    queue: Queue,
    pipeline: RenderPipeline,
}

impl Render {
    pub async fn new(window: &Window, present_mode: PresentMode) -> Render {
        let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to create WebGPU adapter");

        let (device, queue): (Device, Queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::default(),
                    limits: wgpu::Limits::downlevel_webgl2_defaults(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let mut vshader_file =
            File::open("./shaders/vert.spv").expect("Failed to open vert.spv");
        let mut vshader_bytes = vec![];
        vshader_file
            .read_to_end(&mut vshader_bytes)
            .expect("Failed to read vert.spv");
        let vlen = vshader_bytes.len() / 4;
        let vsrc: Vec<u32> = unsafe {
            Vec::from_raw_parts(
                vshader_bytes.as_mut_ptr() as *mut u32,
                vlen,
                vlen,
            )
        };
        std::mem::forget(vshader_bytes);
        let vshader =
            device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::SpirV(Cow::Borrowed(&vsrc)),
            });

        let mut fshader_file =
            File::open("./shaders/frag.spv").expect("Failed to open frag.spv");
        let mut fshader_bytes = vec![];
        fshader_file
            .read_to_end(&mut fshader_bytes)
            .expect("Failed to read vert.spv");
        let flen = fshader_bytes.len() / 4;
        let fsrc: Vec<u32> = unsafe {
            Vec::from_raw_parts(
                fshader_bytes.as_mut_ptr() as *mut u32,
                flen,
                flen,
            )
        };
        std::mem::forget(fshader_bytes);
        let fshader =
            device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::SpirV(Cow::Borrowed(&fsrc)),
            });
        // let mut shader_file = File::open("./shaders/default.wgsl")
        //     .expect("Failed to open default.wgsl");
        // let mut shader_src = String::new();
        // shader_file
        //     .read_to_string(&mut shader_src)
        //     .expect("Failed to read default.wgsl");
        // let shader =
        //     device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        //         label: None,
        //         source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(&shader_src)),
        //     });

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let swapchain_format = surface
            .get_preferred_format(&adapter)
            .expect("Failed to get swapchain format");

        let render_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vshader,
                    entry_point: "main",
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &fshader,
                    entry_point: "main",
                    targets: &[swapchain_format.into()],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
            });

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: window.width as u32,
            height: window.height as u32,
            present_mode: present_mode.to_wgpu(),
        };

        surface.configure(&device, &config);

        Render {
            instance: instance,
            adapter: adapter,
            surface: surface,
            swapchain_format: swapchain_format,
            device: device,
            queue: queue,
            pipeline: render_pipeline,
        }
    }

    pub async fn reconfigure(
        &mut self,
        width: i32,
        height: i32,
        present_mode: PresentMode,
    ) {
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.swapchain_format,
            width: width as u32,
            height: height as u32,
            present_mode: present_mode.to_wgpu(),
        };

        self.surface.configure(&self.device, &config);
    }

    pub fn draw(&self) {
        let frame = match self.surface.get_current_texture() {
            Ok(f) => f,
            Err(_) => return,
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: None },
        );
        {
            let mut rpass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });

            rpass.set_pipeline(&self.pipeline);
            rpass.draw(0..3, 0..1);
        }
        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
