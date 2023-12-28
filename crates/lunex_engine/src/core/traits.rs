use std::borrow::Borrow;

use crate::nodes::prelude::*;
use crate::layout;
use crate::Rect3D;

use super::{UINode, UINodeTree, Container};


/// ## Node user data trait
/// Trait that abstracts over [NodeDataTrait] to provide tailored
/// implementations for [UINodeTree] user data management.
pub trait NodeUserDataTrait {

}


/// ## Sync to node
/// Trait that [Component] types which represent values in [UINodeTree] need to
/// implement to load and store data in [UINodeTree].
pub trait SyncToNode {
    fn load<P>(self, ui: &mut UINodeTree<P>, path: impl Borrow<str>);
    fn save<P>(self, ui: &mut UINodeTree<P>, path: impl Borrow<str>);
}





/// ## Node compute trait
/// Trait with all node layout computation implementations.
pub trait NodeComputeTrait {
    fn compute(&mut self, parent: Rect3D);
}

impl <P> NodeComputeTrait for UINodeTree<P> {
    fn compute(&mut self, parent: Rect3D) {
        self.node.compute(parent);
    }
}

impl <P> NodeComputeTrait for UINode<P> {
    fn compute(&mut self, parent: Rect3D) {
        
        if let Some(container) = &mut self.data {
            println!("HI");
            container.rect = container.layout.compute(parent, 16.0);

            for (_, node) in &mut self.nodes {
                node.compute(container.rect);
            }
        }
    }
}





/// ## Build as node
/// Trait that [Layout] types implement so they can be build as new node.
pub trait BuildAsNode {
    fn build<P:Default>(self, ui: &mut UINodeTree<P>, path: impl Borrow<str>) -> Result<String, NodeTreeError> where Self: Sized;
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

