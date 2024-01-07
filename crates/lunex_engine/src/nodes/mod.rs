mod structs;
pub use structs::*;

mod traits;
pub use traits::*;

pub mod prelude {
    pub use super::{NodeGeneralTrait, NodeDataTrait, NodeTopDataTrait, NodeInitTrait, NodeTreeInitTrait, NodeDisplayTrait};
    pub use super::{Node, NodeTree};
    pub use super::NodeTreeError;
}