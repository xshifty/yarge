pub(crate) type Path = &'static str;

#[derive(Clone)]
pub struct Sprite {
    path: Path,
    frames: [u32; 2],
    position: [i32; 2],
    frame_size: [u32; 2],
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
}

pub struct SpriteBuilder{}

impl SpriteBuilder {
    pub fn build(path: Path, frames: [u32; 2], position: [i32; 2], frame_size: [u32; 2]) -> Sprite {
        Sprite{
            path: path,
            frames: frames,
            position: position,
            frame_size: frame_size,
        }
    }
}
