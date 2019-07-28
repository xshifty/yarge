pub type StageName = &'static str;
pub type StageCallback = &'static Fn(&mut crate::Workspace);

#[derive(Clone)]
pub struct Stage {
    name: StageName,
    callback: StageCallback,
}

impl Stage {
    pub fn new(name: StageName, callback: StageCallback) -> Stage {
        Stage {
            name: name,
            callback: callback,
        }
    }

    pub fn get_name(&mut self) -> StageName {
        self.name
    }

    pub fn get_callback(&mut self) -> StageCallback {
        self.callback
    }
}
