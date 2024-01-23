mod structs;
pub use structs::*;

mod traits;
pub use traits::*;

pub mod prelude {
    pub use super::{UiNode, UiTree};
    pub use super::{InterfaceData, Container, NoData};

    pub use super::{UiNodeCreationTrait, UiNodeDataTrait, UiNodeTreeInitTrait, UiNodeComputeTrait};
    pub use super::{BuildAsNode, SyncToNode};

    //RE-EXPORT FROM NODES                          // NEEDS ABSTRACTION
    pub use crate::nodes::prelude::{NodeGeneralTrait, NodeTopDataTrait, NodeDisplayTrait};
}