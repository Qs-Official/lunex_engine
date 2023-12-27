use thiserror::Error;

/// ## Node map error
/// Error type indicating something went wrong.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum NodeTreeError {
    /// Error that happens when merging nodes. The node being merged must not contain data. Process the data before merging.
    #[error("Data from merging node was not processed/dropped before merging")]
    DataConflict,

    /// Error that happens when merging nodes. Two subnodes share the same name thus cannot be merged.
    #[error("Duplicate name conflict for '{0:}' when trying to merge nodes")]
    DuplicateName (String),

    /// Error that happens when attempting to create a node with a name that is already in use.
    #[error("Name '{0:}' is already in use")]
    NameInUse (String),

    /// Error that happens when the path you provided is not allowed.
    #[error("Path '{0:}' is not allowed")]
    InvalidPath (String),

    /// Error that happens when you try to locate a node that doesn't exist.
    #[error("Unable to locate '{0:}' node")]
    NoNode (String),
}