extern crate sdl2;

use std::collections::HashMap;
use std::fmt;
use sdl2::{
    rect::Rect,
    event::Event,
    keyboard::Keycode,
    render::{Canvas, TextureCreator, Texture},
    image::{LoadTexture, InitFlag},
    video::{FullscreenType, Window, WindowContext},
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
    pub(crate) title: &'static str,
    pub(crate) dimension: [u32; 2],
    pub(crate) current_stage: &'static str,
    pub(crate) stages: HashMap<String, crate::stage::Stage>,
    pub(crate) event_pump: EventPump,
    pub(crate) canvas: Canvas<Window>,
    pub(crate) texture_creator: TextureCreator<WindowContext>,
    pub(crate) sprites: HashMap<String, crate::sprite::Sprite>,
    pub(crate) cache: HashMap<crate::sprite::Path, Texture>,
}

impl fmt::Debug for Workspace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{{ current_stage: {}, title: {}, width: {}, height: {} }}", self.current_stage, self.title, self.dimension[0], self.dimension[1])
    }
}

impl Workspace {
    pub fn get_title(&self) -> &'static str {
        self.title
    }

    pub fn get_dimension(&self) -> [u32; 2] {
        self.dimension
    }

    pub fn get_current_stage_name(&self) -> &'static str {
        self.current_stage
    }

    pub fn get_texture_creator(&mut self) -> &mut TextureCreator<WindowContext> {
        &mut self.texture_creator
    }

    pub fn switch_stage(&mut self, stage_name: &'static str) -> &mut Self {
        if !self.stages.contains_key(stage_name) {
            panic!("There is no stage named {}", stage_name);
        }

        self.current_stage = stage_name;

        self
    }

    pub fn add_stage(&mut self, stage: crate::stage::Stage) -> &mut Self {
        let stage_name = stage.clone().get_name();

        self.stages
            .insert(String::from(stage_name).to_string(), stage);

        self
    }

    pub fn add_sprite(&mut self, name: &'static str, sprite: crate::sprite::Sprite) -> &mut Self {
        self.sprites
            .insert(String::from(name).to_string(), sprite);
        
        self
    }

    pub fn draw_sprite(&mut self, sprite_name: &'static str, position: [i32; 2], dimension: [u32; 2]) -> &mut Self {
        let sprite = self.sprites.get(&String::from(sprite_name).to_string());

        match sprite {
            Some(_) => {
                let mut sprite = sprite.unwrap().clone();

                if !self.cache.contains_key(sprite.path) {
                    println!("Caching {}...", sprite.path);
                    let texture = self.texture_creator.load_texture(sprite.path);
                    self.cache.insert(sprite.path, texture.unwrap());
                }

                let frame_w = sprite.frame_size[0];
                let frame_h = sprite.frame_size[1];
                let current_x = (((sprite.current_frame % sprite.frames[0]) * frame_w) as i32) + sprite.position[0];
                let current_y = frame_h as i32;

                self.canvas.copy(
                    self.cache.get(sprite.path).unwrap(),
                    Rect::new(current_x, current_y, frame_w, frame_h),
                    Rect::new(position[0], position[1], dimension[0], dimension[1])
                ).unwrap();

                sprite.set_current_frame(sprite.get_current_frame() + 1);

                // println!("{} {} = {:?}", current_x, current_y, sprite);

                self.sprites.insert(String::from(sprite_name).to_string(), sprite);
            },
            None => {
                panic!("Sprite {} not registered.", sprite_name)
            }
        }

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

            stage_callback(self, events);

            self.canvas.present();

            ::std::thread::sleep(std::time::Duration::from_millis(1000/12));
        }
    }
}

pub struct WorkspaceBuilder {}

impl WorkspaceBuilder {
    pub fn build(
        title: &'static str,
        dimension: [u32; 2],
    ) -> Workspace {
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();
        let window = video.window(title, dimension[0], dimension[1]).opengl().build().unwrap();
        let canvas = window.into_canvas().accelerated().build().unwrap();
        let texture_creator = canvas.texture_creator();
        let event_pump = context.event_pump().unwrap();

        sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

        Workspace {
            title: title,
            dimension: dimension,
            current_stage: DEFAULT_STAGE_NAME,
            stages: HashMap::new(),
            event_pump: event_pump,
            canvas: canvas,
            texture_creator: texture_creator,
            sprites: HashMap::new(),
            cache: HashMap::new(),
        }
    }
}
