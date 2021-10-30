mod math;
mod render;
mod window;

use render::{PresentMode, Render};
use std::time::Instant;
use window::{input::Key, Window};

async fn run(mut window: Window) {
    let mut render = Render::new(&window, PresentMode::Fifo).await;
    let mut dt_time = Instant::now();

    loop {
        let dt = dt_time.elapsed().as_nanos();
        println!("fps: {}", 1_000_000_000f64 / (dt as f64));
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

        render.draw();
    }
}

fn main() {
    let sdl = sdl2::init().expect("Failed to initialize SDL");
    let window = Window::new(sdl, "Conc", 640, 480);

    pollster::block_on(run(window));
}
