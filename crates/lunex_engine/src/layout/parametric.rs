use crate::import::*;

use crate::{NiceDisplay, Align, Layout, NodeSize, DivSize, StackOrientation};



// I should be able to recreate Solid functionality with Div
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Div { // Most basic type, basically every div is List


    pub width: DivSize<f32>,
    pub min_width: Option<NodeSize<f32>>,
    pub max_width: Option<NodeSize<f32>>,

    /// Overwrite dictated alignement from parent's stack
    pub align_x: Option<Align>,


    pub height: DivSize<f32>,
    pub min_height: Option<NodeSize<f32>>,
    pub max_height: Option<NodeSize<f32>>,
    pub align_y: Option<Align>,

    /// Padding distancing border from context
    pub padding: NodeSize<Vec4>,
    /// Padding used for rendering. 0 by default
    pub border: NodeSize<Vec4>,
    /// Padding distancing other divs from border
    pub margin: NodeSize<Vec4>,

    /// If this div breaks the stack?
    /// Questionable if it shouldn't be a stack limit instead
    pub breaks: bool,


    // Item positioning

    // pub content : Stack || Grid

    pub stack_orientation: StackOrientation,
    pub stack_gap: NodeSize<f32>,

    pub item_flipped: bool,
    pub item_next_gap: NodeSize<f32>,
    pub item_line_gap: NodeSize<f32>,



    //Content GRID || LIST
}
impl Div {
    pub fn new() -> Self {
        Default::default()
    }

    /// ## With padding
    /// Replaces the padding with the new value.
    pub fn pad(mut self, pad: impl Into<NodeSize<Vec4>>) -> Self {
        self.padding = pad.into();
        self
    }

    /// ## With padding horizontal
    /// Replaces the horizontal padding with the new value.
    pub fn pad_x(mut self, pad: impl Into<NodeSize<Vec2>>) -> Self {
        let pad: NodeSize<Vec2> = pad.into();
        let val = pad.get_x();
        self.padding.set_x(val);
        self.padding.set_z(val);
        self
    }

    /// ## With padding vertical
    /// Replaces the vertical padding with the new value.
    pub fn pad_y(mut self, pad: impl Into<NodeSize<f32>>) -> Self {
        let pad: NodeSize<f32> = pad.into();
        self.padding.set_y(pad);
        self.padding.set_w(pad);
        self
    }

    /// ## With padding right
    /// Replaces the right padding with the new value.
    pub fn pad_r(mut self, pad: impl Into<NodeSize<f32>>) -> Self {
        self.padding.set_x(pad);
        self
    }

    /// ## With padding top
    /// Replaces the top padding with the new value.
    pub fn pad_t(mut self, pad: impl Into<NodeSize<f32>>) -> Self {
        self.padding.set_y(pad);
        self
    }

    /// ## With padding left
    /// Replaces the left padding with the new value.
    pub fn pad_l(mut self, pad: impl Into<NodeSize<f32>>) -> Self {
        self.padding.set_z(pad);
        self
    }

    /// ## With padding bottom
    /// Replaces the bottom padding with the new value.
    pub fn pad_b(mut self, pad: impl Into<NodeSize<f32>>) -> Self {
        self.padding.set_w(pad);
        self
    }

    /// ## Pack
    /// Packs the struct into Layout
    pub fn pack(self) -> Layout {
        self.into()
    }
}
impl Into<Layout> for Div {
    fn into(self) -> Layout {
        Layout::Div(self)
    }
}
impl NiceDisplay for Div {
    fn to_nicestr(&self) -> String {
        let t = format!("[pad: ({})]", self.padding.to_nicestr());
        format!("{}", t.black())
    }
}


//pub struct Break; // Just div that skips to new grid line
//pub struct List; //Ver or Hor
//pub struct Grid; //Ver or Hor