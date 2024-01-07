use std::borrow::Borrow;

use crate::nodes::prelude::*;
use crate::layout;
use crate::Rect3D;

use super::{UINode, UINodeTree, Container};




// #============================#
// #=== DIRECT UINODE TRAITS ===#


/// ## UINode user data trait
/// Trait that abstracts over [`NodeDataTrait`] to provide tailored
/// implementations for [`UINodeTree`] data management.
pub trait UINodeDataTrait<T> {
    /// ## Add ui data
    /// Adds new data to this node and returns the previous data.
    /// ## ⚠️ Panics 🚧
    /// Panics if [`UINode`] is missing [`Container`] data _(should not happen unless you used methods not in prelude)_.
    fn add_ui_data(&mut self, data: T) -> Option<T>;

    /// ## Insert ui data
    /// Inserts new data to this node or any other subnode and returns the previous data.
    /// ## ⚠️ Panics 🚧
    /// Panics if [`UINode`] is missing [`Container`] data _(should not happen unless you used methods not in prelude)_.
    fn insert_ui_data(&mut self, path: impl Borrow<str>, data: T) -> Result<Option<T>, NodeTreeError>;

    /// ## Take ui data
    /// Removes data from this node and returns them.
    /// ## ⚠️ Panics 🚧
    /// Panics if [`UINode`] is missing [`Container`] data _(should not happen unless you used methods not in prelude)_.
    fn take_ui_data(&mut self) -> Option<T>;

    /// ## Remove ui data
    /// Removes data from this node or any other subnode and returns them.
    /// ## ⚠️ Panics 🚧
    /// Panics if [`UINode`] is missing [`Container`] data _(should not happen unless you used methods not in prelude)_.
    fn remove_ui_data(&mut self, path: impl Borrow<str>) -> Result<Option<T>, NodeTreeError>;

    /// ## Obtain ui data
    /// Borrows data from this node.
    /// ## ⚠️ Panics 🚧
    /// Panics if [`UINode`] is missing [`Container`] data _(should not happen unless you used methods not in prelude)_.
    fn obtain_ui_data(&self) -> Option<&T>;

    /// ## Obtain ui data mut
    /// Borrows data from this node as mut.
    /// ## ⚠️ Panics 🚧
    /// Panics if [`UINode`] is missing [`Container`] data _(should not happen unless you used methods not in prelude)_.
    fn obtain_ui_data_mut(&mut self) -> Option<&mut T>;

    /// ## Borrow ui data
    /// Borrows data from this node or any other subnode.
    /// ## ⚠️ Panics 🚧
    /// Panics if [`UINode`] is missing [`Container`] data _(should not happen unless you used methods not in prelude)_.
    fn borrow_ui_data(&self, path: impl Borrow<str>) -> Result<Option<&T>, NodeTreeError>;

    /// ## Borrow ui data mut
    /// Borrows data from this node or any other subnode as mut.
    /// ## ⚠️ Panics 🚧
    /// Panics if [`UINode`] is missing [`Container`] data _(should not happen unless you used methods not in prelude)_.
    fn borrow_ui_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T>, NodeTreeError>;
}
impl <T: Default> UINodeDataTrait<T> for UINodeTree<T> {
    fn add_ui_data(&mut self, data: T) -> Option<T> {
        self.node.add_ui_data(data)
    }

    fn insert_ui_data(&mut self, path: impl Borrow<str>, data: T) -> Result<Option<T>, NodeTreeError> {
        self.node.insert_ui_data(path, data)
    }

    fn take_ui_data(&mut self) -> Option<T> {
        self.node.take_ui_data()
    }

    fn remove_ui_data(&mut self, path: impl Borrow<str>) -> Result<Option<T>, NodeTreeError> {
        self.node.remove_ui_data(path)
    }

    fn obtain_ui_data(&self) -> Option<&T> {
        self.node.obtain_ui_data()
    }

    fn obtain_ui_data_mut(&mut self) -> Option<&mut T> {
        self.node.obtain_ui_data_mut()
    }

    fn borrow_ui_data(&self, path: impl Borrow<str>) -> Result<Option<&T>, NodeTreeError> {
        self.node.borrow_ui_data(path)
    }

    fn borrow_ui_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T>, NodeTreeError> {
        self.node.borrow_ui_data_mut(path)
    }
}
impl <T: Default> UINodeDataTrait<T> for UINode<T> {
    fn add_ui_data(&mut self, data: T) -> Option<T> {
        let Some(container) = self.obtain_data_mut() else { panic!("This UINode is missing UI data!") };
        core::mem::replace(&mut container.data, Some(data))
    }

    fn insert_ui_data(&mut self, path: impl Borrow<str>, data: T) -> Result<Option<T>, NodeTreeError> {
        let Some(container) = self.borrow_data_mut(path)? else { panic!("This UINode is missing UI data!") };
        Ok(core::mem::replace(&mut container.data, Some(data)))
    }

    fn take_ui_data(&mut self) -> Option<T> {
        let Some(container) = self.obtain_data_mut() else { panic!("This UINode is missing UI data!") };
        core::mem::replace(&mut container.data, None)
    }

    fn remove_ui_data(&mut self, path: impl Borrow<str>) -> Result<Option<T>, NodeTreeError> {
        let Some(container) = self.borrow_data_mut(path)? else { panic!("This UINode is missing UI data!") };
        Ok(core::mem::replace(&mut container.data, None))
    }

    fn obtain_ui_data(&self) -> Option<&T> {
        let Some(container) = self.obtain_data() else { panic!("This UINode is missing UI data!") };
        container.data.as_ref()
    }

    fn obtain_ui_data_mut(&mut self) -> Option<&mut T> {
        let Some(container) = self.obtain_data_mut() else { panic!("This UINode is missing UI data!") };
        container.data.as_mut()
    }

    fn borrow_ui_data(&self, path: impl Borrow<str>) -> Result<Option<&T>, NodeTreeError> {
        let Some(container) = self.borrow_data(path)? else { panic!("This UINode is missing UI data!") };
        Ok(container.data.as_ref())
    }

    fn borrow_ui_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T>, NodeTreeError> {
        let Some(container) = self.borrow_data_mut(path)? else { panic!("This UINode is missing UI data!") };
        Ok(container.data.as_mut())
    }
}


/// ## UINodetree init trait
/// Trait that abstracts over [`NodeTreeInitTrait`] to provide tailored
/// implementations for [`UINodeTree`] initialization.
pub trait UINodeTreeInitTrait {
    /// ## New
    /// Creates new NodeTree.
    fn new(name: impl Borrow<str>) -> Self;
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