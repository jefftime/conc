extern crate bincode;
extern crate glfw;
extern crate serde;
extern crate serde_json;

mod window;

use window::Window;

fn main() {
    let mut window = Window::new("Conc", 640, 480).unwrap();

    'main: loop {
        if window.should_close {
            break 'main;
        }

        window.update();
    }
}
