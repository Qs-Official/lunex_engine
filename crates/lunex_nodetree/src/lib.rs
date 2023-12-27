mod error;
pub use error::*;

mod nodetree;
pub use nodetree::*;

mod traits;
pub use traits::*;

// #======================#
// #=== PRELUDE EXPORT ===#

pub mod prelude {
    pub use super::NodeTreeError;
    pub use super::{NodeTree, Node, NodeGeneralTrait, NodeDataTrait, NodeDisplayTrait};
}
