use std::borrow::Borrow;

use bevy::ecs::component::Component;

use crate::nodes::prelude::*;
use crate::layout;
use crate::Layout;
use crate::Rect3D;
use crate::import::*;

use super::{UiNode, UiTree, NodeData};




// #============================#
// #=== DIRECT UINODE TRAITS ===#

/// ## UiNodetree init trait
/// Trait that abstracts over [`NodeTreeInitTrait`] to provide tailored
/// implementations for [`UiTree`] initialization.
pub trait UiNodeCreationTrait<N:Default + Component> {
    /// ## Make node
    /// Makes new subnode in this node and returns the new subnodes' name.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::create_node`] for hierarchy creation
    fn make_ui_node(&mut self, name: impl Borrow<str>) -> Result<String, NodeError>;

    /// ## Create node
    /// Creates new subnode in this node or any other subnode and returns the new subnodes' name.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::make_node`] for direct creation
    fn create_ui_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeError>;

    /// ## Obtain or create node
    /// Borrows subnode from this node. If the node doesn't exist, it creates one.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::borrow_or_create_node`] for hierarchy retrieval
    fn obtain_or_create_ui_node(&mut self, name: impl Borrow<str>) -> Result<&UiNode<N>, NodeError>;

    /// ## Obtain or create node mut
    /// Borrows subnode from this node as mut. If the node doesn't exist, it creates one.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::borrow_or_create_node_mut`] for hierarchy retrieval
    fn obtain_or_create_ui_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError>;

    /// ## Borrow or create node
    /// Borrows subnode from this node or any other subnode. If a node in path doesn't exist, it creates one.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::obtain_or_create_node`] for direct retrieval
    fn borrow_or_create_ui_node(&mut self, path: impl Borrow<str>) -> Result<&UiNode<N>, NodeError>;

    /// ## Borrow or create node mut
    /// Borrows subnode from this node or any other subnode as mut. If a node in path doesn't exist, it creates one.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::obtain_or_create_node_mut`] for direct retrieval
    fn borrow_or_create_ui_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError>;  
}
impl <M: Default + Component, N: Default + Component> UiNodeCreationTrait<N> for UiTree<M, N> {
    fn make_ui_node(&mut self, name: impl Borrow<str>) -> Result<String, NodeError>{
        self.node.make_ui_node(name)
    }

    fn create_ui_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeError>{
        self.node.create_ui_node(path)
    }

    fn obtain_or_create_ui_node(&mut self, name: impl Borrow<str>) -> Result<&UiNode<N>, NodeError> {
        self.node.obtain_or_create_ui_node(name)
    }

    fn obtain_or_create_ui_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError> {
        self.node.obtain_or_create_ui_node_mut(name)
    }

    fn borrow_or_create_ui_node(&mut self, path: impl Borrow<str>) -> Result<&UiNode<N>, NodeError> {
        self.node.borrow_or_create_ui_node(path)
    }

    fn borrow_or_create_ui_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError> {
        self.node.borrow_or_create_ui_node_mut(path)
    }
}
impl <N: Default + Component> UiNodeCreationTrait<N> for UiNode<N> {
    fn make_ui_node(&mut self, name: impl Borrow<str>) -> Result<String, NodeError> {
        let n = self.make_node(name)?;
        self.insert_data(n.clone(), NodeData::default())?;
        Ok(n)
    }

    fn create_ui_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeError> {
        let mut node: UiNode<N> = Node::new();
        node.add_data(NodeData::default());
        self.insert_node(path, Node::new())
    }

    fn obtain_or_create_ui_node(&mut self, name: impl Borrow<str>) -> Result<&UiNode<N>, NodeError> {
        let _ = self.make_ui_node(name.borrow());
        self.obtain_node(name)
    }

    fn obtain_or_create_ui_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError> {
        let _ = self.make_ui_node(name.borrow());
        self.obtain_node_mut(name)
    }

    fn borrow_or_create_ui_node(&mut self, path: impl Borrow<str>) -> Result<&UiNode<N>, NodeError> {
        match path.borrow().split_once('/') {
            None => self.obtain_or_create_ui_node(path),
            Some((name, rempath)) => self.obtain_or_create_ui_node_mut(name)?.borrow_or_create_ui_node(rempath),
        }
    }

    fn borrow_or_create_ui_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError> {
        match path.borrow().split_once('/') {
            None => self.obtain_or_create_ui_node_mut(path),
            Some((name, rempath)) => self.obtain_or_create_ui_node_mut(name)?.borrow_or_create_ui_node_mut(rempath),
        }
    }
}

