use indexmap::IndexMap as HashMap;
use colored::Colorize;
use std::borrow::Borrow;
use bevy::ecs::component::Component;

mod error;
use error::NodeMapError;

// #===============================#
// #=== GENERIC IMPLEMENTATIONS ===#




pub trait DirHierarchy<D> {
    /// Adds subnode directly to this node, returns new subdirectories' name
    fn add_node(&mut self, name: impl Borrow<str>, node: D) -> Result<String, NodeMapError>;

    /// Inserts subnode to self or any subnode, returns inserted subdirectories' name
    fn insert_node(&mut self, path: impl Borrow<str>, node: D,) -> Result<String, NodeMapError>;

    /// Creates subnode in root or any subnode, returns new subdirectories' name
    fn create_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeMapError>;

    /// Removes node from self and returns it
    fn take_node(&mut self, name: impl Borrow<str>) -> Result<D, NodeMapError>;

    /// Removes node from self or any subnode and returns it
    fn remove_node(&mut self, path: impl Borrow<str>) -> Result<D, NodeMapError>;

    /// Borrow node from self
    fn obtain_node(&self, name: impl Borrow<str>) -> Result<&D, NodeMapError>;

    /// Borrow node from self
    fn obtain_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut D, NodeMapError>;
  
    /// Borrow node from self or any subnode
    fn borrow_node(&self, path: impl Borrow<str>) -> Result<&D, NodeMapError>;

    /// Borrow node from self or any subnode
    fn borrow_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut D, NodeMapError>;

    /// Merges DirMap or Dir content into itself
    fn merge(&mut self, node: impl Into<D>) -> Result<(), NodeMapError>;

    /// Recursively iterate over all containing directories and their subdirectories and return them in one vector
    fn crawl(&self) -> Vec<&D>;

    /// Generate overview of the inner tree in a stringified form
    fn tree(&self) -> String;

    /// Generate overview of the directories inside the inner tree in a stringified form
    fn tree_dir(&self) -> String;

    /// Returns cached name
    fn get_name(&self) -> &String;

    /// Returns cached depth
    fn get_depth(&self) -> f32;

    /// Returns cached name
    fn get_path(&self) -> &String;
}
pub trait DirFile<T> {
    /// Adds data directly to this node and return existing one
    fn add_data(&mut self, data: T) -> Option<T>;

    /// Inserts data to self or any subnode and return existing one
    fn insert_data(&mut self, path: impl Borrow<str>, data: T) -> Result<Option<T>, NodeMapError>;

    /// Removes data from self and returns it
    fn take_data(&mut self) -> Option<T>;

    /// Removes data from self or any subnode and returns it
    fn remove_data(&mut self, path: impl Borrow<str>) -> Result<Option<T>, NodeMapError>;

    /// Borrow data from self
    fn obtain_data(&self) -> Option<&T>;
    
    /// Borrow data from self
    fn obtain_data_mut(&mut self) -> Option<&mut T>;

    /// Borrow data from self or any subnode
    fn borrow_file(&self, path: impl Borrow<str>) -> Result<Option<&T>, NodeMapError>;
    
    /// Borrow data from self or any subnode
    fn borrow_file_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T>, NodeMapError>;
}





// #===============================#
// #=== DIRMAP IMPLEMENTATIONS ===#


#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct NodeMap<T> {
    /// ## Node
    /// All subnodes this node contains. Treat is as `Read-only` unless you know what you are doing.
    /// Use the struct methods to manipulate the values inside.
    pub node: Node<T>,
}
impl <T> NodeMap<T> {
    /// # New
    /// Creates new NodeMap
    pub fn new(name: impl Borrow<str>) -> Self {
        let mut node = Node::new();
        node.name = name.borrow().into();
        node.path = "".into();
        NodeMap { node }
    }
}

impl <T> DirHierarchy<Node<T>> for NodeMap<T> {
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

    fn tree(&self) -> String {
        self.node.tree()
    }

    fn tree_dir(&self) -> String {
        self.node.tree_dir()
    }

    fn get_name(&self) -> &String {
        &self.node.get_name()
    }

    fn get_depth(&self) -> f32 {
        self.node.get_depth()
    }

    fn get_path(&self) -> &String {
        &self.node.get_path()
    }
}
impl <T> DirFile<T> for NodeMap<T> {
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

    fn borrow_file(&self, path: impl Borrow<str>) -> Result<Option<&T>, NodeMapError> {
        self.node.borrow_file(path)
    }
    
    fn borrow_file_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T>, NodeMapError> {
        self.node.borrow_file_mut(path)
    }
}
impl <T> Into<Node<T>> for NodeMap<T>{
    fn into(self) -> Node<T> {
        self.node
    }
}


