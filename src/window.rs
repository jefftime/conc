use glfw::{Context, Glfw};

pub struct Window {
    pub glfw: Glfw,
    pub should_close: bool,
    window: glfw::Window,
    event_receiver: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    width: u32,
    height: u32,
}

impl Window {
    pub fn new(
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<Window, glfw::InitError> {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.set_key_polling(true);
        window.make_current();

        Ok(Window {
            glfw: glfw,
            window: window,
            event_receiver: events,
            width: width,
            height: height,
            should_close: false,
        })
    }

    pub fn update(&mut self) {
        self.glfw.poll_events();
        let receiver = &self.event_receiver;
        for (_, event) in glfw::flush_messages(&receiver) {
            self.should_close = handle_window_event(event);
        }
    }
}

fn handle_window_event(event: glfw::WindowEvent) -> bool {
    match event {
        glfw::WindowEvent::Key(
            glfw::Key::Escape,
            _,
            glfw::Action::Press,
            _,
        ) => true,
        _ => false,
    }
}
