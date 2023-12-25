
use lunex_layout::prelude::*;
use std::borrow::Borrow;

use lunex_nodemap::prelude::*;

use crate::{UINode, UINodeMap, Container};



/// ## Node compute trait
/// Trait with all node layout computation implementations.
pub trait NodeComputeTrait {
    fn compute(&mut self);
}

/// ## Node user data trait
/// Trait that abstracts over [NodeDataTrait] to provide tailored
/// implementations for [UINodeMap] user data management.
pub trait NodeUserDataTrait {

}

/// ## Build as node
/// Trait that [Layout] types implement so they can be build as new node.
pub trait BuildAsNode {
    fn build<P>(self, ui: &mut UINodeMap<P>, path: impl Borrow<str>) -> Result<String, NodeMapError> where Self: Sized;
}

/// ## Sync to node
/// Trait that [Component] types which represent values in [UINodeMap] need to
/// implement to load and store data in [UINodeMap].
pub trait SyncToNode {
    fn load<P>(self, ui: &mut UINodeMap<P>, path: impl Borrow<str>);
    fn save<P>(self, ui: &mut UINodeMap<P>, path: impl Borrow<str>);
}





impl <P> NodeComputeTrait for UINodeMap<P> {
    fn compute(&mut self) {
        
    }
}


impl BuildAsNode for declarative::Window {
    fn build<P>(self, ui: &mut UINodeMap<P>, path: impl Borrow<str>) -> Result<String, NodeMapError> where Self: Sized {
        ui.create_node(path)?;
        //ui.insert_data(path, data)?;
        Ok(String::new())
    }
}

