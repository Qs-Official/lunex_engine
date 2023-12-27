mod structs;
pub use structs::*;

mod traits;
pub use traits::*;

pub mod prelude {
    pub use super::{NodeGeneralTrait, NodeDataTrait, NodeDisplayTrait};
    pub use super::{Node, NodeTree};
    pub use super::NodeTreeError;
}