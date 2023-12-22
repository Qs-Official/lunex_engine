use std::fmt::Display;

pub use lunex_core::prelude::*;
pub use lunex_layout::prelude::*;
pub use lunex_nodemap::prelude::*;
pub use lunex_preset::prelude::*;
pub use lunex_theme::prelude::*;
pub use lunex_typographic::prelude::*;

pub mod prelude {
    pub use super::{NodeTrait, NodeTraitPrint};
    pub use super::Interface;
}


pub type Interface = NodeMap<(), f32>;


pub struct InterfaceData {
    themes: Theme,
}

#[derive(Debug, Default)]
pub struct Container {
    //layout: Layout,
    //text: Option<TextCapsule>, // It modifies ContentSize though?

    depth: f32,

    roll: f32,
    yaw: f32,
    pitch: f32
}

impl Container {
    pub fn new() -> Container {
        Container::default()
    }
}

/*impl Display for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.roll, self.yaw)
    }
}*/