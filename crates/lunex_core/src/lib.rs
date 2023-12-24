// #=======================#
// #=== RE-EXPORT TYPES ===#

pub use lunex_layout::prelude::*;
pub use lunex_nodemap::prelude::*;
pub use lunex_preset::prelude::*;
pub use lunex_theme::prelude::*;
pub use lunex_types::prelude::*;
pub use lunex_typographic::prelude::*;


pub mod prelude {
    pub use super::{NodeTrait, NodeTraitPrint};
    pub use super::Interface;
}

// #=========================#
// #=== TRAIT DECLARATION ===#

use lunex_layout::Layout;




pub type Interface = NodeMap<InterfaceData, Container>;


pub struct InterfaceData {
    pub themes: Theme,
}

#[derive(Debug, Default)]
pub struct Container {
    rect: Rect3D,
    layout: Layout,
    //text: Option<TextCapsule>, // It modifies ContentSize though?

    //depth: f32,

    //roll: f32,
    //yaw: f32,
    //pitch: f32
}

impl Container {
    pub fn new() -> Container {
        Container::default()
    }
}

impl NiceDisplay for Container {
    fn to_nicestr(&self) -> String {
        self.layout.to_nicestr()
    }
}