// #===========================#
// #=== DIR IMPLEMENTATIONS ===#


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
    /// Optional data this node can have.
    pub data: Option<T>,
    /// ## Nodes
    /// All subnodes this node contains. Treat is as `Read-only` unless you know what you are doing.
    /// Use the struct methods to manipulate the values inside.
    pub nodes: HashMap<String, Node<T>>,
}
impl <T> Node<T> {
    pub fn new() -> Self {
        Node {
            name: "UNASSIGNED DIRECTORY".to_owned(),
            path: "EMPTY PATH".to_owned(),
            depth: 0.0,

            data: None,
            nodes: HashMap::new(),
        }
    }
}
impl <T> Node<T> {
    /// Generate overview of the inner tree and write the mapped output to the given string with data formatted to a certain level depth
    pub(crate) fn cascade_tree(&self, mut string: String, level: u32, param: &str) -> String {
        if !param.contains("no-dir") {
            if let Some(_) = self.data {
                let mut text = String::from("\n  ");
                for _ in 0..level { text += "|    " }
                text += "|-> ";
                string = format!("{}{}{}", string, text.black(), "FILE".bold().bright_cyan());
            }
        }
        for (name, node) in &self.nodes {
            if name.starts_with('.') {continue;}
            let mut text = String::from("\n  ");
            for _ in 0..level { text += "|    " }
            text += "|-> ";
            string = format!("{}{}{}", string, text.black(), name.bold().yellow());
            string = node.cascade_tree(string, level + 1, param);
        }
        string
    }
}
impl <T> DirHierarchy<Node<T>> for Node<T> {
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
            Some ((directory_path, name)) => match self.borrow_node_mut(directory_path) {
                Ok(borrowed_directory) => borrowed_directory.add_node(name, node),
                Err(e) => Err(e),
            }
        }
    }

    fn create_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeMapError>{
        self.insert_node(path, Node::new())
    }

    fn take_node(&mut self, name: impl Borrow<str>) -> Result<Node<T>, NodeMapError> {
        match self.nodes.remove(name.borrow()) {
            Some(node) => Ok(node),
            None => Err(NodeMapError::NoDir(name.borrow().to_owned())),
        }
    }

    fn remove_node(&mut self, path: impl Borrow<str>) -> Result<Node<T>, NodeMapError> {
        match path.borrow().split_once('/') {
            None => self.take_node(path),
            Some((branch, remaining_path)) => match self.borrow_node_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.remove_node(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    fn obtain_node(&self, name: impl Borrow<str>) -> Result<&Node<T>, NodeMapError> {
        if !name.borrow().is_empty() {
            if name.borrow() == "." { return Ok(self) }
            match self.nodes.get(name.borrow()) {
                Some(node) => Ok(node),
                None => Err(NodeMapError::NoDir(name.borrow().to_owned())),
            }
        } else {
            Err(NodeMapError::InvalidPath(name.borrow().to_owned()))
        }
    }

    fn obtain_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut Node<T>, NodeMapError> {
        if !name.borrow().is_empty() {
            if name.borrow() == "." { return Ok(self) }
            match self.nodes.get_mut(name.borrow()) {
                Some(node) => Ok(node),
                None => Err(NodeMapError::NoDir(name.borrow().to_owned())),
            }
        } else {
            Err(NodeMapError::InvalidPath(name.borrow().to_owned()))
        }
    }
  
    fn borrow_node(&self, path: impl Borrow<str>) -> Result<&Node<T>, NodeMapError> {
        match path.borrow().split_once('/') {
            None => self.obtain_node(path),
            Some((branch, remaining_path)) => match self.obtain_node(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_node(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    fn borrow_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut Node<T>, NodeMapError> {
        match path.borrow().split_once('/') {
            None => self.obtain_node_mut(path),
            Some((branch, remaining_path)) => match self.obtain_node_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_node_mut(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    fn merge(&mut self, node: impl Into<Node<T>>) -> Result<(), NodeMapError> {
        let node = node.into();

        if let Some(_) = node.data {
            return Err(NodeMapError::FileConflict);
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

    fn tree(&self) -> String {
        let text = String::new();
        format!(
            "> {}{}",
            self.name.purple().bold().underline(),
            self.cascade_tree(text, 0, "")
        )
    }

    fn tree_dir(&self) -> String {
        let text = String::new();
        format!(
            "> {}{}",
            self.name.purple().bold().underline(),
            self.cascade_tree(text, 0, "no-dir")
        )
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_depth(&self) -> f32 {
        self.depth
    }

    fn get_path(&self) -> &String {
        &self.path
    }
}
impl <T> DirFile<T> for Node<T> {
    fn add_data(&mut self, data: T) -> Option<T>{
        core::mem::replace(&mut self.data, Some(data))
    }

    fn insert_data(&mut self, path: impl Borrow<str>, data: T) -> Result<Option<T>, NodeMapError>{
        if path.borrow().is_empty() {
            Ok(self.add_data(data))
        } else {
            match self.borrow_node_mut(path) {
                Ok(borrowed_directory) => Ok(borrowed_directory.add_data(data)),
                Err(e) => Err(e),
            }
        }
    }

    fn take_data(&mut self) -> Option<T> {
        core::mem::replace(&mut self.data, None)
    }

    fn remove_data(&mut self, path: impl Borrow<str>) -> Result<Option<T>, NodeMapError> {
        match path.borrow().split_once('/') {
            None => Ok(self.take_data()),
            Some((branch, remaining_path)) => match self.borrow_node_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.remove_data(remaining_path),
                Err(e) => Err(e),
            },
        }
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

    fn borrow_file(&self, path: impl Borrow<str>) -> Result<Option<&T> , NodeMapError> {
        match path.borrow().split_once('/') {
            None => Ok(self.obtain_data()),
            Some((branch, remaining_path)) => match self.obtain_node(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_file(remaining_path),
                Err(e) => Err(e),
            },
        }
    }
    
    fn borrow_file_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T> , NodeMapError> {
        match path.borrow().split_once('/') {
            None => Ok(self.obtain_data_mut()),
            Some((branch, remaining_path)) => match self.obtain_node_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_file_mut(remaining_path),
                Err(e) => Err(e),
            },
        }
    }
}
