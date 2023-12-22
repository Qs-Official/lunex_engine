use indexmap::IndexMap as HashMap;
use colored::Colorize;
use std::{borrow::Borrow, fmt::Display};
use bevy::ecs::component::Component;

mod error;
pub use error::NodeMapError;

pub mod prelude {
    pub use super::NodeMapError;
    pub use super::{NodeMap, Node, NodeTrait, NodeTraitPrint};
}

// #=========================#
// #=== TRAIT DECLARATION ===#

/// ## Node trait
/// Trait with all node management implementations.
pub trait NodeTrait<T> {
    /// ## Add node
    /// Adds new subnode to this node and returns new subnodes' name.
    fn add_node(&mut self, name: impl Borrow<str>, node: Node<T>) -> Result<String, NodeMapError>;

    /// ## Insert node
    /// Inserts new subnode to this node or any other subnode and returns new subnodes' name.
    fn insert_node(&mut self, path: impl Borrow<str>, node: Node<T>,) -> Result<String, NodeMapError>;

    /// ## Create node
    /// Creates new subnode in this node or any other subnode and returns new subnodes' name.
    fn create_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeMapError>;

    /// ## Take node
    /// Removes subnode from this node and returns it.
    fn take_node(&mut self, name: impl Borrow<str>) -> Result<Node<T>, NodeMapError>;

    /// ## Remove node
    /// Removes subnode from this node or any other subnode and returns it.
    fn remove_node(&mut self, path: impl Borrow<str>) -> Result<Node<T>, NodeMapError>;

    /// ## Obtain node
    /// Borrows subnode from this node.
    fn obtain_node(&self, name: impl Borrow<str>) -> Result<&Node<T>, NodeMapError>;

    /// ## Obtain node mut
    /// Borrows subnode from this node as mut.
    fn obtain_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut Node<T>, NodeMapError>;

    /// ## Borrow node
    /// Borrows subnode from this node or any other subnode.
    fn borrow_node(&self, path: impl Borrow<str>) -> Result<&Node<T>, NodeMapError>;

    /// ## Borrow node mut
    /// Borrows subnode from this node or any other subnode as mut.
    fn borrow_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut Node<T>, NodeMapError>;

    /// ## Merge
    /// Merges node or nodemap into this node.
    fn merge(&mut self, node: impl Into<Node<T>>) -> Result<(), NodeMapError>;

    /// ## Crawl
    /// Recursively iterates over all subnodes and returns them in a single vector.
    fn crawl(&self) -> Vec<&Node<T>>;

    /// ## Tree node
    /// Generates overview of the inner structure of subnodes as a string.
    fn tree_node(&self, params: impl Borrow<str>) -> String;

    /// ## Get name
    /// Returns name of the node. `Cached` & `Read-only`. Not guaranteed to be correct if node is not put inside nodemap correctly.
    fn get_name(&self) -> &String;

    /// ## Get path
    /// Returns depth within the hierarchy. `Cached` & `Read-only`. Not guaranteed to be correct if node is not put inside nodemap correctly.
    fn get_path(&self) -> &String;

    /// ## Get depth
    /// Returns full path without the name. `Cached` & `Read-only`. Not guaranteed to be correct if node is not put inside nodemap correctly.
    fn get_depth(&self) -> f32;

    /// ## Add data
    /// Adds new data to this node and returns previous data.
    fn add_data(&mut self, data: T) -> Option<T>;

    /// ## Insert data
    /// Inserts new data to this node or any other subnode and returns previous data.
    fn insert_data(&mut self, path: impl Borrow<str>, data: T) -> Result<Option<T>, NodeMapError>;

    /// ## Take data
    /// Removes data from this node and returns it.
    fn take_data(&mut self) -> Option<T>;

    /// ## Remove data
    /// Removes data from this node or any other subnode and returns it.
    fn remove_data(&mut self, path: impl Borrow<str>) -> Result<Option<T>, NodeMapError>;

    /// ## Obtain data
    /// Borrows data from this node.
    fn obtain_data(&self) -> Option<&T>;

