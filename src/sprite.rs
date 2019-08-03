pub(crate) type Path = &'static str;

#[derive(Clone,Debug)]
pub struct Sprite {
    pub(crate) path: Path,
    pub(crate) frames: [u32; 2],
    pub(crate) position: [i32; 2],
    pub(crate) frame_size: [u32; 2],
    pub(crate) current_frame: u32,
}

impl Sprite {
    pub fn get_path(&self) -> Path {
        self.path
    }

    pub fn get_frames(&self) -> [u32; 2] {
        self.frames
    }

    pub fn get_position(&self) -> [i32; 2] {
        self.position
    }

    pub fn get_frame_size(&self) -> [u32; 2] {
        self.frame_size
    }

    pub fn get_current_frame(&self) -> u32 {
        self.current_frame
    }

    pub fn set_current_frame(&mut self, current_frame: u32) -> Self {
        self.current_frame = current_frame;

        if current_frame >= self.frames[0] {
            self.current_frame = 0;
        }

        self.clone()
    }
}

pub struct SpriteBuilder{}

impl SpriteBuilder {
    pub fn build(path: Path, frames: [u32; 2], position: [i32; 2], frame_size: [u32; 2]) -> Sprite {
        Sprite{
            path: path,
            frames: frames,
            position: position,
            frame_size: frame_size,
            current_frame: 0,
        }
    }
}
