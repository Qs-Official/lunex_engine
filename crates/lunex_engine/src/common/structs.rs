use crate::NiceDisplay;
use crate::import::*;
use crate::NodeError;


// #==================#
// #=== ERROR TYPE ===#

/// ## Lunex error
/// Error type indicating something went wrong.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum UiError {
    /// Error that occurs when something went wrong with NodeTree.
    #[error("Something went wrong with NodeTree")]
    NodeError(NodeError),
}
impl From<NodeError> for UiError {
    fn from(value: NodeError) -> Self {
        UiError::NodeError(value)
    }
}



/// ## Rectangle 2D
/// A struct for holding a 2D rectangle data.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rect2D {
    pub pos : Vec2,
    pub size: Vec2,
}
impl Rect2D {
    /// ## Empty Rect2D
    /// A new empty Rect2D. Has `0` size. 
    pub const EMPTY: Rect2D = Rect2D { pos : Vec2::ZERO, size: Vec2::ZERO };
    /// ## New
    /// Creates new empty Window layout.
    pub const fn new() -> Self {
        Rect2D::EMPTY
    }
    /// ## With pos
    /// Replaces the position with the new value.
    pub fn with_pos(mut self, pos: impl Into<Vec2>) -> Self {
        self.pos = pos.into();
        self
    }
    /// ## With x
    /// Replaces the x position with the new value.
    pub fn with_x(mut self, width: f32) -> Self {
        self.pos.x = width;
        self
    }
    /// ## With y
    /// Replaces the y position with the new value.
    pub fn with_y(mut self, height: f32) -> Self {
        self.pos.y = height;
        self
    }
    /// ## With size
    /// Replaces the size with the new value.
    pub fn with_size(mut self, size: impl Into<Vec2>) -> Self {
        self.size = size.into();
        self
    }
    /// ## With width
    /// Replaces the width with the new value.
    pub fn with_width(mut self, width: f32) -> Self {
        self.size.x = width;
        self
    }
    /// ## With height
    /// Replaces the height with the new value.
    pub fn with_height(mut self, height: f32) -> Self {
        self.size.y = height;
        self
    }    
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


/// ## Rectangle 3D
/// A struct for holding a 3D rectangle data.
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
impl NiceDisplay for Rect3D {
    fn to_nicestr(&self) -> String {
        let text = format!("[pos: {} size: {}]", self.pos.to_string(), self.size.to_string());
        format!("{} {}", "Rect3D".bright_magenta(), text.black())
    }
}