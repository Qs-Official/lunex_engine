pub mod prelude {
    pub use super::dec;
}

pub use declarative as dec;









/// # Align
/// Type used for aligning items in parametric containers.
/// * _Range_ : `-1.0 for START to 1.0 for CENTER`
/// * use `Align::START`, `Align::CENTER`, `Align::END`
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Align (pub f32);
impl Align {
    pub const START: Align = Align(-1.0);
    pub const LEFT: Align = Align(-1.0);
    pub const CENTER: Align = Align(0.0);
    pub const MIDDLE: Align = Align(0.0);
    pub const END: Align = Align(1.0);
    pub const RIGHT: Align = Align(1.0);
}


pub enum Container {
    Window,
    Solid,
    //Window3D
    //etc..
    // Modifiers: {} // Here pub general modifiers
}


/// # Declarative
/// Contains declarative type of containers
pub mod declarative {
    use bevy::prelude::*;
    use lunex_core::prelude::*;
    use crate::Align;

    #[derive(Debug, Default, Clone, Copy, PartialEq)]
    pub struct Window {
        pub pos : Amount<Vec2>,
        pub size: Amount<Vec2>,
    }
    impl Window {
        /// # Full Window
        /// Covers 100% of the parenting container
        pub const FULL: Window = Window { pos : Amount::Rt(Vec2::ZERO), size: Amount::Rt(Vec2::splat(100.0)) };
        /// # New
        /// Creates new Window container
        pub fn new() -> Self {
            Window {
                pos : Vec2::ZERO.to_rt(),
                size: Vec2::ZERO.to_rt(),
            }
        }
        /// # With Pos
        /// Modifies the container with the new position
        pub fn with_pos(mut self, pos: Amount<Vec2>) -> Self {
            self.pos = pos;
            self
        }
        /// # With X
        /// Modifies the container with the new x
        pub fn with_x(self, width: Amount<f32>) -> Self {
            self.pos.set_x(width);
            self
        }
        /// # With Y
        /// Modifies the container with the new y
        pub fn with_y(self, height: Amount<f32>) -> Self {
            self.pos.set_y(height);
            self
        }
        /// # With Size
        /// Modifies the container with the new size
        pub fn with_size(mut self, size: Amount<Vec2>) -> Self {
            self.size = size;
            self
        }
        /// # With Width
        /// Modifies the container with the new width
        pub fn with_width(self, width: Amount<f32>) -> Self {
            self.size.set_x(width);
            self
        }
        /// # With Height
        /// Modifies the container with the new height
        pub fn with_height(self, height: Amount<f32>) -> Self {
            self.size.set_y(height);
            self
        }
    }

    #[derive(Debug, Default, Clone, Copy, PartialEq)]
    pub struct Solid {
        pub size: Amount<Vec2>,
        pub align_x: Align,
        pub align_y: Align
    }
    impl Solid {
        /// # New
        /// Creates new Solid container
        pub fn new() -> Self {
            Solid {
                size: Vec2::ONE.to_rt(),
                align_x: Align::CENTER,
                align_y: Align::CENTER,
            }
        }
        /// # With Size
        /// Modifies the container with the new size
        pub fn with_size(mut self, size: Amount<Vec2>) -> Self {
            self.size = size;
            self
        }
        /// # With Width
        /// Modifies the container with the new width
        pub fn with_width(self, width: Amount<f32>) -> Self {
            self.size.set_x(width);
            self
        }
        /// # With Height
        /// Modifies the container with the new height
        pub fn with_height(self, height: Amount<f32>) -> Self {
            self.size.set_y(height);
            self
        }
        /// # With Align X
        /// Modifies the container with the new alignment on X axis
        pub fn with_align_x(mut self, align: Align) -> Self {
            self.align_x = align;
            self
        }
        /// # With Align Y
        /// Modifies the container with the new alignment on Y axis
        pub fn with_align_y(mut self, align: Align) -> Self {
            self.align_y = align;
            self
        }
    }

    pub struct Relative;
}





pub mod parametric {
    pub struct Div; // Most basic type, basically every div is List

    pub struct Br; // Just div with new line
    pub struct List; //Ver or Hor (Flex) or Chain?
    pub struct Grid;    // (Grid)
}