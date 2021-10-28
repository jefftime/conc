mod render;
mod window;

use render::{PresentMode, Render};
use window::{input::Key, Window};

async fn run(mut window: Window) {
    let mut render = Render::new(&window, PresentMode::Fifo).await;

    loop {
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
