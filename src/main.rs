mod math;
mod render;
mod window;

use bytemuck::{cast_slice, Pod, Zeroable};
use render::{PresentMode, Render, Shader};
use std::{fs::File, io::Read, time::Instant};
use wgpu::{RenderPipelineDescriptor, TextureViewDescriptor};
use window::{input::Key, Window};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
struct Vertex {
    pos: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn new(pos: [f32; 3], color: [f32; 3]) -> Vertex {
        Vertex { pos, color }
    }
}

fn create_shader(render: &Render) -> Shader {
    let mut vshader_file =
        File::open("./shaders/vert.spv").expect("Failed to open vert.spv");
    let mut vsrc = vec![];
    vshader_file
        .read_to_end(&mut vsrc)
        .expect("Failed to read vert.spv");

    let mut fshader_file =
        File::open("./shaders/frag.spv").expect("Failed to open frag.spv");
    let mut fsrc = vec![];
    fshader_file
        .read_to_end(&mut fsrc)
        .expect("Failed to read frag.spv");

    render.create_shader(&vsrc, Some(&fsrc))
}

// fn create_buffer(render: &Render) -> Buffer {
//     let data = vec![
//         Vertex::new([0.0, 1.0, 0.0], [1.0, 0.0, 0.0]),
//         Vertex::new([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
//         Vertex::new([0.0, -1.0, 0.0], [0.0, 0.0, 1.0]),
//     ];

//     render.create_vertex_buffer(cast_slice(&data))
// }

async fn run(mut window: Window) {
    let mut render = Render::new(&window, PresentMode::Fifo).await;
    let mut dt_time = Instant::now();

    let shader = create_shader(&render);
    let pipeline = render.create_pipeline(&shader);

    loop {
        let dt = dt_time.elapsed().as_nanos();
        println!("fps: {}", 1_000_000_000_f64 / (dt as f64));
        dt_time = Instant::now();

        if window.should_close {
            break;
        }
        window.update();

        if window.getkey(Key::Escape) {
            break;
        }

        if window.did_resize {
            render.reconfigure(window.width, window.height, PresentMode::Fifo);
        }

        let frame = { render.surface.get_current_texture().unwrap() };
        let view = &frame.texture.create_view(&TextureViewDescriptor {
            ..Default::default()
        });

        let command_buffer = { render.start_commands() };
        command_buffer
            .with_pipeline(&pipeline as *const _, &view)
            .draw()
            .submit(&render.queue);
    }
}

fn main() {
    let sdl = sdl2::init().expect("Failed to initialize SDL");
    let window = Window::new(&sdl, "Conc", 640, 480);

    pollster::block_on(run(window));
}
