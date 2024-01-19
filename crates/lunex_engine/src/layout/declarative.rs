use crate::import::*;
use crate::{NiceDisplay, Align, Cover, Rect2D, NodeSize, NodeSizeEvaluate, Abs};
#[cfg(feature = "bevy")]
use bevy::prelude::Component;

use super::Layout;

/// ## Window Layout
#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Window {
    /// ## Position
    /// Position of the top-left corner.
    pub pos : NodeSize<Vec2>,
    /// ## Size
    /// Size of the layout.
    pub size: NodeSize<Vec2>,
}
impl Window {
    /// ## Full Window
    /// Covers `100%` of the parent layout.
    pub const FULL: Window = Window { pos : NodeSize::new(), size: NodeSize::from_prc(Vec2::splat(100.0)) };
    /// ## Empty Window
    /// A new empty Window. Has `None` size. 
    pub const EMPTY: Window = Window { pos : NodeSize::new(), size: NodeSize::new() };
    /// ## New
    /// Creates new empty Window layout.
    pub const fn new() -> Self {
        Window {
            pos : NodeSize::new(),
            size: NodeSize::from_prc(Vec2::splat(100.0)),
        }
    }
    /// ## With pos
    /// Replaces the position with the new value.
    pub fn with_pos(mut self, pos: impl Into<NodeSize<Vec2>>) -> Self {
        self.pos = pos.into();
        self
    }
    /// ## With x
    /// Replaces the x position with the new value.
    pub fn with_x(mut self, width: impl Into<NodeSize<f32>>) -> Self {
        self.pos.set_x(width);
        self
    }
    /// ## With y
    /// Replaces the y position with the new value.
    pub fn with_y(mut self, height: impl Into<NodeSize<f32>>) -> Self {
        self.pos.set_y(height);
        self
    }
    /// ## With size
    /// Replaces the size with the new value.
    pub fn with_size(mut self, size: impl Into<NodeSize<Vec2>>) -> Self {
        self.size = size.into();
        self
    }
    /// ## With width
    /// Replaces the width with the new value.
    pub fn with_width(mut self, width: impl Into<NodeSize<f32>>) -> Self {
        self.size.set_x(width);
        self
    }
    /// ## With height
    /// Replaces the height with the new value.
    pub fn with_height(mut self, height: impl Into<NodeSize<f32>>) -> Self {
        self.size.set_y(height);
        self
    }
    /// ## Compute
    /// Computes the layout based on given parameters.
    pub fn compute(&self, parent: Rect2D, font_size: f32) -> Rect2D {
        Rect2D {
            pos: self.pos.evaluate(parent.size, font_size),
            size: self.size.evaluate(parent.size, font_size),
        }
    }
}
impl Into<Layout> for Window {
    fn into(self) -> Layout {
        Layout::Window(self)
    }
}
impl NiceDisplay for Window {
    fn to_nicestr(&self) -> String {
        let t = format!("[pos: ({}) size: ({})]", self.pos.to_nicestr(), self.size.to_nicestr());
        format!("{}", t.black())
    }
}

/// ## Solid Layout
#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Solid {
    /// ## Size
    /// Aspect ratio of the sides of the rectangular layout. `1:1 == 10:10 == 100:100`.
    pub size: NodeSize<Vec2>,
    /// ## Align X
    /// Horizontal alignment within parent.
    pub align_x: Align,
    /// ## Align Y
    /// Vertical alignment within parent.
    pub align_y: Align,
    /// ## Cover
    /// Specifies container scaling.
    pub cover: Cover,
}
impl Solid {
    /// ## New
    /// Creates new Solid layout.
    pub fn new() -> Self {
        Solid {
            size: Abs(Vec2::ONE).into(),
            align_x: Align::CENTER,
            align_y: Align::CENTER,
            cover: Cover::Contain,
        }
    }
    /// ## With size
    /// Replaces the size with the new value.
    pub fn with_size(mut self, size: impl Into<NodeSize<Vec2>>) -> Self {
        self.size = size.into();
        self
    }
    /// ## With width
    /// Replaces the width with the new value.
    pub fn with_width(mut self, width: impl Into<NodeSize<f32>>) -> Self {
        self.size.set_x(width);
        self
    }
    /// ## With height
    /// Replaces the height with the new value.
    pub fn with_height(mut self, height: impl Into<NodeSize<f32>>) -> Self {
        self.size.set_y(height);
        self
    }
    /// ## With align x
    /// Replaces the x alignment with the new value.
    pub fn with_align_x(mut self, align: Align) -> Self {
        self.align_x = align;
        self
    }
    /// ## With align y
    /// Replaces the y alignment with the new value.
    pub fn with_align_y(mut self, align: Align) -> Self {
        self.align_y = align;
        self
    }
    /// ## With cover
    /// Replaces both x & y cover values with the new value.
    pub fn with_cover(mut self, cover: Cover) -> Self {
        self.cover = cover;
        self
    }
    /// ## Compute
    /// Computes the layout based on given parameters.
    pub fn compute(&self, parent: Rect2D, font_size: f32) -> Rect2D {
        
        let size = self.size.evaluate(parent.size, font_size);

        let scale = match self.cover {
            Cover::Horizontal => parent.size.x / size.x,
            Cover::Vertical => parent.size.y / size.y,
            Cover::Contain => f32::min(parent.size.x / size.x, parent.size.y / size.y),
            Cover::Full => f32::max(parent.size.x / size.x, parent.size.y / size.y),
        };

        let center_point = Vec2::new(parent.pos.x + parent.size.x / 2.0, parent.pos.y + parent.size.y / 2.0);

        let computed_width = size.x * scale;
        let computed_height = size.y * scale;
        let computed_point = Vec2::new(center_point.x - computed_width / 2.0, center_point.y - computed_height / 2.0);

        Rect2D {
            pos: Vec2::new(
                computed_point.x + (computed_point.x - parent.pos.x) * self.align_x.0,
                computed_point.y + (computed_point.y - parent.pos.y) * self.align_y.0,
            ),
            size: (computed_width, computed_height).into(),
        }
    }
}
impl Into<Layout> for Solid {
    fn into(self) -> Layout {
        Layout::Solid(self)
    }
}
impl NiceDisplay for Solid {
    fn to_nicestr(&self) -> String {
        let t = format!("[size: ({}) align_x: {} align_y: {}]", self.size.to_nicestr(), self.align_x.to_nicestr(), self.align_y.to_nicestr());
        format!("{}", t.black())
    }
}
