use crate::import::*;
use crate::NodeSize;

use super::{Align, DivSize, StackOrientation};



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

//pub struct Break; // Just div that skips to new grid line
//pub struct List; //Ver or Hor
//pub struct Grid; //Ver or Hor