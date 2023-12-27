mod size;
mod traits;

pub use size::*;
pub use traits::*;

// #======================#
// #=== PRELUDE EXPORT ===#

pub mod prelude {
    pub use super::{Abs, Prc, Rem};
    pub use super::NodeSize;
    pub use super::Size;

    pub use super::{Rect2D, Rect3D};
    pub use super::NiceDisplay;
}


// #================================#
// #=== COMMON TYPES DECLARATION ===#

#[cfg(feature = "bevy")]
use bevy::ecs::component::Component;
use glam::f32::{Vec2, Vec3};


/// ## Node link
/// A component that points to a specific node.
#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Debug, Default, Clone, PartialEq)]
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