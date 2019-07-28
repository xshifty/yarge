pub type Title = &'static str;
pub type Width = u32;
pub type Height = u32;

pub use self::{workspace::*, stage::{Stage, StageName, StageCallback}};

pub mod workspace;
pub mod stage;
