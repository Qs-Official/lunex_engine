mod traits;
pub use traits::*;

// #=======================#
// #=== RE-EXPORT TYPES ===#

pub use lunex_engine::prelude::*;
pub use lunex_theme::prelude::*;
//pub use lunex_nodetree::prelude::*;
//pub use lunex_preset::prelude::*;
//pub use lunex_types::prelude::*;
//pub use lunex_typographic::prelude::*;


pub mod prelude {
    // Traits
    pub use super::{NodeComputeTrait, NodeGeneralTrait, NodeDisplayTrait};
    pub use super::{BuildAsNode, SyncToNode};

    // Master
    pub use super::{UINodeTree, UINode};

    pub use lunex_engine::layout;

}

// #=========================#
// #=== TRAIT DECLARATION ===#

use lunex_engine::{Rect3D, NiceDisplay};


pub type UINodeTree<P = ()> = NodeTree<InterfaceData, Container<P>>;
pub type UINode<P = ()> = Node<Container<P>>;


pub struct InterfaceData {
    pub themes: Theme,
}

#[derive(Debug, Default)]
pub struct Container<P> {
    pub data: Option<P>,
    pub rect: Rect3D,
    pub layout: Layout,
    //text: Option<TextCapsule>, // It modifies ContentSize though?

    //depth: f32,

    //roll: f32,
    //yaw: f32,
    //pitch: f32
}

impl <P:Default> Container<P> {
    pub fn new() -> Container<P> {
        Container::default()
    }
}

impl <P> NiceDisplay for Container<P> {
    fn to_nicestr(&self) -> String {
        self.layout.to_nicestr()
    }
}