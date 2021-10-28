use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use sdl2::{event::Event, video::Window as SdlWindow, EventPump, Sdl};

mod input;

pub struct Window {
    pub window: SdlWindow,
    pub event_pump: EventPump,
    pub should_close: bool,
    pub did_resize: bool,
    pub width: i32,
    pub height: i32,
}

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.window.raw_window_handle()
    }
}

impl Window {
    pub fn new(sdl: Sdl, title: &str, width: u32, height: u32) -> Window {
        let sdl_video = sdl
            .video()
            .expect("Failed to initialize SDL2 video subsystem");
        let event_pump = sdl.event_pump().expect("Failed to obtain event pump");

        let window = sdl_video
            .window(title, width, height)
            .opengl()
            .resizable()
            .build()
            .expect("Failed to create SDL window");

        Window {
            window: window,
            event_pump: event_pump,
            should_close: false,
            did_resize: false,
            width: width as i32,
            height: height as i32,
        }
    }

    pub fn update(&mut self) {
        self.did_resize = false;

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.should_close = true,
                Event::Window { win_event, .. } => match win_event {
                    sdl2::event::WindowEvent::Resized(width, height) => {
                        self.width = width;
                        self.height = height;
                        self.did_resize = true;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