    /// ## Obtain data mut
    /// Borrows data from this node as mut.
    fn obtain_data_mut(&mut self) -> Option<&mut T>;

    /// ## Borrow data
    /// Borrows data from this node or any other subnode
    fn borrow_data(&self, path: impl Borrow<str>) -> Result<Option<&T>, NodeMapError>;

    /// ## Borrow data mut
    /// Borrows data from this node or any other subnode as mut
    fn borrow_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T>, NodeMapError>;
}

/// ## Node print
/// Trait with all node print implementations.
pub trait NodeTraitPrint<T> {
    /// ## Tree
    /// Generates overview of the inner structure as a string.
    fn tree(&self, params: impl Borrow<str>) -> String;
}

// #===============================#
// #=== NODEMAP IMPLEMENTATIONS ===#

/// ## NodeMap
/// A hashmap-like data structure for organizing general data into recursive subnodes.
/// Data is indexed and traversed using `paths`.
/// ### Tree
/// ```text
/// > NODEMAP
///  |-> Node_1
///  |    |-> Node_2
///  |    |-> Node_3
///  |    |    |-> Node_4
///  |-> Node_5
///  |    |-> Node_6
/// ```
/// If you want to access `Node_4`, use path `Node_1/Node_3/Node_4` on `NODEMAP`.
/// Or you can use `Node_3/Node_4` on `Node_1` struct to get the same result.
/// ### Paths
/// Whitespaces are allowed in paths, but are not encouraged.
/// Putting a dot as first symbol like this `.name` will hide the node from the tree. If you want to
/// display hidden nodes too, pass `show-hidden` as params to [NodeTrait::tree] method.
/// Just `.` will refer to the same node. `..` is not supported for the sake of simplicity
/// and performance.
/// 
/// You can also not specify the name when creating a node. That will mean the name will be
/// generated. The format is as follows `.||#:N` with `N` being the `.len()` of the `nodes`.
/// Meaning nodes with names like `.||#:0`, `.||#:1`, `.||#:2` can exist. Please refrain from
/// manually using these names or [NodeTrait::add_node] will return errors.
/// ### Generics
/// * (D) => A type holding surface data that is stored in [NodeMap] for all nodes to share.
/// * (T) => A type holding node-specific data that any [Node] can store.
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct NodeMap<D, T> {
    /// ## Top-level data
    /// This top-level data is meant to be shared for every node. Example usage is storing `theme` and other surface data.
    pub data: Option<D>,

    /// ## Node
    /// The starting root node.
    pub node: Node<T>,
}
impl <D, T> NodeMap<D, T> {
    /// ## New
    /// Creates new nodemap.
    pub fn new(name: impl Borrow<str>) -> Self {
        let mut node = Node::new();
        node.name = name.borrow().into();
        node.path = "".into();
        NodeMap { data: None, node }
    }
    
    /// ## Add top-level data
    /// Adds new top-level data and returns previous top-level data.
    pub fn add_topdata(&mut self, data: D) -> Option<D> {
        core::mem::replace(&mut self.data, Some(data))
    }

    /// ## Take top-level data
    /// Removes top-level data and returns it.
    pub fn take_topdata(&mut self) -> Option<D> {
        core::mem::replace(&mut self.data, None)
    }

    /// ## Obtain top-level data
    /// Borrows top-level data.
    pub fn obtain_topdata(&self) -> Option<&D> {
        match &self.data {
            Some(value) => Some(value),
            None => None,
        }
    }

    /// ## Obtain top-level data mut
    /// Borrows top-level data as mut.
    pub fn obtain_topdata_mut(&mut self) -> Option<&mut D> {
        match &mut self.data {
            Some(value) => Some(value),
            None => None,
        }
    }
}
impl <D, T> NodeTrait<T> for NodeMap<D, T> {
    fn add_node(&mut self, name: impl Borrow<str>, node: Node<T>,) -> Result<String, NodeMapError>{
        self.node.add_node(name, node)
    }

    fn insert_node(&mut self, path: impl Borrow<str>, node: Node<T>,) -> Result<String, NodeMapError>{
        self.node.insert_node(path, node)
    }

