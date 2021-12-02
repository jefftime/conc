mod math;
mod render;
mod util;
mod window;

use bytemuck::{cast_slice, Pod, Zeroable};
use math::{Mat4, Vec4};
use obj::Obj;
use render::{
    Buffer, PresentMode, Render, Shader, ShaderAttribute, ShaderAttributeType,
};
use std::{fs::File, io::Read, time::Instant};
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

#[repr(C)]
#[derive(Clone, Copy)]
struct Uniforms {
    color: Vec4,
    mvp: Mat4,
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

fn try_open_obj(filepath: &str) {
    let obj_file = Obj::load(filepath).expect("Couldn't open file");
    // for obj in obj_file.data.objects {
    //     for group in obj.groups {
    //         println!("{:?}", group.polys);
    //     }
    // }
}

async fn run(mut window: Window) {
    let mut render = Render::new(&window, PresentMode::Fifo).await;
    let mut dt_time = Instant::now();

    try_open_obj("./assets/untitled.obj");

    let buffer = create_buffer(&render);
    let shader_layout = render.create_shader_layout([
        ShaderAttribute::new(ShaderAttributeType::Vec3, 0),
        ShaderAttribute::new(ShaderAttributeType::Vec3, 1),
    ]);
    let shader = create_shader(&render);
    let bind_layout = render.create_bind_group_layout();
    let pipeline =
        render.create_pipeline(&shader_layout, &shader, &bind_layout);

    let mut uniform_data = [1.0_f32, 0.3_f32, 0.3_f32, 1.0_f32];
    let uniforms = render.create_uniforms(cast_slice(&uniform_data));
    let bind_group = render.create_bind_group(&bind_layout, &uniforms);

    let mut timer = Instant::now();
    let mut dt_avg = 0.0_f64;
    let mut n_times = 0;

    loop {
        // let dt = 1_000_000_000_f64 / dt_time.elapsed().as_nanos() as f64;
        let dt = dt_time.elapsed().as_nanos() as f64 / 1_000_000_000_f64;
        dt_time = Instant::now();

        if cfg!(debug_assertions) {
            n_times += 1;
            let n_times_f = n_times as f64;
            dt_avg =
                (dt_avg * (n_times_f - 1.0_f64) / n_times_f) + dt / n_times_f;
            if timer.elapsed().as_millis() >= 500 {
                timer = Instant::now();
                // println!("{:.2} average fps", dt_avg);
                n_times = 0;
                dt_avg = 0.0;
            }
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

        uniform_data =
            uniform_data.map(|x| ((x + (0.5_f64 * dt) as f32) % 1.0));
        render.write_buffer(&uniforms, cast_slice(&uniform_data));
        render
            .start_commands()
            .configure_draw(&pipeline, &framebuffer)
            .set_vertices(&buffer)
            .bind_resources(&bind_group)
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
