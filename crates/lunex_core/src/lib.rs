mod size;
use bevy::math::{Vec2, Vec3};
pub use size::*;
pub use lunex_nodemap::NiceDisplay;


//use colored::Colorize;
//use lunex_layout::Layout;

pub mod prelude {
    pub use super::{Abs, Prc, Rem};
    pub use super::NodeSize;
    pub use super::Size;

    pub use super::{Rect2D, Rect3D};
    pub use super::NiceDisplay;
}









#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rect2D {
    pub pos   : Vec2,
    pub size  : Vec2,
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

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rect3D {
    pub pos   : Vec3,
    pub size  : Vec2,
    pub roll: f32,
    pub yaw : f32,
    pub tilt: f32,
}