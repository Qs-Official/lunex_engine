use bevy::utils::thiserror::Error;

/// ## Node map error
/// Error type indicating something went wrong.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum NodeMapError {
    /// Error that happens when merging directories. The directory being merged can contain only one file. Drop the other file before merging.
    #[error("File from merging directory was not dropped before merging")]
    FileConflict,

    /// Error that happens when merging directories. Two directories/files have the same name.
    #[error("Duplicate name conflict for '{0:}' when trying to merge directory")]
    DuplicateName (String),

    /// Error that happens when attempting to create a directory/file with a name that is already in use.
    #[error("Name '{0:}' is already in use")]
    NameInUse (String),

    /// Error that happens when path provided is not allowed.
    #[error("Path '{0:}' is not allowed")]
    InvalidPath (String),

    /// Error that happens when you try to locate a directory that doesn't exist.
    #[error("Unable to locate '{0:}' directory")]
    NoDir (String),

    /// Error that happens when you try to locate a file that doesn't exist.
    #[error("Unable to locate '{0:}' file")]
    NoFile (String),
}