    fn create_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeMapError>{
        self.node.create_node(path)
    }

    fn take_node(&mut self, name: impl Borrow<str>) -> Result<Node<T>, NodeMapError> {
        self.node.take_node(name)
    }

    fn remove_node(&mut self, path: impl Borrow<str>) -> Result<Node<T>, NodeMapError> {
        self.node.remove_node(path)
    }

    fn obtain_node(&self, name: impl Borrow<str>) -> Result<&Node<T>, NodeMapError> {
        self.node.obtain_node(name)
    }

    fn obtain_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut Node<T>, NodeMapError> {
        self.node.obtain_node_mut(name)
    }
  
    fn borrow_node(&self, path: impl Borrow<str>) -> Result<&Node<T>, NodeMapError> {
        self.node.borrow_node(path)
    }

    fn borrow_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut Node<T>, NodeMapError> {
        self.node.borrow_node_mut(path)
    }

    fn merge(&mut self, node: impl Into<Node<T>>) -> Result<(), NodeMapError> {
        self.node.merge(node.into())
    }

    fn crawl(&self) -> Vec<&Node<T>> {
        self.node.crawl()
    }

    fn tree_node(&self, params: impl Borrow<str>) -> String {
        self.node.tree_node(params)
    }

    fn get_name(&self) -> &String {
        &self.node.get_name()
    }

    fn get_path(&self) -> &String {
        &self.node.get_path()
    }

    fn get_depth(&self) -> f32 {
        self.node.get_depth()
    }

    fn add_data(&mut self, data: T) -> Option<T> {
        self.node.add_data(data)
    }

    fn insert_data(&mut self, path: impl Borrow<str>, data: T) -> Result<Option<T>, NodeMapError> {
        self.node.insert_data(path, data)
    }

    fn take_data(&mut self) -> Option<T> {
        self.node.take_data()
    }

    fn remove_data(&mut self, path: impl Borrow<str>) -> Result<Option<T>, NodeMapError> {
        self.node.remove_data(path)
    }

    fn obtain_data(&self) -> Option<&T> {
        self.node.obtain_data()
    }
    
    fn obtain_data_mut(&mut self) -> Option<&mut T> {
        self.node.obtain_data_mut()
    }

    fn borrow_data(&self, path: impl Borrow<str>) -> Result<Option<&T>, NodeMapError> {
        self.node.borrow_data(path)
    }
    
    fn borrow_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T>, NodeMapError> {
        self.node.borrow_data_mut(path)
    }
}
impl <D, T: Display> NodeTraitPrint<T> for NodeMap<D, T> {
    fn tree(&self, params: impl Borrow<str>) -> String {
        self.node.tree(params)
    }
}
impl <D, T> Into<Node<T>> for NodeMap<D, T>{
    fn into(self) -> Node<T> {
        self.node
    }
}


// #============================#
// #=== NODE IMPLEMENTATIONS ===#

