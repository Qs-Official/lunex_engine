use std::borrow::Borrow;
use super::{Node, NodeTreeError};

// #=========================#
// #=== TRAIT DECLARATION ===#

/// ## Node general trait
/// Trait with all node management implementations.
pub trait NodeGeneralTrait<T> {
    /// ## Add node
    /// Adds new subnode to this node and returns the new subnodes' name.
    fn add_node(&mut self, name: impl Borrow<str>, node: impl Into<Node<T>>) -> Result<String, NodeTreeError>;

    /// ## Insert node
    /// Inserts new subnode to this node or any other subnode and returns the new subnodes' name.
    fn insert_node(&mut self, path: impl Borrow<str>, node: impl Into<Node<T>>) -> Result<String, NodeTreeError>;

    /// ## Take node
    /// Removes subnode from this node and returns it.
    fn take_node(&mut self, name: impl Borrow<str>) -> Result<Node<T>, NodeTreeError>;

    /// ## Remove node
    /// Removes subnode from this node or any other subnode and returns it.
    fn remove_node(&mut self, path: impl Borrow<str>) -> Result<Node<T>, NodeTreeError>;

    /// ## Obtain node
    /// Borrows subnode from this node.
    /// ### 📌 Note
    /// * Use [`NodeGeneralTrait::borrow_node`] for hierarchy retrieval
    fn obtain_node(&self, name: impl Borrow<str>) -> Result<&Node<T>, NodeTreeError>;

    /// ## Obtain node mut
    /// Borrows subnode from this node as mut.
    /// ### 📌 Note
    /// * Use [`NodeGeneralTrait::borrow_node_mut`] for hierarchy retrieval
    fn obtain_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut Node<T>, NodeTreeError>;

    /// ## Borrow node
    /// Borrows subnode from this node or any other subnode.
    /// ### 📌 Note
    /// * Use [`NodeGeneralTrait::obtain_node`] for direct retrieval
    fn borrow_node(&self, path: impl Borrow<str>) -> Result<&Node<T>, NodeTreeError>;

    /// ## Borrow node mut
    /// Borrows subnode from this node or any other subnode as mut.
    /// ### 📌 Note
    /// * Use [`NodeGeneralTrait::obtain_node_mut`] for direct retrieval
    fn borrow_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut Node<T>, NodeTreeError>;

    /// ## Merge
    /// Merges subnodes of supplied node or nodetree into this node.
    /// ### ⚠️ Warning
    /// * Any data that supplied node contains will be dropped.
    /// * Returns error if there is a name collision.
    fn merge(&mut self, node: impl Into<Node<T>>) -> Result<(), NodeTreeError>;

    /// ## Crawl
    /// Recursively iterates over all subnodes and returns them in a single vector.
    fn crawl(&self) -> Vec<&Node<T>>;

    /// ## Tree node
    /// Generates overview of the inner structure of subnodes as a printable string.
    /// 
    /// You can supply additional parameters like `show-hidden`.
    /// ### 📌 Note
    /// * Prefer [`NodeDisplayTrait::tree`] method instad if (`T`) implements [`crate::NiceDisplay`]
    fn tree_node(&self, params: impl Borrow<str>) -> String;

    /// ## Get name
    /// Returns name of the node. `Cached` & `Read-only`.
    /// ### ⚠️ Warning
    /// * Not guaranteed to be correct if node is not put inside the hierarchy correctly.
    fn get_name(&self) -> &String;

    /// ## Get path
    /// Returns depth within the hierarchy. `Cached` & `Read-only`.
    /// ### ⚠️ Warning
    /// * Not guaranteed to be correct if node is not put inside the hierarchy correctly.
    fn get_path(&self) -> &String;

    /// ## Get depth
    /// Returns full path without the name. `Cached` & `Read-only`.
    /// ### ⚠️ Warning
    /// * Not guaranteed to be correct if node is not put inside the hierarchy correctly.
    fn get_depth(&self) -> f32;
}

/// ## Node creation trait
/// Trait with all node creation implementations.
pub trait NodeCreationTrait<T> {
    /// ## Make node
    /// Makes new subnode in this node and returns the new subnodes' name.
    fn make_node(&mut self, name: impl Borrow<str>) -> Result<String, NodeTreeError>;

