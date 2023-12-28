use bevy::ecs::component::Component;
use crate::import::*;

use crate::NodeTreeError;


// #==================#
// #=== ERROR TYPE ===#

/// ## NodeTree error
/// Error type indicating something went wrong.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum LunexError {
    /// Error that occurs when something went wrong with NodeTree.
    #[error("Something went wrong with NodeTree")]
    NodeTreeError(NodeTreeError),
}
impl From<NodeTreeError> for LunexError {
    fn from(value: NodeTreeError) -> Self {
        LunexError::NodeTreeError(value)
    }
}




/// ## Node link
/// A component that points to a specific node.
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct NodeLink {
    pub path: String,
}






/// ## Rectangle 3D
/// A struct for holding a 3D rectangle data.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rect2D {
    pub pos : Vec2,
    pub size: Vec2,
}
impl Into<Rect3D> for Rect2D {
    fn into(self) -> Rect3D {
        Rect3D {
            pos: self.pos.extend(0.0),
            size: self.size,
            ..Default::default()
        }
    }
}

/// ## Rectangle 2D
/// A struct for holding a 2D rectangle data.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rect3D {
    pub pos : Vec3,
    pub size: Vec2,
    pub roll: f32,
    pub yaw : f32,
    pub tilt: f32,
}
impl Into<Rect2D> for Rect3D {
    fn into(self) -> Rect2D {
        Rect2D {
            pos: self.pos.truncate(),
            size: self.size,
        }
    }
}