/// ## Node
/// A struct representing organized data in [NodeMap].
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct Node<T> {
    /// ## Name
    /// Name of the node. `Cached` & `Read-only`.
    name: String,
    /// ## Path
    /// Full path without the name. `Cached` & `Read-only`.
    path: String,
    /// ## Depth
    /// Depth within the hierarchy. `Cached` & `Read-only`.
    depth: f32,

    /// ## Data
    /// Optional data this node can have. Example usage is storing `node layout` and other specific data.
    pub data: Option<T>,
    /// ## Nodes
    /// All subnodes this node contains. Treat is as `Read-only` unless you know what you are doing.
    /// Use the struct methods to manipulate the values inside.
    pub nodes: HashMap<String, Node<T>>,
}
impl <T> Node<T> {
    /// ## New
    /// Creates new node.
    pub fn new() -> Self {
        Node {
            name: "".into(),
            path: "".into(),
            depth: 0.0,

            data: None,
            nodes: HashMap::new(),
        }
    }
}
impl <T> Node<T> {
    /// Generate overview of the inner tree and write the mapped output to the given string with data formatted to a certain level depth
    pub(crate) fn cascade_tree(&self, mut string: String, level: u32, param: &str) -> String {
        for (name, node) in &self.nodes {
            if !param.contains("show-hidden") && name.starts_with('.') {continue;}
            let mut text = String::from("\n  ");
            for _ in 0..level { text += "|    " }
            text += "|-> ";
            string = format!("{}{}{}", string, text.black(), name.bold().yellow());
            string = node.cascade_tree(string, level + 1, param);
        }
        string
    }
}
impl <T:Display> Node<T> {
    /// Generate overview of the inner tree and write the mapped output to the given string with data formatted to a certain level depth
    pub(crate) fn cascade_tree_display(&self, mut string: String, level: u32, param: &str) -> String {
        if !param.contains("no-data") {
            if let Some(data) = &self.data {
                println!("THE FUCK");
                let text = String::from(" |= ");
                string = format!("{}{}{}", string, text.black(), data.to_string().bold().bright_cyan());
            }
        }
        for (name, node) in &self.nodes {
            if !param.contains("show-hidden") && name.starts_with('.') {continue;}
            let mut text = String::from("\n  ");
            for _ in 0..level { text += "|    " }
            text += "|-> ";
            string = format!("{}{}{}", string, text.black(), name.bold().yellow());
            string = node.cascade_tree(string, level + 1, param);
        }
        string
    }
}
impl <T> NodeTrait<T> for Node<T> {
    fn add_node(&mut self, name: impl Borrow<str>, mut node: Node<T>) -> Result<String, NodeMapError>{
        if !name.borrow().is_empty() {
            if name.borrow() == "." { return Err(NodeMapError::NameInUse("The special symbol '.' is used to refer to 'self' and is not available for use".to_owned())) }
            if self.nodes.contains_key(name.borrow()) == false {
                node.name = name.borrow().to_owned();
                node.path = if self.path.is_empty() { name.borrow().to_owned() } else { self.path.to_owned() + "/" + name.borrow() };
                node.depth = self.depth + 1.0;
                self.nodes.insert(name.borrow().to_owned(), node);
                Ok(name.borrow().to_owned())
            } else {
                Err(NodeMapError::NameInUse(name.borrow().to_owned()))
            }
        } else {
            let mut generated_name = format!(".||#:{}", self.nodes.len());
            let mut i = 0;
            while self.nodes.contains_key(&generated_name) == true {
                generated_name = format!(".||#:{}", self.nodes.len()+i);
                i += 1;
                if i > 100 { return Err(NodeMapError::InvalidPath("Failed to generate name, max threshold reached!".to_owned())); }
            }
            node.name = generated_name.to_owned();
            node.path = if self.path.is_empty() { generated_name.to_owned() } else { self.path.to_owned() + "/" + &generated_name };
            node.depth = self.depth + 1.0;
            self.nodes.insert(generated_name.to_owned(), node);
            Ok(generated_name)
        }
    }

    fn insert_node(&mut self, path: impl Borrow<str>, node: Node<T>) -> Result<String, NodeMapError>{
        match path.borrow().rsplit_once('/'){
            None => self.add_node(path, node),
            Some((rempath, name)) => self.borrow_node_mut(rempath)?.add_node(name, node),
        }
    }

    fn create_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeMapError>{
        self.insert_node(path, Node::new())
    }

    fn take_node(&mut self, name: impl Borrow<str>) -> Result<Node<T>, NodeMapError> {
        match self.nodes.remove(name.borrow()) {
            Some(node) => Ok(node),
            None => Err(NodeMapError::NoNode(name.borrow().to_owned())),
        }
    }

    fn remove_node(&mut self, path: impl Borrow<str>) -> Result<Node<T>, NodeMapError> {
        match path.borrow().rsplit_once('/') {
            None => self.take_node(path),
            Some((rempath, name)) => self.borrow_node_mut(rempath)?.remove_node(name),
        }
    }

