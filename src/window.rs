use input::Key;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use sdl2::{
    event::{Event, WindowEvent},
    video::Window as SdlWindow,
    EventPump, Sdl,
};

pub mod input;

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
    pub fn new(sdl: &Sdl, title: &str, width: i32, height: i32) -> Window {
        let sdl_video = sdl
            .video()
            .expect("Failed to initialize SDL2 video subsystem");
        let event_pump = sdl.event_pump().expect("Failed to obtain event pump");

        let window = sdl_video
            .window(title, width as u32, height as u32)
            .opengl()
            .resizable()
            .build()
            .expect("Failed to create SDL window");

        Window {
            window,
            event_pump,
            should_close: false,
            did_resize: false,
            width,
            height,
        }
    }

    pub fn update(&mut self) {
        self.did_resize = false;

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.should_close = true,

                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(width, height) => {
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

    pub fn getkey(&self, key: Key) -> bool {
        if let Some(sdl_key) = key.to_sdl2() {
            self.event_pump
                .keyboard_state()
                .is_scancode_pressed(sdl_key)
        } else {
            false
        }
    }
}
