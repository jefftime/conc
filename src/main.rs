// mod math;
mod render;
mod util;
mod window;

use bytemuck::{cast_slice, Pod, Zeroable};
use cgmath::{perspective, point3, vec3, Matrix, Matrix4, Vector3, Vector4};
use render::{
    Buffer, PresentMode, Render, Shader, ShaderAttribute, ShaderAttributeType,
};
use std::{
    fs::File,
    io::Read,
    time::{Instant, SystemTime},
};
use util::srand;
use window::{input::Key, Window};

use crate::util::IntoArray;

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
#[derive(Clone, Copy, Debug)]
struct Uniforms {
    color: Vector4<f32>,
    mvp: Matrix4<f32>,
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

fn create_vertex_data(render: &Render) -> (Buffer, Buffer, usize) {
    let (models, _) =
        tobj::load_obj("./assets/untitled.obj", &tobj::LoadOptions::default())
            .expect("Couldn't open obj file");

    let mesh = &models[0].mesh;
    let mut verts = Vec::new();
    for x in 0..mesh.positions.len() / 3 {
        let ps = mesh
            .positions
            .iter()
            .skip(3 * x)
            .take(3)
            .collect::<IntoArray<&f32, 3>>()
            .array;
        let ps = ps.map(|x| *x);
        let colors = if mesh.vertex_color.len() > 0 {
            mesh.vertex_color
                .iter()
                .skip(3 * x)
                .take(3)
                .collect::<IntoArray<&f32, 3>>()
                .array
                .map(|x| *x)
        } else {
            [1.0, 1.0, 1.0]
        };
        let vertex = Vertex::new(ps, colors);
        verts.push(vertex);
    }

    let indices = mesh.indices.iter().map(|x| *x as u16).collect::<Vec<u16>>();

    (
        render.create_vertex_buffer(cast_slice(&verts)),
        render.create_index_buffer(cast_slice(&indices)),
        indices.len(),
    )
}

fn process_keys(window: &mut Window) {
    if window.getkey(Key::Escape) {
        window.close();
    }
}

async fn run(mut window: Window) {
    let mut render = Render::new(&window, PresentMode::Fifo).await;
    let mut dt_time = Instant::now();

    let (vertices, indices, n_indices) = create_vertex_data(&render);
    println!("n_indices: {}", n_indices);
    let shader_layout = render.create_shader_layout([
        ShaderAttribute::new(ShaderAttributeType::Vec3, 0),
        ShaderAttribute::new(ShaderAttributeType::Vec3, 1),
    ]);
    let shader = create_shader(&render);
    let bind_layout = render.create_bind_group_layout::<Uniforms>();
    let pipeline =
        render.create_pipeline(&shader_layout, &shader, &bind_layout);

    let projection =
        perspective(cgmath::Deg(67.5_f32), 640.0 / 480.0, 1.0, 400.0);
    let orientation = Matrix4::look_at_rh(
        point3(0.0_f32, 1.0, 10.0),
        point3(0.0_f32, 0.0, 0.0),
        Vector3::unit_y(),
    );
    let location = Matrix4::from_translation(vec3::<f32>(0.0, -1.0, -10.0));
    let mvp = projection * location;
    let uniform_data = Uniforms {
        color: Vector4::new(1.0_f32, 0.5, 0.5, 1.0),
        mvp,
    };
    let uniforms = render.create_uniforms(&uniform_data);
    let bind_group = render.create_bind_group(&bind_layout, &uniforms);

    let mut timer = Instant::now();
    let mut dt_avg = 0.0_f64;
    let mut n_times = 0;

    loop {
        let dt = dt_time.elapsed().as_nanos() as f64 / 1_000_000_000_f64;
        dt_time = Instant::now();

        if cfg!(debug_assertions) {
            n_times += 1;
            let n_times_f = n_times as f64;
            dt_avg =
                (dt_avg * (n_times_f - 1.0_f64) / n_times_f) + dt / n_times_f;
            if timer.elapsed().as_millis() >= 500 {
                timer = Instant::now();
                println!("{:.2} average fps", 1.0 / dt_avg);
                n_times = 0;
                dt_avg = 0.0;
            }
        }

        if window.should_close {
            break;
        }
        window.update();

        process_keys(&mut window);

        if window.did_resize {
            render.reconfigure(window.width, window.height, PresentMode::Fifo);
        }

        let framebuffer = render.get_presentation_framebuffer();

        render
            .start_commands()
            .configure_draw(&pipeline, &framebuffer)
            .set_vertices(&vertices)
            .set_indices(&indices)
            .bind_resources(&bind_group)
            .draw(0..n_indices as u32)
            .submit(&render.queue);

        render.present();
    }
}

fn main() {
    srand(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32,
    );
    let sdl = sdl2::init().expect("Failed to initialize SDL");
    let window = Window::new(&sdl, "Conc", 640, 480);

    pollster::block_on(run(window));
}