    fn obtain_node(&self, name: impl Borrow<str>) -> Result<&Node<T>, NodeMapError> {
        if !name.borrow().is_empty() {
            if name.borrow() == "." { return Ok(self) }
            match self.nodes.get(name.borrow()) {
                Some(node) => Ok(node),
                None => Err(NodeMapError::NoNode(name.borrow().into())),
            }
        } else {
            Err(NodeMapError::InvalidPath(name.borrow().into()))
        }
    }

    fn obtain_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut Node<T>, NodeMapError> {
        if !name.borrow().is_empty() {
            if name.borrow() == "." { return Ok(self) }
            match self.nodes.get_mut(name.borrow()) {
                Some(node) => Ok(node),
                None => Err(NodeMapError::NoNode(name.borrow().into())),
            }
        } else {
            Err(NodeMapError::InvalidPath(name.borrow().into()))
        }
    }
  
    fn borrow_node(&self, path: impl Borrow<str>) -> Result<&Node<T>, NodeMapError> {
        match path.borrow().split_once('/') {
            None => self.obtain_node(path),
            Some((name, rempath)) => self.obtain_node(name)?.borrow_node(rempath),
        }
    }

    fn borrow_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut Node<T>, NodeMapError> {
        match path.borrow().split_once('/') {
            None => self.obtain_node_mut(path),
            Some((name, rempath)) => self.obtain_node_mut(name)?.borrow_node_mut(rempath),
        }
    }

    fn merge(&mut self, node: impl Into<Node<T>>) -> Result<(), NodeMapError> {
        let node = node.into();

        if let Some(_) = node.data {
            return Err(NodeMapError::DataConflict);
        }

        for (name, _) in &node.nodes {
            if self.nodes.contains_key(name) {return Err(NodeMapError::DuplicateName(name.to_owned()));}
        }

        for (name, dir) in node.nodes {
            self.insert_node(name, dir)?;
        }

        Ok(())
    }

    fn crawl(&self) -> Vec<&Node<T>> {
        let mut vector = Vec::new();
        for pair in &self.nodes{
            vector.push(pair.1);
            let mut content = pair.1.crawl();
            vector.append(&mut content);
        }
        vector
    }

    fn tree_node(&self, params: impl Borrow<str>) -> String {
        let text = String::new();
        format!(
            "{} {}{}",
            ">".black(),
            self.name.purple().bold().underline(),
            self.cascade_tree(text, 0, params.borrow())
        )
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_path(&self) -> &String {
        &self.path
    }

    fn get_depth(&self) -> f32 {
        self.depth
    }

    fn add_data(&mut self, data: T) -> Option<T> {
        core::mem::replace(&mut self.data, Some(data))
    }

    fn insert_data(&mut self, path: impl Borrow<str>, data: T) -> Result<Option<T>, NodeMapError>{
        Ok(self.borrow_node_mut(path)?.add_data(data))
    }

    fn take_data(&mut self) -> Option<T> {
        core::mem::replace(&mut self.data, None)
    }

    fn remove_data(&mut self, path: impl Borrow<str>) -> Result<Option<T>, NodeMapError> {
        Ok(self.borrow_node_mut(path)?.take_data())
    }

    fn obtain_data(&self) -> Option<&T> {
        match &self.data {
            Some(value) => Some(value),
            None => None,
        }
    }
    
    fn obtain_data_mut(&mut self) -> Option<&mut T> {
        match &mut self.data {
            Some(value) => Some(value),
            None => None,
        }
    }

    fn borrow_data(&self, path: impl Borrow<str>) -> Result<Option<&T> , NodeMapError> {
        Ok(self.borrow_node(path)?.obtain_data())
    }
    
    fn borrow_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T> , NodeMapError> {
        Ok(self.borrow_node_mut(path)?.obtain_data_mut())
    }
}
impl <T:Display> NodeTraitPrint<T> for Node<T> {
    fn tree(&self, params: impl Borrow<str>) -> String {
        let text = String::new();
        format!(
            "{} {}{}",
            ">".black(),
            self.name.purple().bold().underline(),
            self.cascade_tree_display(text, 0, params.borrow())
        )
    }
}
