pub(crate) type Path = &'static str;

#[derive(Clone)]
pub struct Sprite {
    path: Path,
    position: [i32; 2],
    dimension: [u32; 2],
    frame_dimension: [u32; 2],
}

impl Sprite {
    pub fn get_path(&self) -> Path {
        self.path
    }

    pub fn get_position(&self) -> [i32; 2] {
        self.position
    }

    pub fn get_dimension(&self) -> [u32; 2] {
        self.dimension
    }

    pub fn get_frame_dimension(&self) -> [u32; 2] {
        self.frame_dimension
    }
}

pub struct SpriteBuilder{}

impl SpriteBuilder {
    pub fn build(path: Path, position: [i32; 2], dimension: [u32; 2], frame_dimension: [u32; 2]) -> Sprite {
        Sprite{
            path: path,
            position: position,
            dimension: dimension,
            frame_dimension: frame_dimension,
        }
    }
}
