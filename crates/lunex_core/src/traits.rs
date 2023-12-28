use std::borrow::Borrow;

use lunex_engine::nodes::prelude::*;
use lunex_engine::layout;

use crate::{UINode, UINodeTree, Container};



/// ## Node compute trait
/// Trait with all node layout computation implementations.
pub trait NodeComputeTrait {
    fn compute(&mut self);
}

/// ## Node user data trait
/// Trait that abstracts over [NodeDataTrait] to provide tailored
/// implementations for [UINodeTree] user data management.
pub trait NodeUserDataTrait {

}

/// ## Build as node
/// Trait that [Layout] types implement so they can be build as new node.
pub trait BuildAsNode {
    fn build<P:Default>(self, ui: &mut UINodeTree<P>, path: impl Borrow<str>) -> Result<String, NodeTreeError> where Self: Sized;
}

/// ## Sync to node
/// Trait that [Component] types which represent values in [UINodeTree] need to
/// implement to load and store data in [UINodeTree].
pub trait SyncToNode {
    fn load<P>(self, ui: &mut UINodeTree<P>, path: impl Borrow<str>);
    fn save<P>(self, ui: &mut UINodeTree<P>, path: impl Borrow<str>);
}





impl <P> NodeComputeTrait for UINodeTree<P> {
    fn compute(&mut self) {
        
    }
}


impl BuildAsNode for layout::Window {
    fn build<P:Default>(self, ui: &mut UINodeTree<P>, path: impl Borrow<str>) -> Result<String, NodeTreeError> where Self: Sized {
        ui.create_node(path.borrow())?;
        let mut container: Container<P> = Container::new();
        container.layout = self.into();
        ui.insert_data(path, container)?;
        Ok(String::new())
    }
}

