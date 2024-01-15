mod structs;
pub use structs::*;

mod traits;
pub use traits::*;

pub mod prelude {
    pub use super::{UiNode, UiTree};
    pub use super::{UINodeCreationTrait, UINodeDataTrait, UINodeTreeInitTrait, UINodeComputeTrait};
    pub use super::{BuildAsNode, SyncToNode};
}