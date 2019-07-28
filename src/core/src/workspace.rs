extern crate sdl2;

use std::collections::HashMap;
use std::fmt;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    render::Canvas,
    video::{FullscreenType, Window},
    EventPump,
    pixels::Color,
};

#[macro_export]
macro_rules! rgba {
    ($rgba:expr) => {
        Color::RGBA(
            (($rgba as u64 >> 24) & 0xff) as u8,
            (($rgba as u64 >> 16) & 0xff) as u8,
            (($rgba as u64 >> 8) & 0xff) as u8,
            (($rgba as u64 >> 0) & 0xff) as u8,
        )
    };
}

const DEFAULT_STAGE_NAME: &str = "default";

pub type EventList = Vec<Event>;

pub struct Workspace {
    pub(crate) title: crate::Title,
    pub(crate) width: crate::Width,
    pub(crate) height: crate::Height,
    pub(crate) current_stage: &'static str,
    pub(crate) stages: HashMap<String, crate::Stage>,
    pub(crate) event_pump: EventPump,
    pub(crate) canvas: Canvas<Window>,
}

impl fmt::Debug for Workspace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{{ current_stage: {}, title: {}, width: {}, height: {} }}", self.current_stage, self.title, self.width, self.height)
    }
}

impl Workspace {
    pub fn get_title(&mut self) -> crate::Title {
        self.title
    }

    pub fn get_width(&mut self) -> crate::Width {
        self.width
    }

    pub fn get_height(&mut self) -> crate::Height {
        self.height
    }

    pub fn get_current_stage_name(&mut self) -> crate::StageName {
        self.current_stage
    }

    pub fn switch_stage(&mut self, stage_name: crate::StageName) -> &mut Self {
        if !self.stages.contains_key(stage_name) {
            panic!("There is no stage named {}", stage_name);
        }

        self.current_stage = stage_name;

        self
    }

    pub fn add_stage(&mut self, stage: crate::Stage) -> &mut Self {
        let stage_name = stage.clone().get_name();

        self.stages
            .insert(String::from(stage_name).to_string(), stage);

        self
    }

    pub fn bootstrap(&mut self) {
        if !self.stages.contains_key(self.current_stage) {
            panic!("There is no stage named {}", self.current_stage);
        }

        let events: &mut EventList = &mut vec![];

        'game_loop: loop {
            self.canvas.set_draw_color(rgba!(0x306090ff));
            self.canvas.clear();
            events.clear();

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'game_loop,
                    Event::KeyDown {
                        keycode: Some(Keycode::F11),
                        ..
                    } => match self.canvas.window().fullscreen_state() {
                        FullscreenType::True | FullscreenType::Desktop => {
                            self.canvas
                                .window_mut()
                                .set_fullscreen(FullscreenType::Off)
                                .unwrap();
                        }
                        FullscreenType::Off => {
                            self.canvas
                                .window_mut()
                                .set_fullscreen(FullscreenType::True)
                                .unwrap();
                        }
                    },
                    _ => {
                        events.insert(events.len(), event);
                    }
                }
            }

            let mut current_stage = self.stages.get(self.current_stage).unwrap().clone();
            let stage_callback = current_stage.get_callback();

            stage_callback(self);

            self.canvas.present();

            ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

pub struct WorkspaceBuilder {}

impl WorkspaceBuilder {
    pub fn build(
        title: crate::Title,
        width: crate::Width,
        height: crate::Height,
    ) -> Workspace {
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();
        let window = video.window(title, width, height).opengl().build().unwrap();
        let canvas = window.into_canvas().accelerated().build().unwrap();
        let event_pump = context.event_pump().unwrap();

        return Workspace {
            title: title,
            width: width,
            height: height,
            current_stage: DEFAULT_STAGE_NAME,
            stages: HashMap::new(),
            event_pump: event_pump,
            canvas: canvas,
        };
    }
}