    /// ## Create node
    /// Creates new subnode in this node or any other subnode and returns the new subnodes' name.
    fn create_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeTreeError>;

    /// ## Obtain or create node
    /// Borrows subnode from this node. If the node doesn't exist, it creates one.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::borrow_or_create_node`] for hierarchy retrieval
    fn obtain_or_create_node(&mut self, name: impl Borrow<str>) -> Result<&Node<T>, NodeTreeError>;

    /// ## Obtain or create node mut
    /// Borrows subnode from this node as mut. If the node doesn't exist, it creates one.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::borrow_or_create_node_mut`] for hierarchy retrieval
    fn obtain_or_create_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut Node<T>, NodeTreeError>;

    /// ## Borrow or create node
    /// Borrows subnode from this node or any other subnode. If a node in path doesn't exist, it creates one.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::obtain_or_create_node`] for direct retrieval
    fn borrow_or_create_node(&mut self, path: impl Borrow<str>) -> Result<&Node<T>, NodeTreeError>;

    /// ## Borrow or create node mut
    /// Borrows subnode from this node or any other subnode as mut. If a node in path doesn't exist, it creates one.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::obtain_or_create_node_mut`] for direct retrieval
    fn borrow_or_create_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut Node<T>, NodeTreeError>;    
}

/// ## Node data trait
/// Trait with all node data management implementations.
/// Provides mainly raw access methods. Lunex abstacts over
/// this trait with another trait.
pub trait NodeDataTrait<T> {
    /// ## Add data
    /// Adds new data to this node and returns the previous data.
    fn add_data(&mut self, data: T) -> Option<T>;

    /// ## Insert data
    /// Inserts new data to this node or any other subnode and returns the previous data.
    fn insert_data(&mut self, path: impl Borrow<str>, data: T) -> Result<Option<T>, NodeTreeError>;

    /// ## Take data
    /// Removes data from this node and returns them.
    fn take_data(&mut self) -> Option<T>;

    /// ## Remove data
    /// Removes data from this node or any other subnode and returns them.
    fn remove_data(&mut self, path: impl Borrow<str>) -> Result<Option<T>, NodeTreeError>;

    /// ## Obtain data
    /// Borrows data from this node.
    fn obtain_data(&self) -> Option<&T>;

    /// ## Obtain data mut
    /// Borrows data from this node as mut.
    fn obtain_data_mut(&mut self) -> Option<&mut T>;

    /// ## Borrow data
    /// Borrows data from this node or any other subnode.
    fn borrow_data(&self, path: impl Borrow<str>) -> Result<Option<&T>, NodeTreeError>;

    /// ## Borrow data mut
    /// Borrows data from this node or any other subnode as mut.
    fn borrow_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T>, NodeTreeError>;
}

/// ## Node top data trait
/// Trait with all nodetree top-data management implementations.
pub trait NodeTopDataTrait<D> {
    /// ## Add top-level data
    /// Adds new top-level data and returns previous top-level data.
    fn add_topdata(&mut self, data: D) -> Option<D>;

    /// ## Take top-level data
    /// Removes top-level data and returns it.
    fn take_topdata(&mut self) -> Option<D>;

    /// ## Obtain top-level data
    /// Borrows top-level data.
    fn obtain_topdata(&self) -> Option<&D>;

    /// ## Obtain top-level data mut
    /// Borrows top-level data as mut.
    fn obtain_topdata_mut(&mut self) -> Option<&mut D>;
}

/// ## Node init trait
/// Trait with all init methods for empty nodes. Lunex abstacts over
/// this trait with another trait.
pub trait NodeInitTrait {
    /// ## New
    /// Creates new node.
    fn new() -> Self;
}

/// ## Nodetree init trait
/// Trait with init methods for nodetrees. Lunex abstacts over
/// this trait with another trait.
pub trait NodeTreeInitTrait {
    /// ## New
    /// Creates new NodeTree.
    fn new(name: impl Borrow<str>) -> Self;
}

/// ## Node display trait
/// Trait with all node display implementations.
pub trait NodeDisplayTrait<T> {
    /// ## Tree
    /// Generates overview of the inner structure as a string.
    fn tree(&self, params: impl Borrow<str>) -> String;
}