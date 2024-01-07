use std::borrow::Borrow;

use crate::nodes::prelude::*;
use crate::layout;
use crate::Rect3D;

use super::{UINode, UINodeTree, Container};




// #============================#
// #=== DIRECT UINODE TRAITS ===#


/// ## Node user data trait
/// Trait that abstracts over [`NodeDataTrait`] to provide tailored
/// implementations for [`UINodeTree`] data management.
pub trait UINodeDataTrait<T> {
    /// ## Add custom data
    /// Adds new data to this node and returns the previous data.
    /// ## Panics
    /// Panics if somebody wrongly initialized/changed UINode with direct access methods (not in prelude)
    fn add_custom_data(&mut self, data: T) -> Option<T>;

    /// ## Insert custom data
    /// Inserts new data to this node or any other subnode and returns the previous data.
    /// ## Panics
    /// Panics if somebody wrongly initialized/changed UINode with direct access methods (not in prelude)
    fn insert_custom_data(&mut self, path: impl Borrow<str>, data: T) -> Result<Option<T>, NodeTreeError>;

    /// ## Take custom data
    /// Removes data from this node and returns them.
    /// ## Panics
    /// Panics if somebody wrongly initialized/changed UINode with direct access methods (not in prelude)
    fn take_custom_data(&mut self) -> Option<T>;

    /// ## Remove custom data
    /// Removes data from this node or any other subnode and returns them.
    /// ## Panics
    /// Panics if somebody wrongly initialized/changed UINode with direct access methods (not in prelude)
    fn remove_custom_data(&mut self, path: impl Borrow<str>) -> Result<Option<T>, NodeTreeError>;

    /// ## Obtain custom data
    /// Borrows data from this node.
    /// ## Panics
    /// Panics if somebody wrongly initialized/changed UINode with direct access methods (not in prelude)
    fn obtain_custom_data(&self) -> Option<&T>;

    /// ## Obtain custom data mut
    /// Borrows data from this node as mut.
    /// ## Panics
    /// Panics if somebody wrongly initialized/changed UINode with direct access methods (not in prelude)
    fn obtain_custom_data_mut(&mut self) -> Option<&mut T>;

    /// ## Borrow custom data
    /// Borrows data from this node or any other subnode.
    /// ## Panics
    /// Panics if somebody wrongly initialized/changed UINode with direct access methods (not in prelude)
    fn borrow_custom_data(&self, path: impl Borrow<str>) -> Result<Option<&T>, NodeTreeError>;

    /// ## Borrow custom data mut
    /// Borrows data from this node or any other subnode as mut.
    /// ## Panics
    /// Panics if somebody wrongly initialized/changed UINode with direct access methods (not in prelude)
    fn borrow_custom_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T>, NodeTreeError>;
}
impl <T: Default> UINodeDataTrait<T> for UINodeTree<T> {
    fn add_custom_data(&mut self, data: T) -> Option<T> {
        let Some(container) = self.obtain_data_mut() else { panic!("This UINode was wrongly initialized!") };
        core::mem::replace(&mut container.data, Some(data))
    }

    fn insert_custom_data(&mut self, path: impl Borrow<str>, data: T) -> Result<Option<T>, NodeTreeError> {
        let Some(container) = self.borrow_data_mut(path)? else { panic!("This UINode was wrongly initialized!") };
        Ok(core::mem::replace(&mut container.data, Some(data)))
    }

    fn take_custom_data(&mut self) -> Option<T>;

    fn remove_custom_data(&mut self, path: impl Borrow<str>) -> Result<Option<T>, NodeTreeError>;

    fn obtain_custom_data(&self) -> Option<&T>;

    fn obtain_custom_data_mut(&mut self) -> Option<&mut T>;

    fn borrow_custom_data(&self, path: impl Borrow<str>) -> Result<Option<&T>, NodeTreeError>;

    fn borrow_custom_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T>, NodeTreeError>;
}

/// ## Node compute trait
/// Trait with all node layout computation implementations.
pub trait UINodeComputeTrait {
    fn compute(&mut self, parent: Rect3D);
}
impl <P> UINodeComputeTrait for UINodeTree<P> {
    fn compute(&mut self, parent: Rect3D) {
        self.node.compute(parent);
    }
}
impl <P> UINodeComputeTrait for UINode<P> {
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


// #========================================#
// #=== FUNCTIONALITY WITH UINODE TRAITS ===#


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


/// ## Sync to node
/// Trait that [Component] types which represent values in [UINodeTree] need to
/// implement to load and store data in [UINodeTree].
pub trait SyncToNode {
    fn load<P>(self, ui: &mut UINodeTree<P>, path: impl Borrow<str>);
    fn save<P>(self, ui: &mut UINodeTree<P>, path: impl Borrow<str>);
}