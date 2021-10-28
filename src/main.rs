mod render;
mod window;

use render::{PresentMode, Render};
use window::Window;

async fn run(mut window: Window) {
    let mut render = Render::new(&window, PresentMode::Fifo).await;

    loop {
        window.update();
        if window.did_resize {
            render
                .reconfigure(window.width, window.height, PresentMode::Fifo)
                .await;
        }

        render.draw();

        if window.should_close {
            break;
        }
    }
}

fn main() {
    let sdl = sdl2::init().expect("Failed to initialize SDL");
    let window = Window::new(sdl, "Conc", 640, 480);

    pollster::block_on(run(window));
}
