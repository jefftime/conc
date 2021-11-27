mod math;
mod render;
mod util;
mod window;

use bytemuck::{cast_slice, Pod, Zeroable};
use render::{
    Buffer, PresentMode, Render, Shader, ShaderAttribute, ShaderAttributeType,
    ShaderLayout,
};
use std::{fs::File, io::Read, mem::size_of, time::Instant};
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

fn create_buffer(render: &Render) -> Buffer {
    let data = vec![
        Vertex::new([1.0, 0.0, 0.0], [1.0, 1.0, 0.0]),
        Vertex::new([0.0, 1.0, 0.0], [0.0, 1.0, 1.0]),
        Vertex::new([-1.0, -1.0, 0.0], [1.0, 0.0, 1.0]),
    ];

    render.create_vertex_buffer(cast_slice(&data))
}

async fn run(mut window: Window) {
    let mut render = Render::new(&window, PresentMode::Fifo).await;
    let mut dt_time = Instant::now();

    let buffer = create_buffer(&render);
    let shader_layout = render.create_shader_layout([
        ShaderAttribute::new(ShaderAttributeType::Vec3, 0, 0),
        ShaderAttribute::new(
            ShaderAttributeType::Vec3,
            1,
            ShaderAttributeType::Vec3.size(),
        ),
    ]);
    let shader = create_shader(&render);
    let pipeline = render.create_pipeline(&shader_layout, &shader);
    // let bind_group = render
    //     .create_bind_group(shader_layout.get_bind_group_layout(), &buffer);

    let mut timer = Instant::now();
    let mut dt_avg = 0_u128;
    let mut n_times = 0;

    loop {
        let dt = dt_time.elapsed().as_nanos();
        dt_time = Instant::now();

        n_times += 1;
        dt_avg = (dt_avg * (n_times - 1) / n_times) + dt / n_times;
        if timer.elapsed().as_millis() >= 750 {
            timer = Instant::now();
            println!("{:.2} average fps", 1_000_000_000_f64 / (dt_avg as f64));
            n_times = 0;
            dt_avg = 0;
        }

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

        let framebuffer = render.get_presentation_framebuffer();
        render
            .start_commands()
            .configure_draw(&pipeline, &framebuffer)
            // .bind_resources(&bind_group)
            .set_vertices(&buffer)
            .draw()
            .submit(&render.queue);

        render.present();
    }
}

fn main() {
    let sdl = sdl2::init().expect("Failed to initialize SDL");
    let window = Window::new(&sdl, "Conc", 640, 480);

    pollster::block_on(run(window));
}