/// ## UiNode data trait
/// Trait that abstracts over [`NodeDataTrait`] to provide tailored
/// implementations for [`UiTree`] data management.
pub trait UiNodeDataTrait<N> {
    /// ## Add ui data
    /// Adds new data to this node and returns the previous data.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::insert_ui_data`] for hierarchy insert
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn add_ui_data(&mut self, data: N) -> Option<N>;

    /// ## Insert ui data
    /// Inserts new data to this node or any other subnode and returns the previous data.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::add_ui_data`] for direct insert
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn insert_ui_data(&mut self, path: impl Borrow<str>, data: N) -> Result<Option<N>, NodeError>;

    /// ## Take ui data
    /// Removes data from this node and returns them.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::remove_ui_data`] for hierarchy retrieval
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn take_ui_data(&mut self) -> Option<N>;

    /// ## Remove ui data
    /// Removes data from this node or any other subnode and returns them.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::take_ui_data`] for direct retrieval
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn remove_ui_data(&mut self, path: impl Borrow<str>) -> Result<Option<N>, NodeError>;

    /// ## Obtain ui data
    /// Borrows data from this node.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::borrow_ui_data`] for hierarchy retrieval
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn obtain_ui_data(&self) -> Option<&N>;

    /// ## Obtain ui data mut
    /// Borrows data from this node as mut.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::borrow_ui_data_mut`] for hierarchy retrieval
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn obtain_ui_data_mut(&mut self) -> Option<&mut N>;

    /// ## Borrow ui data
    /// Borrows data from this node or any other subnode.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::obtain_ui_data`] for direct retrieval
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn borrow_ui_data(&self, path: impl Borrow<str>) -> Result<Option<&N>, NodeError>;

    /// ## Borrow ui data mut
    /// Borrows data from this node or any other subnode as mut.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::obtain_ui_data_mut`] for direct retrieval
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn borrow_ui_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut N>, NodeError>;
}
impl <M: Default + Component, N: Default + Component> UiNodeDataTrait<N> for UiTree<M, N> {
    fn add_ui_data(&mut self, data: N) -> Option<N> {
        self.node.add_ui_data(data)
    }

    fn insert_ui_data(&mut self, path: impl Borrow<str>, data: N) -> Result<Option<N>, NodeError> {
        self.node.insert_ui_data(path, data)
    }

    fn take_ui_data(&mut self) -> Option<N> {
        self.node.take_ui_data()
    }

    fn remove_ui_data(&mut self, path: impl Borrow<str>) -> Result<Option<N>, NodeError> {
        self.node.remove_ui_data(path)
    }

    fn obtain_ui_data(&self) -> Option<&N> {
        self.node.obtain_ui_data()
    }

    fn obtain_ui_data_mut(&mut self) -> Option<&mut N> {
        self.node.obtain_ui_data_mut()
    }

    fn borrow_ui_data(&self, path: impl Borrow<str>) -> Result<Option<&N>, NodeError> {
        self.node.borrow_ui_data(path)
    }

    fn borrow_ui_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut N>, NodeError> {
        self.node.borrow_ui_data_mut(path)
    }
}
impl <N: Default + Component> UiNodeDataTrait<N> for UiNode<N> {
    fn add_ui_data(&mut self, data: N) -> Option<N> {
        let Some(container) = self.obtain_data_mut() else { panic!("This UiNode is missing Ui data!") };
        core::mem::replace(&mut container.data, Some(data))
    }

    fn insert_ui_data(&mut self, path: impl Borrow<str>, data: N) -> Result<Option<N>, NodeError> {
        let Some(container) = self.borrow_data_mut(path)? else { panic!("This UiNode is missing Ui data!") };
        Ok(core::mem::replace(&mut container.data, Some(data)))
    }

    fn take_ui_data(&mut self) -> Option<N> {
        let Some(container) = self.obtain_data_mut() else { panic!("This UiNode is missing Ui data!") };
        core::mem::replace(&mut container.data, None)
    }

    fn remove_ui_data(&mut self, path: impl Borrow<str>) -> Result<Option<N>, NodeError> {
        let Some(container) = self.borrow_data_mut(path)? else { panic!("This UiNode is missing Ui data!") };
        Ok(core::mem::replace(&mut container.data, None))
    }

    fn obtain_ui_data(&self) -> Option<&N> {
        let Some(container) = self.obtain_data() else { panic!("This UiNode is missing Ui data!") };
        container.data.as_ref()
    }

    fn obtain_ui_data_mut(&mut self) -> Option<&mut N> {
        let Some(container) = self.obtain_data_mut() else { panic!("This UiNode is missing Ui data!") };
        container.data.as_mut()
    }

