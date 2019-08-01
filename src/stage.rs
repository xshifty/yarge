pub type StageCallback = &'static dyn Fn(&mut crate::workspace::Workspace, &mut Vec<sdl2::event::Event>);
type Name = &'static str;

#[derive(Clone)]
pub struct Stage {
    name: &'static str,
    callback: StageCallback,
}

pub struct StageBuilder{}

impl StageBuilder {
    pub fn build(name: Name, callback: StageCallback) -> Stage {
        Stage {
            name: name,
            callback: callback,
        }
    }
}

impl Stage {
    pub fn get_name(&mut self) -> Name {
        self.name
    }

    pub fn get_callback(&mut self) -> StageCallback {
        self.callback
    }
}
