use lunex_core::Amount;

pub mod prelude {
    pub use super::lui;
}

/// # Lunex UI
/// Contains all containers available in
/// ### Declarative
/// * [Window]
/// * [Solid]
/// ### Parametric
/// * [Div]
/// * [List]
/// * [Grid]
pub mod lui {
    pub use crate::declarative::*;
    pub use crate::parametric::*;
}









/// # Align
/// Type used for aligning items in parametric containers.
/// 
/// _Range_ : `-1.0 for START to 1.0 for END`
/// * `Align::START`
/// * `Align::CENTER`
/// * `Align::END`
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


pub struct Container {
    layout: Layout,

    depth: f32,

    roll: f32,
    yaw: f32,
    pitch: f32
}


pub enum Layout {
    Window(declarative::Window),
    Solid(declarative::Solid),
    //Window3D
    //Div
    //Br
}


pub struct Size(Amount<f32>);
impl Size {
    /// ## Extra-small
    pub const XS: Size = Size(Amount::Rem(8.0));
    /// ## Small
    pub const SM: Size = Size(Amount::Rem(8.0));
    /// ## Medium
    pub const MD: Size = Size(Amount::Rem(8.0));
    /// ## Large
    pub const LG: Size = Size(Amount::Rem(8.0));
    /// ## Extra-large
    pub const XL: Size = Size(Amount::Rem(8.0));
    /// ## Extra-large 2
    pub const XL2: Size = Size(Amount::Rem(8.0));
    /// ## Extra-large 3
    pub const XL3: Size = Size(Amount::Rem(8.0));
    /// ## Extra-large 4
    pub const XL4: Size = Size(Amount::Rem(8.0));
    /// ## Extra-large 5
    pub const XL5: Size = Size(Amount::Rem(8.0));
    /// ## Extra-large 6
    pub const XL6: Size = Size(Amount::Rem(8.0));
    /// ## Extra-large 7
    pub const XL7: Size = Size(Amount::Rem(8.0));
}


pub enum Sizing2 {
    Min,
    Max,
}

pub enum Sizing {
    Full,
    Exact(Size),
    //Custom(Amount<Vec2>)
    //Custom(Rem)
}

/// # Declarative Layouts
/// Contains declarative type of containers.
/// You define their exact position. They don't rely on context.
/// They are the primitives of this library.
/// * [Window]
/// * [Solid]
pub mod declarative {
    use bevy::prelude::*;
    use lunex_core::prelude::*;
    use crate::Align;
    use crate::Layout;

    /// # Window Layout
    #[derive(Debug, Default, Clone, Copy, PartialEq)]
    pub struct Window {
        pub pos : Amount<Vec2>,
        pub size: Amount<Vec2>,
    }
    impl Window {
        /// # Full Window
        /// Covers 100% of the parenting container
        pub const FULL: Window = Window { pos : Amount::Prc(Vec2::ZERO), size: Amount::Prc(Vec2::splat(100.0)) };
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
    impl Into<Layout> for Window {
        fn into(self) -> Layout {
            Layout::Window(self)
        }
    }

    /// # Solid Layout
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
    impl Into<Layout> for Solid {
        fn into(self) -> Layout {
            Layout::Solid(self)
        }
    }
}

/// # Parametric Layouts
/// Contains parametric type of containers.
/// You define how they behave based on their neighboring containers.
/// They rely on context. They work similarly as HTML (padding, margin, etc.)
/// * [Div]
/// * [List]
/// * [Grid]
pub mod parametric {
    use bevy::math::Vec4;
    //use crate::Align;
    use crate::{Sizing, Size};

    pub struct Div { // Most basic type, basically every div is List
        //pub font_align: Align, //Maybe move to theming as components? Typography??? - MUST BE IN MASTER STRUCT BCS ITS GENERAL TO ALL CONTAINERS
        pub width: Sizing,
        pub max_width: Option<Size>,
        pub height: Sizing,
        pub max_height: Option<Size>,
        pub padding: Vec4,
        pub margin: Vec4,
    }

    pub struct Br; // Just div with new line class
    pub struct List; //Ver or Hor (Flex) or Chain?
    pub struct Grid;    // (Grid)
}