    fn borrow_ui_data(&self, path: impl Borrow<str>) -> Result<Option<&N>, NodeError> {
        let Some(container) = self.borrow_data(path)? else { panic!("This UiNode is missing Ui data!") };
        Ok(container.data.as_ref())
    }

    fn borrow_ui_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut N>, NodeError> {
        let Some(container) = self.borrow_data_mut(path)? else { panic!("This UiNode is missing Ui data!") };
        Ok(container.data.as_mut())
    }
}


/// ## UiNodetree init trait
/// Trait that abstracts over [`NodeTreeInitTrait`] to provide tailored
/// implementations for [`UiTree`] initialization.
pub trait UiNodeTreeInitTrait {
    /// ## New
    /// Creates new UiTree.
    fn new(name: impl Borrow<str>) -> Self;
}
impl <M: Default + Component, N: Default + Component> UiNodeTreeInitTrait for UiTree<M, N> {
    fn new(name: impl Borrow<str>) -> Self {
        let mut tree: UiTree<M, N> = NodeTreeInitTrait::new(name);
        tree.add_data(NodeData::default());
        tree
    }
}



/// ## Node compute trait
/// Trait with all node layout computation implementations.
pub trait UiNodeComputeTrait {
    fn compute(&mut self, parent: Rect3D);
}
impl <M: Default + Component, N: Default + Component> UiNodeComputeTrait for UiTree<M, N> {
    fn compute(&mut self, parent: Rect3D) {
        self.node.compute(parent);
    }
}
impl <N:Default + Component> UiNodeComputeTrait for UiNode<N> {
    fn compute(&mut self, parent: Rect3D) {

        let depth = self.get_depth();
        
        // Check here if computation is required for partial recalculation
        if let Some(node_data) = &mut self.data {

            let abs_scale = 0.5;
            let font_size = 16.0;

            // COUNTE VARIABLES

            match &node_data.layout {
                Layout::Window(l) => node_data.rect = l.compute(parent.into(), abs_scale, font_size).into(),
                Layout::Solid(l) => node_data.rect = l.compute(parent.into(), abs_scale, font_size).into(),
                _ => {},
            }

            node_data.rect.pos.z = depth;


            let mut content_size = Vec2::ZERO;



            let mut leftover_margin = Vec2::ZERO;

            for (_, node) in &mut self.nodes {
                if let Some(child_node_data) = &mut node.data {

                    match &child_node_data.layout {
                        Layout::Div(l) => {
                            let (rect, margin) = l.compute(node_data.rect.into(), abs_scale, font_size);

                            child_node_data.rect = rect.into();

                            child_node_data.rect.pos.x += leftover_margin.x;

                            leftover_margin.x += margin.x + child_node_data.rect.size.x;
                        },
                        _ => {},
                    }

                }
            }




            for (_, node) in &mut self.nodes {
                node.compute(node_data.rect);
            }
        }
    }
}


// #========================================#
// #=== FUNCTIONALITY WITH UINODE TRAITS ===#


/// ## Build as node
/// Trait that [Layout] types implement so they can be build as new node.
pub trait BuildAsNode {
    fn build<M: Default + Component, N: Default + Component>(self, ui: &mut UiTree<M, N>, path: impl Borrow<str>) -> Result<String, NodeError> where Self: Sized;
}
impl BuildAsNode for layout::Window {
    fn build<M: Default + Component, N: Default + Component>(self, ui: &mut UiTree<M, N>, path: impl Borrow<str>) -> Result<String, NodeError> where Self: Sized {
        ui.create_node(path.borrow())?;
        let mut container: NodeData<N> = NodeData::new();
        container.layout = self.into();
        ui.insert_data(path, container)?;
        Ok(String::new())
    }
}
impl BuildAsNode for layout::Solid {
    fn build<M: Default + Component, N: Default + Component>(self, ui: &mut UiTree<M, N>, path: impl Borrow<str>) -> Result<String, NodeError> where Self: Sized {
        ui.create_node(path.borrow())?;
        let mut container: NodeData<N> = NodeData::new();
        container.layout = self.into();
        ui.insert_data(path, container)?;
        Ok(String::new())
    }
}


/// ## Sync to node
/// Trait that [Component] types which represent values in [UiTree] need to
/// implement to load and store data in [UiTree].
pub trait SyncToNode {
    fn load<M: Default + Component, N: Default + Component>(self, ui: &mut UiTree<M, N>, path: impl Borrow<str>);
    fn save<M: Default + Component, N: Default + Component>(self, ui: &mut UiTree<M, N>, path: impl Borrow<str>);
}





pub trait Extract <T> {
    fn get_extract (&self) -> T;
    fn set_extract (&mut self, val: T) -> T;
}